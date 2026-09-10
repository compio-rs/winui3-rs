#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMDLG_FILTERSPEC {
    pub pszName: windows_core::PWSTR,
    pub pszSpec: windows_core::PWSTR,
}
