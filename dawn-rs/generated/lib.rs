#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum WGSLLanguageFeatureName {
    ReadonlyAndReadwriteStorageTextures = sys::WGPUWGSLLanguageFeatureName::ReadonlyAndReadwriteStorageTextures,
    Packed4x8IntegerDotProduct = sys::WGPUWGSLLanguageFeatureName::Packed4x8IntegerDotProduct,
    UnrestrictedPointerParameters = sys::WGPUWGSLLanguageFeatureName::UnrestrictedPointerParameters,
    PointerCompositeAccess = sys::WGPUWGSLLanguageFeatureName::PointerCompositeAccess,
    SizedBindingArray = sys::WGPUWGSLLanguageFeatureName::SizedBindingArray,
    ChromiumTestingUnimplemented = sys::WGPUWGSLLanguageFeatureName::ChromiumTestingUnimplemented,
    ChromiumTestingUnsafeExperimental = sys::WGPUWGSLLanguageFeatureName::ChromiumTestingUnsafeExperimental,
    ChromiumTestingExperimental = sys::WGPUWGSLLanguageFeatureName::ChromiumTestingExperimental,
    ChromiumTestingShippedWithKillswitch = sys::WGPUWGSLLanguageFeatureName::ChromiumTestingShippedWithKillswitch,
    ChromiumTestingShipped = sys::WGPUWGSLLanguageFeatureName::ChromiumTestingShipped,
}
impl WGSLLanguageFeatureName {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::ReadonlyAndReadwriteStorageTextures,
            Self::Packed4x8IntegerDotProduct,
            Self::UnrestrictedPointerParameters,
            Self::PointerCompositeAccess,
            Self::SizedBindingArray,
            Self::ChromiumTestingUnimplemented,
            Self::ChromiumTestingUnsafeExperimental,
            Self::ChromiumTestingExperimental,
            Self::ChromiumTestingShippedWithKillswitch,
            Self::ChromiumTestingShipped,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum AdapterType {
    DiscreteGPU = sys::WGPUAdapterType::DiscreteGPU,
    IntegratedGPU = sys::WGPUAdapterType::IntegratedGPU,
    CPU = sys::WGPUAdapterType::CPU,
    Unknown = sys::WGPUAdapterType::Unknown,
}
impl AdapterType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::DiscreteGPU, Self::IntegratedGPU, Self::CPU, Self::Unknown].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum AddressMode {
    Undefined = sys::WGPUAddressMode::Undefined,
    ClampToEdge = sys::WGPUAddressMode::ClampToEdge,
    Repeat = sys::WGPUAddressMode::Repeat,
    MirrorRepeat = sys::WGPUAddressMode::MirrorRepeat,
}
impl AddressMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::ClampToEdge, Self::Repeat, Self::MirrorRepeat]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum AlphaMode {
    Opaque = sys::WGPUAlphaMode::Opaque,
    Premultiplied = sys::WGPUAlphaMode::Premultiplied,
    Unpremultiplied = sys::WGPUAlphaMode::Unpremultiplied,
}
impl AlphaMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Opaque, Self::Premultiplied, Self::Unpremultiplied].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum BackendType {
    Undefined = sys::WGPUBackendType::Undefined,
    Null = sys::WGPUBackendType::Null,
    WebGPU = sys::WGPUBackendType::WebGPU,
    D3D11 = sys::WGPUBackendType::D3D11,
    D3D12 = sys::WGPUBackendType::D3D12,
    Metal = sys::WGPUBackendType::Metal,
    Vulkan = sys::WGPUBackendType::Vulkan,
    OpenGL = sys::WGPUBackendType::OpenGL,
    OpenGLES = sys::WGPUBackendType::OpenGLES,
}
impl BackendType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::Null,
            Self::WebGPU,
            Self::D3D11,
            Self::D3D12,
            Self::Metal,
            Self::Vulkan,
            Self::OpenGL,
            Self::OpenGLES,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum BlendFactor {
    Undefined = sys::WGPUBlendFactor::Undefined,
    Zero = sys::WGPUBlendFactor::Zero,
    One = sys::WGPUBlendFactor::One,
    Src = sys::WGPUBlendFactor::Src,
    OneMinusSrc = sys::WGPUBlendFactor::OneMinusSrc,
    SrcAlpha = sys::WGPUBlendFactor::SrcAlpha,
    OneMinusSrcAlpha = sys::WGPUBlendFactor::OneMinusSrcAlpha,
    Dst = sys::WGPUBlendFactor::Dst,
    OneMinusDst = sys::WGPUBlendFactor::OneMinusDst,
    DstAlpha = sys::WGPUBlendFactor::DstAlpha,
    OneMinusDstAlpha = sys::WGPUBlendFactor::OneMinusDstAlpha,
    SrcAlphaSaturated = sys::WGPUBlendFactor::SrcAlphaSaturated,
    Constant = sys::WGPUBlendFactor::Constant,
    OneMinusConstant = sys::WGPUBlendFactor::OneMinusConstant,
    Src1 = sys::WGPUBlendFactor::Src1,
    OneMinusSrc1 = sys::WGPUBlendFactor::OneMinusSrc1,
    Src1Alpha = sys::WGPUBlendFactor::Src1Alpha,
    OneMinusSrc1Alpha = sys::WGPUBlendFactor::OneMinusSrc1Alpha,
}
impl BlendFactor {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::Zero,
            Self::One,
            Self::Src,
            Self::OneMinusSrc,
            Self::SrcAlpha,
            Self::OneMinusSrcAlpha,
            Self::Dst,
            Self::OneMinusDst,
            Self::DstAlpha,
            Self::OneMinusDstAlpha,
            Self::SrcAlphaSaturated,
            Self::Constant,
            Self::OneMinusConstant,
            Self::Src1,
            Self::OneMinusSrc1,
            Self::Src1Alpha,
            Self::OneMinusSrc1Alpha,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum BlendOperation {
    Undefined = sys::WGPUBlendOperation::Undefined,
    Add = sys::WGPUBlendOperation::Add,
    Subtract = sys::WGPUBlendOperation::Subtract,
    ReverseSubtract = sys::WGPUBlendOperation::ReverseSubtract,
    Min = sys::WGPUBlendOperation::Min,
    Max = sys::WGPUBlendOperation::Max,
}
impl BlendOperation {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::Add,
            Self::Subtract,
            Self::ReverseSubtract,
            Self::Min,
            Self::Max,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum BufferBindingType {
    BindingNotUsed = sys::WGPUBufferBindingType::BindingNotUsed,
    Undefined = sys::WGPUBufferBindingType::Undefined,
    Uniform = sys::WGPUBufferBindingType::Uniform,
    Storage = sys::WGPUBufferBindingType::Storage,
    ReadOnlyStorage = sys::WGPUBufferBindingType::ReadOnlyStorage,
}
impl BufferBindingType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::BindingNotUsed,
            Self::Undefined,
            Self::Uniform,
            Self::Storage,
            Self::ReadOnlyStorage,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum BufferMapState {
    Unmapped = sys::WGPUBufferMapState::Unmapped,
    Pending = sys::WGPUBufferMapState::Pending,
    Mapped = sys::WGPUBufferMapState::Mapped,
}
impl BufferMapState {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Unmapped, Self::Pending, Self::Mapped].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum CallbackMode {
    WaitAnyOnly = sys::WGPUCallbackMode::WaitAnyOnly,
    AllowProcessEvents = sys::WGPUCallbackMode::AllowProcessEvents,
    AllowSpontaneous = sys::WGPUCallbackMode::AllowSpontaneous,
}
impl CallbackMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::WaitAnyOnly, Self::AllowProcessEvents, Self::AllowSpontaneous].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum CompareFunction {
    Undefined = sys::WGPUCompareFunction::Undefined,
    Never = sys::WGPUCompareFunction::Never,
    Less = sys::WGPUCompareFunction::Less,
    Equal = sys::WGPUCompareFunction::Equal,
    LessEqual = sys::WGPUCompareFunction::LessEqual,
    Greater = sys::WGPUCompareFunction::Greater,
    NotEqual = sys::WGPUCompareFunction::NotEqual,
    GreaterEqual = sys::WGPUCompareFunction::GreaterEqual,
    Always = sys::WGPUCompareFunction::Always,
}
impl CompareFunction {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::Never,
            Self::Less,
            Self::Equal,
            Self::LessEqual,
            Self::Greater,
            Self::NotEqual,
            Self::GreaterEqual,
            Self::Always,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum CompilationMessageType {
    Error = sys::WGPUCompilationMessageType::Error,
    Warning = sys::WGPUCompilationMessageType::Warning,
    Info = sys::WGPUCompilationMessageType::Info,
}
impl CompilationMessageType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Error, Self::Warning, Self::Info].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum CompositeAlphaMode {
    Auto = sys::WGPUCompositeAlphaMode::Auto,
    Opaque = sys::WGPUCompositeAlphaMode::Opaque,
    Premultiplied = sys::WGPUCompositeAlphaMode::Premultiplied,
    Unpremultiplied = sys::WGPUCompositeAlphaMode::Unpremultiplied,
    Inherit = sys::WGPUCompositeAlphaMode::Inherit,
}
impl CompositeAlphaMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Auto,
            Self::Opaque,
            Self::Premultiplied,
            Self::Unpremultiplied,
            Self::Inherit,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum CullMode {
    Undefined = sys::WGPUCullMode::Undefined,
    None = sys::WGPUCullMode::None,
    Front = sys::WGPUCullMode::Front,
    Back = sys::WGPUCullMode::Back,
}
impl CullMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::None, Self::Front, Self::Back].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum ErrorFilter {
    Validation = sys::WGPUErrorFilter::Validation,
    OutOfMemory = sys::WGPUErrorFilter::OutOfMemory,
    Internal = sys::WGPUErrorFilter::Internal,
}
impl ErrorFilter {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Validation, Self::OutOfMemory, Self::Internal].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum ExternalTextureRotation {
    Rotate0Degrees = sys::WGPUExternalTextureRotation::Rotate0Degrees,
    Rotate90Degrees = sys::WGPUExternalTextureRotation::Rotate90Degrees,
    Rotate180Degrees = sys::WGPUExternalTextureRotation::Rotate180Degrees,
    Rotate270Degrees = sys::WGPUExternalTextureRotation::Rotate270Degrees,
}
impl ExternalTextureRotation {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Rotate0Degrees,
            Self::Rotate90Degrees,
            Self::Rotate180Degrees,
            Self::Rotate270Degrees,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum FeatureLevel {
    Undefined = sys::WGPUFeatureLevel::Undefined,
    Compatibility = sys::WGPUFeatureLevel::Compatibility,
    Core = sys::WGPUFeatureLevel::Core,
}
impl FeatureLevel {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Compatibility, Self::Core].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum FeatureName {
    DepthClipControl = sys::WGPUFeatureName::DepthClipControl,
    Depth32FloatStencil8 = sys::WGPUFeatureName::Depth32FloatStencil8,
    TimestampQuery = sys::WGPUFeatureName::TimestampQuery,
    TextureCompressionBC = sys::WGPUFeatureName::TextureCompressionBC,
    TextureCompressionBCSliced3D = sys::WGPUFeatureName::TextureCompressionBCSliced3D,
    TextureCompressionETC2 = sys::WGPUFeatureName::TextureCompressionETC2,
    TextureCompressionASTC = sys::WGPUFeatureName::TextureCompressionASTC,
    TextureCompressionASTCSliced3D = sys::WGPUFeatureName::TextureCompressionASTCSliced3D,
    IndirectFirstInstance = sys::WGPUFeatureName::IndirectFirstInstance,
    ShaderF16 = sys::WGPUFeatureName::ShaderF16,
    RG11B10UfloatRenderable = sys::WGPUFeatureName::RG11B10UfloatRenderable,
    BGRA8UnormStorage = sys::WGPUFeatureName::BGRA8UnormStorage,
    Float32Filterable = sys::WGPUFeatureName::Float32Filterable,
    Float32Blendable = sys::WGPUFeatureName::Float32Blendable,
    ClipDistances = sys::WGPUFeatureName::ClipDistances,
    DualSourceBlending = sys::WGPUFeatureName::DualSourceBlending,
    Subgroups = sys::WGPUFeatureName::Subgroups,
    CoreFeaturesAndLimits = sys::WGPUFeatureName::CoreFeaturesAndLimits,
    DawnInternalUsages = sys::WGPUFeatureName::DawnInternalUsages,
    DawnMultiPlanarFormats = sys::WGPUFeatureName::DawnMultiPlanarFormats,
    DawnNative = sys::WGPUFeatureName::DawnNative,
    ChromiumExperimentalTimestampQueryInsidePasses = sys::WGPUFeatureName::ChromiumExperimentalTimestampQueryInsidePasses,
    ImplicitDeviceSynchronization = sys::WGPUFeatureName::ImplicitDeviceSynchronization,
    TransientAttachments = sys::WGPUFeatureName::TransientAttachments,
    MSAARenderToSingleSampled = sys::WGPUFeatureName::MSAARenderToSingleSampled,
    D3D11MultithreadProtected = sys::WGPUFeatureName::D3D11MultithreadProtected,
    ANGLETextureSharing = sys::WGPUFeatureName::ANGLETextureSharing,
    PixelLocalStorageCoherent = sys::WGPUFeatureName::PixelLocalStorageCoherent,
    PixelLocalStorageNonCoherent = sys::WGPUFeatureName::PixelLocalStorageNonCoherent,
    Unorm16TextureFormats = sys::WGPUFeatureName::Unorm16TextureFormats,
    Snorm16TextureFormats = sys::WGPUFeatureName::Snorm16TextureFormats,
    MultiPlanarFormatExtendedUsages = sys::WGPUFeatureName::MultiPlanarFormatExtendedUsages,
    MultiPlanarFormatP010 = sys::WGPUFeatureName::MultiPlanarFormatP010,
    HostMappedPointer = sys::WGPUFeatureName::HostMappedPointer,
    MultiPlanarRenderTargets = sys::WGPUFeatureName::MultiPlanarRenderTargets,
    MultiPlanarFormatNv12a = sys::WGPUFeatureName::MultiPlanarFormatNv12a,
    FramebufferFetch = sys::WGPUFeatureName::FramebufferFetch,
    BufferMapExtendedUsages = sys::WGPUFeatureName::BufferMapExtendedUsages,
    AdapterPropertiesMemoryHeaps = sys::WGPUFeatureName::AdapterPropertiesMemoryHeaps,
    AdapterPropertiesD3D = sys::WGPUFeatureName::AdapterPropertiesD3D,
    AdapterPropertiesVk = sys::WGPUFeatureName::AdapterPropertiesVk,
    R8UnormStorage = sys::WGPUFeatureName::R8UnormStorage,
    DawnFormatCapabilities = sys::WGPUFeatureName::DawnFormatCapabilities,
    DawnDrmFormatCapabilities = sys::WGPUFeatureName::DawnDrmFormatCapabilities,
    Norm16TextureFormats = sys::WGPUFeatureName::Norm16TextureFormats,
    MultiPlanarFormatNv16 = sys::WGPUFeatureName::MultiPlanarFormatNv16,
    MultiPlanarFormatNv24 = sys::WGPUFeatureName::MultiPlanarFormatNv24,
    MultiPlanarFormatP210 = sys::WGPUFeatureName::MultiPlanarFormatP210,
    MultiPlanarFormatP410 = sys::WGPUFeatureName::MultiPlanarFormatP410,
    SharedTextureMemoryVkDedicatedAllocation = sys::WGPUFeatureName::SharedTextureMemoryVkDedicatedAllocation,
    SharedTextureMemoryAHardwareBuffer = sys::WGPUFeatureName::SharedTextureMemoryAHardwareBuffer,
    SharedTextureMemoryDmaBuf = sys::WGPUFeatureName::SharedTextureMemoryDmaBuf,
    SharedTextureMemoryOpaqueFD = sys::WGPUFeatureName::SharedTextureMemoryOpaqueFD,
    SharedTextureMemoryZirconHandle = sys::WGPUFeatureName::SharedTextureMemoryZirconHandle,
    SharedTextureMemoryDXGISharedHandle = sys::WGPUFeatureName::SharedTextureMemoryDXGISharedHandle,
    SharedTextureMemoryD3D11Texture2D = sys::WGPUFeatureName::SharedTextureMemoryD3D11Texture2D,
    SharedTextureMemoryIOSurface = sys::WGPUFeatureName::SharedTextureMemoryIOSurface,
    SharedTextureMemoryEGLImage = sys::WGPUFeatureName::SharedTextureMemoryEGLImage,
    SharedFenceVkSemaphoreOpaqueFD = sys::WGPUFeatureName::SharedFenceVkSemaphoreOpaqueFD,
    SharedFenceSyncFD = sys::WGPUFeatureName::SharedFenceSyncFD,
    SharedFenceVkSemaphoreZirconHandle = sys::WGPUFeatureName::SharedFenceVkSemaphoreZirconHandle,
    SharedFenceDXGISharedHandle = sys::WGPUFeatureName::SharedFenceDXGISharedHandle,
    SharedFenceMTLSharedEvent = sys::WGPUFeatureName::SharedFenceMTLSharedEvent,
    SharedBufferMemoryD3D12Resource = sys::WGPUFeatureName::SharedBufferMemoryD3D12Resource,
    StaticSamplers = sys::WGPUFeatureName::StaticSamplers,
    YCbCrVulkanSamplers = sys::WGPUFeatureName::YCbCrVulkanSamplers,
    ShaderModuleCompilationOptions = sys::WGPUFeatureName::ShaderModuleCompilationOptions,
    DawnLoadResolveTexture = sys::WGPUFeatureName::DawnLoadResolveTexture,
    DawnPartialLoadResolveTexture = sys::WGPUFeatureName::DawnPartialLoadResolveTexture,
    MultiDrawIndirect = sys::WGPUFeatureName::MultiDrawIndirect,
    DawnTexelCopyBufferRowAlignment = sys::WGPUFeatureName::DawnTexelCopyBufferRowAlignment,
    FlexibleTextureViews = sys::WGPUFeatureName::FlexibleTextureViews,
    ChromiumExperimentalSubgroupMatrix = sys::WGPUFeatureName::ChromiumExperimentalSubgroupMatrix,
    SharedFenceEGLSync = sys::WGPUFeatureName::SharedFenceEGLSync,
    DawnDeviceAllocatorControl = sys::WGPUFeatureName::DawnDeviceAllocatorControl,
}
impl FeatureName {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::DepthClipControl,
            Self::Depth32FloatStencil8,
            Self::TimestampQuery,
            Self::TextureCompressionBC,
            Self::TextureCompressionBCSliced3D,
            Self::TextureCompressionETC2,
            Self::TextureCompressionASTC,
            Self::TextureCompressionASTCSliced3D,
            Self::IndirectFirstInstance,
            Self::ShaderF16,
            Self::RG11B10UfloatRenderable,
            Self::BGRA8UnormStorage,
            Self::Float32Filterable,
            Self::Float32Blendable,
            Self::ClipDistances,
            Self::DualSourceBlending,
            Self::Subgroups,
            Self::CoreFeaturesAndLimits,
            Self::DawnInternalUsages,
            Self::DawnMultiPlanarFormats,
            Self::DawnNative,
            Self::ChromiumExperimentalTimestampQueryInsidePasses,
            Self::ImplicitDeviceSynchronization,
            Self::TransientAttachments,
            Self::MSAARenderToSingleSampled,
            Self::D3D11MultithreadProtected,
            Self::ANGLETextureSharing,
            Self::PixelLocalStorageCoherent,
            Self::PixelLocalStorageNonCoherent,
            Self::Unorm16TextureFormats,
            Self::Snorm16TextureFormats,
            Self::MultiPlanarFormatExtendedUsages,
            Self::MultiPlanarFormatP010,
            Self::HostMappedPointer,
            Self::MultiPlanarRenderTargets,
            Self::MultiPlanarFormatNv12a,
            Self::FramebufferFetch,
            Self::BufferMapExtendedUsages,
            Self::AdapterPropertiesMemoryHeaps,
            Self::AdapterPropertiesD3D,
            Self::AdapterPropertiesVk,
            Self::R8UnormStorage,
            Self::DawnFormatCapabilities,
            Self::DawnDrmFormatCapabilities,
            Self::Norm16TextureFormats,
            Self::MultiPlanarFormatNv16,
            Self::MultiPlanarFormatNv24,
            Self::MultiPlanarFormatP210,
            Self::MultiPlanarFormatP410,
            Self::SharedTextureMemoryVkDedicatedAllocation,
            Self::SharedTextureMemoryAHardwareBuffer,
            Self::SharedTextureMemoryDmaBuf,
            Self::SharedTextureMemoryOpaqueFD,
            Self::SharedTextureMemoryZirconHandle,
            Self::SharedTextureMemoryDXGISharedHandle,
            Self::SharedTextureMemoryD3D11Texture2D,
            Self::SharedTextureMemoryIOSurface,
            Self::SharedTextureMemoryEGLImage,
            Self::SharedFenceVkSemaphoreOpaqueFD,
            Self::SharedFenceSyncFD,
            Self::SharedFenceVkSemaphoreZirconHandle,
            Self::SharedFenceDXGISharedHandle,
            Self::SharedFenceMTLSharedEvent,
            Self::SharedBufferMemoryD3D12Resource,
            Self::StaticSamplers,
            Self::YCbCrVulkanSamplers,
            Self::ShaderModuleCompilationOptions,
            Self::DawnLoadResolveTexture,
            Self::DawnPartialLoadResolveTexture,
            Self::MultiDrawIndirect,
            Self::DawnTexelCopyBufferRowAlignment,
            Self::FlexibleTextureViews,
            Self::ChromiumExperimentalSubgroupMatrix,
            Self::SharedFenceEGLSync,
            Self::DawnDeviceAllocatorControl,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum FilterMode {
    Undefined = sys::WGPUFilterMode::Undefined,
    Nearest = sys::WGPUFilterMode::Nearest,
    Linear = sys::WGPUFilterMode::Linear,
}
impl FilterMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Nearest, Self::Linear].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum FrontFace {
    Undefined = sys::WGPUFrontFace::Undefined,
    CCW = sys::WGPUFrontFace::CCW,
    CW = sys::WGPUFrontFace::CW,
}
impl FrontFace {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::CCW, Self::CW].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum IndexFormat {
    Undefined = sys::WGPUIndexFormat::Undefined,
    Uint16 = sys::WGPUIndexFormat::Uint16,
    Uint32 = sys::WGPUIndexFormat::Uint32,
}
impl IndexFormat {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Uint16, Self::Uint32].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum LoadOp {
    Undefined = sys::WGPULoadOp::Undefined,
    Load = sys::WGPULoadOp::Load,
    Clear = sys::WGPULoadOp::Clear,
    ExpandResolveTexture = sys::WGPULoadOp::ExpandResolveTexture,
}
impl LoadOp {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Load, Self::Clear, Self::ExpandResolveTexture]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum LoggingType {
    Verbose = sys::WGPULoggingType::Verbose,
    Info = sys::WGPULoggingType::Info,
    Warning = sys::WGPULoggingType::Warning,
    Error = sys::WGPULoggingType::Error,
}
impl LoggingType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Verbose, Self::Info, Self::Warning, Self::Error].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum MipmapFilterMode {
    Undefined = sys::WGPUMipmapFilterMode::Undefined,
    Nearest = sys::WGPUMipmapFilterMode::Nearest,
    Linear = sys::WGPUMipmapFilterMode::Linear,
}
impl MipmapFilterMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Nearest, Self::Linear].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum OptionalBool {
    False = sys::WGPUOptionalBool::False,
    True = sys::WGPUOptionalBool::True,
    Undefined = sys::WGPUOptionalBool::Undefined,
}
impl OptionalBool {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::False, Self::True, Self::Undefined].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum PowerPreference {
    Undefined = sys::WGPUPowerPreference::Undefined,
    LowPower = sys::WGPUPowerPreference::LowPower,
    HighPerformance = sys::WGPUPowerPreference::HighPerformance,
}
impl PowerPreference {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::LowPower, Self::HighPerformance].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum PredefinedColorSpace {
    SRGB = sys::WGPUPredefinedColorSpace::SRGB,
    DisplayP3 = sys::WGPUPredefinedColorSpace::DisplayP3,
}
impl PredefinedColorSpace {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::SRGB, Self::DisplayP3].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum PresentMode {
    Undefined = sys::WGPUPresentMode::Undefined,
    Fifo = sys::WGPUPresentMode::Fifo,
    FifoRelaxed = sys::WGPUPresentMode::FifoRelaxed,
    Immediate = sys::WGPUPresentMode::Immediate,
    Mailbox = sys::WGPUPresentMode::Mailbox,
}
impl PresentMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Fifo, Self::FifoRelaxed, Self::Immediate, Self::Mailbox]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum PrimitiveTopology {
    Undefined = sys::WGPUPrimitiveTopology::Undefined,
    PointList = sys::WGPUPrimitiveTopology::PointList,
    LineList = sys::WGPUPrimitiveTopology::LineList,
    LineStrip = sys::WGPUPrimitiveTopology::LineStrip,
    TriangleList = sys::WGPUPrimitiveTopology::TriangleList,
    TriangleStrip = sys::WGPUPrimitiveTopology::TriangleStrip,
}
impl PrimitiveTopology {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::PointList,
            Self::LineList,
            Self::LineStrip,
            Self::TriangleList,
            Self::TriangleStrip,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum QueryType {
    Occlusion = sys::WGPUQueryType::Occlusion,
    Timestamp = sys::WGPUQueryType::Timestamp,
}
impl QueryType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Occlusion, Self::Timestamp].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum SType {
    ShaderSourceSPIRV = sys::WGPUSType::ShaderSourceSPIRV,
    ShaderSourceWGSL = sys::WGPUSType::ShaderSourceWGSL,
    RenderPassMaxDrawCount = sys::WGPUSType::RenderPassMaxDrawCount,
    SurfaceSourceMetalLayer = sys::WGPUSType::SurfaceSourceMetalLayer,
    SurfaceSourceWindowsHWND = sys::WGPUSType::SurfaceSourceWindowsHWND,
    SurfaceSourceXlibWindow = sys::WGPUSType::SurfaceSourceXlibWindow,
    SurfaceSourceWaylandSurface = sys::WGPUSType::SurfaceSourceWaylandSurface,
    SurfaceSourceAndroidNativeWindow = sys::WGPUSType::SurfaceSourceAndroidNativeWindow,
    SurfaceSourceXCBWindow = sys::WGPUSType::SurfaceSourceXCBWindow,
    SurfaceColorManagement = sys::WGPUSType::SurfaceColorManagement,
    RequestAdapterWebXROptions = sys::WGPUSType::RequestAdapterWebXROptions,
    AdapterPropertiesSubgroups = sys::WGPUSType::AdapterPropertiesSubgroups,
    BindGroupLayoutEntryArraySize = sys::WGPUSType::BindGroupLayoutEntryArraySize,
    TextureBindingViewDimensionDescriptor = sys::WGPUSType::TextureBindingViewDimensionDescriptor,
    EmscriptenSurfaceSourceCanvasHTMLSelector = sys::WGPUSType::EmscriptenSurfaceSourceCanvasHTMLSelector,
    SurfaceDescriptorFromWindowsCoreWindow = sys::WGPUSType::SurfaceDescriptorFromWindowsCoreWindow,
    ExternalTextureBindingEntry = sys::WGPUSType::ExternalTextureBindingEntry,
    ExternalTextureBindingLayout = sys::WGPUSType::ExternalTextureBindingLayout,
    SurfaceDescriptorFromWindowsUWPSwapChainPanel = sys::WGPUSType::SurfaceDescriptorFromWindowsUWPSwapChainPanel,
    DawnTextureInternalUsageDescriptor = sys::WGPUSType::DawnTextureInternalUsageDescriptor,
    DawnEncoderInternalUsageDescriptor = sys::WGPUSType::DawnEncoderInternalUsageDescriptor,
    DawnInstanceDescriptor = sys::WGPUSType::DawnInstanceDescriptor,
    DawnCacheDeviceDescriptor = sys::WGPUSType::DawnCacheDeviceDescriptor,
    DawnAdapterPropertiesPowerPreference = sys::WGPUSType::DawnAdapterPropertiesPowerPreference,
    DawnBufferDescriptorErrorInfoFromWireClient = sys::WGPUSType::DawnBufferDescriptorErrorInfoFromWireClient,
    DawnTogglesDescriptor = sys::WGPUSType::DawnTogglesDescriptor,
    DawnShaderModuleSPIRVOptionsDescriptor = sys::WGPUSType::DawnShaderModuleSPIRVOptionsDescriptor,
    RequestAdapterOptionsLUID = sys::WGPUSType::RequestAdapterOptionsLUID,
    RequestAdapterOptionsGetGLProc = sys::WGPUSType::RequestAdapterOptionsGetGLProc,
    RequestAdapterOptionsD3D11Device = sys::WGPUSType::RequestAdapterOptionsD3D11Device,
    DawnRenderPassColorAttachmentRenderToSingleSampled = sys::WGPUSType::DawnRenderPassColorAttachmentRenderToSingleSampled,
    RenderPassPixelLocalStorage = sys::WGPUSType::RenderPassPixelLocalStorage,
    PipelineLayoutPixelLocalStorage = sys::WGPUSType::PipelineLayoutPixelLocalStorage,
    BufferHostMappedPointer = sys::WGPUSType::BufferHostMappedPointer,
    AdapterPropertiesMemoryHeaps = sys::WGPUSType::AdapterPropertiesMemoryHeaps,
    AdapterPropertiesD3D = sys::WGPUSType::AdapterPropertiesD3D,
    AdapterPropertiesVk = sys::WGPUSType::AdapterPropertiesVk,
    DawnWireWGSLControl = sys::WGPUSType::DawnWireWGSLControl,
    DawnWGSLBlocklist = sys::WGPUSType::DawnWGSLBlocklist,
    DawnDrmFormatCapabilities = sys::WGPUSType::DawnDrmFormatCapabilities,
    ShaderModuleCompilationOptions = sys::WGPUSType::ShaderModuleCompilationOptions,
    ColorTargetStateExpandResolveTextureDawn = sys::WGPUSType::ColorTargetStateExpandResolveTextureDawn,
    RenderPassDescriptorExpandResolveRect = sys::WGPUSType::RenderPassDescriptorExpandResolveRect,
    SharedTextureMemoryVkDedicatedAllocationDescriptor = sys::WGPUSType::SharedTextureMemoryVkDedicatedAllocationDescriptor,
    SharedTextureMemoryAHardwareBufferDescriptor = sys::WGPUSType::SharedTextureMemoryAHardwareBufferDescriptor,
    SharedTextureMemoryDmaBufDescriptor = sys::WGPUSType::SharedTextureMemoryDmaBufDescriptor,
    SharedTextureMemoryOpaqueFDDescriptor = sys::WGPUSType::SharedTextureMemoryOpaqueFDDescriptor,
    SharedTextureMemoryZirconHandleDescriptor = sys::WGPUSType::SharedTextureMemoryZirconHandleDescriptor,
    SharedTextureMemoryDXGISharedHandleDescriptor = sys::WGPUSType::SharedTextureMemoryDXGISharedHandleDescriptor,
    SharedTextureMemoryD3D11Texture2DDescriptor = sys::WGPUSType::SharedTextureMemoryD3D11Texture2DDescriptor,
    SharedTextureMemoryIOSurfaceDescriptor = sys::WGPUSType::SharedTextureMemoryIOSurfaceDescriptor,
    SharedTextureMemoryEGLImageDescriptor = sys::WGPUSType::SharedTextureMemoryEGLImageDescriptor,
    SharedTextureMemoryInitializedBeginState = sys::WGPUSType::SharedTextureMemoryInitializedBeginState,
    SharedTextureMemoryInitializedEndState = sys::WGPUSType::SharedTextureMemoryInitializedEndState,
    SharedTextureMemoryVkImageLayoutBeginState = sys::WGPUSType::SharedTextureMemoryVkImageLayoutBeginState,
    SharedTextureMemoryVkImageLayoutEndState = sys::WGPUSType::SharedTextureMemoryVkImageLayoutEndState,
    SharedTextureMemoryD3DSwapchainBeginState = sys::WGPUSType::SharedTextureMemoryD3DSwapchainBeginState,
    SharedFenceVkSemaphoreOpaqueFDDescriptor = sys::WGPUSType::SharedFenceVkSemaphoreOpaqueFDDescriptor,
    SharedFenceVkSemaphoreOpaqueFDExportInfo = sys::WGPUSType::SharedFenceVkSemaphoreOpaqueFDExportInfo,
    SharedFenceSyncFDDescriptor = sys::WGPUSType::SharedFenceSyncFDDescriptor,
    SharedFenceSyncFDExportInfo = sys::WGPUSType::SharedFenceSyncFDExportInfo,
    SharedFenceVkSemaphoreZirconHandleDescriptor = sys::WGPUSType::SharedFenceVkSemaphoreZirconHandleDescriptor,
    SharedFenceVkSemaphoreZirconHandleExportInfo = sys::WGPUSType::SharedFenceVkSemaphoreZirconHandleExportInfo,
    SharedFenceDXGISharedHandleDescriptor = sys::WGPUSType::SharedFenceDXGISharedHandleDescriptor,
    SharedFenceDXGISharedHandleExportInfo = sys::WGPUSType::SharedFenceDXGISharedHandleExportInfo,
    SharedFenceMTLSharedEventDescriptor = sys::WGPUSType::SharedFenceMTLSharedEventDescriptor,
    SharedFenceMTLSharedEventExportInfo = sys::WGPUSType::SharedFenceMTLSharedEventExportInfo,
    SharedBufferMemoryD3D12ResourceDescriptor = sys::WGPUSType::SharedBufferMemoryD3D12ResourceDescriptor,
    StaticSamplerBindingLayout = sys::WGPUSType::StaticSamplerBindingLayout,
    YCbCrVkDescriptor = sys::WGPUSType::YCbCrVkDescriptor,
    SharedTextureMemoryAHardwareBufferProperties = sys::WGPUSType::SharedTextureMemoryAHardwareBufferProperties,
    AHardwareBufferProperties = sys::WGPUSType::AHardwareBufferProperties,
    DawnTexelCopyBufferRowAlignmentLimits = sys::WGPUSType::DawnTexelCopyBufferRowAlignmentLimits,
    AdapterPropertiesSubgroupMatrixConfigs = sys::WGPUSType::AdapterPropertiesSubgroupMatrixConfigs,
    SharedFenceEGLSyncDescriptor = sys::WGPUSType::SharedFenceEGLSyncDescriptor,
    SharedFenceEGLSyncExportInfo = sys::WGPUSType::SharedFenceEGLSyncExportInfo,
    DawnInjectedInvalidSType = sys::WGPUSType::DawnInjectedInvalidSType,
    DawnCompilationMessageUtf16 = sys::WGPUSType::DawnCompilationMessageUtf16,
    DawnFakeBufferOOMForTesting = sys::WGPUSType::DawnFakeBufferOOMForTesting,
    SurfaceDescriptorFromWindowsWinUISwapChainPanel = sys::WGPUSType::SurfaceDescriptorFromWindowsWinUISwapChainPanel,
    DawnDeviceAllocatorControl = sys::WGPUSType::DawnDeviceAllocatorControl,
    DawnHostMappedPointerLimits = sys::WGPUSType::DawnHostMappedPointerLimits,
    RenderPassDescriptorResolveRect = sys::WGPUSType::RenderPassDescriptorResolveRect,
}
impl SType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::ShaderSourceSPIRV,
            Self::ShaderSourceWGSL,
            Self::RenderPassMaxDrawCount,
            Self::SurfaceSourceMetalLayer,
            Self::SurfaceSourceWindowsHWND,
            Self::SurfaceSourceXlibWindow,
            Self::SurfaceSourceWaylandSurface,
            Self::SurfaceSourceAndroidNativeWindow,
            Self::SurfaceSourceXCBWindow,
            Self::SurfaceColorManagement,
            Self::RequestAdapterWebXROptions,
            Self::AdapterPropertiesSubgroups,
            Self::BindGroupLayoutEntryArraySize,
            Self::TextureBindingViewDimensionDescriptor,
            Self::EmscriptenSurfaceSourceCanvasHTMLSelector,
            Self::SurfaceDescriptorFromWindowsCoreWindow,
            Self::ExternalTextureBindingEntry,
            Self::ExternalTextureBindingLayout,
            Self::SurfaceDescriptorFromWindowsUWPSwapChainPanel,
            Self::DawnTextureInternalUsageDescriptor,
            Self::DawnEncoderInternalUsageDescriptor,
            Self::DawnInstanceDescriptor,
            Self::DawnCacheDeviceDescriptor,
            Self::DawnAdapterPropertiesPowerPreference,
            Self::DawnBufferDescriptorErrorInfoFromWireClient,
            Self::DawnTogglesDescriptor,
            Self::DawnShaderModuleSPIRVOptionsDescriptor,
            Self::RequestAdapterOptionsLUID,
            Self::RequestAdapterOptionsGetGLProc,
            Self::RequestAdapterOptionsD3D11Device,
            Self::DawnRenderPassColorAttachmentRenderToSingleSampled,
            Self::RenderPassPixelLocalStorage,
            Self::PipelineLayoutPixelLocalStorage,
            Self::BufferHostMappedPointer,
            Self::AdapterPropertiesMemoryHeaps,
            Self::AdapterPropertiesD3D,
            Self::AdapterPropertiesVk,
            Self::DawnWireWGSLControl,
            Self::DawnWGSLBlocklist,
            Self::DawnDrmFormatCapabilities,
            Self::ShaderModuleCompilationOptions,
            Self::ColorTargetStateExpandResolveTextureDawn,
            Self::RenderPassDescriptorExpandResolveRect,
            Self::SharedTextureMemoryVkDedicatedAllocationDescriptor,
            Self::SharedTextureMemoryAHardwareBufferDescriptor,
            Self::SharedTextureMemoryDmaBufDescriptor,
            Self::SharedTextureMemoryOpaqueFDDescriptor,
            Self::SharedTextureMemoryZirconHandleDescriptor,
            Self::SharedTextureMemoryDXGISharedHandleDescriptor,
            Self::SharedTextureMemoryD3D11Texture2DDescriptor,
            Self::SharedTextureMemoryIOSurfaceDescriptor,
            Self::SharedTextureMemoryEGLImageDescriptor,
            Self::SharedTextureMemoryInitializedBeginState,
            Self::SharedTextureMemoryInitializedEndState,
            Self::SharedTextureMemoryVkImageLayoutBeginState,
            Self::SharedTextureMemoryVkImageLayoutEndState,
            Self::SharedTextureMemoryD3DSwapchainBeginState,
            Self::SharedFenceVkSemaphoreOpaqueFDDescriptor,
            Self::SharedFenceVkSemaphoreOpaqueFDExportInfo,
            Self::SharedFenceSyncFDDescriptor,
            Self::SharedFenceSyncFDExportInfo,
            Self::SharedFenceVkSemaphoreZirconHandleDescriptor,
            Self::SharedFenceVkSemaphoreZirconHandleExportInfo,
            Self::SharedFenceDXGISharedHandleDescriptor,
            Self::SharedFenceDXGISharedHandleExportInfo,
            Self::SharedFenceMTLSharedEventDescriptor,
            Self::SharedFenceMTLSharedEventExportInfo,
            Self::SharedBufferMemoryD3D12ResourceDescriptor,
            Self::StaticSamplerBindingLayout,
            Self::YCbCrVkDescriptor,
            Self::SharedTextureMemoryAHardwareBufferProperties,
            Self::AHardwareBufferProperties,
            Self::DawnTexelCopyBufferRowAlignmentLimits,
            Self::AdapterPropertiesSubgroupMatrixConfigs,
            Self::SharedFenceEGLSyncDescriptor,
            Self::SharedFenceEGLSyncExportInfo,
            Self::DawnInjectedInvalidSType,
            Self::DawnCompilationMessageUtf16,
            Self::DawnFakeBufferOOMForTesting,
            Self::SurfaceDescriptorFromWindowsWinUISwapChainPanel,
            Self::DawnDeviceAllocatorControl,
            Self::DawnHostMappedPointerLimits,
            Self::RenderPassDescriptorResolveRect,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum SamplerBindingType {
    BindingNotUsed = sys::WGPUSamplerBindingType::BindingNotUsed,
    Undefined = sys::WGPUSamplerBindingType::Undefined,
    Filtering = sys::WGPUSamplerBindingType::Filtering,
    NonFiltering = sys::WGPUSamplerBindingType::NonFiltering,
    Comparison = sys::WGPUSamplerBindingType::Comparison,
}
impl SamplerBindingType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::BindingNotUsed,
            Self::Undefined,
            Self::Filtering,
            Self::NonFiltering,
            Self::Comparison,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum SharedFenceType {
    VkSemaphoreOpaqueFD = sys::WGPUSharedFenceType::VkSemaphoreOpaqueFD,
    SyncFD = sys::WGPUSharedFenceType::SyncFD,
    VkSemaphoreZirconHandle = sys::WGPUSharedFenceType::VkSemaphoreZirconHandle,
    DXGISharedHandle = sys::WGPUSharedFenceType::DXGISharedHandle,
    MTLSharedEvent = sys::WGPUSharedFenceType::MTLSharedEvent,
    EGLSync = sys::WGPUSharedFenceType::EGLSync,
}
impl SharedFenceType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::VkSemaphoreOpaqueFD,
            Self::SyncFD,
            Self::VkSemaphoreZirconHandle,
            Self::DXGISharedHandle,
            Self::MTLSharedEvent,
            Self::EGLSync,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum StencilOperation {
    Undefined = sys::WGPUStencilOperation::Undefined,
    Keep = sys::WGPUStencilOperation::Keep,
    Zero = sys::WGPUStencilOperation::Zero,
    Replace = sys::WGPUStencilOperation::Replace,
    Invert = sys::WGPUStencilOperation::Invert,
    IncrementClamp = sys::WGPUStencilOperation::IncrementClamp,
    DecrementClamp = sys::WGPUStencilOperation::DecrementClamp,
    IncrementWrap = sys::WGPUStencilOperation::IncrementWrap,
    DecrementWrap = sys::WGPUStencilOperation::DecrementWrap,
}
impl StencilOperation {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::Keep,
            Self::Zero,
            Self::Replace,
            Self::Invert,
            Self::IncrementClamp,
            Self::DecrementClamp,
            Self::IncrementWrap,
            Self::DecrementWrap,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum StorageTextureAccess {
    BindingNotUsed = sys::WGPUStorageTextureAccess::BindingNotUsed,
    Undefined = sys::WGPUStorageTextureAccess::Undefined,
    WriteOnly = sys::WGPUStorageTextureAccess::WriteOnly,
    ReadOnly = sys::WGPUStorageTextureAccess::ReadOnly,
    ReadWrite = sys::WGPUStorageTextureAccess::ReadWrite,
}
impl StorageTextureAccess {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::BindingNotUsed,
            Self::Undefined,
            Self::WriteOnly,
            Self::ReadOnly,
            Self::ReadWrite,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum StoreOp {
    Undefined = sys::WGPUStoreOp::Undefined,
    Store = sys::WGPUStoreOp::Store,
    Discard = sys::WGPUStoreOp::Discard,
}
impl StoreOp {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Store, Self::Discard].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum SubgroupMatrixComponentType {
    F32 = sys::WGPUSubgroupMatrixComponentType::F32,
    F16 = sys::WGPUSubgroupMatrixComponentType::F16,
    U32 = sys::WGPUSubgroupMatrixComponentType::U32,
    I32 = sys::WGPUSubgroupMatrixComponentType::I32,
}
impl SubgroupMatrixComponentType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::F32, Self::F16, Self::U32, Self::I32].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum TextureAspect {
    Undefined = sys::WGPUTextureAspect::Undefined,
    All = sys::WGPUTextureAspect::All,
    StencilOnly = sys::WGPUTextureAspect::StencilOnly,
    DepthOnly = sys::WGPUTextureAspect::DepthOnly,
    Plane0Only = sys::WGPUTextureAspect::Plane0Only,
    Plane1Only = sys::WGPUTextureAspect::Plane1Only,
    Plane2Only = sys::WGPUTextureAspect::Plane2Only,
}
impl TextureAspect {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::All,
            Self::StencilOnly,
            Self::DepthOnly,
            Self::Plane0Only,
            Self::Plane1Only,
            Self::Plane2Only,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum TextureDimension {
    Undefined = sys::WGPUTextureDimension::Undefined,
    D1 = sys::WGPUTextureDimension::D1,
    D2 = sys::WGPUTextureDimension::D2,
    D3 = sys::WGPUTextureDimension::D3,
}
impl TextureDimension {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::D1, Self::D2, Self::D3].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum TextureFormat {
    Undefined = sys::WGPUTextureFormat::Undefined,
    R8Unorm = sys::WGPUTextureFormat::R8Unorm,
    R8Snorm = sys::WGPUTextureFormat::R8Snorm,
    R8Uint = sys::WGPUTextureFormat::R8Uint,
    R8Sint = sys::WGPUTextureFormat::R8Sint,
    R16Uint = sys::WGPUTextureFormat::R16Uint,
    R16Sint = sys::WGPUTextureFormat::R16Sint,
    R16Float = sys::WGPUTextureFormat::R16Float,
    RG8Unorm = sys::WGPUTextureFormat::RG8Unorm,
    RG8Snorm = sys::WGPUTextureFormat::RG8Snorm,
    RG8Uint = sys::WGPUTextureFormat::RG8Uint,
    RG8Sint = sys::WGPUTextureFormat::RG8Sint,
    R32Float = sys::WGPUTextureFormat::R32Float,
    R32Uint = sys::WGPUTextureFormat::R32Uint,
    R32Sint = sys::WGPUTextureFormat::R32Sint,
    RG16Uint = sys::WGPUTextureFormat::RG16Uint,
    RG16Sint = sys::WGPUTextureFormat::RG16Sint,
    RG16Float = sys::WGPUTextureFormat::RG16Float,
    RGBA8Unorm = sys::WGPUTextureFormat::RGBA8Unorm,
    RGBA8UnormSrgb = sys::WGPUTextureFormat::RGBA8UnormSrgb,
    RGBA8Snorm = sys::WGPUTextureFormat::RGBA8Snorm,
    RGBA8Uint = sys::WGPUTextureFormat::RGBA8Uint,
    RGBA8Sint = sys::WGPUTextureFormat::RGBA8Sint,
    BGRA8Unorm = sys::WGPUTextureFormat::BGRA8Unorm,
    BGRA8UnormSrgb = sys::WGPUTextureFormat::BGRA8UnormSrgb,
    RGB10A2Uint = sys::WGPUTextureFormat::RGB10A2Uint,
    RGB10A2Unorm = sys::WGPUTextureFormat::RGB10A2Unorm,
    RG11B10Ufloat = sys::WGPUTextureFormat::RG11B10Ufloat,
    RGB9E5Ufloat = sys::WGPUTextureFormat::RGB9E5Ufloat,
    RG32Float = sys::WGPUTextureFormat::RG32Float,
    RG32Uint = sys::WGPUTextureFormat::RG32Uint,
    RG32Sint = sys::WGPUTextureFormat::RG32Sint,
    RGBA16Uint = sys::WGPUTextureFormat::RGBA16Uint,
    RGBA16Sint = sys::WGPUTextureFormat::RGBA16Sint,
    RGBA16Float = sys::WGPUTextureFormat::RGBA16Float,
    RGBA32Float = sys::WGPUTextureFormat::RGBA32Float,
    RGBA32Uint = sys::WGPUTextureFormat::RGBA32Uint,
    RGBA32Sint = sys::WGPUTextureFormat::RGBA32Sint,
    Stencil8 = sys::WGPUTextureFormat::Stencil8,
    Depth16Unorm = sys::WGPUTextureFormat::Depth16Unorm,
    Depth24Plus = sys::WGPUTextureFormat::Depth24Plus,
    Depth24PlusStencil8 = sys::WGPUTextureFormat::Depth24PlusStencil8,
    Depth32Float = sys::WGPUTextureFormat::Depth32Float,
    Depth32FloatStencil8 = sys::WGPUTextureFormat::Depth32FloatStencil8,
    BC1RGBAUnorm = sys::WGPUTextureFormat::BC1RGBAUnorm,
    BC1RGBAUnormSrgb = sys::WGPUTextureFormat::BC1RGBAUnormSrgb,
    BC2RGBAUnorm = sys::WGPUTextureFormat::BC2RGBAUnorm,
    BC2RGBAUnormSrgb = sys::WGPUTextureFormat::BC2RGBAUnormSrgb,
    BC3RGBAUnorm = sys::WGPUTextureFormat::BC3RGBAUnorm,
    BC3RGBAUnormSrgb = sys::WGPUTextureFormat::BC3RGBAUnormSrgb,
    BC4RUnorm = sys::WGPUTextureFormat::BC4RUnorm,
    BC4RSnorm = sys::WGPUTextureFormat::BC4RSnorm,
    BC5RGUnorm = sys::WGPUTextureFormat::BC5RGUnorm,
    BC5RGSnorm = sys::WGPUTextureFormat::BC5RGSnorm,
    BC6HRGBUfloat = sys::WGPUTextureFormat::BC6HRGBUfloat,
    BC6HRGBFloat = sys::WGPUTextureFormat::BC6HRGBFloat,
    BC7RGBAUnorm = sys::WGPUTextureFormat::BC7RGBAUnorm,
    BC7RGBAUnormSrgb = sys::WGPUTextureFormat::BC7RGBAUnormSrgb,
    ETC2RGB8Unorm = sys::WGPUTextureFormat::ETC2RGB8Unorm,
    ETC2RGB8UnormSrgb = sys::WGPUTextureFormat::ETC2RGB8UnormSrgb,
    ETC2RGB8A1Unorm = sys::WGPUTextureFormat::ETC2RGB8A1Unorm,
    ETC2RGB8A1UnormSrgb = sys::WGPUTextureFormat::ETC2RGB8A1UnormSrgb,
    ETC2RGBA8Unorm = sys::WGPUTextureFormat::ETC2RGBA8Unorm,
    ETC2RGBA8UnormSrgb = sys::WGPUTextureFormat::ETC2RGBA8UnormSrgb,
    EACR11Unorm = sys::WGPUTextureFormat::EACR11Unorm,
    EACR11Snorm = sys::WGPUTextureFormat::EACR11Snorm,
    EACRG11Unorm = sys::WGPUTextureFormat::EACRG11Unorm,
    EACRG11Snorm = sys::WGPUTextureFormat::EACRG11Snorm,
    ASTC4x4Unorm = sys::WGPUTextureFormat::ASTC4x4Unorm,
    ASTC4x4UnormSrgb = sys::WGPUTextureFormat::ASTC4x4UnormSrgb,
    ASTC5x4Unorm = sys::WGPUTextureFormat::ASTC5x4Unorm,
    ASTC5x4UnormSrgb = sys::WGPUTextureFormat::ASTC5x4UnormSrgb,
    ASTC5x5Unorm = sys::WGPUTextureFormat::ASTC5x5Unorm,
    ASTC5x5UnormSrgb = sys::WGPUTextureFormat::ASTC5x5UnormSrgb,
    ASTC6x5Unorm = sys::WGPUTextureFormat::ASTC6x5Unorm,
    ASTC6x5UnormSrgb = sys::WGPUTextureFormat::ASTC6x5UnormSrgb,
    ASTC6x6Unorm = sys::WGPUTextureFormat::ASTC6x6Unorm,
    ASTC6x6UnormSrgb = sys::WGPUTextureFormat::ASTC6x6UnormSrgb,
    ASTC8x5Unorm = sys::WGPUTextureFormat::ASTC8x5Unorm,
    ASTC8x5UnormSrgb = sys::WGPUTextureFormat::ASTC8x5UnormSrgb,
    ASTC8x6Unorm = sys::WGPUTextureFormat::ASTC8x6Unorm,
    ASTC8x6UnormSrgb = sys::WGPUTextureFormat::ASTC8x6UnormSrgb,
    ASTC8x8Unorm = sys::WGPUTextureFormat::ASTC8x8Unorm,
    ASTC8x8UnormSrgb = sys::WGPUTextureFormat::ASTC8x8UnormSrgb,
    ASTC10x5Unorm = sys::WGPUTextureFormat::ASTC10x5Unorm,
    ASTC10x5UnormSrgb = sys::WGPUTextureFormat::ASTC10x5UnormSrgb,
    ASTC10x6Unorm = sys::WGPUTextureFormat::ASTC10x6Unorm,
    ASTC10x6UnormSrgb = sys::WGPUTextureFormat::ASTC10x6UnormSrgb,
    ASTC10x8Unorm = sys::WGPUTextureFormat::ASTC10x8Unorm,
    ASTC10x8UnormSrgb = sys::WGPUTextureFormat::ASTC10x8UnormSrgb,
    ASTC10x10Unorm = sys::WGPUTextureFormat::ASTC10x10Unorm,
    ASTC10x10UnormSrgb = sys::WGPUTextureFormat::ASTC10x10UnormSrgb,
    ASTC12x10Unorm = sys::WGPUTextureFormat::ASTC12x10Unorm,
    ASTC12x10UnormSrgb = sys::WGPUTextureFormat::ASTC12x10UnormSrgb,
    ASTC12x12Unorm = sys::WGPUTextureFormat::ASTC12x12Unorm,
    ASTC12x12UnormSrgb = sys::WGPUTextureFormat::ASTC12x12UnormSrgb,
    R16Unorm = sys::WGPUTextureFormat::R16Unorm,
    RG16Unorm = sys::WGPUTextureFormat::RG16Unorm,
    RGBA16Unorm = sys::WGPUTextureFormat::RGBA16Unorm,
    R16Snorm = sys::WGPUTextureFormat::R16Snorm,
    RG16Snorm = sys::WGPUTextureFormat::RG16Snorm,
    RGBA16Snorm = sys::WGPUTextureFormat::RGBA16Snorm,
    R8BG8Biplanar420Unorm = sys::WGPUTextureFormat::R8BG8Biplanar420Unorm,
    R10X6BG10X6Biplanar420Unorm = sys::WGPUTextureFormat::R10X6BG10X6Biplanar420Unorm,
    R8BG8A8Triplanar420Unorm = sys::WGPUTextureFormat::R8BG8A8Triplanar420Unorm,
    R8BG8Biplanar422Unorm = sys::WGPUTextureFormat::R8BG8Biplanar422Unorm,
    R8BG8Biplanar444Unorm = sys::WGPUTextureFormat::R8BG8Biplanar444Unorm,
    R10X6BG10X6Biplanar422Unorm = sys::WGPUTextureFormat::R10X6BG10X6Biplanar422Unorm,
    R10X6BG10X6Biplanar444Unorm = sys::WGPUTextureFormat::R10X6BG10X6Biplanar444Unorm,
    External = sys::WGPUTextureFormat::External,
}
impl TextureFormat {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::R8Unorm,
            Self::R8Snorm,
            Self::R8Uint,
            Self::R8Sint,
            Self::R16Uint,
            Self::R16Sint,
            Self::R16Float,
            Self::RG8Unorm,
            Self::RG8Snorm,
            Self::RG8Uint,
            Self::RG8Sint,
            Self::R32Float,
            Self::R32Uint,
            Self::R32Sint,
            Self::RG16Uint,
            Self::RG16Sint,
            Self::RG16Float,
            Self::RGBA8Unorm,
            Self::RGBA8UnormSrgb,
            Self::RGBA8Snorm,
            Self::RGBA8Uint,
            Self::RGBA8Sint,
            Self::BGRA8Unorm,
            Self::BGRA8UnormSrgb,
            Self::RGB10A2Uint,
            Self::RGB10A2Unorm,
            Self::RG11B10Ufloat,
            Self::RGB9E5Ufloat,
            Self::RG32Float,
            Self::RG32Uint,
            Self::RG32Sint,
            Self::RGBA16Uint,
            Self::RGBA16Sint,
            Self::RGBA16Float,
            Self::RGBA32Float,
            Self::RGBA32Uint,
            Self::RGBA32Sint,
            Self::Stencil8,
            Self::Depth16Unorm,
            Self::Depth24Plus,
            Self::Depth24PlusStencil8,
            Self::Depth32Float,
            Self::Depth32FloatStencil8,
            Self::BC1RGBAUnorm,
            Self::BC1RGBAUnormSrgb,
            Self::BC2RGBAUnorm,
            Self::BC2RGBAUnormSrgb,
            Self::BC3RGBAUnorm,
            Self::BC3RGBAUnormSrgb,
            Self::BC4RUnorm,
            Self::BC4RSnorm,
            Self::BC5RGUnorm,
            Self::BC5RGSnorm,
            Self::BC6HRGBUfloat,
            Self::BC6HRGBFloat,
            Self::BC7RGBAUnorm,
            Self::BC7RGBAUnormSrgb,
            Self::ETC2RGB8Unorm,
            Self::ETC2RGB8UnormSrgb,
            Self::ETC2RGB8A1Unorm,
            Self::ETC2RGB8A1UnormSrgb,
            Self::ETC2RGBA8Unorm,
            Self::ETC2RGBA8UnormSrgb,
            Self::EACR11Unorm,
            Self::EACR11Snorm,
            Self::EACRG11Unorm,
            Self::EACRG11Snorm,
            Self::ASTC4x4Unorm,
            Self::ASTC4x4UnormSrgb,
            Self::ASTC5x4Unorm,
            Self::ASTC5x4UnormSrgb,
            Self::ASTC5x5Unorm,
            Self::ASTC5x5UnormSrgb,
            Self::ASTC6x5Unorm,
            Self::ASTC6x5UnormSrgb,
            Self::ASTC6x6Unorm,
            Self::ASTC6x6UnormSrgb,
            Self::ASTC8x5Unorm,
            Self::ASTC8x5UnormSrgb,
            Self::ASTC8x6Unorm,
            Self::ASTC8x6UnormSrgb,
            Self::ASTC8x8Unorm,
            Self::ASTC8x8UnormSrgb,
            Self::ASTC10x5Unorm,
            Self::ASTC10x5UnormSrgb,
            Self::ASTC10x6Unorm,
            Self::ASTC10x6UnormSrgb,
            Self::ASTC10x8Unorm,
            Self::ASTC10x8UnormSrgb,
            Self::ASTC10x10Unorm,
            Self::ASTC10x10UnormSrgb,
            Self::ASTC12x10Unorm,
            Self::ASTC12x10UnormSrgb,
            Self::ASTC12x12Unorm,
            Self::ASTC12x12UnormSrgb,
            Self::R16Unorm,
            Self::RG16Unorm,
            Self::RGBA16Unorm,
            Self::R16Snorm,
            Self::RG16Snorm,
            Self::RGBA16Snorm,
            Self::R8BG8Biplanar420Unorm,
            Self::R10X6BG10X6Biplanar420Unorm,
            Self::R8BG8A8Triplanar420Unorm,
            Self::R8BG8Biplanar422Unorm,
            Self::R8BG8Biplanar444Unorm,
            Self::R10X6BG10X6Biplanar422Unorm,
            Self::R10X6BG10X6Biplanar444Unorm,
            Self::External,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum TextureSampleType {
    BindingNotUsed = sys::WGPUTextureSampleType::BindingNotUsed,
    Undefined = sys::WGPUTextureSampleType::Undefined,
    Float = sys::WGPUTextureSampleType::Float,
    UnfilterableFloat = sys::WGPUTextureSampleType::UnfilterableFloat,
    Depth = sys::WGPUTextureSampleType::Depth,
    Sint = sys::WGPUTextureSampleType::Sint,
    Uint = sys::WGPUTextureSampleType::Uint,
}
impl TextureSampleType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::BindingNotUsed,
            Self::Undefined,
            Self::Float,
            Self::UnfilterableFloat,
            Self::Depth,
            Self::Sint,
            Self::Uint,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum TextureViewDimension {
    Undefined = sys::WGPUTextureViewDimension::Undefined,
    D1 = sys::WGPUTextureViewDimension::D1,
    D2 = sys::WGPUTextureViewDimension::D2,
    D2Array = sys::WGPUTextureViewDimension::D2Array,
    Cube = sys::WGPUTextureViewDimension::Cube,
    CubeArray = sys::WGPUTextureViewDimension::CubeArray,
    D3 = sys::WGPUTextureViewDimension::D3,
}
impl TextureViewDimension {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Undefined,
            Self::D1,
            Self::D2,
            Self::D2Array,
            Self::Cube,
            Self::CubeArray,
            Self::D3,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum ToneMappingMode {
    Standard = sys::WGPUToneMappingMode::Standard,
    Extended = sys::WGPUToneMappingMode::Extended,
}
impl ToneMappingMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Standard, Self::Extended].into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum VertexFormat {
    Uint8 = sys::WGPUVertexFormat::Uint8,
    Uint8x2 = sys::WGPUVertexFormat::Uint8x2,
    Uint8x4 = sys::WGPUVertexFormat::Uint8x4,
    Sint8 = sys::WGPUVertexFormat::Sint8,
    Sint8x2 = sys::WGPUVertexFormat::Sint8x2,
    Sint8x4 = sys::WGPUVertexFormat::Sint8x4,
    Unorm8 = sys::WGPUVertexFormat::Unorm8,
    Unorm8x2 = sys::WGPUVertexFormat::Unorm8x2,
    Unorm8x4 = sys::WGPUVertexFormat::Unorm8x4,
    Snorm8 = sys::WGPUVertexFormat::Snorm8,
    Snorm8x2 = sys::WGPUVertexFormat::Snorm8x2,
    Snorm8x4 = sys::WGPUVertexFormat::Snorm8x4,
    Uint16 = sys::WGPUVertexFormat::Uint16,
    Uint16x2 = sys::WGPUVertexFormat::Uint16x2,
    Uint16x4 = sys::WGPUVertexFormat::Uint16x4,
    Sint16 = sys::WGPUVertexFormat::Sint16,
    Sint16x2 = sys::WGPUVertexFormat::Sint16x2,
    Sint16x4 = sys::WGPUVertexFormat::Sint16x4,
    Unorm16 = sys::WGPUVertexFormat::Unorm16,
    Unorm16x2 = sys::WGPUVertexFormat::Unorm16x2,
    Unorm16x4 = sys::WGPUVertexFormat::Unorm16x4,
    Snorm16 = sys::WGPUVertexFormat::Snorm16,
    Snorm16x2 = sys::WGPUVertexFormat::Snorm16x2,
    Snorm16x4 = sys::WGPUVertexFormat::Snorm16x4,
    Float16 = sys::WGPUVertexFormat::Float16,
    Float16x2 = sys::WGPUVertexFormat::Float16x2,
    Float16x4 = sys::WGPUVertexFormat::Float16x4,
    Float32 = sys::WGPUVertexFormat::Float32,
    Float32x2 = sys::WGPUVertexFormat::Float32x2,
    Float32x3 = sys::WGPUVertexFormat::Float32x3,
    Float32x4 = sys::WGPUVertexFormat::Float32x4,
    Uint32 = sys::WGPUVertexFormat::Uint32,
    Uint32x2 = sys::WGPUVertexFormat::Uint32x2,
    Uint32x3 = sys::WGPUVertexFormat::Uint32x3,
    Uint32x4 = sys::WGPUVertexFormat::Uint32x4,
    Sint32 = sys::WGPUVertexFormat::Sint32,
    Sint32x2 = sys::WGPUVertexFormat::Sint32x2,
    Sint32x3 = sys::WGPUVertexFormat::Sint32x3,
    Sint32x4 = sys::WGPUVertexFormat::Sint32x4,
    Unorm10_10_10_2 = sys::WGPUVertexFormat::Unorm10_10_10_2,
    Unorm8x4BGRA = sys::WGPUVertexFormat::Unorm8x4BGRA,
}
impl VertexFormat {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Self::Uint8,
            Self::Uint8x2,
            Self::Uint8x4,
            Self::Sint8,
            Self::Sint8x2,
            Self::Sint8x4,
            Self::Unorm8,
            Self::Unorm8x2,
            Self::Unorm8x4,
            Self::Snorm8,
            Self::Snorm8x2,
            Self::Snorm8x4,
            Self::Uint16,
            Self::Uint16x2,
            Self::Uint16x4,
            Self::Sint16,
            Self::Sint16x2,
            Self::Sint16x4,
            Self::Unorm16,
            Self::Unorm16x2,
            Self::Unorm16x4,
            Self::Snorm16,
            Self::Snorm16x2,
            Self::Snorm16x4,
            Self::Float16,
            Self::Float16x2,
            Self::Float16x4,
            Self::Float32,
            Self::Float32x2,
            Self::Float32x3,
            Self::Float32x4,
            Self::Uint32,
            Self::Uint32x2,
            Self::Uint32x3,
            Self::Uint32x4,
            Self::Sint32,
            Self::Sint32x2,
            Self::Sint32x3,
            Self::Sint32x4,
            Self::Unorm10_10_10_2,
            Self::Unorm8x4BGRA,
        ]
            .into_iter()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)]
#[repr(i32)]
pub enum VertexStepMode {
    Undefined = sys::WGPUVertexStepMode::Undefined,
    Vertex = sys::WGPUVertexStepMode::Vertex,
    Instance = sys::WGPUVertexStepMode::Instance,
}
impl VertexStepMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Undefined, Self::Vertex, Self::Instance].into_iter()
    }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct WGPUBufferUsage :
    sys::WGPUFlags { const NONE = sys::WGPUBufferUsage::NONE; const MAP_READ =
    sys::WGPUBufferUsage::MAP_READ; const MAP_WRITE = sys::WGPUBufferUsage::MAP_WRITE;
    const COPY_SRC = sys::WGPUBufferUsage::COPY_SRC; const COPY_DST =
    sys::WGPUBufferUsage::COPY_DST; const INDEX = sys::WGPUBufferUsage::INDEX; const
    VERTEX = sys::WGPUBufferUsage::VERTEX; const UNIFORM = sys::WGPUBufferUsage::UNIFORM;
    const STORAGE = sys::WGPUBufferUsage::STORAGE; const INDIRECT =
    sys::WGPUBufferUsage::INDIRECT; const QUERY_RESOLVE =
    sys::WGPUBufferUsage::QUERY_RESOLVE; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct WGPUColorWriteMask :
    sys::WGPUFlags { const NONE = sys::WGPUColorWriteMask::NONE; const RED =
    sys::WGPUColorWriteMask::RED; const GREEN = sys::WGPUColorWriteMask::GREEN; const
    BLUE = sys::WGPUColorWriteMask::BLUE; const ALPHA = sys::WGPUColorWriteMask::ALPHA;
    const ALL = sys::WGPUColorWriteMask::ALL; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct WGPUHeapProperty :
    sys::WGPUFlags { const NONE = sys::WGPUHeapProperty::NONE; const DEVICE_LOCAL =
    sys::WGPUHeapProperty::DEVICE_LOCAL; const HOST_VISIBLE =
    sys::WGPUHeapProperty::HOST_VISIBLE; const HOST_COHERENT =
    sys::WGPUHeapProperty::HOST_COHERENT; const HOST_UNCACHED =
    sys::WGPUHeapProperty::HOST_UNCACHED; const HOST_CACHED =
    sys::WGPUHeapProperty::HOST_CACHED; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct WGPUMapMode :
    sys::WGPUFlags { const NONE = sys::WGPUMapMode::NONE; const READ =
    sys::WGPUMapMode::READ; const WRITE = sys::WGPUMapMode::WRITE; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct WGPUShaderStage :
    sys::WGPUFlags { const NONE = sys::WGPUShaderStage::NONE; const VERTEX =
    sys::WGPUShaderStage::VERTEX; const FRAGMENT = sys::WGPUShaderStage::FRAGMENT; const
    COMPUTE = sys::WGPUShaderStage::COMPUTE; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] pub struct WGPUTextureUsage :
    sys::WGPUFlags { const NONE = sys::WGPUTextureUsage::NONE; const COPY_SRC =
    sys::WGPUTextureUsage::COPY_SRC; const COPY_DST = sys::WGPUTextureUsage::COPY_DST;
    const TEXTURE_BINDING = sys::WGPUTextureUsage::TEXTURE_BINDING; const STORAGE_BINDING
    = sys::WGPUTextureUsage::STORAGE_BINDING; const RENDER_ATTACHMENT =
    sys::WGPUTextureUsage::RENDER_ATTACHMENT; const TRANSIENT_ATTACHMENT =
    sys::WGPUTextureUsage::TRANSIENT_ATTACHMENT; const STORAGE_ATTACHMENT =
    sys::WGPUTextureUsage::STORAGE_ATTACHMENT; }
}
pub struct Adapter(sys::WGPUAdapter);
impl Clone for Adapter {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuAdapterAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Adapter {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuAdapterRelease(self.0);
        }
    }
}
pub struct BindGroup(sys::WGPUBindGroup);
impl Clone for BindGroup {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuBindGroupAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for BindGroup {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuBindGroupRelease(self.0);
        }
    }
}
pub struct BindGroupLayout(sys::WGPUBindGroupLayout);
impl Clone for BindGroupLayout {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuBindGroupLayoutAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for BindGroupLayout {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuBindGroupLayoutRelease(self.0);
        }
    }
}
pub struct Buffer(sys::WGPUBuffer);
impl Clone for Buffer {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuBufferAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuBufferRelease(self.0);
        }
    }
}
pub struct CommandBuffer(sys::WGPUCommandBuffer);
impl Clone for CommandBuffer {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuCommandBufferAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for CommandBuffer {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuCommandBufferRelease(self.0);
        }
    }
}
pub struct CommandEncoder(sys::WGPUCommandEncoder);
impl Clone for CommandEncoder {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuCommandEncoderAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for CommandEncoder {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuCommandEncoderRelease(self.0);
        }
    }
}
pub struct ComputePassEncoder(sys::WGPUComputePassEncoder);
impl Clone for ComputePassEncoder {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuComputePassEncoderAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for ComputePassEncoder {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuComputePassEncoderRelease(self.0);
        }
    }
}
pub struct ComputePipeline(sys::WGPUComputePipeline);
impl Clone for ComputePipeline {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuComputePipelineAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for ComputePipeline {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuComputePipelineRelease(self.0);
        }
    }
}
pub struct Device(sys::WGPUDevice);
impl Clone for Device {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuDeviceAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuDeviceRelease(self.0);
        }
    }
}
pub struct ExternalTexture(sys::WGPUExternalTexture);
impl Clone for ExternalTexture {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuExternalTextureAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for ExternalTexture {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuExternalTextureRelease(self.0);
        }
    }
}
pub struct Instance(sys::WGPUInstance);
impl Clone for Instance {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuInstanceAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Instance {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuInstanceRelease(self.0);
        }
    }
}
pub struct PipelineLayout(sys::WGPUPipelineLayout);
impl Clone for PipelineLayout {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuPipelineLayoutAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for PipelineLayout {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuPipelineLayoutRelease(self.0);
        }
    }
}
pub struct QuerySet(sys::WGPUQuerySet);
impl Clone for QuerySet {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuQuerySetAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for QuerySet {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuQuerySetRelease(self.0);
        }
    }
}
pub struct Queue(sys::WGPUQueue);
impl Clone for Queue {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuQueueAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuQueueRelease(self.0);
        }
    }
}
pub struct RenderBundle(sys::WGPURenderBundle);
impl Clone for RenderBundle {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuRenderBundleAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for RenderBundle {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuRenderBundleRelease(self.0);
        }
    }
}
pub struct RenderBundleEncoder(sys::WGPURenderBundleEncoder);
impl Clone for RenderBundleEncoder {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuRenderBundleEncoderAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for RenderBundleEncoder {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuRenderBundleEncoderRelease(self.0);
        }
    }
}
pub struct RenderPassEncoder(sys::WGPURenderPassEncoder);
impl Clone for RenderPassEncoder {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuRenderPassEncoderAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for RenderPassEncoder {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuRenderPassEncoderRelease(self.0);
        }
    }
}
pub struct RenderPipeline(sys::WGPURenderPipeline);
impl Clone for RenderPipeline {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuRenderPipelineAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for RenderPipeline {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuRenderPipelineRelease(self.0);
        }
    }
}
pub struct Sampler(sys::WGPUSampler);
impl Clone for Sampler {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuSamplerAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Sampler {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuSamplerRelease(self.0);
        }
    }
}
pub struct ShaderModule(sys::WGPUShaderModule);
impl Clone for ShaderModule {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuShaderModuleAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for ShaderModule {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuShaderModuleRelease(self.0);
        }
    }
}
pub struct SharedBufferMemory(sys::WGPUSharedBufferMemory);
impl Clone for SharedBufferMemory {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuSharedBufferMemoryAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for SharedBufferMemory {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuSharedBufferMemoryRelease(self.0);
        }
    }
}
pub struct SharedFence(sys::WGPUSharedFence);
impl Clone for SharedFence {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuSharedFenceAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for SharedFence {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuSharedFenceRelease(self.0);
        }
    }
}
pub struct SharedTextureMemory(sys::WGPUSharedTextureMemory);
impl Clone for SharedTextureMemory {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuSharedTextureMemoryAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for SharedTextureMemory {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuSharedTextureMemoryRelease(self.0);
        }
    }
}
pub struct Surface(sys::WGPUSurface);
impl Clone for Surface {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuSurfaceAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Surface {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuSurfaceRelease(self.0);
        }
    }
}
pub struct Texture(sys::WGPUTexture);
impl Clone for Texture {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuTextureAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuTextureRelease(self.0);
        }
    }
}
pub struct TextureView(sys::WGPUTextureView);
impl Clone for TextureView {
    fn clone(&self) -> Self {
        unsafe {
            sys::wgpuTextureViewAddRef(self.0);
        }
        Self(self.0)
    }
}
impl Drop for TextureView {
    fn drop(&mut self) {
        unsafe {
            sys::wgpuTextureViewRelease(self.0);
        }
    }
}
