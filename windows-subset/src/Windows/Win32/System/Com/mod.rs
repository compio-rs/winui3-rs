#[inline]
pub unsafe fn CoCreateInstance<P1, T>(rclsid: *const windows_core::GUID, punkouter: P1, dwclscontext: CLSCTX) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : CLSCTX, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { CoCreateInstance(rclsid, punkouter.param().abi(), dwclscontext, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn CoInitializeEx(pvreserved: Option<*const core::ffi::c_void>, dwcoinit: COINIT) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : COINIT) -> windows_core::HRESULT);
    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit) }
}
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
    unsafe { CoTaskMemAlloc(cb) }
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: Option<*const core::ffi::c_void>) {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
    unsafe { CoTaskMemFree(pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoUninitialize() {
    windows_core::link!("ole32.dll" "system" fn CoUninitialize());
    unsafe { CoUninitialize() }
}
pub type ADVANCED_FEATURE_FLAGS = u16;
pub type CLSCTX = u32;
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = 262144;
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = 524288;
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = 8388608;
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = 33554432;
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = 262144;
pub const CLSCTX_ALL: CLSCTX = 23;
pub const CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION: CLSCTX = 67108864;
pub const CLSCTX_APPCONTAINER: CLSCTX = 4194304;
pub const CLSCTX_DISABLE_AAA: CLSCTX = 32768;
pub const CLSCTX_DO_NOT_ELEVATE_SERVER: CLSCTX = 268435456;
pub const CLSCTX_ENABLE_AAA: CLSCTX = 65536;
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = 1048576;
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = 8192;
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = 131072;
pub const CLSCTX_INPROC_HANDLER: CLSCTX = 2;
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = 32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1;
pub const CLSCTX_INPROC_SERVER16: CLSCTX = 8;
pub const CLSCTX_LOCAL_SERVER: CLSCTX = 4;
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = 1024;
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = 4096;
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = 16384;
pub const CLSCTX_PS_DLL: CLSCTX = 2147483648;
pub const CLSCTX_REMOTE_SERVER: CLSCTX = 16;
pub const CLSCTX_RESERVED1: CLSCTX = 64;
pub const CLSCTX_RESERVED2: CLSCTX = 128;
pub const CLSCTX_RESERVED3: CLSCTX = 256;
pub const CLSCTX_RESERVED4: CLSCTX = 512;
pub const CLSCTX_RESERVED5: CLSCTX = 2048;
pub const CLSCTX_RESERVED6: CLSCTX = 16777216;
pub const CLSCTX_SERVER: CLSCTX = 21;
pub const CLSCTX_SERVER_MUST_BE_EQUAL_OR_GREATER_PRIVILEGE: CLSCTX = 134217728;
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
windows_core::imp::define_interface!(IDataObject, IDataObject_Vtbl, 0x0000010e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDataObject, windows_core::IUnknown);
impl IDataObject {
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DUnadvise)(windows_core::Interface::as_raw(self), dwconnection).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetData: usize,
    GetDataHere: usize,
    QueryGetData: usize,
    GetCanonicalFormatEtc: usize,
    SetData: usize,
    EnumFormatEtc: usize,
    DAdvise: usize,
    pub DUnadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    EnumDAdvise: usize,
}
impl windows_core::RuntimeName for IDataObject {}
windows_core::imp::define_interface!(IDispatch, IDispatch_Vtbl, 0x00020400_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIDsOfNames(&self, riid: *const windows_core::GUID, rgsznames: *const windows_core::PCWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetIDsOfNames)(windows_core::Interface::as_raw(self), riid, rgsznames, cnames, lcid, rgdispid as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetTypeInfo: usize,
    pub GetIDsOfNames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::PCWSTR, u32, u32, *mut i32) -> windows_core::HRESULT,
    Invoke: usize,
}
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(ISequentialStream, ISequentialStream_Vtbl, 0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
impl ISequentialStream {
    pub unsafe fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Read)(windows_core::Interface::as_raw(self), pv as _, cb, pcbread.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), pv, cb, pcbwritten.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait ISequentialStream_Impl: windows_core::IUnknownImpl {
    fn Read(&self, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::Result<()>;
    fn Write(&self, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()>;
}
impl ISequentialStream_Vtbl {
    pub const fn new<Identity: ISequentialStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Read<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Read(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn Write<Identity: ISequentialStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISequentialStream_Impl::Write(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Write: Write::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISequentialStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISequentialStream {}
windows_core::imp::define_interface!(IStream, IStream_Vtbl, 0x0000000c_0000_0000_c000_000000000046);
impl core::ops::Deref for IStream {
    type Target = ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
impl IStream {
    pub unsafe fn SetSize(&self, libnewsize: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), libnewsize).ok() }
    }
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: Option<*mut u64>, pcbwritten: Option<*mut u64>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), pstm.param().abi(), cb, pcbread.unwrap_or(core::mem::zeroed()) as _, pcbwritten.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok() }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    Seek: usize,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut u64, *mut u64) -> windows_core::HRESULT,
    Commit: usize,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    LockRegion: usize,
    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    Stat: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IStream {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
