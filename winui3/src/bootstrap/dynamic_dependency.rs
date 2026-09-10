#![allow(non_snake_case)]

use std::sync::OnceLock;

use windows::Win32::{
    Foundation::{ERROR_MOD_NOT_FOUND, FARPROC, FreeLibrary, HMODULE},
    Security::PSID,
    Storage::Packaging::Appx::{AddPackageDependencyOptions, CreatePackageDependencyOptions, PACKAGE_VERSION, PACKAGEDEPENDENCY_CONTEXT, PackageDependencyLifetimeKind, PackageDependencyProcessorArchitectures},
    System::LibraryLoader::{GetModuleHandleExW, GetProcAddress, LoadLibraryW},
};
use windows_core::{HRESULT, PCWSTR, PWSTR, Param, Result, WIN32_ERROR, s, w};

type TryCreatePackageDependencyFn = unsafe extern "system" fn(user: PSID, packagefamilyname: PCWSTR, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: PCWSTR, options: CreatePackageDependencyOptions, packagedependencyid: *mut PWSTR) -> HRESULT;

type AddPackageDependencyFn = unsafe extern "system" fn(packagedependencyid: PCWSTR, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname: *mut PWSTR) -> HRESULT;

type RemovePackageDependencyFn = unsafe extern "system" fn(packagedependencycontext: PACKAGEDEPENDENCY_CONTEXT) -> HRESULT;

type DeletePackageDependencyFn = unsafe extern "system" fn(packagedependencyid: PCWSTR) -> HRESULT;

struct MddLib {
    lib: HMODULE,
    create: TryCreatePackageDependencyFn,
    add: AddPackageDependencyFn,
    remove: RemovePackageDependencyFn,
    delete: DeletePackageDependencyFn,
}

unsafe impl Send for MddLib {}
unsafe impl Sync for MddLib {}

impl Drop for MddLib {
    fn drop(&mut self) {
        unsafe {
            let _ = FreeLibrary(self.lib);
        }
    }
}

static MDD_LIB: OnceLock<Option<MddLib>> = OnceLock::new();

fn load_kernelbase(lib: HMODULE) -> Option<MddLib> {
    unsafe {
        let create = std::mem::transmute::<FARPROC, Option<TryCreatePackageDependencyFn>>(GetProcAddress(lib, s!("TryCreatePackageDependency")))?;
        let add = std::mem::transmute::<FARPROC, Option<AddPackageDependencyFn>>(GetProcAddress(lib, s!("AddPackageDependency")))?;
        let remove = std::mem::transmute::<FARPROC, Option<RemovePackageDependencyFn>>(GetProcAddress(lib, s!("RemovePackageDependency")))?;
        let delete = std::mem::transmute::<FARPROC, Option<DeletePackageDependencyFn>>(GetProcAddress(lib, s!("DeletePackageDependency")))?;
        Some(MddLib { lib, create, add, remove, delete })
    }
}

fn load_app_runtime(lib: HMODULE) -> Option<MddLib> {
    unsafe {
        let create = std::mem::transmute::<FARPROC, Option<TryCreatePackageDependencyFn>>(GetProcAddress(lib, s!("MddTryCreatePackageDependency")))?;
        let add = std::mem::transmute::<FARPROC, Option<AddPackageDependencyFn>>(GetProcAddress(lib, s!("MddAddPackageDependency")))?;
        let remove = std::mem::transmute::<FARPROC, Option<RemovePackageDependencyFn>>(GetProcAddress(lib, s!("MddRemovePackageDependency")))?;
        let delete = std::mem::transmute::<FARPROC, Option<DeletePackageDependencyFn>>(GetProcAddress(lib, s!("MddDeletePackageDependency")))?;
        Some(MddLib { lib, create, add, remove, delete })
    }
}

fn get_mdd_lib() -> Result<&'static MddLib> {
    MDD_LIB
        .get_or_init(|| unsafe {
            let mut lib = HMODULE::default();
            if GetModuleHandleExW(0, w!("kernelbase.dll"), &mut lib).ok().is_ok()
                && let Some(mdd_lib) = load_kernelbase(lib)
            {
                return Some(mdd_lib);
            }
            let lib = LoadLibraryW(w!("Microsoft.WindowsAppRuntime.dll"));
            if !lib.0.is_null() {
                return load_app_runtime(lib);
            }
            None
        })
        .as_ref()
        .ok_or_else(|| WIN32_ERROR(ERROR_MOD_NOT_FOUND).to_hresult().into())
}

#[inline]
pub unsafe fn TryCreatePackageDependency<P1, P5>(user: PSID, packagefamilyname: P1, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: P5, options: CreatePackageDependencyOptions) -> Result<PWSTR>
where
    P1: Param<PCWSTR>,
    P5: Param<PCWSTR>,
{
    let mdd_lib = get_mdd_lib()?;
    unsafe {
        let mut result__ = core::mem::zeroed();
        (mdd_lib.create)(user, packagefamilyname.param().abi(), minversion, packagedependencyprocessorarchitectures, lifetimekind, lifetimeartifact.param().abi(), options, &mut result__).map(|| result__)
    }
}

#[inline]
pub unsafe fn AddPackageDependency<P0>(packagedependencyid: P0, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname: Option<*mut PWSTR>) -> Result<()>
where
    P0: Param<PCWSTR>,
{
    let mdd_lib = get_mdd_lib()?;
    unsafe { (mdd_lib.add)(packagedependencyid.param().abi(), rank, options, packagedependencycontext, packagefullname.unwrap_or(core::mem::zeroed())).ok() }
}

#[inline]
pub unsafe fn RemovePackageDependency(packagedependencycontext: PACKAGEDEPENDENCY_CONTEXT) -> Result<()> {
    let mdd_lib = get_mdd_lib()?;
    unsafe { (mdd_lib.remove)(packagedependencycontext).ok() }
}

#[inline]
pub unsafe fn DeletePackageDependency<P0>(packagedependencyid: P0) -> Result<()>
where
    P0: Param<PCWSTR>,
{
    let mdd_lib = get_mdd_lib()?;
    unsafe { (mdd_lib.delete)(packagedependencyid.param().abi()).ok() }
}
