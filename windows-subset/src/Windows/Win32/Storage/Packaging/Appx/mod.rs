#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentApplicationUserModelId(applicationusermodelidlength : *mut u32, applicationusermodelid : windows_core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR);
    unsafe { GetCurrentApplicationUserModelId(applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: Option<*mut u8>, count: Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageInfo(flags : u32, bufferlength : *mut u32, buffer : *mut u8, count : *mut u32) -> super::super::super::Foundation::WIN32_ERROR);
    unsafe { GetCurrentPackageInfo(flags, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
pub type AddPackageDependencyOptions = i32;
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = 0;
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = 1;
pub type CreatePackageDependencyOptions = i32;
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = 1;
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = 0;
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PACKAGEDEPENDENCY_CONTEXT(pub *mut core::ffi::c_void);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_core::PWSTR,
    pub publisher: windows_core::PWSTR,
    pub resourceId: windows_core::PWSTR,
    pub publisherId: windows_core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: windows_core::PWSTR,
    pub publisher: windows_core::PWSTR,
    pub resourceId: windows_core::PWSTR,
    pub publisherId: windows_core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(target_arch = "x86")]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
pub type PackageDependencyLifetimeKind = i32;
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = 1;
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = 0;
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = 2;
pub type PackageDependencyProcessorArchitectures = i32;
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = 8;
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = 16;
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = 1;
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = 0;
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = 4;
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = 2;
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = 32;
