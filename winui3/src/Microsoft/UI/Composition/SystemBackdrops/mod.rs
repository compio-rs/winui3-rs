#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopAcrylicController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DesktopAcrylicController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DesktopAcrylicController, windows::Foundation::IClosable, super::super::IClosableNotifier, ISystemBackdropController, ISystemBackdropControllerWithTargets);
impl DesktopAcrylicController {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DesktopAcrylicController, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsClosed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsClosed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FallbackColor(&self) -> windows_core::Result<windows::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FallbackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetFallbackColor(&self, value: windows::UI::Color) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFallbackColor)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn LuminosityOpacity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LuminosityOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLuminosityOpacity)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn TintColor(&self) -> windows_core::Result<windows::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TintColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTintColor(&self, value: windows::UI::Color) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTintColor)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn TintOpacity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TintOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTintOpacity)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ResetProperties(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IDesktopAcrylicController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ResetProperties)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IDesktopAcrylicControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetTargetWithWindowId<P1>(&self, windowid: super::super::WindowId, desktopwindowtarget: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTargetWithWindowId)(windows_core::Interface::as_raw(this), windowid, desktopwindowtarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetTargetWithCoreWindow<P0, P1>(&self, corewindow: P0, compositiontarget: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows::UI::Core::CoreWindow>,
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTargetWithCoreWindow)(windows_core::Interface::as_raw(this), corewindow.param().abi(), compositiontarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn AddSystemBackdropTarget<P0>(&self, systembackdroptarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::ICompositionSupportsSystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddSystemBackdropTarget)(windows_core::Interface::as_raw(this), systembackdroptarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAllSystemBackdropTargets)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveSystemBackdropTarget<P0>(&self, systembackdroptarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::ICompositionSupportsSystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveSystemBackdropTarget)(windows_core::Interface::as_raw(this), systembackdroptarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetSystemBackdropConfiguration<P0>(&self, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SystemBackdropConfiguration>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSystemBackdropConfiguration)(windows_core::Interface::as_raw(this), configuration.param().abi()).ok() }
    }
    pub fn StateChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<ISystemBackdropControllerWithTargets>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<ISystemBackdropControllerWithTargets, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveStateChanged))
        }
    }
    fn IDesktopAcrylicControllerStatics<R, F: FnOnce(&IDesktopAcrylicControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DesktopAcrylicController, IDesktopAcrylicControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DesktopAcrylicController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDesktopAcrylicController>();
}
unsafe impl windows_core::Interface for DesktopAcrylicController {
    type Vtable = <IDesktopAcrylicController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDesktopAcrylicController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DesktopAcrylicController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.DesktopAcrylicController";
}
unsafe impl Send for DesktopAcrylicController {}
unsafe impl Sync for DesktopAcrylicController {}
windows_core::imp::define_interface!(IDesktopAcrylicController, IDesktopAcrylicController_Vtbl, 0x7c20a6af_8eb3_5f08_bdfc_6d35e35dfe45);
impl windows_core::RuntimeType for IDesktopAcrylicController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicController");
}
impl windows_core::RuntimeName for IDesktopAcrylicController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicController";
}
pub trait IDesktopAcrylicController_Impl: windows_core::IUnknownImpl {
    fn FallbackColor(&self) -> windows_core::Result<windows::UI::Color>;
    fn SetFallbackColor(&self, value: &windows::UI::Color) -> windows_core::Result<()>;
    fn LuminosityOpacity(&self) -> windows_core::Result<f32>;
    fn SetLuminosityOpacity(&self, value: f32) -> windows_core::Result<()>;
    fn TintColor(&self) -> windows_core::Result<windows::UI::Color>;
    fn SetTintColor(&self, value: &windows::UI::Color) -> windows_core::Result<()>;
    fn TintOpacity(&self) -> windows_core::Result<f32>;
    fn SetTintOpacity(&self, value: f32) -> windows_core::Result<()>;
}
impl IDesktopAcrylicController_Vtbl {
    pub const fn new<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FallbackColor<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopAcrylicController_Impl::FallbackColor(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFallbackColor<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDesktopAcrylicController_Impl::SetFallbackColor(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn LuminosityOpacity<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopAcrylicController_Impl::LuminosityOpacity(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLuminosityOpacity<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDesktopAcrylicController_Impl::SetLuminosityOpacity(this, value).into()
            }
        }
        unsafe extern "system" fn TintColor<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopAcrylicController_Impl::TintColor(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTintColor<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDesktopAcrylicController_Impl::SetTintColor(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn TintOpacity<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopAcrylicController_Impl::TintOpacity(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTintOpacity<Identity: IDesktopAcrylicController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDesktopAcrylicController_Impl::SetTintOpacity(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDesktopAcrylicController, OFFSET>(),
            FallbackColor: FallbackColor::<Identity, OFFSET>,
            SetFallbackColor: SetFallbackColor::<Identity, OFFSET>,
            LuminosityOpacity: LuminosityOpacity::<Identity, OFFSET>,
            SetLuminosityOpacity: SetLuminosityOpacity::<Identity, OFFSET>,
            TintColor: TintColor::<Identity, OFFSET>,
            SetTintColor: SetTintColor::<Identity, OFFSET>,
            TintOpacity: TintOpacity::<Identity, OFFSET>,
            SetTintOpacity: SetTintOpacity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopAcrylicController as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FallbackColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Color) -> windows_core::HRESULT,
    pub LuminosityOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub TintColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Color) -> windows_core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDesktopAcrylicController2, IDesktopAcrylicController2_Vtbl, 0x88e0a368_dfc7_5971_a50b_40df5aa5f5c2);
impl windows_core::RuntimeType for IDesktopAcrylicController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicController2");
}
impl windows_core::RuntimeName for IDesktopAcrylicController2 {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicController2";
}
pub trait IDesktopAcrylicController2_Impl: windows_core::IUnknownImpl {
    fn ResetProperties(&self) -> windows_core::Result<()>;
}
impl IDesktopAcrylicController2_Vtbl {
    pub const fn new<Identity: IDesktopAcrylicController2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResetProperties<Identity: IDesktopAcrylicController2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDesktopAcrylicController2_Impl::ResetProperties(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDesktopAcrylicController2, OFFSET>(),
            ResetProperties: ResetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopAcrylicController2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResetProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDesktopAcrylicController3, IDesktopAcrylicController3_Vtbl, 0x30d917e6_02d3_59ca_b440_bf9d2e7cc140);
impl windows_core::RuntimeType for IDesktopAcrylicController3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicController3");
}
impl windows_core::RuntimeName for IDesktopAcrylicController3 {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicController3";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicController3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDesktopAcrylicControllerStatics, IDesktopAcrylicControllerStatics_Vtbl, 0xa9e8f790_79ef_5416_9b67_6bcfe867c8b7);
impl windows_core::RuntimeType for IDesktopAcrylicControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicControllerStatics");
}
impl windows_core::RuntimeName for IDesktopAcrylicControllerStatics {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.IDesktopAcrylicControllerStatics";
}
pub trait IDesktopAcrylicControllerStatics_Impl: windows_core::IUnknownImpl {
    fn IsSupported(&self) -> windows_core::Result<bool>;
}
impl IDesktopAcrylicControllerStatics_Vtbl {
    pub const fn new<Identity: IDesktopAcrylicControllerStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSupported<Identity: IDesktopAcrylicControllerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopAcrylicControllerStatics_Impl::IsSupported(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDesktopAcrylicControllerStatics, OFFSET>(),
            IsSupported: IsSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopAcrylicControllerStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicaController, IMicaController_Vtbl, 0x2de996a9_0a2a_5889_a89c_1f84060a8cab);
impl windows_core::RuntimeType for IMicaController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.IMicaController");
}
impl windows_core::RuntimeName for IMicaController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.IMicaController";
}
pub trait IMicaController_Impl: windows_core::IUnknownImpl {
    fn FallbackColor(&self) -> windows_core::Result<windows::UI::Color>;
    fn SetFallbackColor(&self, value: &windows::UI::Color) -> windows_core::Result<()>;
    fn LuminosityOpacity(&self) -> windows_core::Result<f32>;
    fn SetLuminosityOpacity(&self, value: f32) -> windows_core::Result<()>;
    fn TintColor(&self) -> windows_core::Result<windows::UI::Color>;
    fn SetTintColor(&self, value: &windows::UI::Color) -> windows_core::Result<()>;
    fn TintOpacity(&self) -> windows_core::Result<f32>;
    fn SetTintOpacity(&self, value: f32) -> windows_core::Result<()>;
}
impl IMicaController_Vtbl {
    pub const fn new<Identity: IMicaController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FallbackColor<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaController_Impl::FallbackColor(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFallbackColor<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMicaController_Impl::SetFallbackColor(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn LuminosityOpacity<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaController_Impl::LuminosityOpacity(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLuminosityOpacity<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMicaController_Impl::SetLuminosityOpacity(this, value).into()
            }
        }
        unsafe extern "system" fn TintColor<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaController_Impl::TintColor(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTintColor<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::UI::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMicaController_Impl::SetTintColor(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn TintOpacity<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaController_Impl::TintOpacity(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTintOpacity<Identity: IMicaController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMicaController_Impl::SetTintOpacity(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMicaController, OFFSET>(),
            FallbackColor: FallbackColor::<Identity, OFFSET>,
            SetFallbackColor: SetFallbackColor::<Identity, OFFSET>,
            LuminosityOpacity: LuminosityOpacity::<Identity, OFFSET>,
            SetLuminosityOpacity: SetLuminosityOpacity::<Identity, OFFSET>,
            TintColor: TintColor::<Identity, OFFSET>,
            SetTintColor: SetTintColor::<Identity, OFFSET>,
            TintOpacity: TintOpacity::<Identity, OFFSET>,
            SetTintOpacity: SetTintOpacity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMicaController as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FallbackColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
    pub SetFallbackColor: unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Color) -> windows_core::HRESULT,
    pub LuminosityOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetLuminosityOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub TintColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Color) -> windows_core::HRESULT,
    pub SetTintColor: unsafe extern "system" fn(*mut core::ffi::c_void, windows::UI::Color) -> windows_core::HRESULT,
    pub TintOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetTintOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicaController2, IMicaController2_Vtbl, 0xf1ed4a52_d9ca_506e_9586_caaefd3aa971);
impl windows_core::RuntimeType for IMicaController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.IMicaController2");
}
impl windows_core::RuntimeName for IMicaController2 {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.IMicaController2";
}
pub trait IMicaController2_Impl: windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<MicaKind>;
    fn SetKind(&self, value: MicaKind) -> windows_core::Result<()>;
    fn ResetProperties(&self) -> windows_core::Result<()>;
}
impl IMicaController2_Vtbl {
    pub const fn new<Identity: IMicaController2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Kind<Identity: IMicaController2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MicaKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaController2_Impl::Kind(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKind<Identity: IMicaController2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: MicaKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMicaController2_Impl::SetKind(this, value).into()
            }
        }
        unsafe extern "system" fn ResetProperties<Identity: IMicaController2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMicaController2_Impl::ResetProperties(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMicaController2, OFFSET>(),
            Kind: Kind::<Identity, OFFSET>,
            SetKind: SetKind::<Identity, OFFSET>,
            ResetProperties: ResetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMicaController2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MicaKind) -> windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, MicaKind) -> windows_core::HRESULT,
    pub ResetProperties: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicaControllerStatics, IMicaControllerStatics_Vtbl, 0x7d85d834_d514_5250_b7c4_0b7850d1efdc);
