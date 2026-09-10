windows_core::imp::define_interface!(IPenDevice, IPenDevice_Vtbl, 0x31856eba_a738_5a8c_b8f6_f97ef68d18ef);
impl windows_core::RuntimeType for IPenDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Devices.Input.IPenDevice");
}
impl windows_core::RuntimeName for IPenDevice {
    const NAME: &'static str = "Windows.Devices.Input.IPenDevice";
}
pub trait IPenDevice_Impl: windows_core::IUnknownImpl {
    fn PenId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IPenDevice_Vtbl {
    pub const fn new<Identity: IPenDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PenId<Identity: IPenDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenDevice_Impl::PenId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPenDevice, OFFSET>(), PenId: PenId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPenDevice as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PenId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPenDevice2, IPenDevice2_Vtbl, 0x0207d327_7fb8_5566_8c34_f8342037b7f9);
impl windows_core::RuntimeType for IPenDevice2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Devices.Input.IPenDevice2");
}
impl windows_core::RuntimeName for IPenDevice2 {
    const NAME: &'static str = "Windows.Devices.Input.IPenDevice2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDevice2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPenDeviceStatics, IPenDeviceStatics_Vtbl, 0x9dfbbe01_0966_5180_bcb4_b85060e39479);
impl windows_core::RuntimeType for IPenDeviceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Devices.Input.IPenDeviceStatics");
}
impl windows_core::RuntimeName for IPenDeviceStatics {
    const NAME: &'static str = "Windows.Devices.Input.IPenDeviceStatics";
}
pub trait IPenDeviceStatics_Impl: windows_core::IUnknownImpl {
    fn GetFromPointerId(&self, pointerId: u32) -> windows_core::Result<PenDevice>;
}
impl IPenDeviceStatics_Vtbl {
    pub const fn new<Identity: IPenDeviceStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFromPointerId<Identity: IPenDeviceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPenDeviceStatics_Impl::GetFromPointerId(this, pointerid) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPenDeviceStatics, OFFSET>(), GetFromPointerId: GetFromPointerId::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPenDeviceStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenDeviceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetFromPointerId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PenDevice(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PenDevice, windows_core::IUnknown, windows_core::IInspectable);
impl PenDevice {
    pub fn PenId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PenId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetFromPointerId(pointerid: u32) -> windows_core::Result<Self> {
        Self::IPenDeviceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFromPointerId)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IPenDeviceStatics<R, F: FnOnce(&IPenDeviceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PenDevice, IPenDeviceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PenDevice {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPenDevice>();
}
unsafe impl windows_core::Interface for PenDevice {
    type Vtable = <IPenDevice as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPenDevice as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PenDevice {
    const NAME: &'static str = "Windows.Devices.Input.PenDevice";
}
unsafe impl Send for PenDevice {}
unsafe impl Sync for PenDevice {}
