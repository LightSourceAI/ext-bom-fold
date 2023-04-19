#![allow(clippy::derive_partial_eq_without_eq, clippy::needless_lifetimes)]
#![allow(rustdoc::invalid_html_tags)]
include!(concat!(env!("OUT_DIR"), "/lightsource.error.rs"));

#[cfg(debug_assertions)]
impl DebugInfo {
    /// Captures a backtrace and populates the debug info.
    pub fn collect() -> Option<Self> {
        let backtrace = backtrace::Backtrace::new();
        Some(Self {
            stack_entries: backtrace.frames().iter().map(|f| format!("{f:?}")).collect(),
            detail: Some(format!("{backtrace:?}")),
        })
    }

    pub fn collect_with(detail: String) -> Option<Self> {
        let backtrace = backtrace::Backtrace::new();
        Some(Self {
            stack_entries: backtrace.frames().iter().map(|f| format!("{f:?}")).collect(),
            detail: Some(detail),
        })
    }
}

#[cfg(not(debug_assertions))]
impl DebugInfo {
    /// In debug builds, captures a backtrace and populates the debug info.
    pub fn collect() -> Option<Self> {
        None
    }

    pub fn collect_with(_detail: String) -> Option<Self> {
        None
    }
}