impl windows_core::RuntimeType for IMicaControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.IMicaControllerStatics");
}
impl windows_core::RuntimeName for IMicaControllerStatics {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.IMicaControllerStatics";
}
pub trait IMicaControllerStatics_Impl: windows_core::IUnknownImpl {
    fn IsSupported(&self) -> windows_core::Result<bool>;
}
impl IMicaControllerStatics_Vtbl {
    pub const fn new<Identity: IMicaControllerStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSupported<Identity: IMicaControllerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaControllerStatics_Impl::IsSupported(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMicaControllerStatics, OFFSET>(), IsSupported: IsSupported::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMicaControllerStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemBackdropConfiguration, ISystemBackdropConfiguration_Vtbl, 0xebcce1b9_0e0c_5431_ab0e_00f3f0669962);
impl windows_core::RuntimeType for ISystemBackdropConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropConfiguration");
}
impl windows_core::RuntimeName for ISystemBackdropConfiguration {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropConfiguration";
}
pub trait ISystemBackdropConfiguration_Impl: windows_core::IUnknownImpl {
    fn HighContrastBackgroundColor(&self) -> windows_core::Result<windows_reference::IReference<windows::UI::Color>>;
    fn SetHighContrastBackgroundColor(&self, value: windows_core::Ref<windows_reference::IReference<windows::UI::Color>>) -> windows_core::Result<()>;
    fn IsHighContrast(&self) -> windows_core::Result<bool>;
    fn SetIsHighContrast(&self, value: bool) -> windows_core::Result<()>;
    fn IsInputActive(&self) -> windows_core::Result<bool>;
    fn SetIsInputActive(&self, value: bool) -> windows_core::Result<()>;
    fn Theme(&self) -> windows_core::Result<SystemBackdropTheme>;
    fn SetTheme(&self, value: SystemBackdropTheme) -> windows_core::Result<()>;
}
impl ISystemBackdropConfiguration_Vtbl {
    pub const fn new<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HighContrastBackgroundColor<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdropConfiguration_Impl::HighContrastBackgroundColor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHighContrastBackgroundColor<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemBackdropConfiguration_Impl::SetHighContrastBackgroundColor(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn IsHighContrast<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdropConfiguration_Impl::IsHighContrast(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsHighContrast<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemBackdropConfiguration_Impl::SetIsHighContrast(this, value).into()
            }
        }
        unsafe extern "system" fn IsInputActive<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdropConfiguration_Impl::IsInputActive(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsInputActive<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemBackdropConfiguration_Impl::SetIsInputActive(this, value).into()
            }
        }
        unsafe extern "system" fn Theme<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SystemBackdropTheme) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdropConfiguration_Impl::Theme(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTheme<Identity: ISystemBackdropConfiguration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SystemBackdropTheme) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemBackdropConfiguration_Impl::SetTheme(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISystemBackdropConfiguration, OFFSET>(),
            HighContrastBackgroundColor: HighContrastBackgroundColor::<Identity, OFFSET>,
            SetHighContrastBackgroundColor: SetHighContrastBackgroundColor::<Identity, OFFSET>,
            IsHighContrast: IsHighContrast::<Identity, OFFSET>,
            SetIsHighContrast: SetIsHighContrast::<Identity, OFFSET>,
            IsInputActive: IsInputActive::<Identity, OFFSET>,
            SetIsInputActive: SetIsInputActive::<Identity, OFFSET>,
            Theme: Theme::<Identity, OFFSET>,
            SetTheme: SetTheme::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemBackdropConfiguration as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropConfiguration_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HighContrastBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetHighContrastBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsHighContrast: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHighContrast: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsInputActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsInputActive: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Theme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SystemBackdropTheme) -> windows_core::HRESULT,
    pub SetTheme: unsafe extern "system" fn(*mut core::ffi::c_void, SystemBackdropTheme) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemBackdropController, ISystemBackdropController_Vtbl, 0x5632d76c_0b74_5b52_aa33_80262068aeb2);
