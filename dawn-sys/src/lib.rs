#![no_std]
#![doc = include_str!("../README.md")]
#![expect(non_upper_case_globals)]

mod raw {
    #![expect(dead_code)]
    #![expect(non_camel_case_types)]
    #![expect(non_snake_case)]

    include!("../generated/bindings.rs");
    include!("../generated/bitmasks.rs");
}

include!("../generated/lib.rs");

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

impl WGPUStringView {
    /// The *null* value of [`WGPUStringView`], as is specified in [WebGPU Headers: Strings](https://webgpu-native.github.io/webgpu-headers/Strings.html).
    /// 
    /// Note that this is distinct from [`WGPUStringView::empty`], and
    /// is not considered as an empty string.
    pub fn null() -> Self {
        Self {
            data: core::ptr::null(),
            length: WGPU_STRLEN,
        }
    }

    /// The empty string, as is specified in [WebGPU Headers: Strings](https://webgpu-native.github.io/webgpu-headers/Strings.html).
    /// 
    /// Note that this is not the only valid empty string representation.
    /// To check if a string is empty, use [`WGPUStringView::is_empty`]
    /// instead of comparing with this value.
    pub fn empty() -> Self {
        Self {
            data: core::ptr::null(),
            length: 0,
        }
    }

    /// Returns `true` if this string is [`WGPUStringView::null`]. `false` otherwise.
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }

    /// Returns `true` if this string is empty. `false` otherwise.
    /// 
    /// This method is equivalent to `self.length == 0`.
    /// 
    /// Note that [`WGPUStringView::null`] is not considered as an empty string,
    /// as is specified in [WebGPU Headers: Strings](https://webgpu-native.github.io/webgpu-headers/Strings.html).
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl From<&str> for WGPUStringView {
    fn from(s: &str) -> Self {
        Self {
            data: s.as_ptr().cast(),
            length: s.len(),
        }
    }
}

impl From<Option<&str>> for WGPUStringView {
    fn from(s: Option<&str>) -> Self {
        s.map_or(Self::null(), Self::from)
    }
}
