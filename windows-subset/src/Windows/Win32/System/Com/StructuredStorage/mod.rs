windows_core::imp::define_interface!(IStorage, IStorage_Vtbl, 0x0000000b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IStorage, windows_core::IUnknown);
impl IStorage {
    pub unsafe fn CopyTo<P3>(&self, rgiidexclude: Option<&[windows_core::GUID]>, snbexclude: Option<*const *const u16>, pstgdest: P3) -> windows_core::Result<()>
    where
        P3: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), rgiidexclude.map_or(0, |slice| slice.len().try_into().unwrap()), rgiidexclude.map_or(core::ptr::null(), |slice| slice.as_ptr()), snbexclude.unwrap_or(core::mem::zeroed()) as _, pstgdest.param().abi()).ok() }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags).ok() }
    }
    pub unsafe fn Revert(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn DestroyElement<P0>(&self, pwcsname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DestroyElement)(windows_core::Interface::as_raw(self), pwcsname.param().abi()).ok() }
    }
    pub unsafe fn RenameElement<P0, P1>(&self, pwcsoldname: P0, pwcsnewname: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RenameElement)(windows_core::Interface::as_raw(self), pwcsoldname.param().abi(), pwcsnewname.param().abi()).ok() }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid).ok() }
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStateBits)(windows_core::Interface::as_raw(self), grfstatebits, grfmask).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateStream: usize,
    OpenStream: usize,
    CreateStorage: usize,
    OpenStorage: usize,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const *const u16, *mut core::ffi::c_void) -> windows_core::HRESULT,
    MoveElementTo: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    EnumElements: usize,
    pub DestroyElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RenameElement: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    SetElementTimes: usize,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetStateBits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    Stat: usize,
}
impl windows_core::RuntimeName for IStorage {}
