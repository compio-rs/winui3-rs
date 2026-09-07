#[inline]
pub unsafe fn FreeLibrary(hlibmodule: HMODULE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> windows_core::BOOL);
    unsafe { FreeLibrary(hlibmodule) }
}
#[inline]
pub unsafe fn GetModuleHandleExW<P1>(
    dwflags: u32,
    lpmodulename: P1,
    phmodule: *mut HMODULE,
) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetModuleHandleExW(dwflags : u32, lpmodulename : windows_core::PCWSTR, phmodule : *mut HMODULE) -> windows_core::BOOL);
    unsafe { GetModuleHandleExW(dwflags, lpmodulename.param().abi(), phmodule as _) }
}
#[inline]
pub unsafe fn GetProcAddress<P1>(hmodule: HMODULE, lpprocname: P1) -> FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : windows_core::PCSTR) -> FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[inline]
pub unsafe fn GetProcessHeap() -> HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
    unsafe { GetProcessHeap() }
}
#[inline]
pub unsafe fn HeapFree(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    lpmem: Option<*const core::ffi::c_void>,
) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe {
        HeapFree(
            hheap as _,
            dwflags,
            lpmem.unwrap_or(core::mem::zeroed()) as _,
        )
    }
}
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> HMODULE);
    unsafe { LoadLibraryW(lplibfilename.param().abi()) }
}
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
    unsafe { RoInitialize(inittype) }
}
pub type AddPackageDependencyOptions = i32;
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = 0;
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = 1;
pub type CreatePackageDependencyOptions = i32;
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution:
    CreatePackageDependencyOptions = 1;
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = 0;
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = 2;
pub const ERROR_MOD_NOT_FOUND: WIN32_ERROR = 126;
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
pub type HEAP_FLAGS = u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMODULE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PACKAGEDEPENDENCY_CONTEXT(pub *mut core::ffi::c_void);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PSID(pub *mut core::ffi::c_void);
pub type PackageDependencyLifetimeKind = i32;
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = 1;
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = 0;
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = 2;
pub type PackageDependencyProcessorArchitectures = i32;
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = 8;
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures =
    16;
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures =
    1;
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = 0;
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = 4;
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = 2;
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures =
    32;
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0;
pub type RO_INIT_TYPE = i32;
pub type WIN32_ERROR = u32;
