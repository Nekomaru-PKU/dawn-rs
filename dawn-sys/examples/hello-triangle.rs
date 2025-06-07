//! This program demonstrates how to set up WebGPU rendering on Windows
//! using the `dawn-sys` crate.

use dawn_sys::*;
use std::{
    cell::RefCell,
    ffi,
    mem,
    ptr,
    rc::Rc,
    thread,
    time::Instant,
};

// --- Main Application Entry Point ---

fn main() {
    use winsafe::prelude::*;
    use winsafe::co::WS;
    use winsafe::gui::{WindowMain, WindowMainOpts};

    // Create the main application window.
    let window = WindowMain::new(WindowMainOpts {
        title: "Hello dawn-rs".into(),
        size: (1280, 800),
        style: WindowMainOpts::default().style |
            WS::MAXIMIZEBOX |
            WS::MINIMIZEBOX |
            WS::SIZEBOX,
        ..Default::default()
    });

    // Use a RefCell to hold the App state, allowing it to be initialized
    // after the window is created and mutated during the render loop.
    let app = Rc::new(RefCell::new(None));

    // Register a handler for the WM_CREATE event. This is where we will
    // initialize our WebGPU application logic.
    window.on().wm_create({
        let window = window.clone();
        let app = app.clone();
        move |_| {
            // Create the App, passing the necessary window handles.
            app.borrow_mut().replace(App::new(
                window.hwnd().hinstance().ptr(),
                window.hwnd().ptr()));
            Ok(0)
        }
    });

    // Register a handler for the WM_DESTROY event to clean up resources.
    window.on().wm_destroy({
        let app = app.clone();
        move || {
            // Drop the App to clean up resources.
            app.borrow_mut().take();
            Ok(())
        }
    });

    // Register a handler for the WM_PAINT event to perform rendering.
    window.on().wm_paint({
        let app = app.clone();
        let window = window.clone();
        move || {
            // On WM_PAINT, we can trigger a render if the app is initialized.
            if let Some(app) = &mut *app.borrow_mut() {
                // Get the current client area size.
                let rect = window.hwnd().GetClientRect()?;
                app.render(rect.right - rect.left, rect.bottom - rect.top);

                // Sleep for a short duration to avoid rendering too fast.
                thread::sleep(std::time::Duration::from_millis(16));

                // Request another paint to keep the rendering loop going.
                window.hwnd().InvalidateRect(None, false)?;
            }
            Ok(())
        }
    });

    // Run the main window loop.
    window.run_main(None).expect("failed to run main window");
}

// --- Helper for WGPUStringView ---

/// Extension trait to make `WGPUStringView` easier to work with.
trait WGPUStringViewExt {
    fn to_string_lossy(&self) -> String;
}

impl WGPUStringViewExt for WGPUStringView {
    /// Converts a WGPUStringView to a Rust String, replacing invalid UTF-8 sequences.
    fn to_string_lossy(&self) -> String {
        if self.data.is_null() {
            return String::new();
        }
        unsafe {
            let slice = std::slice::from_raw_parts(self.data as *const u8, self.length);
            String::from_utf8_lossy(slice).into_owned()
        }
    }
}

// --- WebGPU Callback Functions ---

/// Callback for uncaptured WebGPU errors. Panics with the error message.
unsafe extern "C" fn on_device_error(
    _device: *const WGPUDevice,
    _ty: WGPUErrorType,
    message: WGPUStringView,
    _userdata1: *mut ffi::c_void,
    _userdata2: *mut ffi::c_void) {
    panic!("Device Error: {}", message.to_string_lossy());
}

/// Callback for the adapter request. Stores the resulting adapter in the provided user data.
unsafe extern "C" fn on_request_adapter(
    status: WGPURequestAdapterStatus,
    adapter: WGPUAdapter,
    message: WGPUStringView,
    userdata1: *mut ffi::c_void,
    _userdata2: *mut ffi::c_void) {
    if status == WGPURequestAdapterStatus::Success {
        unsafe { userdata1.cast::<WGPUAdapter>().write(adapter) };
    } else {
        panic!("Failed to get adapter: {}", message.to_string_lossy());
    }
}