impl windows_core::RuntimeType for ISystemBackdropController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropController");
}
windows_core::imp::interface_hierarchy!(ISystemBackdropController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ISystemBackdropController, windows::Foundation::IClosable);
impl ISystemBackdropController {
    pub fn SetTargetWithWindowId<P1>(&self, windowid: super::super::WindowId, desktopwindowtarget: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetTargetWithWindowId)(windows_core::Interface::as_raw(self), windowid, desktopwindowtarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetTargetWithCoreWindow<P0, P1>(&self, corewindow: P0, compositiontarget: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows::UI::Core::CoreWindow>,
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetTargetWithCoreWindow)(windows_core::Interface::as_raw(self), corewindow.param().abi(), compositiontarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for ISystemBackdropController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropController";
}
pub trait ISystemBackdropController_Impl: windows::Foundation::IClosable_Impl {
    fn SetTargetWithWindowId(&self, windowId: &super::super::WindowId, desktopWindowTarget: windows_core::Ref<windows::UI::Composition::CompositionTarget>) -> windows_core::Result<bool>;
    fn SetTargetWithCoreWindow(&self, coreWindow: windows_core::Ref<windows::UI::Core::CoreWindow>, compositionTarget: windows_core::Ref<windows::UI::Composition::CompositionTarget>) -> windows_core::Result<bool>;
}
impl ISystemBackdropController_Vtbl {
    pub const fn new<Identity: ISystemBackdropController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTargetWithWindowId<Identity: ISystemBackdropController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowid: super::super::WindowId, desktopwindowtarget: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdropController_Impl::SetTargetWithWindowId(this, core::mem::transmute(&windowid), core::mem::transmute_copy(&desktopwindowtarget)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetWithCoreWindow<Identity: ISystemBackdropController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, corewindow: *mut core::ffi::c_void, compositiontarget: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdropController_Impl::SetTargetWithCoreWindow(this, core::mem::transmute_copy(&corewindow), core::mem::transmute_copy(&compositiontarget)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISystemBackdropController, OFFSET>(),
            SetTargetWithWindowId: SetTargetWithWindowId::<Identity, OFFSET>,
            SetTargetWithCoreWindow: SetTargetWithCoreWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemBackdropController as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetTargetWithWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::WindowId, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetTargetWithCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemBackdropControllerWithTargets, ISystemBackdropControllerWithTargets_Vtbl, 0x9c56fe7c_98eb_5f89_ad97_dad57fc30c8c);
impl windows_core::RuntimeType for ISystemBackdropControllerWithTargets {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropControllerWithTargets");
}
windows_core::imp::interface_hierarchy!(ISystemBackdropControllerWithTargets, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ISystemBackdropControllerWithTargets, windows::Foundation::IClosable, ISystemBackdropController);
impl ISystemBackdropControllerWithTargets {
    pub fn AddSystemBackdropTarget<P0>(&self, systembackdroptarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::ICompositionSupportsSystemBackdrop>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddSystemBackdropTarget)(windows_core::Interface::as_raw(self), systembackdroptarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllSystemBackdropTargets)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn RemoveSystemBackdropTarget<P0>(&self, systembackdroptarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::ICompositionSupportsSystemBackdrop>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoveSystemBackdropTarget)(windows_core::Interface::as_raw(self), systembackdroptarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetSystemBackdropConfiguration<P0>(&self, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SystemBackdropConfiguration>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSystemBackdropConfiguration)(windows_core::Interface::as_raw(self), configuration.param().abi()).ok() }
    }
    pub fn StateChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).StateChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveStateChanged))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetTargetWithWindowId<P1>(&self, windowid: super::super::WindowId, desktopwindowtarget: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTargetWithWindowId)(windows_core::Interface::as_raw(this), windowid, desktopwindowtarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetTargetWithCoreWindow<P0, P1>(&self, corewindow: P0, compositiontarget: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows::UI::Core::CoreWindow>,
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTargetWithCoreWindow)(windows_core::Interface::as_raw(this), corewindow.param().abi(), compositiontarget.param().abi(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for ISystemBackdropControllerWithTargets {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.ISystemBackdropControllerWithTargets";
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropControllerWithTargets_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    State: usize,
    pub AddSystemBackdropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub RemoveAllSystemBackdropTargets: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSystemBackdropTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetSystemBackdropConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MicaController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MicaController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MicaController, windows::Foundation::IClosable, super::super::IClosableNotifier, ISystemBackdropController, ISystemBackdropControllerWithTargets);
impl MicaController {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MicaController, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsClosed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IClosableNotifier>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsClosed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn FallbackColor(&self) -> windows_core::Result<windows::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FallbackColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetFallbackColor(&self, value: windows::UI::Color) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFallbackColor)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn LuminosityOpacity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LuminosityOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetLuminosityOpacity(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLuminosityOpacity)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn TintColor(&self) -> windows_core::Result<windows::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TintColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTintColor(&self, value: windows::UI::Color) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTintColor)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn TintOpacity(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TintOpacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTintOpacity(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTintOpacity)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Kind(&self) -> windows_core::Result<MicaKind> {
        let this = &windows_core::Interface::cast::<IMicaController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Kind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: MicaKind) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMicaController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKind)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResetProperties(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMicaController2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ResetProperties)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IMicaControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetTargetWithWindowId<P1>(&self, windowid: super::super::WindowId, desktopwindowtarget: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTargetWithWindowId)(windows_core::Interface::as_raw(this), windowid, desktopwindowtarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetTargetWithCoreWindow<P0, P1>(&self, corewindow: P0, compositiontarget: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows::UI::Core::CoreWindow>,
        P1: windows_core::Param<windows::UI::Composition::CompositionTarget>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropController>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetTargetWithCoreWindow)(windows_core::Interface::as_raw(this), corewindow.param().abi(), compositiontarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn AddSystemBackdropTarget<P0>(&self, systembackdroptarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::ICompositionSupportsSystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AddSystemBackdropTarget)(windows_core::Interface::as_raw(this), systembackdroptarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveAllSystemBackdropTargets(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAllSystemBackdropTargets)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveSystemBackdropTarget<P0>(&self, systembackdroptarget: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::ICompositionSupportsSystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveSystemBackdropTarget)(windows_core::Interface::as_raw(this), systembackdroptarget.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn SetSystemBackdropConfiguration<P0>(&self, configuration: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<SystemBackdropConfiguration>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSystemBackdropConfiguration)(windows_core::Interface::as_raw(this), configuration.param().abi()).ok() }
    }
    pub fn StateChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<ISystemBackdropControllerWithTargets>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropControllerWithTargets>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<ISystemBackdropControllerWithTargets, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveStateChanged))
        }
    }
    fn IMicaControllerStatics<R, F: FnOnce(&IMicaControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MicaController, IMicaControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MicaController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicaController>();
}
unsafe impl windows_core::Interface for MicaController {
    type Vtable = <IMicaController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMicaController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MicaController {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.MicaController";
}
unsafe impl Send for MicaController {}
unsafe impl Sync for MicaController {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MicaKind(pub i32);
impl MicaKind {
    pub const Base: Self = Self(0);
    pub const BaseAlt: Self = Self(1);
}
impl windows_core::imp::TypeKind for MicaKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MicaKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Composition.SystemBackdrops.MicaKind;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.MicaKind");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemBackdropConfiguration(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemBackdropConfiguration, windows_core::IUnknown, windows_core::IInspectable);
impl SystemBackdropConfiguration {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemBackdropConfiguration, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn HighContrastBackgroundColor(&self) -> windows_core::Result<windows::UI::Color> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HighContrastBackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)).and_then(|r__: windows_reference::IReference<windows::UI::Color>| r__.Value())
        }
    }
    pub fn SetHighContrastBackgroundColor(&self, value: Option<windows::UI::Color>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<windows::UI::Color> as From<_>>::from);
        unsafe { (windows_core::Interface::vtable(self).SetHighContrastBackgroundColor)(windows_core::Interface::as_raw(self), windows_core::Param::param(value__.as_ref()).abi()).ok() }
    }
    pub fn IsHighContrast(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHighContrast)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHighContrast(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsHighContrast)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsInputActive(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInputActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsInputActive(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsInputActive)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Theme(&self) -> windows_core::Result<SystemBackdropTheme> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Theme)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTheme(&self, value: SystemBackdropTheme) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTheme)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for SystemBackdropConfiguration {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemBackdropConfiguration>();
}
unsafe impl windows_core::Interface for SystemBackdropConfiguration {
    type Vtable = <ISystemBackdropConfiguration as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISystemBackdropConfiguration as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemBackdropConfiguration {
    const NAME: &'static str = "Microsoft.UI.Composition.SystemBackdrops.SystemBackdropConfiguration";
}
unsafe impl Send for SystemBackdropConfiguration {}
unsafe impl Sync for SystemBackdropConfiguration {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SystemBackdropTheme(pub i32);
impl SystemBackdropTheme {
    pub const Default: Self = Self(0);
    pub const Light: Self = Self(1);
    pub const Dark: Self = Self(2);
}
impl windows_core::imp::TypeKind for SystemBackdropTheme {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for SystemBackdropTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Composition.SystemBackdrops.SystemBackdropTheme;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.SystemBackdrops.SystemBackdropTheme");
}
