#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]
#![expect(non_snake_case)]
#![expect(non_upper_case_globals)]

include!("../generated/bindings.rs");
include!("../generated/lib.rs");

impl WGPUStringView {
    pub fn empty() -> Self {
        Self {
            data: core::ptr::null(),
            length: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            data: bytes.as_ptr().cast(),
            length: bytes.len(),
        }
    }

    #[expect(clippy::should_implement_trait, reason = "WGPUStringView::from_str never fails")]
    pub fn from_str(s: &str) -> Self {
        Self::from_bytes(s.as_bytes())
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(
                self.data.cast(),
                self.length)
        }
    }

    #[cfg(feature = "std")]
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(self.as_bytes()).into_owned()
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
