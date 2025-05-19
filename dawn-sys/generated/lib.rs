pub mod WGPUWGSLLanguageFeatureName {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const ReadonlyAndReadwriteStorageTextures: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ReadonlyAndReadwriteStorageTextures;
    pub const Packed4x8IntegerDotProduct: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_Packed4x8IntegerDotProduct;
    pub const UnrestrictedPointerParameters: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_UnrestrictedPointerParameters;
    pub const PointerCompositeAccess: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_PointerCompositeAccess;
    pub const SizedBindingArray: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_SizedBindingArray;
    pub const ChromiumTestingUnimplemented: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnimplemented;
    pub const ChromiumTestingUnsafeExperimental: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingUnsafeExperimental;
    pub const ChromiumTestingExperimental: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingExperimental;
    pub const ChromiumTestingShippedWithKillswitch: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShippedWithKillswitch;
    pub const ChromiumTestingShipped: i32 = raw::WGPUWGSLLanguageFeatureName_WGPUWGSLLanguageFeatureName_ChromiumTestingShipped;
}
pub mod WGPUAdapterType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const DiscreteGPU: i32 = raw::WGPUAdapterType_WGPUAdapterType_DiscreteGPU;
    pub const IntegratedGPU: i32 = raw::WGPUAdapterType_WGPUAdapterType_IntegratedGPU;
    pub const CPU: i32 = raw::WGPUAdapterType_WGPUAdapterType_CPU;
    pub const Unknown: i32 = raw::WGPUAdapterType_WGPUAdapterType_Unknown;
}
pub mod WGPUAddressMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUAddressMode_WGPUAddressMode_Undefined;
    pub const ClampToEdge: i32 = raw::WGPUAddressMode_WGPUAddressMode_ClampToEdge;
    pub const Repeat: i32 = raw::WGPUAddressMode_WGPUAddressMode_Repeat;
    pub const MirrorRepeat: i32 = raw::WGPUAddressMode_WGPUAddressMode_MirrorRepeat;
}
pub mod WGPUAlphaMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Opaque: i32 = raw::WGPUAlphaMode_WGPUAlphaMode_Opaque;
    pub const Premultiplied: i32 = raw::WGPUAlphaMode_WGPUAlphaMode_Premultiplied;
    pub const Unpremultiplied: i32 = raw::WGPUAlphaMode_WGPUAlphaMode_Unpremultiplied;
}
pub mod WGPUBackendType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUBackendType_WGPUBackendType_Undefined;
    pub const Null: i32 = raw::WGPUBackendType_WGPUBackendType_Null;
    pub const WebGPU: i32 = raw::WGPUBackendType_WGPUBackendType_WebGPU;
    pub const D3D11: i32 = raw::WGPUBackendType_WGPUBackendType_D3D11;
    pub const D3D12: i32 = raw::WGPUBackendType_WGPUBackendType_D3D12;
    pub const Metal: i32 = raw::WGPUBackendType_WGPUBackendType_Metal;
    pub const Vulkan: i32 = raw::WGPUBackendType_WGPUBackendType_Vulkan;
    pub const OpenGL: i32 = raw::WGPUBackendType_WGPUBackendType_OpenGL;
    pub const OpenGLES: i32 = raw::WGPUBackendType_WGPUBackendType_OpenGLES;
}
pub mod WGPUBlendFactor {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_Undefined;
    pub const Zero: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_Zero;
    pub const One: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_One;
    pub const Src: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_Src;
    pub const OneMinusSrc: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc;
    pub const SrcAlpha: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_SrcAlpha;
    pub const OneMinusSrcAlpha: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrcAlpha;
    pub const Dst: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_Dst;
    pub const OneMinusDst: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusDst;
    pub const DstAlpha: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_DstAlpha;
    pub const OneMinusDstAlpha: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusDstAlpha;
    pub const SrcAlphaSaturated: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_SrcAlphaSaturated;
    pub const Constant: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_Constant;
    pub const OneMinusConstant: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusConstant;
    pub const Src1: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_Src1;
    pub const OneMinusSrc1: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1;
    pub const Src1Alpha: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_Src1Alpha;
    pub const OneMinusSrc1Alpha: i32 = raw::WGPUBlendFactor_WGPUBlendFactor_OneMinusSrc1Alpha;
}
pub mod WGPUBlendOperation {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUBlendOperation_WGPUBlendOperation_Undefined;
    pub const Add: i32 = raw::WGPUBlendOperation_WGPUBlendOperation_Add;
    pub const Subtract: i32 = raw::WGPUBlendOperation_WGPUBlendOperation_Subtract;
    pub const ReverseSubtract: i32 = raw::WGPUBlendOperation_WGPUBlendOperation_ReverseSubtract;
    pub const Min: i32 = raw::WGPUBlendOperation_WGPUBlendOperation_Min;
    pub const Max: i32 = raw::WGPUBlendOperation_WGPUBlendOperation_Max;
}
pub mod WGPUBufferBindingType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const BindingNotUsed: i32 = raw::WGPUBufferBindingType_WGPUBufferBindingType_BindingNotUsed;
    pub const Undefined: i32 = raw::WGPUBufferBindingType_WGPUBufferBindingType_Undefined;
    pub const Uniform: i32 = raw::WGPUBufferBindingType_WGPUBufferBindingType_Uniform;
    pub const Storage: i32 = raw::WGPUBufferBindingType_WGPUBufferBindingType_Storage;
    pub const ReadOnlyStorage: i32 = raw::WGPUBufferBindingType_WGPUBufferBindingType_ReadOnlyStorage;
}
pub mod WGPUBufferMapState {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Unmapped: i32 = raw::WGPUBufferMapState_WGPUBufferMapState_Unmapped;
    pub const Pending: i32 = raw::WGPUBufferMapState_WGPUBufferMapState_Pending;
    pub const Mapped: i32 = raw::WGPUBufferMapState_WGPUBufferMapState_Mapped;
}
pub mod WGPUCallbackMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const WaitAnyOnly: i32 = raw::WGPUCallbackMode_WGPUCallbackMode_WaitAnyOnly;
    pub const AllowProcessEvents: i32 = raw::WGPUCallbackMode_WGPUCallbackMode_AllowProcessEvents;
    pub const AllowSpontaneous: i32 = raw::WGPUCallbackMode_WGPUCallbackMode_AllowSpontaneous;
}
pub mod WGPUCompareFunction {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_Undefined;
    pub const Never: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_Never;
    pub const Less: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_Less;
    pub const Equal: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_Equal;
    pub const LessEqual: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_LessEqual;
    pub const Greater: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_Greater;
    pub const NotEqual: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_NotEqual;
    pub const GreaterEqual: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_GreaterEqual;
    pub const Always: i32 = raw::WGPUCompareFunction_WGPUCompareFunction_Always;
}
pub mod WGPUCompilationInfoRequestStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_Success;
    pub const CallbackCancelled: i32 = raw::WGPUCompilationInfoRequestStatus_WGPUCompilationInfoRequestStatus_CallbackCancelled;
}
pub mod WGPUCompilationMessageType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Error: i32 = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Error;
    pub const Warning: i32 = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Warning;
    pub const Info: i32 = raw::WGPUCompilationMessageType_WGPUCompilationMessageType_Info;
}
pub mod WGPUCompositeAlphaMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Auto: i32 = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Auto;
    pub const Opaque: i32 = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Opaque;
    pub const Premultiplied: i32 = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Premultiplied;
    pub const Unpremultiplied: i32 = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Unpremultiplied;
    pub const Inherit: i32 = raw::WGPUCompositeAlphaMode_WGPUCompositeAlphaMode_Inherit;
}
pub mod WGPUCreatePipelineAsyncStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_Success;
    pub const CallbackCancelled: i32 = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_CallbackCancelled;
    pub const ValidationError: i32 = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_ValidationError;
    pub const InternalError: i32 = raw::WGPUCreatePipelineAsyncStatus_WGPUCreatePipelineAsyncStatus_InternalError;
}
pub mod WGPUCullMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUCullMode_WGPUCullMode_Undefined;
    pub const None: i32 = raw::WGPUCullMode_WGPUCullMode_None;
    pub const Front: i32 = raw::WGPUCullMode_WGPUCullMode_Front;
    pub const Back: i32 = raw::WGPUCullMode_WGPUCullMode_Back;
}
pub mod WGPUDeviceLostReason {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Unknown: i32 = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_Unknown;
    pub const Destroyed: i32 = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_Destroyed;
    pub const CallbackCancelled: i32 = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_CallbackCancelled;
    pub const FailedCreation: i32 = raw::WGPUDeviceLostReason_WGPUDeviceLostReason_FailedCreation;
}
pub mod WGPUErrorFilter {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Validation: i32 = raw::WGPUErrorFilter_WGPUErrorFilter_Validation;
    pub const OutOfMemory: i32 = raw::WGPUErrorFilter_WGPUErrorFilter_OutOfMemory;
    pub const Internal: i32 = raw::WGPUErrorFilter_WGPUErrorFilter_Internal;
}
pub mod WGPUErrorType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const NoError: i32 = raw::WGPUErrorType_WGPUErrorType_NoError;
    pub const Validation: i32 = raw::WGPUErrorType_WGPUErrorType_Validation;
    pub const OutOfMemory: i32 = raw::WGPUErrorType_WGPUErrorType_OutOfMemory;
    pub const Internal: i32 = raw::WGPUErrorType_WGPUErrorType_Internal;
    pub const Unknown: i32 = raw::WGPUErrorType_WGPUErrorType_Unknown;
}
pub mod WGPUExternalTextureRotation {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Rotate0Degrees: i32 = raw::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate0Degrees;
    pub const Rotate90Degrees: i32 = raw::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate90Degrees;
    pub const Rotate180Degrees: i32 = raw::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate180Degrees;
    pub const Rotate270Degrees: i32 = raw::WGPUExternalTextureRotation_WGPUExternalTextureRotation_Rotate270Degrees;
}
pub mod WGPUFeatureLevel {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUFeatureLevel_WGPUFeatureLevel_Undefined;
    pub const Compatibility: i32 = raw::WGPUFeatureLevel_WGPUFeatureLevel_Compatibility;
    pub const Core: i32 = raw::WGPUFeatureLevel_WGPUFeatureLevel_Core;
}
pub mod WGPUFeatureName {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const DepthClipControl: i32 = raw::WGPUFeatureName_WGPUFeatureName_DepthClipControl;
    pub const Depth32FloatStencil8: i32 = raw::WGPUFeatureName_WGPUFeatureName_Depth32FloatStencil8;
    pub const TimestampQuery: i32 = raw::WGPUFeatureName_WGPUFeatureName_TimestampQuery;
    pub const TextureCompressionBC: i32 = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionBC;
    pub const TextureCompressionBCSliced3D: i32 = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionBCSliced3D;
    pub const TextureCompressionETC2: i32 = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionETC2;
    pub const TextureCompressionASTC: i32 = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTC;
    pub const TextureCompressionASTCSliced3D: i32 = raw::WGPUFeatureName_WGPUFeatureName_TextureCompressionASTCSliced3D;
    pub const IndirectFirstInstance: i32 = raw::WGPUFeatureName_WGPUFeatureName_IndirectFirstInstance;
    pub const ShaderF16: i32 = raw::WGPUFeatureName_WGPUFeatureName_ShaderF16;
    pub const RG11B10UfloatRenderable: i32 = raw::WGPUFeatureName_WGPUFeatureName_RG11B10UfloatRenderable;
    pub const BGRA8UnormStorage: i32 = raw::WGPUFeatureName_WGPUFeatureName_BGRA8UnormStorage;
    pub const Float32Filterable: i32 = raw::WGPUFeatureName_WGPUFeatureName_Float32Filterable;
    pub const Float32Blendable: i32 = raw::WGPUFeatureName_WGPUFeatureName_Float32Blendable;
    pub const ClipDistances: i32 = raw::WGPUFeatureName_WGPUFeatureName_ClipDistances;
    pub const DualSourceBlending: i32 = raw::WGPUFeatureName_WGPUFeatureName_DualSourceBlending;
    pub const Subgroups: i32 = raw::WGPUFeatureName_WGPUFeatureName_Subgroups;
    pub const CoreFeaturesAndLimits: i32 = raw::WGPUFeatureName_WGPUFeatureName_CoreFeaturesAndLimits;
    pub const DawnInternalUsages: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnInternalUsages;
    pub const DawnMultiPlanarFormats: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnMultiPlanarFormats;
    pub const DawnNative: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnNative;
    pub const ChromiumExperimentalTimestampQueryInsidePasses: i32 = raw::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalTimestampQueryInsidePasses;
    pub const ImplicitDeviceSynchronization: i32 = raw::WGPUFeatureName_WGPUFeatureName_ImplicitDeviceSynchronization;
    pub const TransientAttachments: i32 = raw::WGPUFeatureName_WGPUFeatureName_TransientAttachments;
    pub const MSAARenderToSingleSampled: i32 = raw::WGPUFeatureName_WGPUFeatureName_MSAARenderToSingleSampled;
    pub const D3D11MultithreadProtected: i32 = raw::WGPUFeatureName_WGPUFeatureName_D3D11MultithreadProtected;
    pub const ANGLETextureSharing: i32 = raw::WGPUFeatureName_WGPUFeatureName_ANGLETextureSharing;
    pub const PixelLocalStorageCoherent: i32 = raw::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageCoherent;
    pub const PixelLocalStorageNonCoherent: i32 = raw::WGPUFeatureName_WGPUFeatureName_PixelLocalStorageNonCoherent;
    pub const Unorm16TextureFormats: i32 = raw::WGPUFeatureName_WGPUFeatureName_Unorm16TextureFormats;
    pub const Snorm16TextureFormats: i32 = raw::WGPUFeatureName_WGPUFeatureName_Snorm16TextureFormats;
    pub const MultiPlanarFormatExtendedUsages: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatExtendedUsages;
    pub const MultiPlanarFormatP010: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP010;
    pub const HostMappedPointer: i32 = raw::WGPUFeatureName_WGPUFeatureName_HostMappedPointer;
    pub const MultiPlanarRenderTargets: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarRenderTargets;
    pub const MultiPlanarFormatNv12a: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv12a;
    pub const FramebufferFetch: i32 = raw::WGPUFeatureName_WGPUFeatureName_FramebufferFetch;
    pub const BufferMapExtendedUsages: i32 = raw::WGPUFeatureName_WGPUFeatureName_BufferMapExtendedUsages;
    pub const AdapterPropertiesMemoryHeaps: i32 = raw::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesMemoryHeaps;
    pub const AdapterPropertiesD3D: i32 = raw::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesD3D;
    pub const AdapterPropertiesVk: i32 = raw::WGPUFeatureName_WGPUFeatureName_AdapterPropertiesVk;
    pub const R8UnormStorage: i32 = raw::WGPUFeatureName_WGPUFeatureName_R8UnormStorage;
    pub const DawnFormatCapabilities: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnFormatCapabilities;
    pub const DawnDrmFormatCapabilities: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnDrmFormatCapabilities;
    pub const Norm16TextureFormats: i32 = raw::WGPUFeatureName_WGPUFeatureName_Norm16TextureFormats;
    pub const MultiPlanarFormatNv16: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv16;
    pub const MultiPlanarFormatNv24: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatNv24;
    pub const MultiPlanarFormatP210: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP210;
    pub const MultiPlanarFormatP410: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiPlanarFormatP410;
    pub const SharedTextureMemoryVkDedicatedAllocation: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryVkDedicatedAllocation;
    pub const SharedTextureMemoryAHardwareBuffer: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryAHardwareBuffer;
    pub const SharedTextureMemoryDmaBuf: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDmaBuf;
    pub const SharedTextureMemoryOpaqueFD: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryOpaqueFD;
    pub const SharedTextureMemoryZirconHandle: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryZirconHandle;
    pub const SharedTextureMemoryDXGISharedHandle: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryDXGISharedHandle;
    pub const SharedTextureMemoryD3D11Texture2D: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryD3D11Texture2D;
    pub const SharedTextureMemoryIOSurface: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryIOSurface;
    pub const SharedTextureMemoryEGLImage: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedTextureMemoryEGLImage;
    pub const SharedFenceVkSemaphoreOpaqueFD: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreOpaqueFD;
    pub const SharedFenceSyncFD: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedFenceSyncFD;
    pub const SharedFenceVkSemaphoreZirconHandle: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedFenceVkSemaphoreZirconHandle;
    pub const SharedFenceDXGISharedHandle: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedFenceDXGISharedHandle;
    pub const SharedFenceMTLSharedEvent: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedFenceMTLSharedEvent;
    pub const SharedBufferMemoryD3D12Resource: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedBufferMemoryD3D12Resource;
    pub const StaticSamplers: i32 = raw::WGPUFeatureName_WGPUFeatureName_StaticSamplers;
    pub const YCbCrVulkanSamplers: i32 = raw::WGPUFeatureName_WGPUFeatureName_YCbCrVulkanSamplers;
    pub const ShaderModuleCompilationOptions: i32 = raw::WGPUFeatureName_WGPUFeatureName_ShaderModuleCompilationOptions;
    pub const DawnLoadResolveTexture: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnLoadResolveTexture;
    pub const DawnPartialLoadResolveTexture: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnPartialLoadResolveTexture;
    pub const MultiDrawIndirect: i32 = raw::WGPUFeatureName_WGPUFeatureName_MultiDrawIndirect;
    pub const DawnTexelCopyBufferRowAlignment: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnTexelCopyBufferRowAlignment;
    pub const FlexibleTextureViews: i32 = raw::WGPUFeatureName_WGPUFeatureName_FlexibleTextureViews;
    pub const ChromiumExperimentalSubgroupMatrix: i32 = raw::WGPUFeatureName_WGPUFeatureName_ChromiumExperimentalSubgroupMatrix;
    pub const SharedFenceEGLSync: i32 = raw::WGPUFeatureName_WGPUFeatureName_SharedFenceEGLSync;
    pub const DawnDeviceAllocatorControl: i32 = raw::WGPUFeatureName_WGPUFeatureName_DawnDeviceAllocatorControl;
}
pub mod WGPUFilterMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUFilterMode_WGPUFilterMode_Undefined;
    pub const Nearest: i32 = raw::WGPUFilterMode_WGPUFilterMode_Nearest;
    pub const Linear: i32 = raw::WGPUFilterMode_WGPUFilterMode_Linear;
}
pub mod WGPUFrontFace {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUFrontFace_WGPUFrontFace_Undefined;
    pub const CCW: i32 = raw::WGPUFrontFace_WGPUFrontFace_CCW;
    pub const CW: i32 = raw::WGPUFrontFace_WGPUFrontFace_CW;
}
pub mod WGPUIndexFormat {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUIndexFormat_WGPUIndexFormat_Undefined;
    pub const Uint16: i32 = raw::WGPUIndexFormat_WGPUIndexFormat_Uint16;
    pub const Uint32: i32 = raw::WGPUIndexFormat_WGPUIndexFormat_Uint32;
}
pub mod WGPULoadOp {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPULoadOp_WGPULoadOp_Undefined;
    pub const Load: i32 = raw::WGPULoadOp_WGPULoadOp_Load;
    pub const Clear: i32 = raw::WGPULoadOp_WGPULoadOp_Clear;
    pub const ExpandResolveTexture: i32 = raw::WGPULoadOp_WGPULoadOp_ExpandResolveTexture;
}
pub mod WGPULoggingType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Verbose: i32 = raw::WGPULoggingType_WGPULoggingType_Verbose;
    pub const Info: i32 = raw::WGPULoggingType_WGPULoggingType_Info;
    pub const Warning: i32 = raw::WGPULoggingType_WGPULoggingType_Warning;
    pub const Error: i32 = raw::WGPULoggingType_WGPULoggingType_Error;
}
pub mod WGPUMapAsyncStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Success;
    pub const CallbackCancelled: i32 = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_CallbackCancelled;
    pub const Error: i32 = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Error;
    pub const Aborted: i32 = raw::WGPUMapAsyncStatus_WGPUMapAsyncStatus_Aborted;
}
pub mod WGPUMipmapFilterMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Undefined;
    pub const Nearest: i32 = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Nearest;
    pub const Linear: i32 = raw::WGPUMipmapFilterMode_WGPUMipmapFilterMode_Linear;
}
pub mod WGPUOptionalBool {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const False: i32 = raw::WGPUOptionalBool_WGPUOptionalBool_False;
    pub const True: i32 = raw::WGPUOptionalBool_WGPUOptionalBool_True;
    pub const Undefined: i32 = raw::WGPUOptionalBool_WGPUOptionalBool_Undefined;
}
pub mod WGPUPopErrorScopeStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Success;
    pub const CallbackCancelled: i32 = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_CallbackCancelled;
    pub const Error: i32 = raw::WGPUPopErrorScopeStatus_WGPUPopErrorScopeStatus_Error;
}
pub mod WGPUPowerPreference {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUPowerPreference_WGPUPowerPreference_Undefined;
    pub const LowPower: i32 = raw::WGPUPowerPreference_WGPUPowerPreference_LowPower;
    pub const HighPerformance: i32 = raw::WGPUPowerPreference_WGPUPowerPreference_HighPerformance;
}
pub mod WGPUPredefinedColorSpace {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const SRGB: i32 = raw::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_SRGB;
    pub const DisplayP3: i32 = raw::WGPUPredefinedColorSpace_WGPUPredefinedColorSpace_DisplayP3;
}
pub mod WGPUPresentMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUPresentMode_WGPUPresentMode_Undefined;
    pub const Fifo: i32 = raw::WGPUPresentMode_WGPUPresentMode_Fifo;
    pub const FifoRelaxed: i32 = raw::WGPUPresentMode_WGPUPresentMode_FifoRelaxed;
    pub const Immediate: i32 = raw::WGPUPresentMode_WGPUPresentMode_Immediate;
    pub const Mailbox: i32 = raw::WGPUPresentMode_WGPUPresentMode_Mailbox;
}
pub mod WGPUPrimitiveTopology {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_Undefined;
    pub const PointList: i32 = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_PointList;
    pub const LineList: i32 = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineList;
    pub const LineStrip: i32 = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_LineStrip;
    pub const TriangleList: i32 = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleList;
    pub const TriangleStrip: i32 = raw::WGPUPrimitiveTopology_WGPUPrimitiveTopology_TriangleStrip;
}
pub mod WGPUQueryType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Occlusion: i32 = raw::WGPUQueryType_WGPUQueryType_Occlusion;
    pub const Timestamp: i32 = raw::WGPUQueryType_WGPUQueryType_Timestamp;
}
pub mod WGPUQueueWorkDoneStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Success;
    pub const CallbackCancelled: i32 = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_CallbackCancelled;
    pub const Error: i32 = raw::WGPUQueueWorkDoneStatus_WGPUQueueWorkDoneStatus_Error;
}
pub mod WGPURequestAdapterStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Success;
    pub const CallbackCancelled: i32 = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_CallbackCancelled;
    pub const Unavailable: i32 = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Unavailable;
    pub const Error: i32 = raw::WGPURequestAdapterStatus_WGPURequestAdapterStatus_Error;
}
pub mod WGPURequestDeviceStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Success;
    pub const CallbackCancelled: i32 = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_CallbackCancelled;
    pub const Error: i32 = raw::WGPURequestDeviceStatus_WGPURequestDeviceStatus_Error;
}
pub mod WGPUSType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const ShaderSourceSPIRV: i32 = raw::WGPUSType_WGPUSType_ShaderSourceSPIRV;
    pub const ShaderSourceWGSL: i32 = raw::WGPUSType_WGPUSType_ShaderSourceWGSL;
    pub const RenderPassMaxDrawCount: i32 = raw::WGPUSType_WGPUSType_RenderPassMaxDrawCount;
    pub const SurfaceSourceMetalLayer: i32 = raw::WGPUSType_WGPUSType_SurfaceSourceMetalLayer;
    pub const SurfaceSourceWindowsHWND: i32 = raw::WGPUSType_WGPUSType_SurfaceSourceWindowsHWND;
    pub const SurfaceSourceXlibWindow: i32 = raw::WGPUSType_WGPUSType_SurfaceSourceXlibWindow;
    pub const SurfaceSourceWaylandSurface: i32 = raw::WGPUSType_WGPUSType_SurfaceSourceWaylandSurface;
    pub const SurfaceSourceAndroidNativeWindow: i32 = raw::WGPUSType_WGPUSType_SurfaceSourceAndroidNativeWindow;
    pub const SurfaceSourceXCBWindow: i32 = raw::WGPUSType_WGPUSType_SurfaceSourceXCBWindow;
    pub const SurfaceColorManagement: i32 = raw::WGPUSType_WGPUSType_SurfaceColorManagement;
    pub const RequestAdapterWebXROptions: i32 = raw::WGPUSType_WGPUSType_RequestAdapterWebXROptions;
    pub const AdapterPropertiesSubgroups: i32 = raw::WGPUSType_WGPUSType_AdapterPropertiesSubgroups;
    pub const BindGroupLayoutEntryArraySize: i32 = raw::WGPUSType_WGPUSType_BindGroupLayoutEntryArraySize;
    pub const TextureBindingViewDimensionDescriptor: i32 = raw::WGPUSType_WGPUSType_TextureBindingViewDimensionDescriptor;
    pub const EmscriptenSurfaceSourceCanvasHTMLSelector: i32 = raw::WGPUSType_WGPUSType_EmscriptenSurfaceSourceCanvasHTMLSelector;
    pub const SurfaceDescriptorFromWindowsCoreWindow: i32 = raw::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsCoreWindow;
    pub const ExternalTextureBindingEntry: i32 = raw::WGPUSType_WGPUSType_ExternalTextureBindingEntry;
    pub const ExternalTextureBindingLayout: i32 = raw::WGPUSType_WGPUSType_ExternalTextureBindingLayout;
    pub const SurfaceDescriptorFromWindowsUWPSwapChainPanel: i32 = raw::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsUWPSwapChainPanel;
    pub const DawnTextureInternalUsageDescriptor: i32 = raw::WGPUSType_WGPUSType_DawnTextureInternalUsageDescriptor;
    pub const DawnEncoderInternalUsageDescriptor: i32 = raw::WGPUSType_WGPUSType_DawnEncoderInternalUsageDescriptor;
    pub const DawnInstanceDescriptor: i32 = raw::WGPUSType_WGPUSType_DawnInstanceDescriptor;
    pub const DawnCacheDeviceDescriptor: i32 = raw::WGPUSType_WGPUSType_DawnCacheDeviceDescriptor;
    pub const DawnAdapterPropertiesPowerPreference: i32 = raw::WGPUSType_WGPUSType_DawnAdapterPropertiesPowerPreference;
    pub const DawnBufferDescriptorErrorInfoFromWireClient: i32 = raw::WGPUSType_WGPUSType_DawnBufferDescriptorErrorInfoFromWireClient;
    pub const DawnTogglesDescriptor: i32 = raw::WGPUSType_WGPUSType_DawnTogglesDescriptor;
    pub const DawnShaderModuleSPIRVOptionsDescriptor: i32 = raw::WGPUSType_WGPUSType_DawnShaderModuleSPIRVOptionsDescriptor;
    pub const RequestAdapterOptionsLUID: i32 = raw::WGPUSType_WGPUSType_RequestAdapterOptionsLUID;
    pub const RequestAdapterOptionsGetGLProc: i32 = raw::WGPUSType_WGPUSType_RequestAdapterOptionsGetGLProc;
    pub const RequestAdapterOptionsD3D11Device: i32 = raw::WGPUSType_WGPUSType_RequestAdapterOptionsD3D11Device;
    pub const DawnRenderPassColorAttachmentRenderToSingleSampled: i32 = raw::WGPUSType_WGPUSType_DawnRenderPassColorAttachmentRenderToSingleSampled;
    pub const RenderPassPixelLocalStorage: i32 = raw::WGPUSType_WGPUSType_RenderPassPixelLocalStorage;
    pub const PipelineLayoutPixelLocalStorage: i32 = raw::WGPUSType_WGPUSType_PipelineLayoutPixelLocalStorage;
    pub const BufferHostMappedPointer: i32 = raw::WGPUSType_WGPUSType_BufferHostMappedPointer;
    pub const AdapterPropertiesMemoryHeaps: i32 = raw::WGPUSType_WGPUSType_AdapterPropertiesMemoryHeaps;
    pub const AdapterPropertiesD3D: i32 = raw::WGPUSType_WGPUSType_AdapterPropertiesD3D;
    pub const AdapterPropertiesVk: i32 = raw::WGPUSType_WGPUSType_AdapterPropertiesVk;
    pub const DawnWireWGSLControl: i32 = raw::WGPUSType_WGPUSType_DawnWireWGSLControl;
    pub const DawnWGSLBlocklist: i32 = raw::WGPUSType_WGPUSType_DawnWGSLBlocklist;
    pub const DawnDrmFormatCapabilities: i32 = raw::WGPUSType_WGPUSType_DawnDrmFormatCapabilities;
    pub const ShaderModuleCompilationOptions: i32 = raw::WGPUSType_WGPUSType_ShaderModuleCompilationOptions;
    pub const ColorTargetStateExpandResolveTextureDawn: i32 = raw::WGPUSType_WGPUSType_ColorTargetStateExpandResolveTextureDawn;
    pub const RenderPassDescriptorExpandResolveRect: i32 = raw::WGPUSType_WGPUSType_RenderPassDescriptorExpandResolveRect;
    pub const SharedTextureMemoryVkDedicatedAllocationDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryVkDedicatedAllocationDescriptor;
    pub const SharedTextureMemoryAHardwareBufferDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferDescriptor;
    pub const SharedTextureMemoryDmaBufDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryDmaBufDescriptor;
    pub const SharedTextureMemoryOpaqueFDDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryOpaqueFDDescriptor;
    pub const SharedTextureMemoryZirconHandleDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryZirconHandleDescriptor;
    pub const SharedTextureMemoryDXGISharedHandleDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryDXGISharedHandleDescriptor;
    pub const SharedTextureMemoryD3D11Texture2DDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryD3D11Texture2DDescriptor;
    pub const SharedTextureMemoryIOSurfaceDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryIOSurfaceDescriptor;
    pub const SharedTextureMemoryEGLImageDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryEGLImageDescriptor;
    pub const SharedTextureMemoryInitializedBeginState: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryInitializedBeginState;
    pub const SharedTextureMemoryInitializedEndState: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryInitializedEndState;
    pub const SharedTextureMemoryVkImageLayoutBeginState: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutBeginState;
    pub const SharedTextureMemoryVkImageLayoutEndState: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryVkImageLayoutEndState;
    pub const SharedTextureMemoryD3DSwapchainBeginState: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryD3DSwapchainBeginState;
    pub const SharedFenceVkSemaphoreOpaqueFDDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDDescriptor;
    pub const SharedFenceVkSemaphoreOpaqueFDExportInfo: i32 = raw::WGPUSType_WGPUSType_SharedFenceVkSemaphoreOpaqueFDExportInfo;
    pub const SharedFenceSyncFDDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedFenceSyncFDDescriptor;
    pub const SharedFenceSyncFDExportInfo: i32 = raw::WGPUSType_WGPUSType_SharedFenceSyncFDExportInfo;
    pub const SharedFenceVkSemaphoreZirconHandleDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleDescriptor;
    pub const SharedFenceVkSemaphoreZirconHandleExportInfo: i32 = raw::WGPUSType_WGPUSType_SharedFenceVkSemaphoreZirconHandleExportInfo;
    pub const SharedFenceDXGISharedHandleDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleDescriptor;
    pub const SharedFenceDXGISharedHandleExportInfo: i32 = raw::WGPUSType_WGPUSType_SharedFenceDXGISharedHandleExportInfo;
    pub const SharedFenceMTLSharedEventDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedFenceMTLSharedEventDescriptor;
    pub const SharedFenceMTLSharedEventExportInfo: i32 = raw::WGPUSType_WGPUSType_SharedFenceMTLSharedEventExportInfo;
    pub const SharedBufferMemoryD3D12ResourceDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedBufferMemoryD3D12ResourceDescriptor;
    pub const StaticSamplerBindingLayout: i32 = raw::WGPUSType_WGPUSType_StaticSamplerBindingLayout;
    pub const YCbCrVkDescriptor: i32 = raw::WGPUSType_WGPUSType_YCbCrVkDescriptor;
    pub const SharedTextureMemoryAHardwareBufferProperties: i32 = raw::WGPUSType_WGPUSType_SharedTextureMemoryAHardwareBufferProperties;
    pub const AHardwareBufferProperties: i32 = raw::WGPUSType_WGPUSType_AHardwareBufferProperties;
    pub const DawnTexelCopyBufferRowAlignmentLimits: i32 = raw::WGPUSType_WGPUSType_DawnTexelCopyBufferRowAlignmentLimits;
    pub const AdapterPropertiesSubgroupMatrixConfigs: i32 = raw::WGPUSType_WGPUSType_AdapterPropertiesSubgroupMatrixConfigs;
    pub const SharedFenceEGLSyncDescriptor: i32 = raw::WGPUSType_WGPUSType_SharedFenceEGLSyncDescriptor;
    pub const SharedFenceEGLSyncExportInfo: i32 = raw::WGPUSType_WGPUSType_SharedFenceEGLSyncExportInfo;
    pub const DawnInjectedInvalidSType: i32 = raw::WGPUSType_WGPUSType_DawnInjectedInvalidSType;
    pub const DawnCompilationMessageUtf16: i32 = raw::WGPUSType_WGPUSType_DawnCompilationMessageUtf16;
    pub const DawnFakeBufferOOMForTesting: i32 = raw::WGPUSType_WGPUSType_DawnFakeBufferOOMForTesting;
    pub const SurfaceDescriptorFromWindowsWinUISwapChainPanel: i32 = raw::WGPUSType_WGPUSType_SurfaceDescriptorFromWindowsWinUISwapChainPanel;
    pub const DawnDeviceAllocatorControl: i32 = raw::WGPUSType_WGPUSType_DawnDeviceAllocatorControl;
    pub const DawnHostMappedPointerLimits: i32 = raw::WGPUSType_WGPUSType_DawnHostMappedPointerLimits;
    pub const RenderPassDescriptorResolveRect: i32 = raw::WGPUSType_WGPUSType_RenderPassDescriptorResolveRect;
}
pub mod WGPUSamplerBindingType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const BindingNotUsed: i32 = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_BindingNotUsed;
    pub const Undefined: i32 = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Undefined;
    pub const Filtering: i32 = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Filtering;
    pub const NonFiltering: i32 = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_NonFiltering;
    pub const Comparison: i32 = raw::WGPUSamplerBindingType_WGPUSamplerBindingType_Comparison;
}
pub mod WGPUSharedFenceType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const VkSemaphoreOpaqueFD: i32 = raw::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreOpaqueFD;
    pub const SyncFD: i32 = raw::WGPUSharedFenceType_WGPUSharedFenceType_SyncFD;
    pub const VkSemaphoreZirconHandle: i32 = raw::WGPUSharedFenceType_WGPUSharedFenceType_VkSemaphoreZirconHandle;
    pub const DXGISharedHandle: i32 = raw::WGPUSharedFenceType_WGPUSharedFenceType_DXGISharedHandle;
    pub const MTLSharedEvent: i32 = raw::WGPUSharedFenceType_WGPUSharedFenceType_MTLSharedEvent;
    pub const EGLSync: i32 = raw::WGPUSharedFenceType_WGPUSharedFenceType_EGLSync;
}
pub mod WGPUStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPUStatus_WGPUStatus_Success;
    pub const Error: i32 = raw::WGPUStatus_WGPUStatus_Error;
}
pub mod WGPUStencilOperation {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_Undefined;
    pub const Keep: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_Keep;
    pub const Zero: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_Zero;
    pub const Replace: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_Replace;
    pub const Invert: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_Invert;
    pub const IncrementClamp: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_IncrementClamp;
    pub const DecrementClamp: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_DecrementClamp;
    pub const IncrementWrap: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_IncrementWrap;
    pub const DecrementWrap: i32 = raw::WGPUStencilOperation_WGPUStencilOperation_DecrementWrap;
}
pub mod WGPUStorageTextureAccess {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const BindingNotUsed: i32 = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_BindingNotUsed;
    pub const Undefined: i32 = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_Undefined;
    pub const WriteOnly: i32 = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_WriteOnly;
    pub const ReadOnly: i32 = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadOnly;
    pub const ReadWrite: i32 = raw::WGPUStorageTextureAccess_WGPUStorageTextureAccess_ReadWrite;
}
pub mod WGPUStoreOp {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUStoreOp_WGPUStoreOp_Undefined;
    pub const Store: i32 = raw::WGPUStoreOp_WGPUStoreOp_Store;
    pub const Discard: i32 = raw::WGPUStoreOp_WGPUStoreOp_Discard;
}
pub mod WGPUSubgroupMatrixComponentType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const F32: i32 = raw::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F32;
    pub const F16: i32 = raw::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_F16;
    pub const U32: i32 = raw::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_U32;
    pub const I32: i32 = raw::WGPUSubgroupMatrixComponentType_WGPUSubgroupMatrixComponentType_I32;
}
pub mod WGPUSurfaceGetCurrentTextureStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const SuccessOptimal: i32 = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessOptimal;
    pub const SuccessSuboptimal: i32 = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_SuccessSuboptimal;
    pub const Timeout: i32 = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Timeout;
    pub const Outdated: i32 = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Outdated;
    pub const Lost: i32 = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Lost;
    pub const Error: i32 = raw::WGPUSurfaceGetCurrentTextureStatus_WGPUSurfaceGetCurrentTextureStatus_Error;
}
pub mod WGPUTextureAspect {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUTextureAspect_WGPUTextureAspect_Undefined;
    pub const All: i32 = raw::WGPUTextureAspect_WGPUTextureAspect_All;
    pub const StencilOnly: i32 = raw::WGPUTextureAspect_WGPUTextureAspect_StencilOnly;
    pub const DepthOnly: i32 = raw::WGPUTextureAspect_WGPUTextureAspect_DepthOnly;
    pub const Plane0Only: i32 = raw::WGPUTextureAspect_WGPUTextureAspect_Plane0Only;
    pub const Plane1Only: i32 = raw::WGPUTextureAspect_WGPUTextureAspect_Plane1Only;
    pub const Plane2Only: i32 = raw::WGPUTextureAspect_WGPUTextureAspect_Plane2Only;
}
pub mod WGPUTextureDimension {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUTextureDimension_WGPUTextureDimension_Undefined;
    pub const D1: i32 = raw::WGPUTextureDimension_WGPUTextureDimension_1D;
    pub const D2: i32 = raw::WGPUTextureDimension_WGPUTextureDimension_2D;
    pub const D3: i32 = raw::WGPUTextureDimension_WGPUTextureDimension_3D;
}
pub mod WGPUTextureFormat {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_Undefined;
    pub const R8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8Unorm;
    pub const R8Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8Snorm;
    pub const R8Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8Uint;
    pub const R8Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8Sint;
    pub const R16Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R16Uint;
    pub const R16Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R16Sint;
    pub const R16Float: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R16Float;
    pub const RG8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Unorm;
    pub const RG8Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Snorm;
    pub const RG8Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Uint;
    pub const RG8Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG8Sint;
    pub const R32Float: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R32Float;
    pub const R32Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R32Uint;
    pub const R32Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R32Sint;
    pub const RG16Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Uint;
    pub const RG16Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Sint;
    pub const RG16Float: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Float;
    pub const RGBA8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Unorm;
    pub const RGBA8UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8UnormSrgb;
    pub const RGBA8Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Snorm;
    pub const RGBA8Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Uint;
    pub const RGBA8Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA8Sint;
    pub const BGRA8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BGRA8Unorm;
    pub const BGRA8UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BGRA8UnormSrgb;
    pub const RGB10A2Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Uint;
    pub const RGB10A2Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGB10A2Unorm;
    pub const RG11B10Ufloat: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG11B10Ufloat;
    pub const RGB9E5Ufloat: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGB9E5Ufloat;
    pub const RG32Float: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Float;
    pub const RG32Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Uint;
    pub const RG32Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG32Sint;
    pub const RGBA16Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Uint;
    pub const RGBA16Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Sint;
    pub const RGBA16Float: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Float;
    pub const RGBA32Float: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Float;
    pub const RGBA32Uint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Uint;
    pub const RGBA32Sint: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA32Sint;
    pub const Stencil8: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_Stencil8;
    pub const Depth16Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth16Unorm;
    pub const Depth24Plus: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth24Plus;
    pub const Depth24PlusStencil8: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth24PlusStencil8;
    pub const Depth32Float: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth32Float;
    pub const Depth32FloatStencil8: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_Depth32FloatStencil8;
    pub const BC1RGBAUnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnorm;
    pub const BC1RGBAUnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC1RGBAUnormSrgb;
    pub const BC2RGBAUnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnorm;
    pub const BC2RGBAUnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC2RGBAUnormSrgb;
    pub const BC3RGBAUnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnorm;
    pub const BC3RGBAUnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC3RGBAUnormSrgb;
    pub const BC4RUnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC4RUnorm;
    pub const BC4RSnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC4RSnorm;
    pub const BC5RGUnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC5RGUnorm;
    pub const BC5RGSnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC5RGSnorm;
    pub const BC6HRGBUfloat: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBUfloat;
    pub const BC6HRGBFloat: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC6HRGBFloat;
    pub const BC7RGBAUnorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnorm;
    pub const BC7RGBAUnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_BC7RGBAUnormSrgb;
    pub const ETC2RGB8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8Unorm;
    pub const ETC2RGB8UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8UnormSrgb;
    pub const ETC2RGB8A1Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1Unorm;
    pub const ETC2RGB8A1UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGB8A1UnormSrgb;
    pub const ETC2RGBA8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8Unorm;
    pub const ETC2RGBA8UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ETC2RGBA8UnormSrgb;
    pub const EACR11Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_EACR11Unorm;
    pub const EACR11Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_EACR11Snorm;
    pub const EACRG11Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_EACRG11Unorm;
    pub const EACRG11Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_EACRG11Snorm;
    pub const ASTC4x4Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4Unorm;
    pub const ASTC4x4UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC4x4UnormSrgb;
    pub const ASTC5x4Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4Unorm;
    pub const ASTC5x4UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x4UnormSrgb;
    pub const ASTC5x5Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5Unorm;
    pub const ASTC5x5UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC5x5UnormSrgb;
    pub const ASTC6x5Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5Unorm;
    pub const ASTC6x5UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x5UnormSrgb;
    pub const ASTC6x6Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6Unorm;
    pub const ASTC6x6UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC6x6UnormSrgb;
    pub const ASTC8x5Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5Unorm;
    pub const ASTC8x5UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x5UnormSrgb;
    pub const ASTC8x6Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6Unorm;
    pub const ASTC8x6UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x6UnormSrgb;
    pub const ASTC8x8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8Unorm;
    pub const ASTC8x8UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC8x8UnormSrgb;
    pub const ASTC10x5Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5Unorm;
    pub const ASTC10x5UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x5UnormSrgb;
    pub const ASTC10x6Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6Unorm;
    pub const ASTC10x6UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x6UnormSrgb;
    pub const ASTC10x8Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8Unorm;
    pub const ASTC10x8UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x8UnormSrgb;
    pub const ASTC10x10Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10Unorm;
    pub const ASTC10x10UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC10x10UnormSrgb;
    pub const ASTC12x10Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10Unorm;
    pub const ASTC12x10UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x10UnormSrgb;
    pub const ASTC12x12Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12Unorm;
    pub const ASTC12x12UnormSrgb: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_ASTC12x12UnormSrgb;
    pub const R16Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R16Unorm;
    pub const RG16Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Unorm;
    pub const RGBA16Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Unorm;
    pub const R16Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R16Snorm;
    pub const RG16Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RG16Snorm;
    pub const RGBA16Snorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_RGBA16Snorm;
    pub const R8BG8Biplanar420Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar420Unorm;
    pub const R10X6BG10X6Biplanar420Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar420Unorm;
    pub const R8BG8A8Triplanar420Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8BG8A8Triplanar420Unorm;
    pub const R8BG8Biplanar422Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar422Unorm;
    pub const R8BG8Biplanar444Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R8BG8Biplanar444Unorm;
    pub const R10X6BG10X6Biplanar422Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar422Unorm;
    pub const R10X6BG10X6Biplanar444Unorm: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_R10X6BG10X6Biplanar444Unorm;
    pub const External: i32 = raw::WGPUTextureFormat_WGPUTextureFormat_External;
}
pub mod WGPUTextureSampleType {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const BindingNotUsed: i32 = raw::WGPUTextureSampleType_WGPUTextureSampleType_BindingNotUsed;
    pub const Undefined: i32 = raw::WGPUTextureSampleType_WGPUTextureSampleType_Undefined;
    pub const Float: i32 = raw::WGPUTextureSampleType_WGPUTextureSampleType_Float;
    pub const UnfilterableFloat: i32 = raw::WGPUTextureSampleType_WGPUTextureSampleType_UnfilterableFloat;
    pub const Depth: i32 = raw::WGPUTextureSampleType_WGPUTextureSampleType_Depth;
    pub const Sint: i32 = raw::WGPUTextureSampleType_WGPUTextureSampleType_Sint;
    pub const Uint: i32 = raw::WGPUTextureSampleType_WGPUTextureSampleType_Uint;
}
pub mod WGPUTextureViewDimension {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_Undefined;
    pub const D1: i32 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_1D;
    pub const D2: i32 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_2D;
    pub const D2Array: i32 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_2DArray;
    pub const Cube: i32 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_Cube;
    pub const CubeArray: i32 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_CubeArray;
    pub const D3: i32 = raw::WGPUTextureViewDimension_WGPUTextureViewDimension_3D;
}
pub mod WGPUToneMappingMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Standard: i32 = raw::WGPUToneMappingMode_WGPUToneMappingMode_Standard;
    pub const Extended: i32 = raw::WGPUToneMappingMode_WGPUToneMappingMode_Extended;
}
pub mod WGPUVertexFormat {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Uint8: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8;
    pub const Uint8x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8x2;
    pub const Uint8x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint8x4;
    pub const Sint8: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8;
    pub const Sint8x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8x2;
    pub const Sint8x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint8x4;
    pub const Unorm8: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8;
    pub const Unorm8x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x2;
    pub const Unorm8x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4;
    pub const Snorm8: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8;
    pub const Snorm8x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8x2;
    pub const Snorm8x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm8x4;
    pub const Uint16: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16;
    pub const Uint16x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16x2;
    pub const Uint16x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint16x4;
    pub const Sint16: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16;
    pub const Sint16x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16x2;
    pub const Sint16x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint16x4;
    pub const Unorm16: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16;
    pub const Unorm16x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16x2;
    pub const Unorm16x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm16x4;
    pub const Snorm16: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16;
    pub const Snorm16x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16x2;
    pub const Snorm16x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Snorm16x4;
    pub const Float16: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16;
    pub const Float16x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16x2;
    pub const Float16x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float16x4;
    pub const Float32: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32;
    pub const Float32x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x2;
    pub const Float32x3: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x3;
    pub const Float32x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Float32x4;
    pub const Uint32: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32;
    pub const Uint32x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x2;
    pub const Uint32x3: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x3;
    pub const Uint32x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Uint32x4;
    pub const Sint32: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32;
    pub const Sint32x2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x2;
    pub const Sint32x3: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x3;
    pub const Sint32x4: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Sint32x4;
    pub const Unorm10_10_10_2: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm10_10_10_2;
    pub const Unorm8x4BGRA: i32 = raw::WGPUVertexFormat_WGPUVertexFormat_Unorm8x4BGRA;
}
pub mod WGPUVertexStepMode {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Undefined: i32 = raw::WGPUVertexStepMode_WGPUVertexStepMode_Undefined;
    pub const Vertex: i32 = raw::WGPUVertexStepMode_WGPUVertexStepMode_Vertex;
    pub const Instance: i32 = raw::WGPUVertexStepMode_WGPUVertexStepMode_Instance;
}
pub mod WGPUWaitStatus {
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    use crate::raw;
    pub const Success: i32 = raw::WGPUWaitStatus_WGPUWaitStatus_Success;
    pub const TimedOut: i32 = raw::WGPUWaitStatus_WGPUWaitStatus_TimedOut;
    pub const Error: i32 = raw::WGPUWaitStatus_WGPUWaitStatus_Error;
}
pub mod WGPUBufferUsage {
    #![allow(non_snake_case)]
    use crate::raw;
    pub const NONE: u64 = raw::WGPUBufferUsage_None;
    pub const MAP_READ: u64 = raw::WGPUBufferUsage_MapRead;
    pub const MAP_WRITE: u64 = raw::WGPUBufferUsage_MapWrite;
    pub const COPY_SRC: u64 = raw::WGPUBufferUsage_CopySrc;
    pub const COPY_DST: u64 = raw::WGPUBufferUsage_CopyDst;
    pub const INDEX: u64 = raw::WGPUBufferUsage_Index;
    pub const VERTEX: u64 = raw::WGPUBufferUsage_Vertex;
    pub const UNIFORM: u64 = raw::WGPUBufferUsage_Uniform;
    pub const STORAGE: u64 = raw::WGPUBufferUsage_Storage;
    pub const INDIRECT: u64 = raw::WGPUBufferUsage_Indirect;
    pub const QUERY_RESOLVE: u64 = raw::WGPUBufferUsage_QueryResolve;
}
pub mod WGPUColorWriteMask {
    #![allow(non_snake_case)]
    use crate::raw;
    pub const NONE: u64 = raw::WGPUColorWriteMask_None;
    pub const RED: u64 = raw::WGPUColorWriteMask_Red;
    pub const GREEN: u64 = raw::WGPUColorWriteMask_Green;
    pub const BLUE: u64 = raw::WGPUColorWriteMask_Blue;
    pub const ALPHA: u64 = raw::WGPUColorWriteMask_Alpha;
    pub const ALL: u64 = raw::WGPUColorWriteMask_All;
}
pub mod WGPUHeapProperty {
    #![allow(non_snake_case)]
    use crate::raw;
    pub const NONE: u64 = raw::WGPUHeapProperty_None;
    pub const DEVICE_LOCAL: u64 = raw::WGPUHeapProperty_DeviceLocal;
    pub const HOST_VISIBLE: u64 = raw::WGPUHeapProperty_HostVisible;
    pub const HOST_COHERENT: u64 = raw::WGPUHeapProperty_HostCoherent;
    pub const HOST_UNCACHED: u64 = raw::WGPUHeapProperty_HostUncached;
    pub const HOST_CACHED: u64 = raw::WGPUHeapProperty_HostCached;
}
pub mod WGPUMapMode {
    #![allow(non_snake_case)]
    use crate::raw;
    pub const NONE: u64 = raw::WGPUMapMode_None;
    pub const READ: u64 = raw::WGPUMapMode_Read;
    pub const WRITE: u64 = raw::WGPUMapMode_Write;
}
pub mod WGPUShaderStage {
    #![allow(non_snake_case)]
    use crate::raw;
    pub const NONE: u64 = raw::WGPUShaderStage_None;
    pub const VERTEX: u64 = raw::WGPUShaderStage_Vertex;
    pub const FRAGMENT: u64 = raw::WGPUShaderStage_Fragment;
    pub const COMPUTE: u64 = raw::WGPUShaderStage_Compute;
}
pub mod WGPUTextureUsage {
    #![allow(non_snake_case)]
    use crate::raw;
    pub const NONE: u64 = raw::WGPUTextureUsage_None;
    pub const COPY_SRC: u64 = raw::WGPUTextureUsage_CopySrc;
    pub const COPY_DST: u64 = raw::WGPUTextureUsage_CopyDst;
    pub const TEXTURE_BINDING: u64 = raw::WGPUTextureUsage_TextureBinding;
    pub const STORAGE_BINDING: u64 = raw::WGPUTextureUsage_StorageBinding;
    pub const RENDER_ATTACHMENT: u64 = raw::WGPUTextureUsage_RenderAttachment;
    pub const TRANSIENT_ATTACHMENT: u64 = raw::WGPUTextureUsage_TransientAttachment;
    pub const STORAGE_ATTACHMENT: u64 = raw::WGPUTextureUsage_StorageAttachment;
}
pub use raw::WGPUINTERNAL_HAVE_EMDAWNWEBGPU_HEADER;
pub use raw::WGPUAHardwareBufferProperties;
pub use raw::WGPUAdapterInfo;
pub use raw::WGPUAdapterPropertiesD3D;
pub use raw::WGPUAdapterPropertiesMemoryHeaps;
pub use raw::WGPUAdapterPropertiesSubgroupMatrixConfigs;
pub use raw::WGPUAdapterPropertiesSubgroups;
pub use raw::WGPUAdapterPropertiesVk;
pub use raw::WGPUBindGroupDescriptor;
pub use raw::WGPUBindGroupEntry;
pub use raw::WGPUBindGroupLayoutDescriptor;
pub use raw::WGPUBindGroupLayoutEntry;
pub use raw::WGPUBindGroupLayoutEntryArraySize;
pub use raw::WGPUBlendComponent;
pub use raw::WGPUBlendState;
pub use raw::WGPUBufferBindingLayout;
pub use raw::WGPUBufferDescriptor;
pub use raw::WGPUBufferHostMappedPointer;
pub use raw::WGPUColor;
pub use raw::WGPUColorTargetState;
pub use raw::WGPUColorTargetStateExpandResolveTextureDawn;
pub use raw::WGPUCommandBufferDescriptor;
pub use raw::WGPUCommandEncoderDescriptor;
pub use raw::WGPUCompilationInfo;
pub use raw::WGPUCompilationMessage;
pub use raw::WGPUComputePassDescriptor;
pub use raw::WGPUComputePipelineDescriptor;
pub use raw::WGPUComputeState;
pub use raw::WGPUConstantEntry;
pub use raw::WGPUCopyTextureForBrowserOptions;
pub use raw::WGPUDawnWGSLBlocklist;
pub use raw::WGPUDawnAdapterPropertiesPowerPreference;
pub use raw::WGPUDawnBufferDescriptorErrorInfoFromWireClient;
pub use raw::WGPUDawnCacheDeviceDescriptor;
pub use raw::WGPUDawnCompilationMessageUtf16;
pub use raw::WGPUDawnDeviceAllocatorControl;
pub use raw::WGPUDawnDrmFormatCapabilities;
pub use raw::WGPUDawnDrmFormatProperties;
pub use raw::WGPUDawnEncoderInternalUsageDescriptor;
pub use raw::WGPUDawnFakeBufferOOMForTesting;
pub use raw::WGPUDawnFormatCapabilities;
pub use raw::WGPUDawnHostMappedPointerLimits;
pub use raw::WGPUDawnInjectedInvalidSType;
pub use raw::WGPUDawnRenderPassColorAttachmentRenderToSingleSampled;
pub use raw::WGPUDawnShaderModuleSPIRVOptionsDescriptor;
pub use raw::WGPUDawnTexelCopyBufferRowAlignmentLimits;
pub use raw::WGPUDawnTextureInternalUsageDescriptor;
pub use raw::WGPUDawnTogglesDescriptor;
pub use raw::WGPUDawnWireWGSLControl;
pub use raw::WGPUDepthStencilState;
pub use raw::WGPUDeviceDescriptor;
pub use raw::WGPUEmscriptenSurfaceSourceCanvasHTMLSelector;
pub use raw::WGPUExtent2D;
pub use raw::WGPUExtent3D;
pub use raw::WGPUExternalTextureBindingEntry;
pub use raw::WGPUExternalTextureBindingLayout;
pub use raw::WGPUExternalTextureDescriptor;
pub use raw::WGPUFragmentState;
pub use raw::WGPUFuture;
pub use raw::WGPUFutureWaitInfo;
pub use raw::WGPUImageCopyExternalTexture;
pub use raw::WGPUInstanceCapabilities;
pub use raw::WGPUInstanceDescriptor;
pub use raw::WGPULimits;
pub use raw::WGPUMemoryHeapInfo;
pub use raw::WGPUMultisampleState;
pub use raw::WGPUOrigin2D;
pub use raw::WGPUOrigin3D;
pub use raw::WGPUPassTimestampWrites;
pub use raw::WGPUPipelineLayoutDescriptor;
pub use raw::WGPUPipelineLayoutPixelLocalStorage;
pub use raw::WGPUPipelineLayoutStorageAttachment;
pub use raw::WGPUPrimitiveState;
pub use raw::WGPUQuerySetDescriptor;
pub use raw::WGPUQueueDescriptor;
pub use raw::WGPURenderBundleDescriptor;
pub use raw::WGPURenderBundleEncoderDescriptor;
pub use raw::WGPURenderPassColorAttachment;
pub use raw::WGPURenderPassDepthStencilAttachment;
pub use raw::WGPURenderPassDescriptor;
pub use raw::WGPURenderPassDescriptorExpandResolveRect;
pub use raw::WGPURenderPassDescriptorResolveRect;
pub use raw::WGPURenderPassMaxDrawCount;
pub use raw::WGPURenderPassPixelLocalStorage;
pub use raw::WGPURenderPassStorageAttachment;
pub use raw::WGPURenderPipelineDescriptor;
pub use raw::WGPURequestAdapterWebXROptions;
pub use raw::WGPURequestAdapterOptions;
pub use raw::WGPUSamplerBindingLayout;
pub use raw::WGPUSamplerDescriptor;
pub use raw::WGPUShaderModuleCompilationOptions;
pub use raw::WGPUShaderModuleDescriptor;
pub use raw::WGPUShaderSourceSPIRV;
pub use raw::WGPUShaderSourceWGSL;
pub use raw::WGPUSharedBufferMemoryBeginAccessDescriptor;
pub use raw::WGPUSharedBufferMemoryDescriptor;
pub use raw::WGPUSharedBufferMemoryEndAccessState;
pub use raw::WGPUSharedBufferMemoryProperties;
pub use raw::WGPUSharedFenceDXGISharedHandleDescriptor;
pub use raw::WGPUSharedFenceDXGISharedHandleExportInfo;
pub use raw::WGPUSharedFenceEGLSyncDescriptor;
pub use raw::WGPUSharedFenceEGLSyncExportInfo;
pub use raw::WGPUSharedFenceMTLSharedEventDescriptor;
pub use raw::WGPUSharedFenceMTLSharedEventExportInfo;
pub use raw::WGPUSharedFenceDescriptor;
pub use raw::WGPUSharedFenceExportInfo;
pub use raw::WGPUSharedFenceSyncFDDescriptor;
pub use raw::WGPUSharedFenceSyncFDExportInfo;
pub use raw::WGPUSharedFenceVkSemaphoreOpaqueFDDescriptor;
pub use raw::WGPUSharedFenceVkSemaphoreOpaqueFDExportInfo;
pub use raw::WGPUSharedFenceVkSemaphoreZirconHandleDescriptor;
pub use raw::WGPUSharedFenceVkSemaphoreZirconHandleExportInfo;
pub use raw::WGPUSharedTextureMemoryD3DSwapchainBeginState;
pub use raw::WGPUSharedTextureMemoryDXGISharedHandleDescriptor;
pub use raw::WGPUSharedTextureMemoryEGLImageDescriptor;
pub use raw::WGPUSharedTextureMemoryIOSurfaceDescriptor;
pub use raw::WGPUSharedTextureMemoryAHardwareBufferDescriptor;
pub use raw::WGPUSharedTextureMemoryAHardwareBufferProperties;
pub use raw::WGPUSharedTextureMemoryBeginAccessDescriptor;
pub use raw::WGPUSharedTextureMemoryDescriptor;
pub use raw::WGPUSharedTextureMemoryDmaBufDescriptor;
pub use raw::WGPUSharedTextureMemoryDmaBufPlane;
pub use raw::WGPUSharedTextureMemoryEndAccessState;
pub use raw::WGPUSharedTextureMemoryOpaqueFDDescriptor;
pub use raw::WGPUSharedTextureMemoryProperties;
pub use raw::WGPUSharedTextureMemoryVkDedicatedAllocationDescriptor;
pub use raw::WGPUSharedTextureMemoryVkImageLayoutBeginState;
pub use raw::WGPUSharedTextureMemoryVkImageLayoutEndState;
pub use raw::WGPUSharedTextureMemoryZirconHandleDescriptor;
pub use raw::WGPUStaticSamplerBindingLayout;
pub use raw::WGPUStencilFaceState;
pub use raw::WGPUStorageTextureBindingLayout;
pub use raw::WGPUStringView;
pub use raw::WGPUSubgroupMatrixConfig;
pub use raw::WGPUSupportedWGSLLanguageFeatures;
pub use raw::WGPUSupportedFeatures;
pub use raw::WGPUSurfaceCapabilities;
pub use raw::WGPUSurfaceColorManagement;
pub use raw::WGPUSurfaceConfiguration;
pub use raw::WGPUSurfaceDescriptor;
pub use raw::WGPUSurfaceDescriptorFromWindowsUWPSwapChainPanel;
pub use raw::WGPUSurfaceDescriptorFromWindowsWinUISwapChainPanel;
pub use raw::WGPUSurfaceDescriptorFromWindowsCoreWindow;
pub use raw::WGPUSurfaceSourceXCBWindow;
pub use raw::WGPUSurfaceSourceAndroidNativeWindow;
pub use raw::WGPUSurfaceSourceMetalLayer;
pub use raw::WGPUSurfaceSourceWaylandSurface;
pub use raw::WGPUSurfaceSourceWindowsHWND;
pub use raw::WGPUSurfaceSourceXlibWindow;
pub use raw::WGPUSurfaceTexture;
pub use raw::WGPUTexelCopyBufferInfo;
pub use raw::WGPUTexelCopyBufferLayout;
pub use raw::WGPUTexelCopyTextureInfo;
pub use raw::WGPUTextureBindingLayout;
pub use raw::WGPUTextureBindingViewDimensionDescriptor;
pub use raw::WGPUTextureDescriptor;
pub use raw::WGPUTextureViewDescriptor;
pub use raw::WGPUVertexAttribute;
pub use raw::WGPUVertexBufferLayout;
pub use raw::WGPUVertexState;
pub use raw::WGPUYCbCrVkDescriptor;
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
pub use raw::WGPULoggingCallback;
pub use raw::WGPULoggingCallbackInfo;
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
pub use raw::wgpuGetInstanceCapabilities;
pub use raw::wgpuGetProcAddress;
pub use raw::WGPUCallback;
pub use raw::WGPUDawnLoadCacheDataFunction;
pub use raw::WGPUDawnStoreCacheDataFunction;
pub use raw::WGPUProc;
pub use raw::WGPUAdapter;
pub use raw::wgpuAdapterAddRef;
pub use raw::wgpuAdapterRelease;
pub use raw::wgpuAdapterGetInstance;
pub use raw::wgpuAdapterGetLimits;
pub use raw::wgpuAdapterGetInfo;
pub use raw::wgpuAdapterHasFeature;
pub use raw::wgpuAdapterGetFeatures;
pub use raw::wgpuAdapterRequestDevice;
pub use raw::wgpuAdapterCreateDevice;
pub use raw::wgpuAdapterGetFormatCapabilities;
pub use raw::WGPUBindGroup;
pub use raw::wgpuBindGroupAddRef;
pub use raw::wgpuBindGroupRelease;
pub use raw::wgpuBindGroupSetLabel;
pub use raw::WGPUBindGroupLayout;
pub use raw::wgpuBindGroupLayoutAddRef;
pub use raw::wgpuBindGroupLayoutRelease;
pub use raw::wgpuBindGroupLayoutSetLabel;
pub use raw::WGPUBuffer;
pub use raw::wgpuBufferAddRef;
pub use raw::wgpuBufferRelease;
pub use raw::wgpuBufferMapAsync;
pub use raw::wgpuBufferGetMappedRange;
pub use raw::wgpuBufferGetConstMappedRange;
pub use raw::wgpuBufferWriteMappedRange;
pub use raw::wgpuBufferReadMappedRange;
pub use raw::wgpuBufferSetLabel;
pub use raw::wgpuBufferGetUsage;
pub use raw::wgpuBufferGetSize;
pub use raw::wgpuBufferGetMapState;
pub use raw::wgpuBufferUnmap;
pub use raw::wgpuBufferDestroy;
pub use raw::WGPUCommandBuffer;
pub use raw::wgpuCommandBufferAddRef;
pub use raw::wgpuCommandBufferRelease;
pub use raw::wgpuCommandBufferSetLabel;
pub use raw::WGPUCommandEncoder;
pub use raw::wgpuCommandEncoderAddRef;
pub use raw::wgpuCommandEncoderRelease;
pub use raw::wgpuCommandEncoderFinish;
pub use raw::wgpuCommandEncoderBeginComputePass;
pub use raw::wgpuCommandEncoderBeginRenderPass;
pub use raw::wgpuCommandEncoderCopyBufferToBuffer;
pub use raw::wgpuCommandEncoderCopyBufferToTexture;
pub use raw::wgpuCommandEncoderCopyTextureToBuffer;
pub use raw::wgpuCommandEncoderCopyTextureToTexture;
pub use raw::wgpuCommandEncoderClearBuffer;
pub use raw::wgpuCommandEncoderInjectValidationError;
pub use raw::wgpuCommandEncoderInsertDebugMarker;
pub use raw::wgpuCommandEncoderPopDebugGroup;
pub use raw::wgpuCommandEncoderPushDebugGroup;
pub use raw::wgpuCommandEncoderResolveQuerySet;
pub use raw::wgpuCommandEncoderWriteBuffer;
pub use raw::wgpuCommandEncoderWriteTimestamp;
pub use raw::wgpuCommandEncoderSetLabel;
pub use raw::WGPUComputePassEncoder;
pub use raw::wgpuComputePassEncoderAddRef;
pub use raw::wgpuComputePassEncoderRelease;
pub use raw::wgpuComputePassEncoderInsertDebugMarker;
pub use raw::wgpuComputePassEncoderPopDebugGroup;
pub use raw::wgpuComputePassEncoderPushDebugGroup;
pub use raw::wgpuComputePassEncoderSetPipeline;
pub use raw::wgpuComputePassEncoderSetBindGroup;
pub use raw::wgpuComputePassEncoderWriteTimestamp;
pub use raw::wgpuComputePassEncoderDispatchWorkgroups;
pub use raw::wgpuComputePassEncoderDispatchWorkgroupsIndirect;
pub use raw::wgpuComputePassEncoderEnd;
pub use raw::wgpuComputePassEncoderSetLabel;
pub use raw::wgpuComputePassEncoderSetImmediateData;
pub use raw::WGPUComputePipeline;
pub use raw::wgpuComputePipelineAddRef;
pub use raw::wgpuComputePipelineRelease;
pub use raw::wgpuComputePipelineGetBindGroupLayout;
pub use raw::wgpuComputePipelineSetLabel;
pub use raw::WGPUDevice;
pub use raw::wgpuDeviceAddRef;
pub use raw::wgpuDeviceRelease;
pub use raw::wgpuDeviceCreateBindGroup;
pub use raw::wgpuDeviceCreateBindGroupLayout;
pub use raw::wgpuDeviceCreateBuffer;
pub use raw::wgpuDeviceCreateErrorBuffer;
pub use raw::wgpuDeviceCreateCommandEncoder;
pub use raw::wgpuDeviceCreateComputePipeline;
pub use raw::wgpuDeviceCreateComputePipelineAsync;
pub use raw::wgpuDeviceCreateExternalTexture;
pub use raw::wgpuDeviceCreateErrorExternalTexture;
pub use raw::wgpuDeviceCreatePipelineLayout;
pub use raw::wgpuDeviceCreateQuerySet;
pub use raw::wgpuDeviceCreateRenderPipelineAsync;
pub use raw::wgpuDeviceCreateRenderBundleEncoder;
pub use raw::wgpuDeviceCreateRenderPipeline;
pub use raw::wgpuDeviceCreateSampler;
pub use raw::wgpuDeviceCreateShaderModule;
pub use raw::wgpuDeviceCreateErrorShaderModule;
pub use raw::wgpuDeviceCreateTexture;
pub use raw::wgpuDeviceImportSharedBufferMemory;
pub use raw::wgpuDeviceImportSharedTextureMemory;
pub use raw::wgpuDeviceImportSharedFence;
pub use raw::wgpuDeviceCreateErrorTexture;
pub use raw::wgpuDeviceDestroy;
pub use raw::wgpuDeviceGetAHardwareBufferProperties;
pub use raw::wgpuDeviceGetLimits;
pub use raw::wgpuDeviceGetLostFuture;
pub use raw::wgpuDeviceHasFeature;
pub use raw::wgpuDeviceGetFeatures;
pub use raw::wgpuDeviceGetAdapterInfo;
pub use raw::wgpuDeviceGetAdapter;
pub use raw::wgpuDeviceGetQueue;
pub use raw::wgpuDeviceInjectError;
pub use raw::wgpuDeviceForceLoss;
pub use raw::wgpuDeviceTick;
pub use raw::wgpuDeviceSetLoggingCallback;
pub use raw::wgpuDevicePushErrorScope;
pub use raw::wgpuDevicePopErrorScope;
pub use raw::wgpuDeviceSetLabel;
pub use raw::wgpuDeviceValidateTextureDescriptor;
pub use raw::WGPUExternalTexture;
pub use raw::wgpuExternalTextureAddRef;
pub use raw::wgpuExternalTextureRelease;
pub use raw::wgpuExternalTextureSetLabel;
pub use raw::wgpuExternalTextureDestroy;
pub use raw::wgpuExternalTextureExpire;
pub use raw::wgpuExternalTextureRefresh;
pub use raw::WGPUInstance;
pub use raw::wgpuInstanceAddRef;
pub use raw::wgpuInstanceRelease;
pub use raw::wgpuInstanceCreateSurface;
pub use raw::wgpuInstanceProcessEvents;
pub use raw::wgpuInstanceWaitAny;
pub use raw::wgpuInstanceRequestAdapter;
pub use raw::wgpuInstanceHasWGSLLanguageFeature;
pub use raw::wgpuInstanceGetWGSLLanguageFeatures;
pub use raw::WGPUPipelineLayout;
pub use raw::wgpuPipelineLayoutAddRef;
pub use raw::wgpuPipelineLayoutRelease;
pub use raw::wgpuPipelineLayoutSetLabel;
pub use raw::WGPUQuerySet;
pub use raw::wgpuQuerySetAddRef;
pub use raw::wgpuQuerySetRelease;
pub use raw::wgpuQuerySetSetLabel;
pub use raw::wgpuQuerySetGetType;
pub use raw::wgpuQuerySetGetCount;
pub use raw::wgpuQuerySetDestroy;
pub use raw::WGPUQueue;
pub use raw::wgpuQueueAddRef;
pub use raw::wgpuQueueRelease;
pub use raw::wgpuQueueSubmit;
pub use raw::wgpuQueueOnSubmittedWorkDone;
pub use raw::wgpuQueueWriteBuffer;
pub use raw::wgpuQueueWriteTexture;
pub use raw::wgpuQueueCopyTextureForBrowser;
pub use raw::wgpuQueueCopyExternalTextureForBrowser;
pub use raw::wgpuQueueSetLabel;
pub use raw::WGPURenderBundle;
pub use raw::wgpuRenderBundleAddRef;
pub use raw::wgpuRenderBundleRelease;
pub use raw::wgpuRenderBundleSetLabel;
pub use raw::WGPURenderBundleEncoder;
pub use raw::wgpuRenderBundleEncoderAddRef;
pub use raw::wgpuRenderBundleEncoderRelease;
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
pub use raw::wgpuRenderBundleEncoderSetImmediateData;
pub use raw::WGPURenderPassEncoder;
pub use raw::wgpuRenderPassEncoderAddRef;
pub use raw::wgpuRenderPassEncoderRelease;
pub use raw::wgpuRenderPassEncoderSetPipeline;
pub use raw::wgpuRenderPassEncoderSetBindGroup;
pub use raw::wgpuRenderPassEncoderDraw;
pub use raw::wgpuRenderPassEncoderDrawIndexed;
pub use raw::wgpuRenderPassEncoderDrawIndirect;
pub use raw::wgpuRenderPassEncoderDrawIndexedIndirect;
pub use raw::wgpuRenderPassEncoderMultiDrawIndirect;
pub use raw::wgpuRenderPassEncoderMultiDrawIndexedIndirect;
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
pub use raw::wgpuRenderPassEncoderWriteTimestamp;
pub use raw::wgpuRenderPassEncoderPixelLocalStorageBarrier;
pub use raw::wgpuRenderPassEncoderEnd;
pub use raw::wgpuRenderPassEncoderSetLabel;
pub use raw::wgpuRenderPassEncoderSetImmediateData;
pub use raw::WGPURenderPipeline;
pub use raw::wgpuRenderPipelineAddRef;
pub use raw::wgpuRenderPipelineRelease;
pub use raw::wgpuRenderPipelineGetBindGroupLayout;
pub use raw::wgpuRenderPipelineSetLabel;
pub use raw::WGPUSampler;
pub use raw::wgpuSamplerAddRef;
pub use raw::wgpuSamplerRelease;
pub use raw::wgpuSamplerSetLabel;
pub use raw::WGPUShaderModule;
pub use raw::wgpuShaderModuleAddRef;
pub use raw::wgpuShaderModuleRelease;
pub use raw::wgpuShaderModuleGetCompilationInfo;
pub use raw::wgpuShaderModuleSetLabel;
pub use raw::WGPUSharedBufferMemory;
pub use raw::wgpuSharedBufferMemoryAddRef;
pub use raw::wgpuSharedBufferMemoryRelease;
pub use raw::wgpuSharedBufferMemorySetLabel;
pub use raw::wgpuSharedBufferMemoryGetProperties;
pub use raw::wgpuSharedBufferMemoryCreateBuffer;
pub use raw::wgpuSharedBufferMemoryBeginAccess;
pub use raw::wgpuSharedBufferMemoryEndAccess;
pub use raw::wgpuSharedBufferMemoryIsDeviceLost;
pub use raw::WGPUSharedFence;
pub use raw::wgpuSharedFenceAddRef;
pub use raw::wgpuSharedFenceRelease;
pub use raw::wgpuSharedFenceExportInfo;
pub use raw::WGPUSharedTextureMemory;
pub use raw::wgpuSharedTextureMemoryAddRef;
pub use raw::wgpuSharedTextureMemoryRelease;
pub use raw::wgpuSharedTextureMemorySetLabel;
pub use raw::wgpuSharedTextureMemoryGetProperties;
pub use raw::wgpuSharedTextureMemoryCreateTexture;
pub use raw::wgpuSharedTextureMemoryBeginAccess;
pub use raw::wgpuSharedTextureMemoryEndAccess;
pub use raw::wgpuSharedTextureMemoryIsDeviceLost;
pub use raw::WGPUSurface;
pub use raw::wgpuSurfaceAddRef;
pub use raw::wgpuSurfaceRelease;
pub use raw::wgpuSurfaceConfigure;
pub use raw::wgpuSurfaceGetCapabilities;
pub use raw::wgpuSurfaceGetCurrentTexture;
pub use raw::wgpuSurfacePresent;
pub use raw::wgpuSurfaceUnconfigure;
pub use raw::wgpuSurfaceSetLabel;
pub use raw::WGPUTexture;
pub use raw::wgpuTextureAddRef;
pub use raw::wgpuTextureRelease;
pub use raw::wgpuTextureCreateView;
pub use raw::wgpuTextureCreateErrorView;
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
pub use raw::wgpuTextureViewAddRef;
pub use raw::wgpuTextureViewRelease;
pub use raw::wgpuTextureViewSetLabel;
pub use raw::wgpuAdapterInfoFreeMembers;
pub use raw::wgpuAdapterPropertiesMemoryHeapsFreeMembers;
pub use raw::wgpuAdapterPropertiesSubgroupMatrixConfigsFreeMembers;
pub use raw::wgpuDawnDrmFormatCapabilitiesFreeMembers;
pub use raw::wgpuSharedBufferMemoryEndAccessStateFreeMembers;
pub use raw::wgpuSharedTextureMemoryEndAccessStateFreeMembers;
pub use raw::wgpuSupportedFeaturesFreeMembers;
pub use raw::wgpuSupportedWGSLLanguageFeaturesFreeMembers;
pub use raw::wgpuSurfaceCapabilitiesFreeMembers;