/// Callback for the device request. Stores the resulting device in the provided user data.
unsafe extern "C" fn on_request_device(
    status: WGPURequestDeviceStatus,
    device: WGPUDevice,
    message: WGPUStringView,
    userdata1: *mut ffi::c_void,
    _userdata2: *mut ffi::c_void) {
    if status == WGPURequestDeviceStatus::Success {
        unsafe { userdata1.cast::<WGPUDevice>().write(device) };
    } else {
        panic!("Failed to get device: {}", message.to_string_lossy());
    }
}

// --- Data Structures ---

/// Represents a single vertex with position, UV coordinates, and color.
#[repr(C)]
struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 3],
}

/// The main application struct, holding all WebGPU resources.
struct App {
    // Core WebGPU objects
    instance: WGPUInstance,
    surface: WGPUSurface,
    adapter: WGPUAdapter,
    device: WGPUDevice,
    queue: WGPUQueue,

    // Rendering resources
    vertex_buffer: WGPUBuffer,
    uniform_buffer: WGPUBuffer,
    pipeline: WGPURenderPipeline,
    bind_group: WGPUBindGroup,

    // Swap chain / surface state
    swap_chain_configured: bool,
    current_width: i32,
    current_height: i32,

    // Timing
    start_time: Instant,
}

// --- Application Implementation ---

impl App {
    // --- Constants ---
    const K_VERTEX_STRIDE: u64 = mem::size_of::<Vertex>() as u64;
    const K_VERTEX_DATA: [Vertex; 3] = [
        Vertex {
            position: [-0.00, 0.75],
            uv: [25.0, 50.0],
            color: [1.0, 0.0, 0.0],
        },
        Vertex {
            position: [0.75, -0.50],
            uv: [0.0, 0.0],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [-0.75, -0.50],
            uv: [50.0, 0.0],
            color: [0.0, 0.0, 1.0],
        },
    ];
    const K_TEXTURE_WIDTH: u32 = 2;
    const K_TEXTURE_HEIGHT: u32 = 2;
    const K_SWAP_CHAIN_FORMAT: WGPUTextureFormat = WGPUTextureFormat::BGRA8Unorm;

    // --- WGSL Shader Source ---
    const WGSL_SHADER: &'static str = r"
        struct VertexIn {
            @location(0) pos: vec2f,
            @location(1) uv: vec2f,
            @location(2) color: vec3f
        }

        struct VertexOut {
            @builtin(position) pos: vec4f,
            @location(0)       uv: vec2f,
            @location(1)       color: vec3f
        }

        @group(0) @binding(0) var<uniform> g_transform: mat4x4f;
        @group(0) @binding(1) var g_texture: texture_2d<f32>;
        @group(0) @binding(2) var g_sampler: sampler;

        @vertex
        fn vs(in: VertexIn) -> VertexOut {
            var out: VertexOut;
            out.pos = g_transform * vec4f(in.pos, 0.0, 1.0);
            out.uv = in.uv;
            out.color = in.color;
            return out;
        }

        @fragment
        fn fs(in: VertexOut) -> @location(0) vec4f {
            let tex: vec4f = textureSample(g_texture, g_sampler, in.uv);
            return vec4f(in.color, 1.0) * tex;
        }
    ";

