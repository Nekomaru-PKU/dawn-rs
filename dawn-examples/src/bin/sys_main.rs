use dawn_sys::*;

use std::{
    cell::Cell,
    ffi,
    mem,
    ptr,
    rc::Rc,
};

fn main() {
    use winsafe::prelude::*;
    use winsafe::gui::{
        WindowMain,
        WindowMainOpts,
    };

    let window = WindowMain::new(WindowMainOpts {
        title: "dawn-examples".into(),
        size: (960, 600),
        ..Default::default()
    });

    let app = Rc::new(Cell::new(Option::<App>::None));

    window.on().wm_create({
        let window = window.clone();
        let app = app.clone();
        move |_| Ok({
            app.set(Some(App::new(
                window.hwnd().hinstance().ptr(),
                window.hwnd().ptr())));
            0
        })
    });

    window.run_main(None).expect("failed to run main window");
}

unsafe extern "C" fn on_device_error(
    _device: *const WGPUDevice,
    _ty: i32,
    message: WGPUStringView,
    _userdata1: *mut ffi::c_void,
    _userdata2: *mut ffi::c_void) {
    panic!("{}", String::from_utf8_lossy(message.as_bytes()));
}

unsafe extern "C" fn on_request_adapter(
    status: i32,
    adapter: WGPUAdapter,
    message: WGPUStringView,
    userdata1: *mut ffi::c_void,
    _userdata2: *mut ffi::c_void) {
    if status == WGPURequestAdapterStatus::Success {
        let result_ptr = userdata1 as *mut WGPUAdapter;
        if !result_ptr.is_null() {
            unsafe {
                *result_ptr = adapter;
            }
        } else {
            panic!("userdata1 is null");
        }
    } else {
    panic!("{}", String::from_utf8_lossy(message.as_bytes()));
    }
}

unsafe extern "C" fn on_request_device(
    status: i32,
    device: WGPUDevice,
    message: WGPUStringView,
    userdata1: *mut ffi::c_void,
    _userdata2: *mut ffi::c_void) {
    if status == WGPURequestDeviceStatus::Success {
        let result_ptr = userdata1 as *mut WGPUDevice;
        if !result_ptr.is_null() {
            unsafe {
                *result_ptr = device;
            }
        } else {
            panic!("userdata1 is null");
        }
    } else {
    panic!("{}", String::from_utf8_lossy(message.as_bytes()));
    }
}

struct App {

}

struct Vertex {
    position: [f32; 2],
    uv: [f32; 2],
    color: [f32; 3],
}

impl App {
    const K_VERTEX_STRIDE: usize = mem::size_of::<Vertex>();
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

    const K_SWAPCHAIN_FORMAT: i32 = WGPUTextureFormat::BGRA8Unorm;

    fn new(
        hinstance: *mut ffi::c_void,
        hwnd: *mut ffi::c_void)
     -> Self {
        eprintln!("creating instance ...");
        let instance = unsafe {
            wgpuCreateInstance(ptr::null())
        };
        assert!(!instance.is_null());

        eprintln!("creating surface ...");
        let mut surface_desc_0 = WGPUSurfaceSourceWindowsHWND {
            chain: WGPUChainedStruct {
                sType: WGPUSType::SurfaceSourceWindowsHWND,
                next: ptr::null_mut(),
            },
            hwnd,
            hinstance,
        };
        let surface_desc = WGPUSurfaceDescriptor {
            nextInChain: &mut surface_desc_0.chain,
            label: WGPUStringView {
                data: ptr::null(),
                length: 0,
            },
        };
        let surface = unsafe {
            wgpuInstanceCreateSurface(instance, &surface_desc)
        };
        assert!(!surface.is_null());

        eprintln!("creating adapter ...");
        let mut adapter: WGPUAdapter = ptr::null_mut();
        let adapter_options = WGPURequestAdapterOptions {
            nextInChain: ptr::null_mut(),
            featureLevel: WGPUFeatureLevel::Core,
            powerPreference: WGPUPowerPreference::HighPerformance,
            forceFallbackAdapter: 0,
            backendType: WGPUBackendType::Undefined,
            compatibleSurface: surface,
        };
        let adapter_callback_info = WGPURequestAdapterCallbackInfo {
            nextInChain: ptr::null_mut(),
            mode: WGPUCallbackMode::WaitAnyOnly,
            callback: Some(on_request_adapter),
            userdata1: ptr::from_mut(&mut adapter).cast(),
            userdata2: ptr::null_mut(),
        };
        let adapter_future = unsafe {
            wgpuInstanceRequestAdapter(
                instance,
                &adapter_options,
                adapter_callback_info)
        };
        let mut wait = WGPUFutureWaitInfo {
            future: adapter_future,
            completed: WGPU_FALSE,
        };
        let status = unsafe {
            wgpuInstanceWaitAny(
                instance,
                1,
                &mut wait,
                0)
        };
        assert!(status == WGPUWaitStatus::Success);

        let mut adapter_info = unsafe {
            mem::zeroed()
        };
        let status = unsafe {
            wgpuAdapterGetInfo(adapter, &mut adapter_info)
        };
        assert!(status == WGPUWaitStatus::Success);
        eprintln!("adapter: {}", String::from_utf8_lossy(adapter_info.device.as_bytes()));
        unsafe { wgpuAdapterInfoFreeMembers(adapter_info) };

        eprintln!("creating device and queue ...");
        let device_desc = unsafe {
            WGPUDeviceDescriptor {
                uncapturedErrorCallbackInfo: WGPUUncapturedErrorCallbackInfo {
                    callback: Some(on_device_error),
                    ..mem::zeroed()
                },
                ..mem::zeroed()
            }
        };
        let mut device: WGPUDevice = ptr::null_mut();
        let device_callback = WGPURequestDeviceCallbackInfo {
            nextInChain: ptr::null_mut(),
            mode: WGPUCallbackMode::WaitAnyOnly,
            callback: Some(on_request_device),
            userdata1: ptr::from_mut(&mut device).cast(),
            userdata2: ptr::null_mut(),
        };
        let device_future = unsafe {
            wgpuAdapterRequestDevice(
                adapter,
                &device_desc,
                device_callback)
        };
        let mut wait = WGPUFutureWaitInfo {
            future: device_future,
            completed: WGPU_FALSE,
        };
        let status = unsafe {
            wgpuInstanceWaitAny(
                instance,
                1,
                &mut wait,
                0)
        };
        assert!(status == WGPUWaitStatus::Success);

        let queue = unsafe {
            wgpuDeviceGetQueue(device)
        };
        assert!(!queue.is_null());

        let vertex_buffer_desc = WGPUBufferDescriptor {
            label: WGPUStringView {
                data: ptr::null(),
                length: 0,
            },
            size: (Self::K_VERTEX_STRIDE * Self::K_VERTEX_DATA.len()) as _,
            usage: WGPUBufferUsage::VERTEX | WGPUBufferUsage::COPY_DST,
            mappedAtCreation: WGPU_FALSE,
            nextInChain: ptr::null_mut(),
        };
        let vertex_buffer = unsafe {
            wgpuDeviceCreateBuffer(
                device,
                &vertex_buffer_desc)
        };
        unsafe {
            wgpuQueueWriteBuffer(
                queue,
                vertex_buffer,
                0,
                Self::K_VERTEX_DATA.as_ptr().cast(),
                (Self::K_VERTEX_STRIDE * Self::K_VERTEX_DATA.len()) as _);
        }

        Self {

        }
    }
}
