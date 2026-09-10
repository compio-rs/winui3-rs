#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HCURSOR(pub *mut core::ffi::c_void);
