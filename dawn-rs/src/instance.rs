use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub struct InstanceDescriptor {
    pub capabilities: InstanceCapabilities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[derive(Default)]
pub struct InstanceCapabilities {
    pub timed_wait_any_enable: bool,
    pub timed_wait_any_max_count: usize,
}

impl Instance {
    pub fn new(desc: Option<&InstanceDescriptor>) -> Option<Self> {
        let raw_instance = if let Some(desc) = desc {
            let mut raw_desc = sys::WGPUInstanceDescriptor {
                nextInChain: ptr::null_mut(),
                capabilities: sys::WGPUInstanceCapabilities {
                    nextInChain: ptr::null_mut(),
                    timedWaitAnyEnable:
                        desc.capabilities.timed_wait_any_enable.into(),
                    timedWaitAnyMaxCount:
                        desc.capabilities.timed_wait_any_max_count,
                },
            };
            unsafe { sys::wgpuCreateInstance(ptr::from_mut(&mut raw_desc)) }
        } else {
            unsafe { sys::wgpuCreateInstance(ptr::null_mut()) }
        };

        (!raw_instance.is_null()).then_some(Self(raw_instance))
    }

    pub fn get_capabilities() -> InstanceCapabilities {
        let mut capabilities = unsafe { mem::zeroed() };
        unsafe { sys::wgpuGetInstanceCapabilities(&mut capabilities) };

        InstanceCapabilities {
            timed_wait_any_enable:
                capabilities.timedWaitAnyEnable != 0,
            timed_wait_any_max_count:
                capabilities.timedWaitAnyMaxCount,
        }
    }

    pub fn process_events(&self) {
        unsafe { sys::wgpuInstanceProcessEvents(self.0) }
    }

    pub fn has_wgsl_language_feature(&self, feature: WGSLLanguageFeatureName) -> bool {
        unsafe { sys::wgpuInstanceHasWGSLLanguageFeature(self.0, feature as _) != 0 }
    }

    pub fn get_wgsl_language_features(&self) -> Option<WGSLLanguageFeatures> {
        let mut features = unsafe { mem::zeroed() };
        let status = unsafe {
            sys::wgpuInstanceGetWGSLLanguageFeatures(
                self.0,
                ptr::from_mut(&mut features))
        };
        if status != sys::WGPUStatus::Success { return None; }
        if features.features.is_null() { return None; }
        Some(WGSLLanguageFeatures::new(features))
    }
}

pub struct WGSLLanguageFeatures(sys::WGPUSupportedWGSLLanguageFeatures);

impl WGSLLanguageFeatures {
    fn new(inner: sys::WGPUSupportedWGSLLanguageFeatures) -> Self {
        debug_assert!(!inner.features.is_null());
        debug_assert!({
            let arr = unsafe {
                slice::from_raw_parts(
                    inner.features,
                    inner.featureCount)
            };
            arr.iter().all(|&repr| WGSLLanguageFeatureName::from_repr(repr).is_some())
        });

        Self(inner)
    }
}

impl Deref for WGSLLanguageFeatures {
    type Target = [WGSLLanguageFeatureName];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(
                self.0.features.cast(),
                self.0.featureCount)
        }
    }
}

impl Drop for WGSLLanguageFeatures {
    fn drop(&mut self) {
        unsafe { sys::wgpuSupportedWGSLLanguageFeaturesFreeMembers(self.0) }
    }
}
