windows_core::imp::define_interface!(IPointerPoint, IPointerPoint_Vtbl, 0xe995317d_7296_42d9_8233_c5be73b74a4a);
impl windows_core::RuntimeType for IPointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Input.IPointerPoint");
}
impl windows_core::RuntimeName for IPointerPoint {
    const NAME: &'static str = "Windows.UI.Input.IPointerPoint";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PointerDevice: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub RawPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
    pub PointerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub FrameId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointPhysicalPosition, IPointerPointPhysicalPosition_Vtbl, 0x003185a3_a5e7_4859_9c0b_89340204806c);
impl windows_core::RuntimeType for IPointerPointPhysicalPosition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Input.IPointerPointPhysicalPosition");
}
impl windows_core::RuntimeName for IPointerPointPhysicalPosition {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointPhysicalPosition";
}
pub trait IPointerPointPhysicalPosition_Impl: windows_core::IUnknownImpl {
    fn IsPhysicalPositionSupported(&self) -> windows_core::Result<bool>;
    fn PhysicalPosition(&self) -> windows_core::Result<super::super::Foundation::Point>;
}
impl IPointerPointPhysicalPosition_Vtbl {
    pub const fn new<Identity: IPointerPointPhysicalPosition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPhysicalPositionSupported<Identity: IPointerPointPhysicalPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointPhysicalPosition_Impl::IsPhysicalPositionSupported(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PhysicalPosition<Identity: IPointerPointPhysicalPosition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointPhysicalPosition_Impl::PhysicalPosition(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPointPhysicalPosition, OFFSET>(),
            IsPhysicalPositionSupported: IsPhysicalPositionSupported::<Identity, OFFSET>,
            PhysicalPosition: PhysicalPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointPhysicalPosition as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointPhysicalPosition_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPhysicalPositionSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PhysicalPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointStatics, IPointerPointStatics_Vtbl, 0xa506638d_2a1a_413e_bc75_9f38381cc069);
impl windows_core::RuntimeType for IPointerPointStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Input.IPointerPointStatics");
}
impl windows_core::RuntimeName for IPointerPointStatics {
    const NAME: &'static str = "Windows.UI.Input.IPointerPointStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIntermediatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerPoint, windows_core::IUnknown, windows_core::IInspectable);
impl PointerPoint {
    pub fn Position(&self) -> windows_core::Result<super::super::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn RawPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RawPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn FrameId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FrameId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInContact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInContact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPhysicalPositionSupported(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPointerPointPhysicalPosition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPhysicalPositionSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PhysicalPosition(&self) -> windows_core::Result<super::super::Foundation::Point> {
        let this = &windows_core::Interface::cast::<IPointerPointPhysicalPosition>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PhysicalPosition)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetCurrentPoint(pointerid: u32) -> windows_core::Result<Self> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPoint)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetIntermediatePoints(pointerid: u32) -> windows_core::Result<windows_collections::IVector<Self>> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIntermediatePoints)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerPoint, IPointerPointStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerPoint>();
}
unsafe impl windows_core::Interface for PointerPoint {
    type Vtable = <IPointerPoint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPoint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Windows.UI.Input.PointerPoint";
}
