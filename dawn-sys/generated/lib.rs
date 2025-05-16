pub use raw::WGPUFlags;
pub use raw::WGPUBool;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "strum",
    derive(strum::Display, strum::EnumString, strum::FromRepr, strum::IntoStaticStr)
)]
#[repr(u32)]
pub enum WGPUAdapterType {
    DiscreteGPU = raw::WGPUAdapterType_WGPUAdapterType_DiscreteGPU as _,
    IntegratedGPU = raw::WGPUAdapterType_WGPUAdapterType_IntegratedGPU as _,
    CPU = raw::WGPUAdapterType_WGPUAdapterType_CPU as _,
    Unknown = raw::WGPUAdapterType_WGPUAdapterType_Unknown as _,
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
#[repr(u32)]
pub enum WGPUAddressMode {
    Undefined = raw::WGPUAddressMode_WGPUAddressMode_Undefined as _,
    ClampToEdge = raw::WGPUAddressMode_WGPUAddressMode_ClampToEdge as _,
    Repeat = raw::WGPUAddressMode_WGPUAddressMode_Repeat as _,
    MirrorRepeat = raw::WGPUAddressMode_WGPUAddressMode_MirrorRepeat as _,
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
#[repr(u32)]
pub enum WGPUBackendType {
    Undefined = raw::WGPUBackendType_WGPUBackendType_Undefined as _,
    Null = raw::WGPUBackendType_WGPUBackendType_Null as _,
    WebGPU = raw::WGPUBackendType_WGPUBackendType_WebGPU as _,
    D3D11 = raw::WGPUBackendType_WGPUBackendType_D3D11 as _,
    D3D12 = raw::WGPUBackendType_WGPUBackendType_D3D12 as _,
    Metal = raw::WGPUBackendType_WGPUBackendType_Metal as _,
    Vulkan = raw::WGPUBackendType_WGPUBackendType_Vulkan as _,
    OpenGL = raw::WGPUBackendType_WGPUBackendType_OpenGL as _,
    OpenGLES = raw::WGPUBackendType_WGPUBackendType_OpenGLES as _,
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
#[repr(u32)]
pub enum WGPUBlendFactor {
    Undefined = raw::WGPUBlendFactor_WGPUBlendFactor_Undefined as _,
    Zero = raw::WGPUBlendFactor_WGPUBlendFactor_Zero as _,
    One = raw::WGPUBlendFactor_WGPUBlendFactor_One as _,
    Src = raw::WGPUBlendFactor_WGPUBlendFactor_Src as _,
    OneMinusSrc = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc as _,
    SrcAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_SrcAlpha as _,
    OneMinusSrcAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrcAlpha as _,
    Dst = raw::WGPUBlendFactor_WGPUBlendFactor_Dst as _,
    OneMinusDst = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusDst as _,
    DstAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_DstAlpha as _,
    OneMinusDstAlpha = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusDstAlpha as _,
    SrcAlphaSaturated = raw::WGPUBlendFactor_WGPUBlendFactor_SrcAlphaSaturated as _,
    Constant = raw::WGPUBlendFactor_WGPUBlendFactor_Constant as _,
    OneMinusConstant = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusConstant as _,
    Src1 = raw::WGPUBlendFactor_WGPUBlendFactor_Src1 as _,
    OneMinusSrc1 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1 as _,
    Src1Alpha = raw::WGPUBlendFactor_WGPUBlendFactor_Src1Alpha as _,
    OneMinusSrc1Alpha = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1Alpha as _,
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
#[repr(u32)]
pub enum WGPUBlendOperation {
    Undefined = raw::WGPUBlendOperation_WGPUBlendOperation_Undefined as _,
    Add = raw::WGPUBlendOperation_WGPUBlendOperation_Add as _,
    Subtract = raw::WGPUBlendOperation_WGPUBlendOperation_Subtract as _,
    ReverseSubtract = raw::WGPUBlendOperation_WGPUBlendOperation_ReverseSubtract as _,
    Min = raw::WGPUBlendOperation_WGPUBlendOperation_Min as _,
    Max = raw::WGPUBlendOperation_WGPUBlendOperation_Max as _,
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
#[repr(u32)]
pub enum WGPUBufferBindingType {
    BindingNotUsed = raw::WGPUBufferBindingType_WGPUBufferBindingType_BindingNotUsed
        as _,
    Undefined = raw::WGPUBufferBindingType_WGPUBufferBindingType_Undefined as _,
    Uniform = raw::WGPUBufferBindingType_WGPUBufferBindingType_Uniform as _,
    Storage = raw::WGPUBufferBindingType_WGPUBufferBindingType_Storage as _,
    ReadOnlyStorage = raw::WGPUBufferBindingType_WGPUBufferBindingType_ReadOnlyStorage
        as _,
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
#[repr(u32)]
pub enum WGPUBufferMapState {
    Unmapped = raw::WGPUBufferMapState_WGPUBufferMapState_Unmapped as _,
    Pending = raw::WGPUBufferMapState_WGPUBufferMapState_Pending as _,
    Mapped = raw::WGPUBufferMapState_WGPUBufferMapState_Mapped as _,
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
#[repr(u32)]
pub enum WGPUCallbackMode {
    WaitAnyOnly = raw::WGPUCallbackMode_WGPUCallbackMode_WaitAnyOnly as _,
    AllowProcessEvents = raw::WGPUCallbackMode_WGPUCallbackMode_AllowProcessEvents as _,
    AllowSpontaneous = raw::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous as _,
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
#[repr(u32)]
pub enum WGPUCompareFunction {
    Undefined = raw::WGPUCompareFunction_WGPUCompareFunction_Undefined as _,
    Never = raw::WGPUCompareFunction_WGPUCompareFunction_Never as _,
    Less = raw::WGPUCompareFunction_WGPUCompareFunction_Less as _,
    Equal = raw::WGPUCompareFunction_WGPUCompareFunction_Equal as _,
    LessEqual = raw::WGPUCompareFunction_WGPUCompareFunction_LessEqual as _,
    Greater = raw::WGPUCompareFunction_WGPUCompareFunction_Greater as _,
    NotEqual = raw::WGPUCompareFunction_WGPUCompareFunction_NotEqual as _,
    GreaterEqual = raw::WGPUCompareFunction_WGPUCompareFunction_GreaterEqual as _,
    Always = raw::WGPUCompareFunction_WGPUCompareFunction_Always as _,
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
#[repr(u32)]
pub enum WGPUCompilationInfoRequestStatus {
    Success = raw::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_Success
        as _,
    CallbackCancelled = raw::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_CallbackCancelled
        as _,
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
#[repr(u32)]
pub enum WGPUCompilationMessageType {
    Error = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Error as _,
    Warning = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Warning as _,
    Info = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Info as _,
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
#[repr(u32)]
pub enum WGPUCompositeAlphaMode {
    Auto = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Auto as _,
    Opaque = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Opaque as _,
    Premultiplied = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Premultiplied
        as _,
    Unpremultiplied = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Unpremultiplied
        as _,
    Inherit = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Inherit as _,
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
#[repr(u32)]
pub enum WGPUCreatePipelineAsyncStatus {
    Success = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_Success
        as _,
    CallbackCancelled = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_CallbackCancelled
        as _,
    ValidationError = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_ValidationError
        as _,
    InternalError = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_InternalError
        as _,
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
#[repr(u32)]
pub enum WGPUCullMode {
    Undefined = raw::WGPUCullMode_WGPUCullMode_Undefined as _,
    None = raw::WGPUCullMode_WGPUCullMode_None as _,
    Front = raw::WGPUCullMode_WGPUCullMode_Front as _,
    Back = raw::WGPUCullMode_WGPUCullMode_Back as _,
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
#[repr(u32)]
pub enum WGPUDeviceLostReason {
    Unknown = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_Unknown as _,
    Destroyed = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_Destroyed as _,
    CallbackCancelled = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_CallbackCancelled
        as _,
    FailedCreation = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_FailedCreation as _,
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
#[repr(u32)]
pub enum WGPUErrorFilter {
    Validation = raw::WGPUErrorFilter_WGPUErrorFilter_Validation as _,
    OutOfMemory = raw::WGPUErrorFilter_WGPUErrorFilter_OutOfMemory as _,
    Internal = raw::WGPUErrorFilter_WGPUErrorFilter_Internal as _,
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
#[repr(u32)]
pub enum WGPUErrorType {
    NoError = raw::WGPUErrorType_WGPUErrorType_NoError as _,
    Validation = raw::WGPUErrorType_WGPUErrorType_Validation as _,
    OutOfMemory = raw::WGPUErrorType_WGPUErrorType_OutOfMemory as _,
    Internal = raw::WGPUErrorType_WGPUErrorType_Internal as _,
    Unknown = raw::WGPUErrorType_WGPUErrorType_Unknown as _,
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
#[repr(u32)]
pub enum WGPUFeatureLevel {
    Undefined = raw::WGPUFeatureLevel_WGPUFeatureLevel_Undefined as _,
    Compatibility = raw::WGPUFeatureLevel_WGPUFeatureLevel_Compatibility as _,
    Core = raw::WGPUFeatureLevel_WGPUFeatureLevel_Core as _,
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
#[repr(u32)]
pub enum WGPUFeatureName {
    DepthClipControl = raw::WGPUFeatureName_WGPUFeatureName_DepthClipControl as _,
    Depth32FloatStencil8 = raw::WGPUFeatureName_WGPUFeatureName_Depth32FloatStencil8
        as _,
    TimestampQuery = raw::WGPUFeatureName_WGPUFeatureName_TimestampQuery as _,
    TextureCompressionBC = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionBC
        as _,
    TextureCompressionBCSliced3D = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionBCSliced3D
        as _,
    TextureCompressionETC2 = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionETC2
        as _,
    TextureCompressionASTC = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTC
        as _,
    TextureCompressionASTCSliced3D = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTCSliced3D
        as _,
    IndirectFirstInstance = raw::WGPUFeatureName_WGPUFeatureName_IndirectFirstInstance
        as _,
    ShaderF16 = raw::WGPUFeatureName_WGPUFeatureName_ShaderF16 as _,
    RG11B10UfloatRenderable = raw::WGPUFeatureName_WGPUFeatureName_RG11B10UfloatRenderable
        as _,
    BGRA8UnormStorage = raw::WGPUFeatureName_WGPUFeatureName_BGRA8UnormStorage as _,
    Float32Filterable = raw::WGPUFeatureName_WGPUFeatureName_Float32Filterable as _,
    Float32Blendable = raw::WGPUFeatureName_WGPUFeatureName_Float32Blendable as _,
    ClipDistances = raw::WGPUFeatureName_WGPUFeatureName_ClipDistances as _,
    DualSourceBlending = raw::WGPUFeatureName_WGPUFeatureName_DualSourceBlending as _,
    Subgroups = raw::WGPUFeatureName_WGPUFeatureName_Subgroups as _,
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
#[repr(u32)]
pub enum WGPUFilterMode {
    Undefined = raw::WGPUFilterMode_WGPUFilterMode_Undefined as _,
    Nearest = raw::WGPUFilterMode_WGPUFilterMode_Nearest as _,
    Linear = raw::WGPUFilterMode_WGPUFilterMode_Linear as _,
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
#[repr(u32)]
pub enum WGPUFrontFace {
    Undefined = raw::WGPUFrontFace_WGPUFrontFace_Undefined as _,
    CCW = raw::WGPUFrontFace_WGPUFrontFace_CCW as _,
    CW = raw::WGPUFrontFace_WGPUFrontFace_CW as _,
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
#[repr(u32)]
pub enum WGPUIndexFormat {
    Undefined = raw::WGPUIndexFormat_WGPUIndexFormat_Undefined as _,
    Uint16 = raw::WGPUIndexFormat_WGPUIndexFormat_Uint16 as _,
    Uint32 = raw::WGPUIndexFormat_WGPUIndexFormat_Uint32 as _,
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
#[repr(u32)]
pub enum WGPUInstanceFeatureName {
    TimedWaitAnyEnable = raw::WGPUInstanceFeatureName_WGPUInstanceFeatureName_TimedWaitAnyEnable
        as _,
    ShaderSourceSPIRV = raw::WGPUInstanceFeatureName_WGPUInstanceFeatureName_ShaderSourceSPIRV
        as _,
    MultipleDevicesPerAdapter = raw::WGPUInstanceFeatureName_WGPUInstanceFeatureName_MultipleDevicesPerAdapter
        as _,
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
#[repr(u32)]
pub enum WGPULoadOp {
    Undefined = raw::WGPULoadOp_WGPULoadOp_Undefined as _,
    Load = raw::WGPULoadOp_WGPULoadOp_Load as _,
    Clear = raw::WGPULoadOp_WGPULoadOp_Clear as _,
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
#[repr(u32)]
pub enum WGPUMapAsyncStatus {
    Success = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Success as _,
    CallbackCancelled = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_CallbackCancelled
        as _,
    Error = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Error as _,
    Aborted = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Aborted as _,
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
#[repr(u32)]
pub enum WGPUMipmapFilterMode {
    Undefined = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Undefined as _,
    Nearest = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Nearest as _,
    Linear = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Linear as _,
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
#[repr(u32)]
pub enum WGPUOptionalBool {
    False = raw::WGPUOptionalBool_WGPUOptionalBool_False as _,
    True = raw::WGPUOptionalBool_WGPUOptionalBool_True as _,
    Undefined = raw::WGPUOptionalBool_WGPUOptionalBool_Undefined as _,
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
#[repr(u32)]
pub enum WGPUPopErrorScopeStatus {
    Success = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Success as _,
    CallbackCancelled = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_CallbackCancelled
        as _,
    Error = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Error as _,
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
#[repr(u32)]
pub enum WGPUPowerPreference {
    Undefined = raw::WGPUPowerPreference_WGPUPowerPreference_Undefined as _,
    LowPower = raw::WGPUPowerPreference_WGPUPowerPreference_LowPower as _,
    HighPerformance = raw::WGPUPowerPreference_WGPUPowerPreference_HighPerformance as _,
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
#[repr(u32)]
pub enum WGPUPredefinedColorSpace {
    SRGB = raw::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_SRGB as _,
    DisplayP3 = raw::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_DisplayP3 as _,
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
#[repr(u32)]
pub enum WGPUPresentMode {
    Undefined = raw::WGPUPresentMode_WGPUPresentMode_Undefined as _,
    Fifo = raw::WGPUPresentMode_WGPUPresentMode_Fifo as _,
    FifoRelaxed = raw::WGPUPresentMode_WGPUPresentMode_FifoRelaxed as _,
    Immediate = raw::WGPUPresentMode_WGPUPresentMode_Immediate as _,
    Mailbox = raw::WGPUPresentMode_WGPUPresentMode_Mailbox as _,
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
#[repr(u32)]
pub enum WGPUPrimitiveTopology {
    Undefined = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_Undefined as _,
    PointList = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_PointList as _,
    LineList = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineList as _,
    LineStrip = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineStrip as _,
    TriangleList = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleList as _,
    TriangleStrip = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleStrip as _,
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
#[repr(u32)]
pub enum WGPUQueryType {
    Occlusion = raw::WGPUQueryType_WGPUQueryType_Occlusion as _,
    Timestamp = raw::WGPUQueryType_WGPUQueryType_Timestamp as _,
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
#[repr(u32)]
pub enum WGPUQueueWorkDoneStatus {
    Success = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Success as _,
    CallbackCancelled = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_CallbackCancelled
        as _,
    Error = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Error as _,
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
#[repr(u32)]
pub enum WGPURequestAdapterStatus {
    Success = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Success as _,
    CallbackCancelled = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_CallbackCancelled
        as _,
    Unavailable = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Unavailable
        as _,
    Error = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Error as _,
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
#[repr(u32)]
pub enum WGPURequestDeviceStatus {
    Success = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Success as _,
    CallbackCancelled = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_CallbackCancelled
        as _,
    Error = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Error as _,
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
#[repr(u32)]
pub enum WGPUSType {
    ShaderSourceSPIRV = raw::WGPUSType_WGPUSType_ShaderSourceSPIRV as _,
    ShaderSourceWGSL = raw::WGPUSType_WGPUSType_ShaderSourceWGSL as _,
    RenderPassMaxDrawCount = raw::WGPUSType_WGPUSType_RenderPassMaxDrawCount as _,
    SurfaceSourceMetalLayer = raw::WGPUSType_WGPUSType_SurfaceSourceMetalLayer as _,
    SurfaceSourceWindowsHWND = raw::WGPUSType_WGPUSType_SurfaceSourceWindowsHWND as _,
    SurfaceSourceXlibWindow = raw::WGPUSType_WGPUSType_SurfaceSourceXlibWindow as _,
    SurfaceSourceWaylandSurface = raw::WGPUSType_WGPUSType_SurfaceSourceWaylandSurface
        as _,
    SurfaceSourceAndroidNativeWindow = raw::WGPUSType_WGPUSType_SurfaceSourceAndroidNativeWindow
        as _,
    SurfaceSourceXCBWindow = raw::WGPUSType_WGPUSType_SurfaceSourceXCBWindow as _,
    SurfaceColorManagement = raw::WGPUSType_WGPUSType_SurfaceColorManagement as _,
    RequestAdapterWebXROptions = raw::WGPUSType_WGPUSType_RequestAdapterWebXROptions
        as _,
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
#[repr(u32)]
pub enum WGPUSamplerBindingType {
    BindingNotUsed = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_BindingNotUsed
        as _,
    Undefined = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Undefined as _,
    Filtering = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Filtering as _,
    NonFiltering = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_NonFiltering as _,
    Comparison = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Comparison as _,
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
#[repr(u32)]
pub enum WGPUStatus {
    Success = raw::WGPUStatus_WGPUStatus_Success as _,
    Error = raw::WGPUStatus_WGPUStatus_Error as _,
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
#[repr(u32)]
pub enum WGPUStencilOperation {
    Undefined = raw::WGPUStencilOperation_WGPUStencilOperation_Undefined as _,
    Keep = raw::WGPUStencilOperation_WGPUStencilOperation_Keep as _,
    Zero = raw::WGPUStencilOperation_WGPUStencilOperation_Zero as _,
    Replace = raw::WGPUStencilOperation_WGPUStencilOperation_Replace as _,
    Invert = raw::WGPUStencilOperation_WGPUStencilOperation_Invert as _,
    IncrementClamp = raw::WGPUStencilOperation_WGPUStencilOperation_IncrementClamp as _,
    DecrementClamp = raw::WGPUStencilOperation_WGPUStencilOperation_DecrementClamp as _,
    IncrementWrap = raw::WGPUStencilOperation_WGPUStencilOperation_IncrementWrap as _,
    DecrementWrap = raw::WGPUStencilOperation_WGPUStencilOperation_DecrementWrap as _,
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
#[repr(u32)]
pub enum WGPUStorageTextureAccess {
    BindingNotUsed = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_BindingNotUsed
        as _,
    Undefined = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_Undefined as _,
    WriteOnly = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_WriteOnly as _,
    ReadOnly = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadOnly as _,
    ReadWrite = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadWrite as _,
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
#[repr(u32)]
pub enum WGPUStoreOp {
    Undefined = raw::WGPUStoreOp_WGPUStoreOp_Undefined as _,
    Store = raw::WGPUStoreOp_WGPUStoreOp_Store as _,
    Discard = raw::WGPUStoreOp_WGPUStoreOp_Discard as _,
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
#[repr(u32)]
pub enum WGPUSurfaceGetCurrentTextureStatus {
    SuccessOptimal = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal
        as _,
    SuccessSuboptimal = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal
        as _,
    Timeout = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Timeout
        as _,
    Outdated = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Outdated
        as _,
    Lost = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Lost
        as _,
    Error = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Error
        as _,
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
#[repr(u32)]
pub enum WGPUTextureAspect {
    Undefined = raw::WGPUTextureAspect_WGPUTextureAspect_Undefined as _,
    All = raw::WGPUTextureAspect_WGPUTextureAspect_All as _,
    StencilOnly = raw::WGPUTextureAspect_WGPUTextureAspect_StencilOnly as _,
    DepthOnly = raw::WGPUTextureAspect_WGPUTextureAspect_DepthOnly as _,
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
#[repr(u32)]
pub enum WGPUTextureDimension {
    Undefined = raw::WGPUTextureDimension_WGPUTextureDimension_Undefined as _,
    D1 = raw::WGPUTextureDimension_WGPUTextureDimension_1D as _,
    D2 = raw::WGPUTextureDimension_WGPUTextureDimension_2D as _,
    D3 = raw::WGPUTextureDimension_WGPUTextureDimension_3D as _,
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
#[repr(u32)]
pub enum WGPUTextureFormat {
    Undefined = raw::WGPUTextureFormat_WGPUTextureFormat_Undefined as _,
    R8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_R8Unorm as _,
    R8Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_R8Snorm as _,
    R8Uint = raw::WGPUTextureFormat_WGPUTextureFormat_R8Uint as _,
    R8Sint = raw::WGPUTextureFormat_WGPUTextureFormat_R8Sint as _,
    R16Uint = raw::WGPUTextureFormat_WGPUTextureFormat_R16Uint as _,
    R16Sint = raw::WGPUTextureFormat_WGPUTextureFormat_R16Sint as _,
    R16Float = raw::WGPUTextureFormat_WGPUTextureFormat_R16Float as _,
    RG8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Unorm as _,
    RG8Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Snorm as _,
    RG8Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Uint as _,
    RG8Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Sint as _,
    R32Float = raw::WGPUTextureFormat_WGPUTextureFormat_R32Float as _,
    R32Uint = raw::WGPUTextureFormat_WGPUTextureFormat_R32Uint as _,
    R32Sint = raw::WGPUTextureFormat_WGPUTextureFormat_R32Sint as _,
    RG16Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Uint as _,
    RG16Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Sint as _,
    RG16Float = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Float as _,
    RGBA8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Unorm as _,
    RGBA8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8UnormSrgb as _,
    RGBA8Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Snorm as _,
    RGBA8Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Uint as _,
    RGBA8Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Sint as _,
    BGRA8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_BGRA8Unorm as _,
    BGRA8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BGRA8UnormSrgb as _,
    RGB10A2Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Uint as _,
    RGB10A2Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Unorm as _,
    RG11B10Ufloat = raw::WGPUTextureFormat_WGPUTextureFormat_RG11B10Ufloat as _,
    RGB9E5Ufloat = raw::WGPUTextureFormat_WGPUTextureFormat_RGB9E5Ufloat as _,
    RG32Float = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Float as _,
    RG32Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Uint as _,
    RG32Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Sint as _,
    RGBA16Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Uint as _,
    RGBA16Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Sint as _,
    RGBA16Float = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Float as _,
    RGBA32Float = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Float as _,
    RGBA32Uint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Uint as _,
    RGBA32Sint = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Sint as _,
    Stencil8 = raw::WGPUTextureFormat_WGPUTextureFormat_Stencil8 as _,
    Depth16Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_Depth16Unorm as _,
    Depth24Plus = raw::WGPUTextureFormat_WGPUTextureFormat_Depth24Plus as _,
    Depth24PlusStencil8 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth24PlusStencil8
        as _,
    Depth32Float = raw::WGPUTextureFormat_WGPUTextureFormat_Depth32Float as _,
    Depth32FloatStencil8 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth32FloatStencil8
        as _,
    BC1RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnorm as _,
    BC1RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnormSrgb as _,
    BC2RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnorm as _,
    BC2RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnormSrgb as _,
    BC3RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnorm as _,
    BC3RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnormSrgb as _,
    BC4RUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC4RUnorm as _,
    BC4RSnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC4RSnorm as _,
    BC5RGUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC5RGUnorm as _,
    BC5RGSnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC5RGSnorm as _,
    BC6HRGBUfloat = raw::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBUfloat as _,
    BC6HRGBFloat = raw::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBFloat as _,
    BC7RGBAUnorm = raw::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnorm as _,
    BC7RGBAUnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnormSrgb as _,
    ETC2RGB8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8Unorm as _,
    ETC2RGB8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8UnormSrgb as _,
    ETC2RGB8A1Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1Unorm as _,
    ETC2RGB8A1UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1UnormSrgb
        as _,
    ETC2RGBA8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8Unorm as _,
    ETC2RGBA8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8UnormSrgb
        as _,
    EACR11Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACR11Unorm as _,
    EACR11Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACR11Snorm as _,
    EACRG11Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACRG11Unorm as _,
    EACRG11Snorm = raw::WGPUTextureFormat_WGPUTextureFormat_EACRG11Snorm as _,
    ASTC4x4Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4Unorm as _,
    ASTC4x4UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4UnormSrgb as _,
    ASTC5x4Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4Unorm as _,
    ASTC5x4UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4UnormSrgb as _,
    ASTC5x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5Unorm as _,
    ASTC5x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5UnormSrgb as _,
    ASTC6x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5Unorm as _,
    ASTC6x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5UnormSrgb as _,
    ASTC6x6Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6Unorm as _,
    ASTC6x6UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6UnormSrgb as _,
    ASTC8x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5Unorm as _,
    ASTC8x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5UnormSrgb as _,
    ASTC8x6Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6Unorm as _,
    ASTC8x6UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6UnormSrgb as _,
    ASTC8x8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8Unorm as _,
    ASTC8x8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8UnormSrgb as _,
    ASTC10x5Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5Unorm as _,
    ASTC10x5UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5UnormSrgb as _,
    ASTC10x6Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6Unorm as _,
    ASTC10x6UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6UnormSrgb as _,
    ASTC10x8Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8Unorm as _,
    ASTC10x8UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8UnormSrgb as _,
    ASTC10x10Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10Unorm as _,
    ASTC10x10UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10UnormSrgb
        as _,
    ASTC12x10Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10Unorm as _,
    ASTC12x10UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10UnormSrgb
        as _,
    ASTC12x12Unorm = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12Unorm as _,
    ASTC12x12UnormSrgb = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12UnormSrgb
        as _,
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
#[repr(u32)]
pub enum WGPUTextureSampleType {
    BindingNotUsed = raw::WGPUTextureSampleType_WGPUTextureSampleType_BindingNotUsed
        as _,
    Undefined = raw::WGPUTextureSampleType_WGPUTextureSampleType_Undefined as _,
    Float = raw::WGPUTextureSampleType_WGPUTextureSampleType_Float as _,
    UnfilterableFloat = raw::WGPUTextureSampleType_WGPUTextureSampleType_UnfilterableFloat
        as _,
    Depth = raw::WGPUTextureSampleType_WGPUTextureSampleType_Depth as _,
    Sint = raw::WGPUTextureSampleType_WGPUTextureSampleType_Sint as _,
    Uint = raw::WGPUTextureSampleType_WGPUTextureSampleType_Uint as _,
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
#[repr(u32)]
pub enum WGPUTextureViewDimension {
    Undefined = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_Undefined as _,
    D1 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_1D as _,
    D2 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_2D as _,
    D2Array = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_2DArray as _,
    Cube = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_Cube as _,
    CubeArray = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_CubeArray as _,
    D3 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_3D as _,
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
#[repr(u32)]
pub enum WGPUToneMappingMode {
    Standard = raw::WGPUToneMappingMode_WGPUToneMappingMode_Standard as _,
    Extended = raw::WGPUToneMappingMode_WGPUToneMappingMode_Extended as _,
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
#[repr(u32)]
pub enum WGPUVertexFormat {
    Uint8 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8 as _,
    Uint8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8x2 as _,
    Uint8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8x4 as _,
    Sint8 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8 as _,
    Sint8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8x2 as _,
    Sint8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8x4 as _,
    Unorm8 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8 as _,
    Unorm8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x2 as _,
    Unorm8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4 as _,
    Snorm8 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8 as _,
    Snorm8x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8x2 as _,
    Snorm8x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8x4 as _,
    Uint16 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16 as _,
    Uint16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16x2 as _,
    Uint16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16x4 as _,
    Sint16 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16 as _,
    Sint16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16x2 as _,
    Sint16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16x4 as _,
    Unorm16 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16 as _,
    Unorm16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16x2 as _,
    Unorm16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16x4 as _,
    Snorm16 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16 as _,
    Snorm16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16x2 as _,
    Snorm16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16x4 as _,
    Float16 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16 as _,
    Float16x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16x2 as _,
    Float16x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16x4 as _,
    Float32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32 as _,
    Float32x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x2 as _,
    Float32x3 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x3 as _,
    Float32x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x4 as _,
    Uint32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32 as _,
    Uint32x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x2 as _,
    Uint32x3 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x3 as _,
    Uint32x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x4 as _,
    Sint32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32 as _,
    Sint32x2 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x2 as _,
    Sint32x3 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x3 as _,
    Sint32x4 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x4 as _,
    Unorm10_10_10_2 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm10_10_10_2 as _,
    Unorm8x4BGRA = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4BGRA as _,
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
#[repr(u32)]
pub enum WGPUVertexStepMode {
    Undefined = raw::WGPUVertexStepMode_WGPUVertexStepMode_Undefined as _,
    Vertex = raw::WGPUVertexStepMode_WGPUVertexStepMode_Vertex as _,
    Instance = raw::WGPUVertexStepMode_WGPUVertexStepMode_Instance as _,
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
#[repr(u32)]
pub enum WGPUWaitStatus {
    Success = raw::WGPUWaitStatus_WGPUWaitStatus_Success as _,
    TimedOut = raw::WGPUWaitStatus_WGPUWaitStatus_TimedOut as _,
    Error = raw::WGPUWaitStatus_WGPUWaitStatus_Error as _,
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
#[repr(u32)]
pub enum WGPUWGSLLanguageFeatureName {
    ReadonlyAndReadwriteStorageTextures = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ReadonlyAndReadwriteStorageTextures
        as _,
    Packed4x8IntegerDotProduct = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_Packed4x8IntegerDotProduct
        as _,
    UnrestrictedPointerParameters = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UnrestrictedPointerParameters
        as _,
    PointerCompositeAccess = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_PointerCompositeAccess
        as _,
}
impl WGPUWGSLLanguageFeatureName {
    pub fn to_str(self) -> &'static str {
        self.into()
    }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUBufferUsage : WGPUFlags { const NONE = raw::WGPUBufferUsage_None as _; const
    MAP_READ = raw::WGPUBufferUsage_MapRead as _; const MAP_WRITE =
    raw::WGPUBufferUsage_MapWrite as _; const COPY_SRC = raw::WGPUBufferUsage_CopySrc as
    _; const COPY_DST = raw::WGPUBufferUsage_CopyDst as _; const INDEX =
    raw::WGPUBufferUsage_Index as _; const VERTEX = raw::WGPUBufferUsage_Vertex as _;
    const UNIFORM = raw::WGPUBufferUsage_Uniform as _; const STORAGE =
    raw::WGPUBufferUsage_Storage as _; const INDIRECT = raw::WGPUBufferUsage_Indirect as
    _; const QUERY_RESOLVE = raw::WGPUBufferUsage_QueryResolve as _; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUColorWriteMask : WGPUFlags { const NONE = raw::WGPUColorWriteMask_None as _;
    const RED = raw::WGPUColorWriteMask_Red as _; const GREEN =
    raw::WGPUColorWriteMask_Green as _; const BLUE = raw::WGPUColorWriteMask_Blue as _;
    const ALPHA = raw::WGPUColorWriteMask_Alpha as _; const ALL =
    raw::WGPUColorWriteMask_All as _; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUMapMode : WGPUFlags { const NONE = raw::WGPUMapMode_None as _; const READ =
    raw::WGPUMapMode_Read as _; const WRITE = raw::WGPUMapMode_Write as _; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUShaderStage : WGPUFlags { const NONE = raw::WGPUShaderStage_None as _; const
    VERTEX = raw::WGPUShaderStage_Vertex as _; const FRAGMENT =
    raw::WGPUShaderStage_Fragment as _; const COMPUTE = raw::WGPUShaderStage_Compute as
    _; }
}
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] #[repr(transparent)] pub struct
    WGPUTextureUsage : WGPUFlags { const NONE = raw::WGPUTextureUsage_None as _; const
    COPY_SRC = raw::WGPUTextureUsage_CopySrc as _; const COPY_DST =
    raw::WGPUTextureUsage_CopyDst as _; const TEXTURE_BINDING =
    raw::WGPUTextureUsage_TextureBinding as _; const STORAGE_BINDING =
    raw::WGPUTextureUsage_StorageBinding as _; const RENDER_ATTACHMENT =
    raw::WGPUTextureUsage_RenderAttachment as _; }
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
pub use raw::WGPUCompilationInfoCallback;
pub use raw::WGPUCreateComputePipelineAsyncCallback;
pub use raw::WGPUCreateRenderPipelineAsyncCallback;
pub use raw::WGPUDeviceLostCallback;
pub use raw::WGPUPopErrorScopeCallback;
pub use raw::WGPUQueueWorkDoneCallback;
pub use raw::WGPURequestAdapterCallback;
pub use raw::WGPURequestDeviceCallback;
pub use raw::WGPUUncapturedErrorCallback;
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