    /// Creates a new `App` instance, initializing all WebGPU resources.
    fn new(hinstance: *mut ffi::c_void, hwnd: *mut ffi::c_void) -> Self {
        unsafe {
            // --- Instance, Surface, Adapter, Device ---
            // This setup mirrors the logic from the C example and hello-dawn.rs

            println!("Creating WebGPU instance...");
            let instance = wgpuCreateInstance(ptr::null());
            assert!(!instance.is_null());

            println!("Creating WebGPU surface...");
            let mut surface_desc_0 = WGPUSurfaceSourceWindowsHWND {
                chain: WGPUChainedStruct {
                    sType: WGPUSType::SurfaceSourceWindowsHWND,
                    next: ptr::null_mut(),
                },
                hinstance,
                hwnd,
            };
            let surface_desc = WGPUSurfaceDescriptor {
                nextInChain: &mut surface_desc_0.chain,
                label: std::mem::zeroed(),
            };
            let surface = wgpuInstanceCreateSurface(instance, &surface_desc);
            assert!(!surface.is_null());

            println!("Requesting adapter...");
            let mut adapter: WGPUAdapter = ptr::null_mut();
            let adapter_options = WGPURequestAdapterOptions {
                compatibleSurface: surface,
                ..mem::zeroed()
            };
            let adapter_callback_info = WGPURequestAdapterCallbackInfo {
                mode: WGPUCallbackMode::WaitAnyOnly,
                callback: Some(on_request_adapter),
                userdata1: &mut adapter as *mut _ as *mut ffi::c_void,
                ..mem::zeroed()
            };
            let future =
                wgpuInstanceRequestAdapter(instance, &adapter_options, adapter_callback_info);
            let mut wait_info = WGPUFutureWaitInfo {
                future,
                completed: WGPU_FALSE,
            };
            let status = wgpuInstanceWaitAny(instance, 1, &mut wait_info, 0);
            assert!(status == WGPUWaitStatus::Success);
            assert!(!adapter.is_null());

            let mut adapter_info = mem::zeroed();
            wgpuAdapterGetInfo(adapter, &mut adapter_info);
            println!(
                "Got adapter: {}",
                adapter_info.device.to_string_lossy()
            );

            println!("Requesting device...");
            let mut device: WGPUDevice = ptr::null_mut();
            let device_desc = WGPUDeviceDescriptor {
                uncapturedErrorCallbackInfo: WGPUUncapturedErrorCallbackInfo {
                    callback: Some(on_device_error),
                    ..mem::zeroed()
                },
                ..mem::zeroed()
            };
            let device_callback = WGPURequestDeviceCallbackInfo {
                mode: WGPUCallbackMode::WaitAnyOnly,
                callback: Some(on_request_device),
                userdata1: &mut device as *mut _ as *mut ffi::c_void,
                ..mem::zeroed()
            };
            let future = wgpuAdapterRequestDevice(adapter, &device_desc, device_callback);
            let mut wait_info = WGPUFutureWaitInfo {
                future,
                completed: WGPU_FALSE,
            };
            let status = wgpuInstanceWaitAny(instance, 1, &mut wait_info, 0);
            assert!(status == WGPUWaitStatus::Success);
            assert!(!device.is_null());

            let queue = wgpuDeviceGetQueue(device);
            assert!(!queue.is_null());

            // --- Vertex Buffer ---
            println!("Creating vertex buffer...");
            let vertex_buffer_desc = WGPUBufferDescriptor {
                usage: WGPUBufferUsage::VERTEX | WGPUBufferUsage::COPY_DST,
                size: mem::size_of_val(&Self::K_VERTEX_DATA) as u64,
                ..mem::zeroed()
            };
            let vertex_buffer = wgpuDeviceCreateBuffer(device, &vertex_buffer_desc);
            wgpuQueueWriteBuffer(
                queue,
                vertex_buffer,
                0,
                Self::K_VERTEX_DATA.as_ptr() as *const ffi::c_void,
                mem::size_of_val(&Self::K_VERTEX_DATA));

            // --- Uniform Buffer ---
            println!("Creating uniform buffer...");
            let uniform_buffer_desc = WGPUBufferDescriptor {
                usage: WGPUBufferUsage::UNIFORM | WGPUBufferUsage::COPY_DST,
                size: (4 * 4 * mem::size_of::<f32>()) as u64, // 4x4 matrix
                ..mem::zeroed()
            };
            let uniform_buffer = wgpuDeviceCreateBuffer(device, &uniform_buffer_desc);

            // --- Texture and Sampler ---
            println!("Creating texture and sampler...");

            // Checkerboard texture
            let pixels: [u32; 4] = [0x80000000, 0xffffffff, 0xffffffff, 0x80000000];

            let texture_desc = WGPUTextureDescriptor {
                usage: WGPUTextureUsage::TEXTURE_BINDING | WGPUTextureUsage::COPY_DST,
                dimension: WGPUTextureDimension::D2,
                size: WGPUExtent3D {
                    width: Self::K_TEXTURE_WIDTH,
                    height: Self::K_TEXTURE_HEIGHT,
                    depthOrArrayLayers: 1,
                },
                format: WGPUTextureFormat::RGBA8Unorm,
                mipLevelCount: 1,
                sampleCount: 1,
                ..mem::zeroed()
            };
            let texture = wgpuDeviceCreateTexture(device, &texture_desc);

            let dst = WGPUTexelCopyTextureInfo {
                texture,
                mipLevel: 0,
                ..mem::zeroed()
            };
            let src_layout = WGPUTexelCopyBufferLayout {
                bytesPerRow: Self::K_TEXTURE_WIDTH * 4,
                rowsPerImage: Self::K_TEXTURE_HEIGHT,
                ..mem::zeroed()
            };
            wgpuQueueWriteTexture(
                queue,
                &dst,
                pixels.as_ptr() as *const ffi::c_void,
                mem::size_of_val(&pixels),
                &src_layout,
                &texture_desc.size);

            let view_desc = WGPUTextureViewDescriptor {
                format: texture_desc.format,
                dimension: WGPUTextureViewDimension::D2,
                mipLevelCount: 1,
                arrayLayerCount: 1,
                ..mem::zeroed()
            };
            let texture_view = wgpuTextureCreateView(texture, &view_desc);
            wgpuTextureRelease(texture);

            let sampler_desc = WGPUSamplerDescriptor {
                nextInChain: ptr::null_mut(),
                label: std::mem::zeroed(),
                addressModeU: WGPUAddressMode::Repeat,
                addressModeV: WGPUAddressMode::Repeat,
                addressModeW: WGPUAddressMode::Repeat,
                magFilter: WGPUFilterMode::Nearest,
                minFilter: WGPUFilterMode::Nearest,
                mipmapFilter: WGPUMipmapFilterMode::Nearest,
                lodMaxClamp: 1.0,
                lodMinClamp: 0.0,
                maxAnisotropy: 1,
                compare: WGPUCompareFunction::Undefined,
            };
            let sampler = wgpuDeviceCreateSampler(device, &sampler_desc);

            // --- Shader Module ---
            println!("Creating shader module...");
            let mut wgsl_source = WGPUShaderSourceWGSL {
                chain: WGPUChainedStruct {
                    sType: WGPUSType::ShaderSourceWGSL,
                    next: ptr::null_mut(),
                },
                code: WGPUStringView {
                    data: Self::WGSL_SHADER.as_ptr() as *const i8,
                    length: Self::WGSL_SHADER.len(),
                },
            };
            let shader_desc = WGPUShaderModuleDescriptor {
                nextInChain: &mut wgsl_source.chain,
                ..mem::zeroed()
            };
            let shaders = wgpuDeviceCreateShaderModule(device, &shader_desc);

            // --- Render Pipeline & Bind Group ---
            println!("Creating render pipeline...");

            let bind_group_layout_entries = [
                WGPUBindGroupLayoutEntry {
                    binding: 0,
                    visibility: WGPUShaderStage::VERTEX,
                    buffer: WGPUBufferBindingLayout {
                        type_: WGPUBufferBindingType::Uniform,
                        ..mem::zeroed()
                    },
                    ..mem::zeroed()
                },
                WGPUBindGroupLayoutEntry {
                    binding: 1,
                    visibility: WGPUShaderStage::FRAGMENT,
                    texture: WGPUTextureBindingLayout {
                        sampleType: WGPUTextureSampleType::Float,
                        viewDimension: WGPUTextureViewDimension::D2,
                        ..mem::zeroed()
                    },
                    ..mem::zeroed()
                },
                WGPUBindGroupLayoutEntry {
                    binding: 2,
                    visibility: WGPUShaderStage::FRAGMENT,
                    sampler: WGPUSamplerBindingLayout {
                        type_: WGPUSamplerBindingType::Filtering,
                        ..mem::zeroed()
                    },
                    ..mem::zeroed()
                },
            ];
            let bind_group_layout_desc = WGPUBindGroupLayoutDescriptor {
                entryCount: 3,
                entries: bind_group_layout_entries.as_ptr(),
                ..mem::zeroed()
            };
            let bind_group_layout =
                wgpuDeviceCreateBindGroupLayout(device, &bind_group_layout_desc);

            let pipeline_layout_desc = WGPUPipelineLayoutDescriptor {
                bindGroupLayoutCount: 1,
                bindGroupLayouts: &bind_group_layout,
                ..mem::zeroed()
            };
            let pipeline_layout = wgpuDeviceCreatePipelineLayout(device, &pipeline_layout_desc);

            let vertex_attributes = [
                WGPUVertexAttribute {
                    nextInChain: ptr::null_mut(),
                    format: WGPUVertexFormat::Float32x2,
                    offset: 0,
                    shaderLocation: 0,
                },
                WGPUVertexAttribute {
                    nextInChain: ptr::null_mut(),
                    format: WGPUVertexFormat::Float32x2,
                    offset: (2 * 4),
                    shaderLocation: 1,
                },
                WGPUVertexAttribute {
                    nextInChain: ptr::null_mut(),
                    format: WGPUVertexFormat::Float32x3,
                    offset: (4 * 4),
                    shaderLocation: 2,
                },
            ];
            let vertex_buffer_layout = WGPUVertexBufferLayout {
                nextInChain: ptr::null_mut(),
                arrayStride: Self::K_VERTEX_STRIDE,
                stepMode: WGPUVertexStepMode::Vertex,
                attributeCount: 3,
                attributes: vertex_attributes.as_ptr(),
            };

            let blend_state = WGPUBlendState {
                color: WGPUBlendComponent {
                    operation: WGPUBlendOperation::Add,
                    srcFactor: WGPUBlendFactor::SrcAlpha,
                    dstFactor: WGPUBlendFactor::OneMinusSrcAlpha,
                },
                alpha: WGPUBlendComponent {
                    operation: WGPUBlendOperation::Add,
                    srcFactor: WGPUBlendFactor::SrcAlpha,
                    dstFactor: WGPUBlendFactor::OneMinusSrcAlpha,
                },
            };
            let color_target = WGPUColorTargetState {
                nextInChain: ptr::null_mut(),
                format: Self::K_SWAP_CHAIN_FORMAT,
                blend: &blend_state,
                writeMask: WGPUColorWriteMask::ALL,
            };
            let mut fragment_state = WGPUFragmentState {
                module: shaders,
                entryPoint: WGPUStringView {
                    data: "fs".as_ptr() as *const i8,
                    length: 2,
                },
                targetCount: 1,
                targets: &color_target,
                ..mem::zeroed()
            };

            let pipeline_desc = WGPURenderPipelineDescriptor {
                layout: pipeline_layout,
                primitive: WGPUPrimitiveState {
                    topology: WGPUPrimitiveTopology::TriangleList,
                    frontFace: WGPUFrontFace::CCW,
                    cullMode: WGPUCullMode::None,
                    ..mem::zeroed()
                },
                vertex: WGPUVertexState {
                    module: shaders,
                    entryPoint: WGPUStringView {
                        data: "vs".as_ptr() as *const i8,
                        length: 2,
                    },
                    bufferCount: 1,
                    buffers: &vertex_buffer_layout,
                    ..mem::zeroed()
                },
                fragment: &mut fragment_state,
                multisample: WGPUMultisampleState {
                    count: 1,
                    mask: 0xffffffff,
                    ..mem::zeroed()
                },
                ..mem::zeroed()
            };
            let pipeline = wgpuDeviceCreateRenderPipeline(device, &pipeline_desc);
            wgpuPipelineLayoutRelease(pipeline_layout);
            wgpuShaderModuleRelease(shaders);

            let bind_group_entries = [
                WGPUBindGroupEntry {
                    binding: 0,
                    buffer: uniform_buffer,
                    size: (4 * 4 * mem::size_of::<f32>()) as u64,
                    ..mem::zeroed()
                },
                WGPUBindGroupEntry {
                    binding: 1,
                    textureView: texture_view,
                    ..mem::zeroed()
                },
                WGPUBindGroupEntry {
                    binding: 2,
                    sampler: sampler,
                    ..mem::zeroed()
                },
            ];
            let bind_group_desc = WGPUBindGroupDescriptor {
                layout: bind_group_layout,
                entryCount: 3,
                entries: bind_group_entries.as_ptr(),
                ..mem::zeroed()
            };
            let bind_group = wgpuDeviceCreateBindGroup(device, &bind_group_desc);
            wgpuBindGroupLayoutRelease(bind_group_layout);

            // Release resources now owned by the pipeline/bind group.
            wgpuSamplerRelease(sampler);
            wgpuTextureViewRelease(texture_view);

            Self {
                instance,
                surface,
                adapter,
                device,
                queue,
                vertex_buffer,
                uniform_buffer,
                pipeline,
                bind_group,
                swap_chain_configured: false,
                current_width: 0,
                current_height: 0,
                start_time: Instant::now(),
            }
        }
    }

