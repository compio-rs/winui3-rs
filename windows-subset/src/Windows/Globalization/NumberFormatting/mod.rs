windows_core::imp::define_interface!(INumberFormatter2, INumberFormatter2_Vtbl, 0xd4a8c1f0_80d0_4b0d_a89e_882c1e8f8310);
impl windows_core::RuntimeType for INumberFormatter2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Globalization.NumberFormatting.INumberFormatter2");
}
windows_core::imp::interface_hierarchy!(INumberFormatter2, windows_core::IUnknown, windows_core::IInspectable);
impl INumberFormatter2 {
    pub fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatInt)(windows_core::Interface::as_raw(self), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatUInt)(windows_core::Interface::as_raw(self), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FormatDouble)(windows_core::Interface::as_raw(self), value, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for INumberFormatter2 {
    const NAME: &'static str = "Windows.Globalization.NumberFormatting.INumberFormatter2";
}
pub trait INumberFormatter2_Impl: windows_core::IUnknownImpl {
    fn FormatInt(&self, value: i64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatUInt(&self, value: u64) -> windows_core::Result<windows_core::HSTRING>;
    fn FormatDouble(&self, value: f64) -> windows_core::Result<windows_core::HSTRING>;
}
impl INumberFormatter2_Vtbl {
    pub const fn new<Identity: INumberFormatter2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FormatInt<Identity: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter2_Impl::FormatInt(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FormatUInt<Identity: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter2_Impl::FormatUInt(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FormatDouble<Identity: INumberFormatter2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INumberFormatter2_Impl::FormatDouble(this, value) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INumberFormatter2, OFFSET>(),
            FormatInt: FormatInt::<Identity, OFFSET>,
            FormatUInt: FormatUInt::<Identity, OFFSET>,
            FormatDouble: FormatDouble::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INumberFormatter2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INumberFormatter2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FormatInt: unsafe extern "system" fn(*mut core::ffi::c_void, i64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatUInt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FormatDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
