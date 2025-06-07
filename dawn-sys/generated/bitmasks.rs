::bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    #[repr(transparent)] pub struct WGPUBufferUsage : WGPUFlags { const NONE = 0u64;
    const MAP_READ = 1u64; const MAP_WRITE = 2u64; const COPY_SRC = 4u64; const COPY_DST
    = 8u64; const INDEX = 16u64; const VERTEX = 32u64; const UNIFORM = 64u64; const
    STORAGE = 128u64; const INDIRECT = 256u64; const QUERY_RESOLVE = 512u64; }
}
::bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    #[repr(transparent)] pub struct WGPUColorWriteMask : WGPUFlags { const NONE = 0u64;
    const RED = 1u64; const GREEN = 2u64; const BLUE = 4u64; const ALPHA = 8u64; const
    ALL = 15u64; }
}
::bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    #[repr(transparent)] pub struct WGPUHeapProperty : WGPUFlags { const NONE = 0u64;
    const DEVICE_LOCAL = 1u64; const HOST_VISIBLE = 2u64; const HOST_COHERENT = 4u64;
    const HOST_UNCACHED = 8u64; const HOST_CACHED = 16u64; }
}
::bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    #[repr(transparent)] pub struct WGPUMapMode : WGPUFlags { const NONE = 0u64; const
    READ = 1u64; const WRITE = 2u64; }
}
::bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    #[repr(transparent)] pub struct WGPUShaderStage : WGPUFlags { const NONE = 0u64;
    const VERTEX = 1u64; const FRAGMENT = 2u64; const COMPUTE = 4u64; }
}
::bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
    #[repr(transparent)] pub struct WGPUTextureUsage : WGPUFlags { const NONE = 0u64;
    const COPY_SRC = 1u64; const COPY_DST = 2u64; const TEXTURE_BINDING = 4u64; const
    STORAGE_BINDING = 8u64; const RENDER_ATTACHMENT = 16u64; const TRANSIENT_ATTACHMENT =
    32u64; const STORAGE_ATTACHMENT = 64u64; }
}