    /// Renders a single frame.
    fn render(&mut self, width: i32, height: i32) {
        let time = self.start_time.elapsed().as_secs_f32();

        unsafe {
            // Process any pending async events.
            wgpuInstanceProcessEvents(self.instance);

            // Reconfigure the swap chain if the window has been resized.
            if !self.swap_chain_configured
                || width != self.current_width
                || height != self.current_height {
                if self.swap_chain_configured {
                    println!("Unconfiguring swap chain...");
                    wgpuSurfaceUnconfigure(self.surface);
                    self.swap_chain_configured = false;
                }
                if width != 0 && height != 0 {
                    println!("Configuring swap chain to {}x{}...", width, height);
                    let config = WGPUSurfaceConfiguration {
                        device: self.device,
                        format: Self::K_SWAP_CHAIN_FORMAT,
                        usage: WGPUTextureUsage::RENDER_ATTACHMENT,
                        width: width as u32,
                        height: height as u32,
                        presentMode: WGPUPresentMode::Fifo,
                        ..mem::zeroed()
                    };
                    wgpuSurfaceConfigure(self.surface, &config);
                    self.swap_chain_configured = true;
                }
                self.current_width = width;
                self.current_height = height;
            }

            // Don't render if the window is minimized.
            if !self.swap_chain_configured {
                return;
            }

            // --- Acquire Swap Chain Texture ---
            let mut surface_tex = mem::zeroed();
            wgpuSurfaceGetCurrentTexture(self.surface, &mut surface_tex);
            if surface_tex.status != WGPUSurfaceGetCurrentTextureStatus::SuccessOptimal {
                eprintln!(
                    "Failed to get current surface texture: status {:?}",
                    surface_tex.status);
                // Can sometimes happen on resize, skip a frame.
                if surface_tex.texture != ptr::null_mut() {
                    wgpuTextureRelease(surface_tex.texture);
                }
                return;
            }
            let surface_view_desc = WGPUTextureViewDescriptor {
                format: wgpuTextureGetFormat(surface_tex.texture),
                dimension: WGPUTextureViewDimension::D2,
                mipLevelCount: 1,
                arrayLayerCount: 1,
                ..mem::zeroed()
            };
            let surface_view = wgpuTextureCreateView(surface_tex.texture, &surface_view_desc);
            assert!(!surface_view.is_null());

            // --- Update Uniforms ---
            let angle = time * 2.0 * std::f32::consts::PI / 20.0;
            let aspect = self.current_height as f32 / self.current_width as f32;
            let matrix: [f32; 16] = [
                angle.cos() * aspect,
                -angle.sin(),
                0.0,
                0.0,
                angle.sin() * aspect,
                angle.cos(),
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            ];
            wgpuQueueWriteBuffer(
                self.queue,
                self.uniform_buffer,
                0,
                matrix.as_ptr() as *const _,
                mem::size_of_val(&matrix));

            // --- Encode and Submit Commands ---
            let encoder = wgpuDeviceCreateCommandEncoder(self.device, ptr::null());

            let color_attachment = WGPURenderPassColorAttachment {
                view: surface_view,
                depthSlice: WGPU_DEPTH_SLICE_UNDEFINED,
                loadOp: WGPULoadOp::Clear,
                storeOp: WGPUStoreOp::Store,
                clearValue: WGPUColor {
                    r: 0.392,
                    g: 0.584,
                    b: 0.929,
                    a: 1.0,
                },
                ..mem::zeroed()
            };
            let pass_desc = WGPURenderPassDescriptor {
                colorAttachmentCount: 1,
                colorAttachments: &color_attachment,
                ..mem::zeroed()
            };

            let pass = wgpuCommandEncoderBeginRenderPass(encoder, &pass_desc);
            wgpuRenderPassEncoderSetViewport(pass, 0.0, 0.0, width as f32, height as f32, 0.0, 1.0);
            wgpuRenderPassEncoderSetPipeline(pass, self.pipeline);
            wgpuRenderPassEncoderSetBindGroup(pass, 0, self.bind_group, 0, ptr::null());
            wgpuRenderPassEncoderSetVertexBuffer(pass, 0, self.vertex_buffer, 0, WGPU_WHOLE_SIZE);
            wgpuRenderPassEncoderDraw(pass, 3, 1, 0, 0);
            wgpuRenderPassEncoderEnd(pass);
            wgpuRenderPassEncoderRelease(pass);

            let command = wgpuCommandEncoderFinish(encoder, ptr::null());
            wgpuQueueSubmit(self.queue, 1, &command);

            // --- Present to Screen ---
            wgpuSurfacePresent(self.surface);

            // --- Release temporary resources ---
            wgpuCommandBufferRelease(command);
            wgpuCommandEncoderRelease(encoder);
            wgpuTextureViewRelease(surface_view);
            wgpuTextureRelease(surface_tex.texture);
        }
    }
}

/// Implement the Drop trait to automatically release WebGPU resources
/// when the `App` goes out of scope.
impl Drop for App {
    fn drop(&mut self) {
        println!("Cleaning up WebGPU resources...");
        unsafe {
            wgpuBindGroupRelease(self.bind_group);
            wgpuRenderPipelineRelease(self.pipeline);
            wgpuBufferRelease(self.uniform_buffer);
            wgpuBufferRelease(self.vertex_buffer);
            wgpuQueueRelease(self.queue);
            wgpuDeviceRelease(self.device);
            wgpuAdapterRelease(self.adapter);
            wgpuSurfaceRelease(self.surface);
            wgpuInstanceRelease(self.instance);
        }
        println!("Cleanup complete.");
    }
}
