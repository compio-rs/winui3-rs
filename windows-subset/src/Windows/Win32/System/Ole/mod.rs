windows_core::imp::define_interface!(IRecordInfo, IRecordInfo_Vtbl, 0x0000002f_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRecordInfo, windows_core::IUnknown);
impl IRecordInfo {
    pub unsafe fn RecordInit(&self, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecordInit)(windows_core::Interface::as_raw(self), pvnew as _).ok() }
    }
    pub unsafe fn RecordClear(&self, pvexisting: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecordClear)(windows_core::Interface::as_raw(self), pvexisting).ok() }
    }
    pub unsafe fn RecordCopy(&self, pvexisting: *const core::ffi::c_void, pvnew: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecordCopy)(windows_core::Interface::as_raw(self), pvexisting, pvnew as _).ok() }
    }
    pub unsafe fn GetGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub unsafe fn GetField<P1>(&self, pvdata: *const core::ffi::c_void, szfieldname: P1) -> windows_core::Result<super::Variant::VARIANT>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetField)(windows_core::Interface::as_raw(self), pvdata, szfieldname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub unsafe fn GetFieldNoCopy<P1>(&self, pvdata: *const core::ffi::c_void, szfieldname: P1, pvarfield: *mut super::Variant::VARIANT, ppvdatacarray: *mut *mut core::ffi::c_void) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetFieldNoCopy)(windows_core::Interface::as_raw(self), pvdata, szfieldname.param().abi(), pvarfield, ppvdatacarray as _).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub unsafe fn PutField<P2>(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: P2, pvarfield: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutField)(windows_core::Interface::as_raw(self), wflags, pvdata as _, szfieldname.param().abi(), pvarfield).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub unsafe fn PutFieldNoCopy<P2>(&self, wflags: u32, pvdata: *mut core::ffi::c_void, szfieldname: P2, pvarfield: *const super::Variant::VARIANT) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).PutFieldNoCopy)(windows_core::Interface::as_raw(self), wflags, pvdata as _, szfieldname.param().abi(), pvarfield).ok() }
    }
    pub unsafe fn GetFieldNames(&self, pcnames: *mut u32, rgbstrnames: *mut windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFieldNames)(windows_core::Interface::as_raw(self), pcnames as _, core::mem::transmute(rgbstrnames)).ok() }
    }
    pub unsafe fn IsMatchingType<P0>(&self, precordinfo: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsMatchingType)(windows_core::Interface::as_raw(self), precordinfo.param().abi()) }
    }
    pub unsafe fn RecordCreate(&self) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).RecordCreate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RecordCreateCopy(&self, pvsource: *const core::ffi::c_void, ppvdest: *mut *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecordCreateCopy)(windows_core::Interface::as_raw(self), pvsource, ppvdest as _).ok() }
    }
    pub unsafe fn RecordDestroy(&self, pvrecord: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RecordDestroy)(windows_core::Interface::as_raw(self), pvrecord).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecordInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RecordInit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordClear: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetTypeInfo: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub GetField: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Variant")))]
    GetField: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub GetFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::PCWSTR, *mut super::Variant::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Variant")))]
    GetFieldNoCopy: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub PutField: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Variant")))]
    PutField: usize,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
    pub PutFieldNoCopy: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, windows_core::PCWSTR, *const super::Variant::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_Variant")))]
    PutFieldNoCopy: usize,
    pub GetFieldNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsMatchingType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::BOOL,
    pub RecordCreate: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut core::ffi::c_void,
    pub RecordCreateCopy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RecordDestroy: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl windows_core::RuntimeName for IRecordInfo {}
