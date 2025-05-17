#![no_std]
#![doc = include_str!("../README.md")]

mod raw {
    #![allow(clippy::all)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(unused)]

    include!("../generated/bindings.rs");
}

include!("../generated/lib.rs");

pub use raw::{
    WGPUStringView,
    WGPUFlags,
    WGPUBool,
    WGPUChainedStruct,
};

impl WGPUStringView {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        WGPUStringView {
            data: bytes.as_ptr() as _,
            length: bytes.len(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.data as _,
                self.length)
        }
    }
}

/// `true` value of [`WGPUBool`].
pub const WGPU_TRUE: WGPUBool = 1;

/// `false` value of [`WGPUBool`].
pub const WGPU_FALSE: WGPUBool = 0;

/// Indicates no array layer count is specified.
pub const WGPU_ARRAY_LAYER_COUNT_UNDEFINED: u32 = u32::MAX;

/// Indicates no copy stride is specified.
pub const WGPU_COPY_STRIDE_UNDEFINED: u32 = u32::MAX;

/// Indicates no depth clear value is specified.
pub const WGPU_DEPTH_CLEAR_VALUE_UNDEFINED: f32 = f32::NAN;

/// Indicates no depth slice is specified.
pub const WGPU_DEPTH_SLICE_UNDEFINED: u32 = u32::MAX;

/// For `u32` limits, indicates no limit value is specified.
pub const WGPU_LIMIT_U32_UNDEFINED: u32 = u32::MAX;

/// For `u64` limits, indicates no limit value is specified.
pub const WGPU_LIMIT_U64_UNDEFINED: u64 = u64::MAX;

/// Indicates no mip level count is specified.
pub const WGPU_MIP_LEVEL_COUNT_UNDEFINED: u32 = u32::MAX;

/// Indicates no query set index is specified.
pub const WGPU_QUERY_SET_INDEX_UNDEFINED: u32 = u32::MAX;

/// Sentinel value used in [`WGPUStringView`].
pub const WGPU_STRLEN: usize = usize::MAX;

/// Indicates a size extending to the end of the buffer.
pub const WGPU_WHOLE_MAP_SIZE: usize = usize::MAX;

/// Indicates a size extending to the end of the buffer.
pub const WGPU_WHOLE_SIZE: u64 = u64::MAX;
