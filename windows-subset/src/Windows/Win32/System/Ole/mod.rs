#[inline]
pub unsafe fn OleInitialize(pvreserved: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn OleInitialize(pvreserved : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { OleInitialize(pvreserved.unwrap_or(core::mem::zeroed()) as _).ok() }
}
#[inline]
pub unsafe fn OleUninitialize() {
    windows_core::link!("ole32.dll" "system" fn OleUninitialize());
    unsafe { OleUninitialize() }
}
#[inline]
pub unsafe fn RegisterDragDrop<P1>(hwnd: super::super::Foundation::HWND, pdroptarget: P1) -> windows_core::Result<()>
where
    P1: windows_core::Param<IDropTarget>,
{
    windows_core::link!("ole32.dll" "system" fn RegisterDragDrop(hwnd : super::super::Foundation::HWND, pdroptarget : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RegisterDragDrop(hwnd, pdroptarget.param().abi()).ok() }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
#[inline]
pub unsafe fn ReleaseStgMedium(param0: *mut super::Com::STGMEDIUM) {
    windows_core::link!("ole32.dll" "system" fn ReleaseStgMedium(param0 : *mut super::Com::STGMEDIUM));
    unsafe { ReleaseStgMedium(param0) }
}
#[inline]
pub unsafe fn RevokeDragDrop(hwnd: super::super::Foundation::HWND) -> windows_core::Result<()> {
    windows_core::link!("ole32.dll" "system" fn RevokeDragDrop(hwnd : super::super::Foundation::HWND) -> windows_core::HRESULT);
    unsafe { RevokeDragDrop(hwnd).ok() }
}
pub type DROPEFFECT = u32;
pub const DROPEFFECT_COPY: DROPEFFECT = 1;
pub const DROPEFFECT_LINK: DROPEFFECT = 4;
pub const DROPEFFECT_MOVE: DROPEFFECT = 2;
pub const DROPEFFECT_NONE: DROPEFFECT = 0;
pub const DROPEFFECT_SCROLL: DROPEFFECT = 2147483648;
windows_core::imp::define_interface!(IDropTarget, IDropTarget_Vtbl, 0x00000122_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDropTarget, windows_core::IUnknown);
impl IDropTarget {
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn DragEnter<P0>(&self, pdataobj: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).DragEnter)(windows_core::Interface::as_raw(self), pdataobj.param().abi(), grfkeystate, pt, pdweffect as _).ok() }
    }
    #[cfg(feature = "Win32_System_SystemServices")]
    pub unsafe fn DragOver(&self, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DragOver)(windows_core::Interface::as_raw(self), grfkeystate, pt, pdweffect as _).ok() }
    }
    pub unsafe fn DragLeave(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DragLeave)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub unsafe fn Drop<P0>(&self, pdataobj: P0, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Com::IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).Drop)(windows_core::Interface::as_raw(self), pdataobj.param().abi(), grfkeystate, pt, pdweffect as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub DragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::SystemServices::MODIFIERKEYS_FLAGS, super::super::Foundation::POINTL, *mut DROPEFFECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    DragEnter: usize,
    #[cfg(feature = "Win32_System_SystemServices")]
    pub DragOver: unsafe extern "system" fn(*mut core::ffi::c_void, super::SystemServices::MODIFIERKEYS_FLAGS, super::super::Foundation::POINTL, *mut DROPEFFECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_SystemServices"))]
    DragOver: usize,
    pub DragLeave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
    pub Drop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::SystemServices::MODIFIERKEYS_FLAGS, super::super::Foundation::POINTL, *mut DROPEFFECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices")))]
    Drop: usize,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
pub trait IDropTarget_Impl: windows_core::IUnknownImpl {
    fn DragEnter(&self, pdataobj: windows_core::Ref<super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>;
    fn DragOver(&self, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>;
    fn DragLeave(&self) -> windows_core::Result<()>;
    fn Drop(&self, pdataobj: windows_core::Ref<super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl IDropTarget_Vtbl {
    pub const fn new<Identity: IDropTarget_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DragEnter<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobj: *mut core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::DragEnter(this, core::mem::transmute_copy(&pdataobj), core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn DragOver<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::DragOver(this, core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        unsafe extern "system" fn DragLeave<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::DragLeave(this).into()
            }
        }
        unsafe extern "system" fn Drop<Identity: IDropTarget_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobj: *mut core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDropTarget_Impl::Drop(this, core::mem::transmute_copy(&pdataobj), core::mem::transmute_copy(&grfkeystate), core::mem::transmute(&pt), core::mem::transmute_copy(&pdweffect)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DragEnter: DragEnter::<Identity, OFFSET>,
            DragOver: DragOver::<Identity, OFFSET>,
            DragLeave: DragLeave::<Identity, OFFSET>,
            Drop: Drop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropTarget as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl windows_core::RuntimeName for IDropTarget {}
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
