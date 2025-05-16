#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUAdapterType {
    DiscreteGPU = raw::WGPUAdapterType_WGPUAdapterType_DiscreteGPU,
    IntegratedGPU = raw::WGPUAdapterType_WGPUAdapterType_IntegratedGPU,
    CPU = raw::WGPUAdapterType_WGPUAdapterType_CPU,
    Unknown = raw::WGPUAdapterType_WGPUAdapterType_Unknown,
}
impl WGPUAdapterType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUAddressMode {
    Undefined = raw::WGPUAddressMode_WGPUAddressMode_Undefined,
    ClampToEdge = raw::WGPUAddressMode_WGPUAddressMode_ClampToEdge,
    Repeat = raw::WGPUAddressMode_WGPUAddressMode_Repeat,
    MirrorRepeat = raw::WGPUAddressMode_WGPUAddressMode_MirrorRepeat,
}
impl WGPUAddressMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUBackendType {
    Undefined = raw::WGPUBackendType_WGPUBackendType_Undefined,
    Null = raw::WGPUBackendType_WGPUBackendType_Null,
    WebGPU = raw::WGPUBackendType_WGPUBackendType_WebGPU,
    D3D11 = raw::WGPUBackendType_WGPUBackendType_D3D11,
    D3D12 = raw::WGPUBackendType_WGPUBackendType_D3D12,
    Metal = raw::WGPUBackendType_WGPUBackendType_Metal,
    Vulkan = raw::WGPUBackendType_WGPUBackendType_Vulkan,
    OpenGL = raw::WGPUBackendType_WGPUBackendType_OpenGL,
    OpenGLES = raw::WGPUBackendType_WGPUBackendType_OpenGLES,
}
impl WGPUBackendType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUBlendFactor {
    Undefined = raw::WGPUBlendFactor_WGPUBlendFactor_Undefined,
    Zero = raw::WGPUBlendFactor_WGPUBlendFactor_Zero,
    One = raw::WGPUBlendFactor_WGPUBlendFactor_One,
    Src = raw::WGPUBlendFactor_WGPUBlendFactor_Src,
    OneMinusSrc = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc,
    SrcAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_SrcAlpha,
    OneMinusSrcAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrcAlpha,
    Dst = raw::WGPUBlendFactor_WGPUBlendFactor_Dst,
    OneMinusDst = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusDst,
    DstAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_DstAlpha,
    OneMinusDstAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusDstAlpha,
    SrcAlphaSaturated = raw::WGPUBlendFactor_WGPUBlendFactor_SrcAlphaSaturated,
    Constant = raw::WGPUBlendFactor_WGPUBlendFactor_Constant,
    OneMinusConstant = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusConstant,
    Src1 = raw::WGPUBlendFactor_WGPUBlendFactor_Src1,
    OneMinusSrc1 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1,
    Src1Alpha = raw::WGPUBlendFactor_WGPUBlendFactor_Src1Alpha,
    OneMinusSrc1Alpha = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1Alpha,
}
impl WGPUBlendFactor {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUBlendOperation {
    Undefined = raw::WGPUBlendOperation_WGPUBlendOperation_Undefined,
    Add = raw::WGPUBlendOperation_WGPUBlendOperation_Add,
    Subtract = raw::WGPUBlendOperation_WGPUBlendOperation_Subtract,
    ReverseSubtract = raw::WGPUBlendOperation_WGPUBlendOperation_ReverseSubtract,
    Min = raw::WGPUBlendOperation_WGPUBlendOperation_Min,
    Max = raw::WGPUBlendOperation_WGPUBlendOperation_Max,
}
impl WGPUBlendOperation {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUBufferBindingType {
    BindingNotUsed = raw::WGPUBufferBindingType_WGPUBufferBindingType_BindingNotUsed,
    Undefined = raw::WGPUBufferBindingType_WGPUBufferBindingType_Undefined,
    Uniform = raw::WGPUBufferBindingType_WGPUBufferBindingType_Uniform,
    Storage = raw::WGPUBufferBindingType_WGPUBufferBindingType_Storage,
    ReadOnlyStorage = raw::WGPUBufferBindingType_WGPUBufferBindingType_ReadOnlyStorage,
}
impl WGPUBufferBindingType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUBufferMapState {
    Unmapped = raw::WGPUBufferMapState_WGPUBufferMapState_Unmapped,
    Pending = raw::WGPUBufferMapState_WGPUBufferMapState_Pending,
    Mapped = raw::WGPUBufferMapState_WGPUBufferMapState_Mapped,
}
impl WGPUBufferMapState {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUCallbackMode {
    WaitAnyOnly = raw::WGPUCallbackMode_WGPUCallbackMode_WaitAnyOnly,
    AllowProcessEvents = raw::WGPUCallbackMode_WGPUCallbackMode_AllowProcessEvents,
    AllowSpontaneous = raw::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous,
}
impl WGPUCallbackMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUCompareFunction {
    Undefined = raw::WGPUCompareFunction_WGPUCompareFunction_Undefined,
    Never = raw::WGPUCompareFunction_WGPUCompareFunction_Never,
    Less = raw::WGPUCompareFunction_WGPUCompareFunction_Less,
    Equal = raw::WGPUCompareFunction_WGPUCompareFunction_Equal,
    LessEqual = raw::WGPUCompareFunction_WGPUCompareFunction_LessEqual,
    Greater = raw::WGPUCompareFunction_WGPUCompareFunction_Greater,
    NotEqual = raw::WGPUCompareFunction_WGPUCompareFunction_NotEqual,
    GreaterEqual = raw::WGPUCompareFunction_WGPUCompareFunction_GreaterEqual,
    Always = raw::WGPUCompareFunction_WGPUCompareFunction_Always,
}
impl WGPUCompareFunction {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUCompilationInfoRequestStatus {
    Success = raw::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_Success,
    CallbackCancelled = raw::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_CallbackCancelled,
}
impl WGPUCompilationInfoRequestStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUCompilationMessageType {
    Error = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Error,
    Warning = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Warning,
    Info = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Info,
}
impl WGPUCompilationMessageType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUCompositeAlphaMode {
    Auto = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Auto,
    Opaque = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Opaque,
    Premultiplied = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Premultiplied,
    Unpremultiplied = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Unpremultiplied,
    Inherit = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Inherit,
}
impl WGPUCompositeAlphaMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUCreatePipelineAsyncStatus {
    Success = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_Success,
    CallbackCancelled = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_CallbackCancelled,
    ValidationError = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_ValidationError,
    InternalError = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_InternalError,
}
impl WGPUCreatePipelineAsyncStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUCullMode {
    Undefined = raw::WGPUCullMode_WGPUCullMode_Undefined,
    None = raw::WGPUCullMode_WGPUCullMode_None,
    Front = raw::WGPUCullMode_WGPUCullMode_Front,
    Back = raw::WGPUCullMode_WGPUCullMode_Back,
}
impl WGPUCullMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUDeviceLostReason {
    Unknown = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_Unknown,
    Destroyed = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_Destroyed,
    CallbackCancelled = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_CallbackCancelled,
    FailedCreation = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_FailedCreation,
}
impl WGPUDeviceLostReason {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUErrorFilter {
    Validation = raw::WGPUErrorFilter_WGPUErrorFilter_Validation,
    OutOfMemory = raw::WGPUErrorFilter_WGPUErrorFilter_OutOfMemory,
    Internal = raw::WGPUErrorFilter_WGPUErrorFilter_Internal,
}
impl WGPUErrorFilter {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUErrorType {
    NoError = raw::WGPUErrorType_WGPUErrorType_NoError,
    Validation = raw::WGPUErrorType_WGPUErrorType_Validation,
    OutOfMemory = raw::WGPUErrorType_WGPUErrorType_OutOfMemory,
    Internal = raw::WGPUErrorType_WGPUErrorType_Internal,
    Unknown = raw::WGPUErrorType_WGPUErrorType_Unknown,
}
impl WGPUErrorType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUFeatureLevel {
    Undefined = raw::WGPUFeatureLevel_WGPUFeatureLevel_Undefined,
    Compatibility = raw::WGPUFeatureLevel_WGPUFeatureLevel_Compatibility,
    Core = raw::WGPUFeatureLevel_WGPUFeatureLevel_Core,
}
impl WGPUFeatureLevel {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUFeatureName {
    DepthClipControl = raw::WGPUFeatureName_WGPUFeatureName_DepthClipControl,
    Depth32FloatStencil8 = raw::WGPUFeatureName_WGPUFeatureName_Depth32FloatStencil8,
    TimestampQuery = raw::WGPUFeatureName_WGPUFeatureName_TimestampQuery,
    TextureCompressionBC = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionBC,
    TextureCompressionBCSliced3D = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionBCSliced3D,
    TextureCompressionETC2 = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionETC2,
    TextureCompressionASTC = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTC,
    TextureCompressionASTCSliced3D = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTCSliced3D,
    IndirectFirstInstance = raw::WGPUFeatureName_WGPUFeatureName_IndirectFirstInstance,
    ShaderF16 = raw::WGPUFeatureName_WGPUFeatureName_ShaderF16,
    RG11B10UfloatRenderable = raw::WGPUFeatureName_WGPUFeatureName_RG11B10UfloatRenderable,
    BGRA8UnormStorage = raw::WGPUFeatureName_WGPUFeatureName_BGRA8UnormStorage,
    Float32Filterable = raw::WGPUFeatureName_WGPUFeatureName_Float32Filterable,
    Float32Blendable = raw::WGPUFeatureName_WGPUFeatureName_Float32Blendable,
    ClipDistances = raw::WGPUFeatureName_WGPUFeatureName_ClipDistances,
    DualSourceBlending = raw::WGPUFeatureName_WGPUFeatureName_DualSourceBlending,
    Subgroups = raw::WGPUFeatureName_WGPUFeatureName_Subgroups,
}
impl WGPUFeatureName {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUFilterMode {
    Undefined = raw::WGPUFilterMode_WGPUFilterMode_Undefined,
    Nearest = raw::WGPUFilterMode_WGPUFilterMode_Nearest,
    Linear = raw::WGPUFilterMode_WGPUFilterMode_Linear,
}
impl WGPUFilterMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUFrontFace {
    Undefined = raw::WGPUFrontFace_WGPUFrontFace_Undefined,
    CCW = raw::WGPUFrontFace_WGPUFrontFace_CCW,
    CW = raw::WGPUFrontFace_WGPUFrontFace_CW,
}
impl WGPUFrontFace {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUIndexFormat {
    Undefined = raw::WGPUIndexFormat_WGPUIndexFormat_Undefined,
    Uint16 = raw::WGPUIndexFormat_WGPUIndexFormat_Uint16,
    Uint32 = raw::WGPUIndexFormat_WGPUIndexFormat_Uint32,
}
impl WGPUIndexFormat {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUInstanceFeatureName {
    TimedWaitAnyEnable = raw::WGPUInstanceFeatureName_WGPUInstanceFeatureName_TimedWaitAnyEnable,
    ShaderSourceSPIRV = raw::WGPUInstanceFeatureName_WGPUInstanceFeatureName_ShaderSourceSPIRV,
    MultipleDevicesPerAdapter = raw::WGPUInstanceFeatureName_WGPUInstanceFeatureName_MultipleDevicesPerAdapter,
}
impl WGPUInstanceFeatureName {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPULoadOp {
    Undefined = raw::WGPULoadOp_WGPULoadOp_Undefined,
    Load = raw::WGPULoadOp_WGPULoadOp_Load,
    Clear = raw::WGPULoadOp_WGPULoadOp_Clear,
}
impl WGPULoadOp {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUMapAsyncStatus {
    Success = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Success,
    CallbackCancelled = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_CallbackCancelled,
    Error = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Error,
    Aborted = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Aborted,
}
impl WGPUMapAsyncStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUMipmapFilterMode {
    Undefined = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Undefined,
    Nearest = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Nearest,
    Linear = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Linear,
}
impl WGPUMipmapFilterMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUOptionalBool {
    False = raw::WGPUOptionalBool_WGPUOptionalBool_False,
    True = raw::WGPUOptionalBool_WGPUOptionalBool_True,
    Undefined = raw::WGPUOptionalBool_WGPUOptionalBool_Undefined,
}
impl WGPUOptionalBool {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUPopErrorScopeStatus {
    Success = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Success,
    CallbackCancelled = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_CallbackCancelled,
    Error = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Error,
}
impl WGPUPopErrorScopeStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUPowerPreference {
    Undefined = raw::WGPUPowerPreference_WGPUPowerPreference_Undefined,
    LowPower = raw::WGPUPowerPreference_WGPUPowerPreference_LowPower,
    HighPerformance = raw::WGPUPowerPreference_WGPUPowerPreference_HighPerformance,
}
impl WGPUPowerPreference {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUPredefinedColorSpace {
    SRGB = raw::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_SRGB,
    DisplayP3 = raw::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_DisplayP3,
}
impl WGPUPredefinedColorSpace {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUPresentMode {
    Undefined = raw::WGPUPresentMode_WGPUPresentMode_Undefined,
    Fifo = raw::WGPUPresentMode_WGPUPresentMode_Fifo,
    FifoRelaxed = raw::WGPUPresentMode_WGPUPresentMode_FifoRelaxed,
    Immediate = raw::WGPUPresentMode_WGPUPresentMode_Immediate,
    Mailbox = raw::WGPUPresentMode_WGPUPresentMode_Mailbox,
}
impl WGPUPresentMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUPrimitiveTopology {
    Undefined = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_Undefined,
    PointList = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_PointList,
    LineList = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineList,
    LineStrip = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineStrip,
    TriangleList = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleList,
    TriangleStrip = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleStrip,
}
impl WGPUPrimitiveTopology {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUQueryType {
    Occlusion = raw::WGPUQueryType_WGPUQueryType_Occlusion,
    Timestamp = raw::WGPUQueryType_WGPUQueryType_Timestamp,
}
impl WGPUQueryType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUQueueWorkDoneStatus {
    Success = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Success,
    CallbackCancelled = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_CallbackCancelled,
    Error = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Error,
}
impl WGPUQueueWorkDoneStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPURequestAdapterStatus {
    Success = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Success,
    CallbackCancelled = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_CallbackCancelled,
    Unavailable = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Unavailable,
    Error = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Error,
}
impl WGPURequestAdapterStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPURequestDeviceStatus {
    Success = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Success,
    CallbackCancelled = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_CallbackCancelled,
    Error = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Error,
}
impl WGPURequestDeviceStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUSType {
    ShaderSourceSPIRV = raw::WGPUSType_WGPUSType_ShaderSourceSPIRV,
    ShaderSourceWGSL = raw::WGPUSType_WGPUSType_ShaderSourceWGSL,
    RenderPassMaxDrawCount = raw::WGPUSType_WGPUSType_RenderPassMaxDrawCount,
    SurfaceSourceMetalLayer = raw::WGPUSType_WGPUSType_SurfaceSourceMetalLayer,
    SurfaceSourceWindowsHWND = raw::WGPUSType_WGPUSType_SurfaceSourceWindowsHWND,
    SurfaceSourceXlibWindow = raw::WGPUSType_WGPUSType_SurfaceSourceXlibWindow,
    SurfaceSourceWaylandSurface = raw::WGPUSType_WGPUSType_SurfaceSourceWaylandSurface,
    SurfaceSourceAndroidNativeWindow = raw::WGPUSType_WGPUSType_SurfaceSourceAndroidNativeWindow,
    SurfaceSourceXCBWindow = raw::WGPUSType_WGPUSType_SurfaceSourceXCBWindow,
    SurfaceColorManagement = raw::WGPUSType_WGPUSType_SurfaceColorManagement,
    RequestAdapterWebXROptions = raw::WGPUSType_WGPUSType_RequestAdapterWebXROptions,
}
impl WGPUSType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUSamplerBindingType {
    BindingNotUsed = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_BindingNotUsed,
    Undefined = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Undefined,
    Filtering = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Filtering,
    NonFiltering = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_NonFiltering,
    Comparison = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Comparison,
}
impl WGPUSamplerBindingType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUStatus {
    Success = raw::WGPUStatus_WGPUStatus_Success,
    Error = raw::WGPUStatus_WGPUStatus_Error,
}
impl WGPUStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUStencilOperation {
    Undefined = raw::WGPUStencilOperation_WGPUStencilOperation_Undefined,
    Keep = raw::WGPUStencilOperation_WGPUStencilOperation_Keep,
    Zero = raw::WGPUStencilOperation_WGPUStencilOperation_Zero,
    Replace = raw::WGPUStencilOperation_WGPUStencilOperation_Replace,
    Invert = raw::WGPUStencilOperation_WGPUStencilOperation_Invert,
    IncrementClamp = raw::WGPUStencilOperation_WGPUStencilOperation_IncrementClamp,
    DecrementClamp = raw::WGPUStencilOperation_WGPUStencilOperation_DecrementClamp,
    IncrementWrap = raw::WGPUStencilOperation_WGPUStencilOperation_IncrementWrap,
    DecrementWrap = raw::WGPUStencilOperation_WGPUStencilOperation_DecrementWrap,
}
impl WGPUStencilOperation {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUStorageTextureAccess {
    BindingNotUsed = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_BindingNotUsed,
    Undefined = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_Undefined,
    WriteOnly = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_WriteOnly,
    ReadOnly = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadOnly,
    ReadWrite = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadWrite,
}
impl WGPUStorageTextureAccess {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUStoreOp {
    Undefined = raw::WGPUStoreOp_WGPUStoreOp_Undefined,
    Store = raw::WGPUStoreOp_WGPUStoreOp_Store,
    Discard = raw::WGPUStoreOp_WGPUStoreOp_Discard,
}
impl WGPUStoreOp {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUSurfaceGetCurrentTextureStatus {
    SuccessOptimal = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal,
    SuccessSuboptimal = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal,
    Timeout = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Timeout,
    Outdated = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Outdated,
    Lost = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Lost,
    Error = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Error,
}
impl WGPUSurfaceGetCurrentTextureStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUTextureAspect {
    Undefined = raw::WGPUTextureAspect_WGPUTextureAspect_Undefined,
    All = raw::WGPUTextureAspect_WGPUTextureAspect_All,
    StencilOnly = raw::WGPUTextureAspect_WGPUTextureAspect_StencilOnly,
    DepthOnly = raw::WGPUTextureAspect_WGPUTextureAspect_DepthOnly,
}
impl WGPUTextureAspect {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUTextureDimension {
    Undefined = raw::WGPUTextureDimension_WGPUTextureDimension_Undefined,
    D1 = raw::WGPUTextureDimension_WGPUTextureDimension_1D,
    D2 = raw::WGPUTextureDimension_WGPUTextureDimension_2D,
    D3 = raw::WGPUTextureDimension_WGPUTextureDimension_3D,
}
impl WGPUTextureDimension {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUTextureFormat {
    Undefined = raw::WGPUTextureFormat_WGPUTextureFormat_Undefined,
    R8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_R8Unorm,
    R8Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_R8Snorm,
    R8Uint = raw::WGPUTextureFormat_WGPUTextureFormat_R8Uint,
    R8Sint = raw::WGPUTextureFormat_WGPUTextureFormat_R8Sint,
    R16Uint = raw::WGPUTextureFormat_WGPUTextureFormat_R16Uint,
    R16Sint = raw::WGPUTextureFormat_WGPUTextureFormat_R16Sint,
    R16Float = raw::WGPUTextureFormat_WGPUTextureFormat_R16Float,
    RG8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Unorm,
    RG8Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Snorm,
    RG8Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Uint,
    RG8Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Sint,
    R32Float = raw::WGPUTextureFormat_WGPUTextureFormat_R32Float,
    R32Uint = raw::WGPUTextureFormat_WGPUTextureFormat_R32Uint,
    R32Sint = raw::WGPUTextureFormat_WGPUTextureFormat_R32Sint,
    RG16Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Uint,
    RG16Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Sint,
    RG16Float = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Float,
    RGBA8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Unorm,
    RGBA8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8UnormSrgb,
    RGBA8Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Snorm,
    RGBA8Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Uint,
    RGBA8Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Sint,
    BGRA8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_BGRA8Unorm,
    BGRA8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BGRA8UnormSrgb,
    RGB10A2Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Uint,
    RGB10A2Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Unorm,
    RG11B10Ufloat = raw::WGPUTextureFormat_WGPUTextureFormat_RG11B10Ufloat,
    RGB9E5Ufloat = raw::WGPUTextureFormat_WGPUTextureFormat_RGB9E5Ufloat,
    RG32Float = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Float,
    RG32Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Uint,
    RG32Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Sint,
    RGBA16Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Uint,
    RGBA16Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Sint,
    RGBA16Float = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Float,
    RGBA32Float = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Float,
    RGBA32Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Uint,
    RGBA32Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Sint,
    Stencil8 = raw::WGPUTextureFormat_WGPUTextureFormat_Stencil8,
    Depth16Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_Depth16Unorm,
    Depth24Plus = raw::WGPUTextureFormat_WGPUTextureFormat_Depth24Plus,
    Depth24PlusStencil8 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth24PlusStencil8,
    Depth32Float = raw::WGPUTextureFormat_WGPUTextureFormat_Depth32Float,
    Depth32FloatStencil8 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth32FloatStencil8,
    BC1RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnorm,
    BC1RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnormSrgb,
    BC2RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnorm,
    BC2RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnormSrgb,
    BC3RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnorm,
    BC3RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnormSrgb,
    BC4RUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC4RUnorm,
    BC4RSnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC4RSnorm,
    BC5RGUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC5RGUnorm,
    BC5RGSnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC5RGSnorm,
    BC6HRGBUfloat = raw::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBUfloat,
    BC6HRGBFloat = raw::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBFloat,
    BC7RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnorm,
    BC7RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnormSrgb,
    ETC2RGB8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8Unorm,
    ETC2RGB8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8UnormSrgb,
    ETC2RGB8A1Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1Unorm,
    ETC2RGB8A1UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1UnormSrgb,
    ETC2RGBA8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8Unorm,
    ETC2RGBA8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8UnormSrgb,
    EACR11Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACR11Unorm,
    EACR11Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACR11Snorm,
    EACRG11Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACRG11Unorm,
    EACRG11Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACRG11Snorm,
    ASTC4x4Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4Unorm,
    ASTC4x4UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4UnormSrgb,
    ASTC5x4Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4Unorm,
    ASTC5x4UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4UnormSrgb,
    ASTC5x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5Unorm,
    ASTC5x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5UnormSrgb,
    ASTC6x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5Unorm,
    ASTC6x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5UnormSrgb,
    ASTC6x6Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6Unorm,
    ASTC6x6UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6UnormSrgb,
    ASTC8x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5Unorm,
    ASTC8x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5UnormSrgb,
    ASTC8x6Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6Unorm,
    ASTC8x6UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6UnormSrgb,
    ASTC8x8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8Unorm,
    ASTC8x8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8UnormSrgb,
    ASTC10x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5Unorm,
    ASTC10x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5UnormSrgb,
    ASTC10x6Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6Unorm,
    ASTC10x6UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6UnormSrgb,
    ASTC10x8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8Unorm,
    ASTC10x8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8UnormSrgb,
    ASTC10x10Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10Unorm,
    ASTC10x10UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10UnormSrgb,
    ASTC12x10Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10Unorm,
    ASTC12x10UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10UnormSrgb,
    ASTC12x12Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12Unorm,
    ASTC12x12UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12UnormSrgb,
}
impl WGPUTextureFormat {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUTextureSampleType {
    BindingNotUsed = raw::WGPUTextureSampleType_WGPUTextureSampleType_BindingNotUsed,
    Undefined = raw::WGPUTextureSampleType_WGPUTextureSampleType_Undefined,
    Float = raw::WGPUTextureSampleType_WGPUTextureSampleType_Float,
    UnfilterableFloat = raw::WGPUTextureSampleType_WGPUTextureSampleType_UnfilterableFloat,
    Depth = raw::WGPUTextureSampleType_WGPUTextureSampleType_Depth,
    Sint = raw::WGPUTextureSampleType_WGPUTextureSampleType_Sint,
    Uint = raw::WGPUTextureSampleType_WGPUTextureSampleType_Uint,
}
impl WGPUTextureSampleType {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUTextureViewDimension {
    Undefined = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_Undefined,
    D1 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_1D,
    D2 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_2D,
    D2Array = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_2DArray,
    Cube = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_Cube,
    CubeArray = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_CubeArray,
    D3 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_3D,
}
impl WGPUTextureViewDimension {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUToneMappingMode {
    Standard = raw::WGPUToneMappingMode_WGPUToneMappingMode_Standard,
    Extended = raw::WGPUToneMappingMode_WGPUToneMappingMode_Extended,
}
impl WGPUToneMappingMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUVertexFormat {
    Uint8 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8,
    Uint8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8x2,
    Uint8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8x4,
    Sint8 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8,
    Sint8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8x2,
    Sint8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8x4,
    Unorm8 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8,
    Unorm8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x2,
    Unorm8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4,
    Snorm8 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8,
    Snorm8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8x2,
    Snorm8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8x4,
    Uint16 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16,
    Uint16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16x2,
    Uint16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16x4,
    Sint16 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16,
    Sint16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16x2,
    Sint16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16x4,
    Unorm16 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16,
    Unorm16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16x2,
    Unorm16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16x4,
    Snorm16 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16,
    Snorm16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16x2,
    Snorm16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16x4,
    Float16 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16,
    Float16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16x2,
    Float16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16x4,
    Float32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32,
    Float32x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x2,
    Float32x3 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x3,
    Float32x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x4,
    Uint32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32,
    Uint32x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x2,
    Uint32x3 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x3,
    Uint32x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x4,
    Sint32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32,
    Sint32x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x2,
    Sint32x3 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x3,
    Sint32x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x4,
    Unorm10_10_10_2 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm10_10_10_2,
    Unorm8x4BGRA = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4BGRA,
}
impl WGPUVertexFormat {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUVertexStepMode {
    Undefined = raw::WGPUVertexStepMode_WGPUVertexStepMode_Undefined,
    Vertex = raw::WGPUVertexStepMode_WGPUVertexStepMode_Vertex,
    Instance = raw::WGPUVertexStepMode_WGPUVertexStepMode_Instance,
}
impl WGPUVertexStepMode {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUWaitStatus {
    Success = raw::WGPUWaitStatus_WGPUWaitStatus_Success,
    TimedOut = raw::WGPUWaitStatus_WGPUWaitStatus_TimedOut,
    Error = raw::WGPUWaitStatus_WGPUWaitStatus_Error,
}
impl WGPUWaitStatus {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(i32)]
pub enum WGPUWGSLLanguageFeatureName {
    ReadonlyAndReadwriteStorageTextures = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ReadonlyAndReadwriteStorageTextures,
    Packed4x8IntegerDotProduct = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_Packed4x8IntegerDotProduct,
    UnrestrictedPointerParameters = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UnrestrictedPointerParameters,
    PointerCompositeAccess = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_PointerCompositeAccess,
}
impl WGPUWGSLLanguageFeatureName {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUBufferUsage : WGPUFlags { const NONE = raw::WGPUBufferUsage_None; const MAP_READ
    = raw::WGPUBufferUsage_MapRead; const MAP_WRITE = raw::WGPUBufferUsage_MapWrite;
    const COPY_SRC = raw::WGPUBufferUsage_CopySrc; const COPY_DST =
    raw::WGPUBufferUsage_CopyDst; const INDEX = raw::WGPUBufferUsage_Index; const VERTEX
    = raw::WGPUBufferUsage_Vertex; const UNIFORM = raw::WGPUBufferUsage_Uniform; const
    STORAGE = raw::WGPUBufferUsage_Storage; const INDIRECT =
    raw::WGPUBufferUsage_Indirect; const QUERY_RESOLVE =
    raw::WGPUBufferUsage_QueryResolve; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUColorWriteMask : WGPUFlags { const NONE = raw::WGPUColorWriteMask_None; const RED
    = raw::WGPUColorWriteMask_Red; const GREEN = raw::WGPUColorWriteMask_Green; const
    BLUE = raw::WGPUColorWriteMask_Blue; const ALPHA = raw::WGPUColorWriteMask_Alpha;
    const ALL = raw::WGPUColorWriteMask_All; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUMapMode : WGPUFlags { const NONE = raw::WGPUMapMode_None; const READ =
    raw::WGPUMapMode_Read; const WRITE = raw::WGPUMapMode_Write; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUShaderStage : WGPUFlags { const NONE = raw::WGPUShaderStage_None; const VERTEX =
    raw::WGPUShaderStage_Vertex; const FRAGMENT = raw::WGPUShaderStage_Fragment; const
    COMPUTE = raw::WGPUShaderStage_Compute; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUTextureUsage : WGPUFlags { const NONE = raw::WGPUTextureUsage_None; const
    COPY_SRC = raw::WGPUTextureUsage_CopySrc; const COPY_DST =
    raw::WGPUTextureUsage_CopyDst; const TEXTURE_BINDING =
    raw::WGPUTextureUsage_TextureBinding; const STORAGE_BINDING =
    raw::WGPUTextureUsage_StorageBinding; const RENDER_ATTACHMENT =
    raw::WGPUTextureUsage_RenderAttachment; }
}
pub use raw::WGPUAdapterInfo;
pub use raw::WGPUBindGroupDescriptor;
pub use raw::WGPUBindGroupEntry;
pub use raw::WGPUBindGroupLayoutDescriptor;
pub use raw::WGPUBindGroupLayoutEntry;
pub use raw::WGPUBlendComponent;
pub use raw::WGPUBlendState;
pub use raw::WGPUBufferBindingLayout;
pub use raw::WGPUBufferDescriptor;
pub use raw::WGPUColor;
pub use raw::WGPUColorTargetState;
pub use raw::WGPUCommandBufferDescriptor;
pub use raw::WGPUCommandEncoderDescriptor;
pub use raw::WGPUCompilationInfo;
pub use raw::WGPUCompilationMessage;
pub use raw::WGPUComputePassDescriptor;
pub use raw::WGPUComputePipelineDescriptor;
pub use raw::WGPUComputeState;
pub use raw::WGPUConstantEntry;
pub use raw::WGPUDepthStencilState;
pub use raw::WGPUDeviceDescriptor;
pub use raw::WGPUExtent3D;
pub use raw::WGPUFragmentState;
pub use raw::WGPUFuture;
pub use raw::WGPUFutureWaitInfo;
pub use raw::WGPUInstanceDescriptor;
pub use raw::WGPUInstanceLimits;
pub use raw::WGPULimits;
pub use raw::WGPUMultisampleState;
pub use raw::WGPUOrigin3D;
pub use raw::WGPUPassTimestampWrites;
pub use raw::WGPUPipelineLayoutDescriptor;
pub use raw::WGPUPrimitiveState;
pub use raw::WGPUQuerySetDescriptor;
pub use raw::WGPUQueueDescriptor;
pub use raw::WGPURenderBundleDescriptor;
pub use raw::WGPURenderBundleEncoderDescriptor;
pub use raw::WGPURenderPassColorAttachment;
pub use raw::WGPURenderPassDepthStencilAttachment;
pub use raw::WGPURenderPassDescriptor;
pub use raw::WGPURenderPassMaxDrawCount;
pub use raw::WGPURenderPipelineDescriptor;
pub use raw::WGPURequestAdapterOptions;
pub use raw::WGPURequestAdapterWebXROptions;
pub use raw::WGPUSamplerBindingLayout;
pub use raw::WGPUSamplerDescriptor;
pub use raw::WGPUShaderModuleDescriptor;
pub use raw::WGPUShaderSourceSPIRV;
pub use raw::WGPUShaderSourceWGSL;
pub use raw::WGPUStencilFaceState;
pub use raw::WGPUStorageTextureBindingLayout;
pub use raw::WGPUSupportedFeatures;
pub use raw::WGPUSupportedInstanceFeatures;
pub use raw::WGPUSupportedWGSLLanguageFeatures;
pub use raw::WGPUSurfaceCapabilities;
pub use raw::WGPUSurfaceColorManagement;
pub use raw::WGPUSurfaceConfiguration;
pub use raw::WGPUSurfaceDescriptor;
pub use raw::WGPUSurfaceSourceAndroidNativeWindow;
pub use raw::WGPUSurfaceSourceMetalLayer;
pub use raw::WGPUSurfaceSourceWaylandSurface;
pub use raw::WGPUSurfaceSourceWindowsHWND;
pub use raw::WGPUSurfaceSourceXCBWindow;
pub use raw::WGPUSurfaceSourceXlibWindow;
pub use raw::WGPUSurfaceTexture;
pub use raw::WGPUTexelCopyBufferInfo;
pub use raw::WGPUTexelCopyBufferLayout;
pub use raw::WGPUTexelCopyTextureInfo;
pub use raw::WGPUTextureBindingLayout;
pub use raw::WGPUTextureDescriptor;
pub use raw::WGPUTextureViewDescriptor;
pub use raw::WGPUVertexAttribute;
pub use raw::WGPUVertexBufferLayout;
pub use raw::WGPUVertexState;
pub use raw::WGPUBufferMapCallback;
pub use raw::WGPUBufferMapCallbackInfo;
pub use raw::WGPUCompilationInfoCallback;
pub use raw::WGPUCompilationInfoCallbackInfo;
pub use raw::WGPUCreateComputePipelineAsyncCallback;
pub use raw::WGPUCreateComputePipelineAsyncCallbackInfo;
pub use raw::WGPUCreateRenderPipelineAsyncCallback;
pub use raw::WGPUCreateRenderPipelineAsyncCallbackInfo;
pub use raw::WGPUDeviceLostCallback;
pub use raw::WGPUDeviceLostCallbackInfo;
pub use raw::WGPUPopErrorScopeCallback;
pub use raw::WGPUPopErrorScopeCallbackInfo;
pub use raw::WGPUQueueWorkDoneCallback;
pub use raw::WGPUQueueWorkDoneCallbackInfo;
pub use raw::WGPURequestAdapterCallback;
pub use raw::WGPURequestAdapterCallbackInfo;
pub use raw::WGPURequestDeviceCallback;
pub use raw::WGPURequestDeviceCallbackInfo;
pub use raw::WGPUUncapturedErrorCallback;
pub use raw::WGPUUncapturedErrorCallbackInfo;
pub use raw::wgpuCreateInstance;
pub use raw::wgpuGetInstanceFeatures;
pub use raw::wgpuGetInstanceLimits;
pub use raw::wgpuHasInstanceFeature;
pub use raw::WGPUAdapter;
pub use raw::wgpuAdapterGetLimits;
pub use raw::wgpuAdapterHasFeature;
pub use raw::wgpuAdapterGetFeatures;
pub use raw::wgpuAdapterGetInfo;
pub use raw::wgpuAdapterRequestDevice;
pub use raw::WGPUBindGroup;
pub use raw::wgpuBindGroupSetLabel;
pub use raw::WGPUBindGroupLayout;
pub use raw::wgpuBindGroupLayoutSetLabel;
pub use raw::WGPUBuffer;
pub use raw::wgpuBufferMapAsync;
pub use raw::wgpuBufferGetMappedRange;
pub use raw::wgpuBufferGetConstMappedRange;
pub use raw::wgpuBufferReadMappedRange;
pub use raw::wgpuBufferWriteMappedRange;
pub use raw::wgpuBufferSetLabel;
pub use raw::wgpuBufferGetUsage;
pub use raw::wgpuBufferGetSize;
pub use raw::wgpuBufferGetMapState;
pub use raw::wgpuBufferUnmap;
pub use raw::wgpuBufferDestroy;
pub use raw::WGPUCommandBuffer;
pub use raw::wgpuCommandBufferSetLabel;
pub use raw::WGPUCommandEncoder;
pub use raw::wgpuCommandEncoderFinish;
pub use raw::wgpuCommandEncoderBeginComputePass;
pub use raw::wgpuCommandEncoderBeginRenderPass;
pub use raw::wgpuCommandEncoderCopyBufferToBuffer;
pub use raw::wgpuCommandEncoderCopyBufferToTexture;
pub use raw::wgpuCommandEncoderCopyTextureToBuffer;
pub use raw::wgpuCommandEncoderCopyTextureToTexture;
pub use raw::wgpuCommandEncoderClearBuffer;
pub use raw::wgpuCommandEncoderInsertDebugMarker;
pub use raw::wgpuCommandEncoderPopDebugGroup;
pub use raw::wgpuCommandEncoderPushDebugGroup;
pub use raw::wgpuCommandEncoderResolveQuerySet;
pub use raw::wgpuCommandEncoderWriteTimestamp;
pub use raw::wgpuCommandEncoderSetLabel;
pub use raw::WGPUComputePassEncoder;
pub use raw::wgpuComputePassEncoderInsertDebugMarker;
pub use raw::wgpuComputePassEncoderPopDebugGroup;
pub use raw::wgpuComputePassEncoderPushDebugGroup;
pub use raw::wgpuComputePassEncoderSetPipeline;
pub use raw::wgpuComputePassEncoderSetBindGroup;
pub use raw::wgpuComputePassEncoderDispatchWorkgroups;
pub use raw::wgpuComputePassEncoderDispatchWorkgroupsIndirect;
pub use raw::wgpuComputePassEncoderEnd;
pub use raw::wgpuComputePassEncoderSetLabel;
pub use raw::WGPUComputePipeline;
pub use raw::wgpuComputePipelineGetBindGroupLayout;
pub use raw::wgpuComputePipelineSetLabel;
pub use raw::WGPUDevice;
pub use raw::wgpuDeviceCreateBindGroup;
pub use raw::wgpuDeviceCreateBindGroupLayout;
pub use raw::wgpuDeviceCreateBuffer;
pub use raw::wgpuDeviceCreateCommandEncoder;
pub use raw::wgpuDeviceCreateComputePipeline;
pub use raw::wgpuDeviceCreateComputePipelineAsync;
pub use raw::wgpuDeviceCreatePipelineLayout;
pub use raw::wgpuDeviceCreateQuerySet;
pub use raw::wgpuDeviceCreateRenderPipelineAsync;
pub use raw::wgpuDeviceCreateRenderBundleEncoder;
pub use raw::wgpuDeviceCreateRenderPipeline;
pub use raw::wgpuDeviceCreateSampler;
pub use raw::wgpuDeviceCreateShaderModule;
pub use raw::wgpuDeviceCreateTexture;
pub use raw::wgpuDeviceDestroy;
pub use raw::wgpuDeviceGetLostFuture;
pub use raw::wgpuDeviceGetLimits;
pub use raw::wgpuDeviceHasFeature;
pub use raw::wgpuDeviceGetFeatures;
pub use raw::wgpuDeviceGetAdapterInfo;
pub use raw::wgpuDeviceGetQueue;
pub use raw::wgpuDevicePushErrorScope;
pub use raw::wgpuDevicePopErrorScope;
pub use raw::wgpuDeviceSetLabel;
pub use raw::WGPUInstance;
pub use raw::wgpuInstanceCreateSurface;
pub use raw::wgpuInstanceGetWGSLLanguageFeatures;
pub use raw::wgpuInstanceHasWGSLLanguageFeature;
pub use raw::wgpuInstanceProcessEvents;
pub use raw::wgpuInstanceRequestAdapter;
pub use raw::wgpuInstanceWaitAny;
pub use raw::WGPUPipelineLayout;
pub use raw::wgpuPipelineLayoutSetLabel;
pub use raw::WGPUQuerySet;
pub use raw::wgpuQuerySetSetLabel;
pub use raw::wgpuQuerySetGetType;
pub use raw::wgpuQuerySetGetCount;
pub use raw::wgpuQuerySetDestroy;
pub use raw::WGPUQueue;
pub use raw::wgpuQueueSubmit;
pub use raw::wgpuQueueOnSubmittedWorkDone;
pub use raw::wgpuQueueWriteBuffer;
pub use raw::wgpuQueueWriteTexture;
pub use raw::wgpuQueueSetLabel;
pub use raw::WGPURenderBundle;
pub use raw::wgpuRenderBundleSetLabel;
pub use raw::WGPURenderBundleEncoder;
pub use raw::wgpuRenderBundleEncoderSetPipeline;
pub use raw::wgpuRenderBundleEncoderSetBindGroup;
pub use raw::wgpuRenderBundleEncoderDraw;
pub use raw::wgpuRenderBundleEncoderDrawIndexed;
pub use raw::wgpuRenderBundleEncoderDrawIndirect;
pub use raw::wgpuRenderBundleEncoderDrawIndexedIndirect;
pub use raw::wgpuRenderBundleEncoderInsertDebugMarker;
pub use raw::wgpuRenderBundleEncoderPopDebugGroup;
pub use raw::wgpuRenderBundleEncoderPushDebugGroup;
pub use raw::wgpuRenderBundleEncoderSetVertexBuffer;
pub use raw::wgpuRenderBundleEncoderSetIndexBuffer;
pub use raw::wgpuRenderBundleEncoderFinish;
pub use raw::wgpuRenderBundleEncoderSetLabel;
pub use raw::WGPURenderPassEncoder;
pub use raw::wgpuRenderPassEncoderSetPipeline;
pub use raw::wgpuRenderPassEncoderSetBindGroup;
pub use raw::wgpuRenderPassEncoderDraw;
pub use raw::wgpuRenderPassEncoderDrawIndexed;
pub use raw::wgpuRenderPassEncoderDrawIndirect;
pub use raw::wgpuRenderPassEncoderDrawIndexedIndirect;
pub use raw::wgpuRenderPassEncoderExecuteBundles;
pub use raw::wgpuRenderPassEncoderInsertDebugMarker;
pub use raw::wgpuRenderPassEncoderPopDebugGroup;
pub use raw::wgpuRenderPassEncoderPushDebugGroup;
pub use raw::wgpuRenderPassEncoderSetStencilReference;
pub use raw::wgpuRenderPassEncoderSetBlendConstant;
pub use raw::wgpuRenderPassEncoderSetViewport;
pub use raw::wgpuRenderPassEncoderSetScissorRect;
pub use raw::wgpuRenderPassEncoderSetVertexBuffer;
pub use raw::wgpuRenderPassEncoderSetIndexBuffer;
pub use raw::wgpuRenderPassEncoderBeginOcclusionQuery;
pub use raw::wgpuRenderPassEncoderEndOcclusionQuery;
pub use raw::wgpuRenderPassEncoderEnd;
pub use raw::wgpuRenderPassEncoderSetLabel;
pub use raw::WGPURenderPipeline;
pub use raw::wgpuRenderPipelineGetBindGroupLayout;
pub use raw::wgpuRenderPipelineSetLabel;
pub use raw::WGPUSampler;
pub use raw::wgpuSamplerSetLabel;
pub use raw::WGPUShaderModule;
pub use raw::wgpuShaderModuleGetCompilationInfo;
pub use raw::wgpuShaderModuleSetLabel;
pub use raw::WGPUSurface;
pub use raw::wgpuSurfaceConfigure;
pub use raw::wgpuSurfaceGetCapabilities;
pub use raw::wgpuSurfaceGetCurrentTexture;
pub use raw::wgpuSurfacePresent;
pub use raw::wgpuSurfaceUnconfigure;
pub use raw::wgpuSurfaceSetLabel;
pub use raw::WGPUTexture;
pub use raw::wgpuTextureCreateView;
pub use raw::wgpuTextureSetLabel;
pub use raw::wgpuTextureGetWidth;
pub use raw::wgpuTextureGetHeight;
pub use raw::wgpuTextureGetDepthOrArrayLayers;
pub use raw::wgpuTextureGetMipLevelCount;
pub use raw::wgpuTextureGetSampleCount;
pub use raw::wgpuTextureGetDimension;
pub use raw::wgpuTextureGetFormat;
pub use raw::wgpuTextureGetUsage;
pub use raw::wgpuTextureDestroy;
pub use raw::WGPUTextureView;
pub use raw::wgpuTextureViewSetLabel;
