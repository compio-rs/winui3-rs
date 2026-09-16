#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CastingSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CastingSource, windows_core::IUnknown, windows_core::IInspectable);
impl CastingSource {
    pub fn PreferredSourceUri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PreferredSourceUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetPreferredSourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPreferredSourceUri)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for CastingSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICastingSource>();
}
unsafe impl windows_core::Interface for CastingSource {
    type Vtable = <ICastingSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICastingSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CastingSource {
    const NAME: &'static str = "Windows.Media.Casting.CastingSource";
}
unsafe impl Send for CastingSource {}
unsafe impl Sync for CastingSource {}
windows_core::imp::define_interface!(ICastingSource, ICastingSource_Vtbl, 0xf429ea72_3467_47e6_a027_522923e9d727);
impl windows_core::RuntimeType for ICastingSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Casting.ICastingSource");
}
impl windows_core::RuntimeName for ICastingSource {
    const NAME: &'static str = "Windows.Media.Casting.ICastingSource";
}
pub trait ICastingSource_Impl: windows_core::IUnknownImpl {
    fn PreferredSourceUri(&self) -> windows_core::Result<super::super::Foundation::Uri>;
    fn SetPreferredSourceUri(&self, value: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<()>;
}
impl ICastingSource_Vtbl {
    pub const fn new<Identity: ICastingSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreferredSourceUri<Identity: ICastingSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICastingSource_Impl::PreferredSourceUri(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreferredSourceUri<Identity: ICastingSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICastingSource_Impl::SetPreferredSourceUri(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICastingSource, OFFSET>(),
            PreferredSourceUri: PreferredSourceUri::<Identity, OFFSET>,
            SetPreferredSourceUri: SetPreferredSourceUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICastingSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PreferredSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPreferredSourceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
