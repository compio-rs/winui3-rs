#[cfg(feature = "UI_Xaml_Media_DxInterop")]
pub mod DxInterop;
#[cfg(feature = "UI_Xaml_Media_Imaging")]
pub mod Imaging;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArcSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ArcSegment, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ArcSegment, PathSegment, super::DependencyObject);
impl ArcSegment {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ArcSegment, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Point(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Point)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPoint(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPoint)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Size(&self) -> windows_core::Result<windows::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: windows::Foundation::Size) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn RotationAngle(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RotationAngle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAngle(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRotationAngle)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsLargeArc(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLargeArc)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLargeArc(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsLargeArc)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn SweepDirection(&self) -> windows_core::Result<SweepDirection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SweepDirection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetSweepDirection(&self, value: SweepDirection) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSweepDirection)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IArcSegmentStatics<R, F: FnOnce(&IArcSegmentStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ArcSegment, IArcSegmentStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ArcSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IArcSegment>();
}
unsafe impl windows_core::Interface for ArcSegment {
    type Vtable = <IArcSegment as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IArcSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ArcSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ArcSegment";
}
unsafe impl Send for ArcSegment {}
unsafe impl Sync for ArcSegment {}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Brush(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(Brush, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(Brush, super::super::Composition::IAnimationObject, super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl Brush {
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Opacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IBrushFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IBrushFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IBrushFactory<R, F: FnOnce(&IBrushFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Brush, IBrushFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBrushStatics<R, F: FnOnce(&IBrushStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Brush, IBrushStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for Brush {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBrush>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for Brush {
    type Vtable = <IBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBrush as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for Brush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Brush";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for Brush {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for Brush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DesktopAcrylicBackdrop(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DesktopAcrylicBackdrop, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DesktopAcrylicBackdrop, SystemBackdrop, super::DependencyObject);
impl DesktopAcrylicBackdrop {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IDesktopAcrylicBackdropFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IDesktopAcrylicBackdropFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub fn GetDefaultSystemBackdropConfiguration<P0, P1>(&self, target: P0, xamlroot: P1) -> windows_core::Result<super::super::Composition::SystemBackdrops::SystemBackdropConfiguration>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdrop>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultSystemBackdropConfiguration)(windows_core::Interface::as_raw(this), target.param().abi(), xamlroot.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetConnected<P0, P1>(&self, connectedtarget: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTargetConnected)(windows_core::Interface::as_raw(this), connectedtarget.param().abi(), xamlroot.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetDisconnected<P0>(&self, disconnectedtarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTargetDisconnected)(windows_core::Interface::as_raw(this), disconnectedtarget.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnDefaultSystemBackdropConfigurationChanged<P0, P1>(&self, target: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDefaultSystemBackdropConfigurationChanged)(windows_core::Interface::as_raw(this), target.param().abi(), xamlroot.param().abi()).ok() }
    }
    fn IDesktopAcrylicBackdropFactory<R, F: FnOnce(&IDesktopAcrylicBackdropFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DesktopAcrylicBackdrop, IDesktopAcrylicBackdropFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DesktopAcrylicBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDesktopAcrylicBackdrop>();
}
unsafe impl windows_core::Interface for DesktopAcrylicBackdrop {
    type Vtable = <IDesktopAcrylicBackdrop as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDesktopAcrylicBackdrop as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DesktopAcrylicBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.DesktopAcrylicBackdrop";
}
unsafe impl Send for DesktopAcrylicBackdrop {}
unsafe impl Sync for DesktopAcrylicBackdrop {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FontFamily(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FontFamily, windows_core::IUnknown, windows_core::IInspectable);
impl FontFamily {
    pub fn Source(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CreateInstanceWithName(familyname: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IFontFamilyFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(familyname), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceWithName_compose<T>(familyname: &windows_core::HSTRING, compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IFontFamilyFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(familyname), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn XamlAutoFontFamily() -> windows_core::Result<Self> {
        Self::IFontFamilyStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlAutoFontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFontFamilyFactory<R, F: FnOnce(&IFontFamilyFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontFamily, IFontFamilyFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFontFamilyStatics<R, F: FnOnce(&IFontFamilyStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FontFamily, IFontFamilyStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FontFamily {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFontFamily>();
}
unsafe impl windows_core::Interface for FontFamily {
    type Vtable = <IFontFamily as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFontFamily as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FontFamily {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.FontFamily";
}
unsafe impl Send for FontFamily {}
unsafe impl Sync for FontFamily {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Geometry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Geometry, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Geometry, super::DependencyObject);
impl Geometry {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Bounds(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bounds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Empty() -> windows_core::Result<Self> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Empty)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn StandardFlatteningTolerance() -> windows_core::Result<f64> {
        Self::IGeometryStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StandardFlatteningTolerance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IGeometryStatics<R, F: FnOnce(&IGeometryStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Geometry, IGeometryStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Geometry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGeometry>();
}
unsafe impl windows_core::Interface for Geometry {
    type Vtable = <IGeometry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGeometry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Geometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Geometry";
}
unsafe impl Send for Geometry {}
unsafe impl Sync for Geometry {}
windows_core::imp::define_interface!(IArcSegment, IArcSegment_Vtbl, 0x6b7ce02b_87be_5acb_9d3b_c9964c6962d0);
impl windows_core::RuntimeType for IArcSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IArcSegment");
}
impl windows_core::RuntimeName for IArcSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IArcSegment";
}
pub trait IArcSegment_Impl: windows_core::IUnknownImpl {
    fn Point(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn SetPoint(&self, value: &windows::Foundation::Point) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<windows::Foundation::Size>;
    fn SetSize(&self, value: &windows::Foundation::Size) -> windows_core::Result<()>;
    fn RotationAngle(&self) -> windows_core::Result<f64>;
    fn SetRotationAngle(&self, value: f64) -> windows_core::Result<()>;
    fn IsLargeArc(&self) -> windows_core::Result<bool>;
    fn SetIsLargeArc(&self, value: bool) -> windows_core::Result<()>;
    fn SweepDirection(&self) -> windows_core::Result<SweepDirection>;
    fn SetSweepDirection(&self, value: SweepDirection) -> windows_core::Result<()>;
}
impl IArcSegment_Vtbl {
    pub const fn new<Identity: IArcSegment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Point<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IArcSegment_Impl::Point(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPoint<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IArcSegment_Impl::SetPoint(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Size<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IArcSegment_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IArcSegment_Impl::SetSize(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn RotationAngle<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IArcSegment_Impl::RotationAngle(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotationAngle<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IArcSegment_Impl::SetRotationAngle(this, value).into()
            }
        }
        unsafe extern "system" fn IsLargeArc<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IArcSegment_Impl::IsLargeArc(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsLargeArc<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IArcSegment_Impl::SetIsLargeArc(this, value).into()
            }
        }
        unsafe extern "system" fn SweepDirection<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut SweepDirection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IArcSegment_Impl::SweepDirection(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSweepDirection<Identity: IArcSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: SweepDirection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IArcSegment_Impl::SetSweepDirection(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IArcSegment, OFFSET>(),
            Point: Point::<Identity, OFFSET>,
            SetPoint: SetPoint::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            RotationAngle: RotationAngle::<Identity, OFFSET>,
            SetRotationAngle: SetRotationAngle::<Identity, OFFSET>,
            IsLargeArc: IsLargeArc::<Identity, OFFSET>,
            SetIsLargeArc: SetIsLargeArc::<Identity, OFFSET>,
            SweepDirection: SweepDirection::<Identity, OFFSET>,
            SetSweepDirection: SetSweepDirection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IArcSegment as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Size) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Size) -> windows_core::HRESULT,
    pub RotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsLargeArc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsLargeArc: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SweepDirection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SweepDirection) -> windows_core::HRESULT,
    pub SetSweepDirection: unsafe extern "system" fn(*mut core::ffi::c_void, SweepDirection) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IArcSegmentStatics, IArcSegmentStatics_Vtbl, 0x5ba7ccb3_5bc7_5038_99c5_93dc730230cf);
impl windows_core::RuntimeType for IArcSegmentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IArcSegmentStatics");
}
impl windows_core::RuntimeName for IArcSegmentStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IArcSegmentStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IArcSegmentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IBrush, IBrush_Vtbl, 0x2de3cb83_1329_5679_88f8_c822bc5442cb);
impl windows_core::RuntimeType for IBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IBrush");
}
impl windows_core::RuntimeName for IBrush {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IBrush";
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Opacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBrushFactory, IBrushFactory_Vtbl, 0xb5258717_6c49_5ba5_87fd_35df382647a5);
impl windows_core::RuntimeType for IBrushFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IBrushFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IBrushFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IBrushFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait IBrushFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<Brush>;
}
#[cfg(feature = "UI_Composition")]
impl IBrushFactory_Vtbl {
    pub const fn new<Identity: IBrushFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IBrushFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBrushFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBrushFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBrushFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IBrushOverrides, IBrushOverrides_Vtbl, 0xb6b08394_bacf_53db_9ac7_be1c693e3513);
impl windows_core::RuntimeType for IBrushOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IBrushOverrides");
}
windows_core::imp::interface_hierarchy!(IBrushOverrides, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IBrushOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IBrushOverrides";
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IBrushStatics, IBrushStatics_Vtbl, 0x5b854f50_f818_5f01_91b0_28132d3f5957);
impl windows_core::RuntimeType for IBrushStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IBrushStatics");
}
impl windows_core::RuntimeName for IBrushStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IBrushStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IBrushStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDesktopAcrylicBackdrop, IDesktopAcrylicBackdrop_Vtbl, 0xbfd9915b_82a6_5df6_aff0_a4824ddc1143);
impl windows_core::RuntimeType for IDesktopAcrylicBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IDesktopAcrylicBackdrop");
}
impl windows_core::RuntimeName for IDesktopAcrylicBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IDesktopAcrylicBackdrop";
}
pub trait IDesktopAcrylicBackdrop_Impl: windows_core::IUnknownImpl {}
impl IDesktopAcrylicBackdrop_Vtbl {
    pub const fn new<Identity: IDesktopAcrylicBackdrop_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDesktopAcrylicBackdrop, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopAcrylicBackdrop as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IDesktopAcrylicBackdropFactory, IDesktopAcrylicBackdropFactory_Vtbl, 0x00922e6d_ae51_564a_bce2_1973d5e463dd);
impl windows_core::RuntimeType for IDesktopAcrylicBackdropFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IDesktopAcrylicBackdropFactory");
}
impl windows_core::RuntimeName for IDesktopAcrylicBackdropFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IDesktopAcrylicBackdropFactory";
}
pub trait IDesktopAcrylicBackdropFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<DesktopAcrylicBackdrop>;
}
impl IDesktopAcrylicBackdropFactory_Vtbl {
    pub const fn new<Identity: IDesktopAcrylicBackdropFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IDesktopAcrylicBackdropFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDesktopAcrylicBackdropFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDesktopAcrylicBackdropFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDesktopAcrylicBackdropFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopAcrylicBackdropFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFontFamily, IFontFamily_Vtbl, 0x18fa5bc1_7294_527c_bb02_b213e0b3a2a3);
impl windows_core::RuntimeType for IFontFamily {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IFontFamily");
}
impl windows_core::RuntimeName for IFontFamily {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IFontFamily";
}
pub trait IFontFamily_Impl: windows_core::IUnknownImpl {
    fn Source(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IFontFamily_Vtbl {
    pub const fn new<Identity: IFontFamily_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Source<Identity: IFontFamily_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFontFamily_Impl::Source(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFontFamily, OFFSET>(), Source: Source::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontFamily as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamily_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFontFamilyFactory, IFontFamilyFactory_Vtbl, 0x61b88a77_d0f9_5e9e_8c28_eda01fede22e);
impl windows_core::RuntimeType for IFontFamilyFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IFontFamilyFactory");
}
impl windows_core::RuntimeName for IFontFamilyFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IFontFamilyFactory";
}
pub trait IFontFamilyFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstanceWithName(&self, familyName: &windows_core::HSTRING, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<FontFamily>;
}
impl IFontFamilyFactory_Vtbl {
    pub const fn new<Identity: IFontFamilyFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstanceWithName<Identity: IFontFamilyFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, familyname: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFontFamilyFactory_Impl::CreateInstanceWithName(this, core::mem::transmute(&familyname), core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFontFamilyFactory, OFFSET>(),
            CreateInstanceWithName: CreateInstanceWithName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontFamilyFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFontFamilyStatics, IFontFamilyStatics_Vtbl, 0xb3eadceb_c471_58fe_93d0_d71b04a7fd54);
impl windows_core::RuntimeType for IFontFamilyStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IFontFamilyStatics");
}
impl windows_core::RuntimeName for IFontFamilyStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IFontFamilyStatics";
}
pub trait IFontFamilyStatics_Impl: windows_core::IUnknownImpl {
    fn XamlAutoFontFamily(&self) -> windows_core::Result<FontFamily>;
}
impl IFontFamilyStatics_Vtbl {
    pub const fn new<Identity: IFontFamilyStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn XamlAutoFontFamily<Identity: IFontFamilyStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFontFamilyStatics_Impl::XamlAutoFontFamily(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFontFamilyStatics, OFFSET>(),
            XamlAutoFontFamily: XamlAutoFontFamily::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFontFamilyStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFontFamilyStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XamlAutoFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeometry, IGeometry_Vtbl, 0xdc102dcc_3be2_5414_8599_94b6e76ef39b);
impl windows_core::RuntimeType for IGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IGeometry");
}
impl windows_core::RuntimeName for IGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IGeometry";
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Transform: usize,
    SetTransform: usize,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGeometryFactory, IGeometryFactory_Vtbl, 0x4edcd536_7949_548a_a9b1_6ff03b951cf3);
impl windows_core::RuntimeType for IGeometryFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IGeometryFactory");
}
impl windows_core::RuntimeName for IGeometryFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IGeometryFactory";
}
pub trait IGeometryFactory_Impl: windows_core::IUnknownImpl {}
impl IGeometryFactory_Vtbl {
    pub const fn new<Identity: IGeometryFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IGeometryFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGeometryFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IGeometryStatics, IGeometryStatics_Vtbl, 0x349f78d0_4978_5742_b7d2_b34ea2c95600);
impl windows_core::RuntimeType for IGeometryStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IGeometryStatics");
}
impl windows_core::RuntimeName for IGeometryStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IGeometryStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometryStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Empty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StandardFlatteningTolerance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IImageSource, IImageSource_Vtbl, 0x6c2038f6_d6d5_55e9_9b9e_082f12dbff60);
impl windows_core::RuntimeType for IImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IImageSource");
}
impl windows_core::RuntimeName for IImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IImageSource";
}
pub trait IImageSource_Impl: windows_core::IUnknownImpl {}
impl IImageSource_Vtbl {
    pub const fn new<Identity: IImageSource_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IImageSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IImageSourceFactory, IImageSourceFactory_Vtbl, 0x0b1e64a3_e353_5901_b84b_ae9842aea5cd);
impl windows_core::RuntimeType for IImageSourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IImageSourceFactory");
}
impl windows_core::RuntimeName for IImageSourceFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IImageSourceFactory";
}
pub trait IImageSourceFactory_Impl: windows_core::IUnknownImpl {}
impl IImageSourceFactory_Vtbl {
    pub const fn new<Identity: IImageSourceFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IImageSourceFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageSourceFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageSourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ILineSegment, ILineSegment_Vtbl, 0x0c618e54_d883_588c_8875_bd8dfd6a6a3e);
impl windows_core::RuntimeType for ILineSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.ILineSegment");
}
impl windows_core::RuntimeName for ILineSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ILineSegment";
}
pub trait ILineSegment_Impl: windows_core::IUnknownImpl {
    fn Point(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn SetPoint(&self, value: &windows::Foundation::Point) -> windows_core::Result<()>;
}
impl ILineSegment_Vtbl {
    pub const fn new<Identity: ILineSegment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Point<Identity: ILineSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILineSegment_Impl::Point(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPoint<Identity: ILineSegment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILineSegment_Impl::SetPoint(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILineSegment, OFFSET>(),
            Point: Point::<Identity, OFFSET>,
            SetPoint: SetPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILineSegment as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILineSegmentStatics, ILineSegmentStatics_Vtbl, 0xc3ec48a9_b9c0_561f_9925_d1d1b2a6bae6);
impl windows_core::RuntimeType for ILineSegmentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.ILineSegmentStatics");
}
impl windows_core::RuntimeName for ILineSegmentStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ILineSegmentStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct ILineSegmentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMicaBackdrop, IMicaBackdrop_Vtbl, 0xc156a404_3dac_593a_b1f3_7a33c289dc83);
impl windows_core::RuntimeType for IMicaBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IMicaBackdrop");
}
#[cfg(feature = "UI_Composition_SystemBackdrops")]
impl windows_core::RuntimeName for IMicaBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IMicaBackdrop";
}
#[cfg(feature = "UI_Composition_SystemBackdrops")]
pub trait IMicaBackdrop_Impl: windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<super::super::Composition::SystemBackdrops::MicaKind>;
    fn SetKind(&self, value: super::super::Composition::SystemBackdrops::MicaKind) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition_SystemBackdrops")]
impl IMicaBackdrop_Vtbl {
    pub const fn new<Identity: IMicaBackdrop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Kind<Identity: IMicaBackdrop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Composition::SystemBackdrops::MicaKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaBackdrop_Impl::Kind(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKind<Identity: IMicaBackdrop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Composition::SystemBackdrops::MicaKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMicaBackdrop_Impl::SetKind(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMicaBackdrop, OFFSET>(),
            Kind: Kind::<Identity, OFFSET>,
            SetKind: SetKind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMicaBackdrop as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Composition::SystemBackdrops::MicaKind) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition_SystemBackdrops"))]
    Kind: usize,
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Composition::SystemBackdrops::MicaKind) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition_SystemBackdrops"))]
    SetKind: usize,
}
windows_core::imp::define_interface!(IMicaBackdropFactory, IMicaBackdropFactory_Vtbl, 0x774379ce_74bd_59d4_849d_d99c4184d838);
impl windows_core::RuntimeType for IMicaBackdropFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IMicaBackdropFactory");
}
impl windows_core::RuntimeName for IMicaBackdropFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IMicaBackdropFactory";
}
pub trait IMicaBackdropFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<MicaBackdrop>;
}
impl IMicaBackdropFactory_Vtbl {
    pub const fn new<Identity: IMicaBackdropFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IMicaBackdropFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMicaBackdropFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMicaBackdropFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMicaBackdropFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdropFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMicaBackdropStatics, IMicaBackdropStatics_Vtbl, 0xa63abdce_c796_5509_9f4d_072bc1e599f1);
impl windows_core::RuntimeType for IMicaBackdropStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IMicaBackdropStatics");
}
impl windows_core::RuntimeName for IMicaBackdropStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IMicaBackdropStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMicaBackdropStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPathFigure, IPathFigure_Vtbl, 0x0ee00712_bf65_5f27_9c06_14abdf6656d7);
impl windows_core::RuntimeType for IPathFigure {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IPathFigure");
}
impl windows_core::RuntimeName for IPathFigure {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IPathFigure";
}
pub trait IPathFigure_Impl: windows_core::IUnknownImpl {
    fn Segments(&self) -> windows_core::Result<PathSegmentCollection>;
    fn SetSegments(&self, value: windows_core::Ref<PathSegmentCollection>) -> windows_core::Result<()>;
    fn StartPoint(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn SetStartPoint(&self, value: &windows::Foundation::Point) -> windows_core::Result<()>;
    fn IsClosed(&self) -> windows_core::Result<bool>;
    fn SetIsClosed(&self, value: bool) -> windows_core::Result<()>;
    fn IsFilled(&self) -> windows_core::Result<bool>;
    fn SetIsFilled(&self, value: bool) -> windows_core::Result<()>;
}
impl IPathFigure_Vtbl {
    pub const fn new<Identity: IPathFigure_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Segments<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPathFigure_Impl::Segments(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSegments<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPathFigure_Impl::SetSegments(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartPoint<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPathFigure_Impl::StartPoint(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartPoint<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPathFigure_Impl::SetStartPoint(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn IsClosed<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPathFigure_Impl::IsClosed(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsClosed<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPathFigure_Impl::SetIsClosed(this, value).into()
            }
        }
        unsafe extern "system" fn IsFilled<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPathFigure_Impl::IsFilled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsFilled<Identity: IPathFigure_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPathFigure_Impl::SetIsFilled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPathFigure, OFFSET>(),
            Segments: Segments::<Identity, OFFSET>,
            SetSegments: SetSegments::<Identity, OFFSET>,
            StartPoint: StartPoint::<Identity, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, OFFSET>,
            IsClosed: IsClosed::<Identity, OFFSET>,
            SetIsClosed: SetIsClosed::<Identity, OFFSET>,
            IsFilled: IsFilled::<Identity, OFFSET>,
            SetIsFilled: SetIsFilled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPathFigure as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigure_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Segments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSegments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
    pub IsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsFilled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsFilled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPathFigureStatics, IPathFigureStatics_Vtbl, 0x93bc33c4_879a_5edb_b8d7_7ecb861a7314);
impl windows_core::RuntimeType for IPathFigureStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IPathFigureStatics");
}
impl windows_core::RuntimeName for IPathFigureStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IPathFigureStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFigureStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPathGeometry, IPathGeometry_Vtbl, 0x11b9d95d_d3d9_5337_a05c_73a27a2b5124);
impl windows_core::RuntimeType for IPathGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IPathGeometry");
}
impl windows_core::RuntimeName for IPathGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IPathGeometry";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    FillRule: usize,
    SetFillRule: usize,
    pub Figures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFigures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPathGeometryStatics, IPathGeometryStatics_Vtbl, 0xd7f408fe_6c3a_5cce_91cc_c5a95d4b345a);
impl windows_core::RuntimeType for IPathGeometryStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IPathGeometryStatics");
}
impl windows_core::RuntimeName for IPathGeometryStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IPathGeometryStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathGeometryStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPathSegment, IPathSegment_Vtbl, 0xb922ebbe_08f0_57e9_8785_7e57097f3bd4);
impl windows_core::RuntimeType for IPathSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IPathSegment");
}
impl windows_core::RuntimeName for IPathSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IPathSegment";
}
pub trait IPathSegment_Impl: windows_core::IUnknownImpl {}
impl IPathSegment_Vtbl {
    pub const fn new<Identity: IPathSegment_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPathSegment, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPathSegment as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPathSegmentFactory, IPathSegmentFactory_Vtbl, 0x0559a4ff_ac4b_5bb7_b541_d373960e083b);
impl windows_core::RuntimeType for IPathSegmentFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IPathSegmentFactory");
}
impl windows_core::RuntimeName for IPathSegmentFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IPathSegmentFactory";
}
pub trait IPathSegmentFactory_Impl: windows_core::IUnknownImpl {}
impl IPathSegmentFactory_Vtbl {
    pub const fn new<Identity: IPathSegmentFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPathSegmentFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPathSegmentFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathSegmentFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IRectangleGeometry, IRectangleGeometry_Vtbl, 0xb6143890_a5f5_54e0_ab42_d88bab451f04);
impl windows_core::RuntimeType for IRectangleGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IRectangleGeometry");
}
impl windows_core::RuntimeName for IRectangleGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IRectangleGeometry";
}
pub trait IRectangleGeometry_Impl: windows_core::IUnknownImpl {
    fn Rect(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn SetRect(&self, value: &windows::Foundation::Rect) -> windows_core::Result<()>;
}
impl IRectangleGeometry_Vtbl {
    pub const fn new<Identity: IRectangleGeometry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Rect<Identity: IRectangleGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRectangleGeometry_Impl::Rect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRect<Identity: IRectangleGeometry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRectangleGeometry_Impl::SetRect(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRectangleGeometry, OFFSET>(),
            Rect: Rect::<Identity, OFFSET>,
            SetRect: SetRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRectangleGeometry as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometry_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Rect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub SetRect: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRectangleGeometryStatics, IRectangleGeometryStatics_Vtbl, 0x1ae7ac26_8a8b_55a5_b035_586e2b642919);
impl windows_core::RuntimeType for IRectangleGeometryStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IRectangleGeometryStatics");
}
impl windows_core::RuntimeName for IRectangleGeometryStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IRectangleGeometryStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IRectangleGeometryStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISystemBackdrop, ISystemBackdrop_Vtbl, 0x5aeed5c4_37ac_5852_b73f_1b76ebc3205f);
impl windows_core::RuntimeType for ISystemBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.ISystemBackdrop");
}
#[cfg(feature = "UI_Composition_SystemBackdrops")]
impl windows_core::RuntimeName for ISystemBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ISystemBackdrop";
}
#[cfg(feature = "UI_Composition_SystemBackdrops")]
pub trait ISystemBackdrop_Impl: windows_core::IUnknownImpl {
    fn GetDefaultSystemBackdropConfiguration(&self, target: windows_core::Ref<super::super::Composition::ICompositionSupportsSystemBackdrop>, xamlRoot: windows_core::Ref<super::XamlRoot>) -> windows_core::Result<super::super::Composition::SystemBackdrops::SystemBackdropConfiguration>;
}
#[cfg(feature = "UI_Composition_SystemBackdrops")]
impl ISystemBackdrop_Vtbl {
    pub const fn new<Identity: ISystemBackdrop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDefaultSystemBackdropConfiguration<Identity: ISystemBackdrop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void, xamlroot: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdrop_Impl::GetDefaultSystemBackdropConfiguration(this, core::mem::transmute_copy(&target), core::mem::transmute_copy(&xamlroot)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISystemBackdrop, OFFSET>(),
            GetDefaultSystemBackdropConfiguration: GetDefaultSystemBackdropConfiguration::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemBackdrop as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub GetDefaultSystemBackdropConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition_SystemBackdrops"))]
    GetDefaultSystemBackdropConfiguration: usize,
}
windows_core::imp::define_interface!(ISystemBackdropFactory, ISystemBackdropFactory_Vtbl, 0x1e07656b_fad2_5b29_913f_b6748bc45942);
impl windows_core::RuntimeType for ISystemBackdropFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.ISystemBackdropFactory");
}
impl windows_core::RuntimeName for ISystemBackdropFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ISystemBackdropFactory";
}
pub trait ISystemBackdropFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<SystemBackdrop>;
}
impl ISystemBackdropFactory_Vtbl {
    pub const fn new<Identity: ISystemBackdropFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ISystemBackdropFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISystemBackdropFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISystemBackdropFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemBackdropFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISystemBackdropOverrides, ISystemBackdropOverrides_Vtbl, 0xeb1f5399_cad7_5611_b637_09d76a07e708);
impl windows_core::RuntimeType for ISystemBackdropOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.ISystemBackdropOverrides");
}
windows_core::imp::interface_hierarchy!(ISystemBackdropOverrides, windows_core::IUnknown, windows_core::IInspectable);
impl ISystemBackdropOverrides {
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetConnected<P0, P1>(&self, connectedtarget: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnTargetConnected)(windows_core::Interface::as_raw(self), connectedtarget.param().abi(), xamlroot.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetDisconnected<P0>(&self, disconnectedtarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnTargetDisconnected)(windows_core::Interface::as_raw(self), disconnectedtarget.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnDefaultSystemBackdropConfigurationChanged<P0, P1>(&self, target: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDefaultSystemBackdropConfigurationChanged)(windows_core::Interface::as_raw(self), target.param().abi(), xamlroot.param().abi()).ok() }
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ISystemBackdropOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ISystemBackdropOverrides";
}
#[cfg(feature = "UI_Composition")]
pub trait ISystemBackdropOverrides_Impl: windows_core::IUnknownImpl {
    fn OnTargetConnected(&self, connectedTarget: windows_core::Ref<super::super::Composition::ICompositionSupportsSystemBackdrop>, xamlRoot: windows_core::Ref<super::XamlRoot>) -> windows_core::Result<()>;
    fn OnTargetDisconnected(&self, disconnectedTarget: windows_core::Ref<super::super::Composition::ICompositionSupportsSystemBackdrop>) -> windows_core::Result<()>;
    fn OnDefaultSystemBackdropConfigurationChanged(&self, target: windows_core::Ref<super::super::Composition::ICompositionSupportsSystemBackdrop>, xamlRoot: windows_core::Ref<super::XamlRoot>) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition")]
impl ISystemBackdropOverrides_Vtbl {
    pub const fn new<Identity: ISystemBackdropOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnTargetConnected<Identity: ISystemBackdropOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectedtarget: *mut core::ffi::c_void, xamlroot: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemBackdropOverrides_Impl::OnTargetConnected(this, core::mem::transmute_copy(&connectedtarget), core::mem::transmute_copy(&xamlroot)).into()
            }
        }
        unsafe extern "system" fn OnTargetDisconnected<Identity: ISystemBackdropOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, disconnectedtarget: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemBackdropOverrides_Impl::OnTargetDisconnected(this, core::mem::transmute_copy(&disconnectedtarget)).into()
            }
        }
        unsafe extern "system" fn OnDefaultSystemBackdropConfigurationChanged<Identity: ISystemBackdropOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: *mut core::ffi::c_void, xamlroot: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISystemBackdropOverrides_Impl::OnDefaultSystemBackdropConfigurationChanged(this, core::mem::transmute_copy(&target), core::mem::transmute_copy(&xamlroot)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISystemBackdropOverrides, OFFSET>(),
            OnTargetConnected: OnTargetConnected::<Identity, OFFSET>,
            OnTargetDisconnected: OnTargetDisconnected::<Identity, OFFSET>,
            OnDefaultSystemBackdropConfigurationChanged: OnDefaultSystemBackdropConfigurationChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISystemBackdropOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemBackdropOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub OnTargetConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    OnTargetConnected: usize,
    #[cfg(feature = "UI_Composition")]
    pub OnTargetDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    OnTargetDisconnected: usize,
    #[cfg(feature = "UI_Composition")]
    pub OnDefaultSystemBackdropConfigurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    OnDefaultSystemBackdropConfigurationChanged: usize,
}
windows_core::imp::define_interface!(IXamlLight, IXamlLight_Vtbl, 0xdcd20139_8cd5_5da5_a25c_2b7b813d8d58);
impl windows_core::RuntimeType for IXamlLight {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IXamlLight");
}
impl windows_core::RuntimeName for IXamlLight {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlLight";
}
pub trait IXamlLight_Impl: windows_core::IUnknownImpl {}
impl IXamlLight_Vtbl {
    pub const fn new<Identity: IXamlLight_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlLight, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlLight as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLight_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXamlLightFactory, IXamlLightFactory_Vtbl, 0x76da6306_96fc_553e_bb39_9a4801d06f48);
impl windows_core::RuntimeType for IXamlLightFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IXamlLightFactory");
}
impl windows_core::RuntimeName for IXamlLightFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlLightFactory";
}
pub trait IXamlLightFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<XamlLight>;
}
impl IXamlLightFactory_Vtbl {
    pub const fn new<Identity: IXamlLightFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IXamlLightFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlLightFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlLightFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlLightFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXamlLightOverrides, IXamlLightOverrides_Vtbl, 0x696d4f30_92ee_540d_ad70_33d4489514d0);
impl windows_core::RuntimeType for IXamlLightOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IXamlLightOverrides");
}
windows_core::imp::interface_hierarchy!(IXamlLightOverrides, windows_core::IUnknown, windows_core::IInspectable);
impl IXamlLightOverrides {
    pub fn GetId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnConnected<P0>(&self, newelement: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnected)(windows_core::Interface::as_raw(self), newelement.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnDisconnected<P0>(&self, oldelement: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDisconnected)(windows_core::Interface::as_raw(self), oldelement.param().abi()).ok() }
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IXamlLightOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlLightOverrides";
}
#[cfg(feature = "UI_Composition")]
pub trait IXamlLightOverrides_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn OnConnected(&self, newElement: windows_core::Ref<super::UIElement>) -> windows_core::Result<()>;
    fn OnDisconnected(&self, oldElement: windows_core::Ref<super::UIElement>) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition")]
impl IXamlLightOverrides_Vtbl {
    pub const fn new<Identity: IXamlLightOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlLightOverrides_Impl::GetId(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnConnected<Identity: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlLightOverrides_Impl::OnConnected(this, core::mem::transmute_copy(&newelement)).into()
            }
        }
        unsafe extern "system" fn OnDisconnected<Identity: IXamlLightOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldelement: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlLightOverrides_Impl::OnDisconnected(this, core::mem::transmute_copy(&oldelement)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlLightOverrides, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            OnConnected: OnConnected::<Identity, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlLightOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub OnConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    OnConnected: usize,
    #[cfg(feature = "UI_Composition")]
    pub OnDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    OnDisconnected: usize,
}
windows_core::imp::define_interface!(IXamlLightProtected, IXamlLightProtected_Vtbl, 0xc307bf12_fdaf_54ca_a631_ad0e86263c6e);
impl windows_core::RuntimeType for IXamlLightProtected {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IXamlLightProtected");
}
impl windows_core::RuntimeName for IXamlLightProtected {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlLightProtected";
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightProtected_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXamlLightStatics, IXamlLightStatics_Vtbl, 0xa2d8ea26_26ff_5374_b1dd_f232d5604f6a);
impl windows_core::RuntimeType for IXamlLightStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.IXamlLightStatics");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IXamlLightStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.IXamlLightStatics";
}
#[cfg(feature = "UI_Composition")]
pub trait IXamlLightStatics_Impl: windows_core::IUnknownImpl {
    fn AddTargetElement(&self, lightId: &windows_core::HSTRING, element: windows_core::Ref<super::UIElement>) -> windows_core::Result<()>;
    fn RemoveTargetElement(&self, lightId: &windows_core::HSTRING, element: windows_core::Ref<super::UIElement>) -> windows_core::Result<()>;
    fn AddTargetBrush(&self, lightId: &windows_core::HSTRING, brush: windows_core::Ref<Brush>) -> windows_core::Result<()>;
    fn RemoveTargetBrush(&self, lightId: &windows_core::HSTRING, brush: windows_core::Ref<Brush>) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition")]
impl IXamlLightStatics_Vtbl {
    pub const fn new<Identity: IXamlLightStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTargetElement<Identity: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lightid: *mut core::ffi::c_void, element: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlLightStatics_Impl::AddTargetElement(this, core::mem::transmute(&lightid), core::mem::transmute_copy(&element)).into()
            }
        }
        unsafe extern "system" fn RemoveTargetElement<Identity: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lightid: *mut core::ffi::c_void, element: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlLightStatics_Impl::RemoveTargetElement(this, core::mem::transmute(&lightid), core::mem::transmute_copy(&element)).into()
            }
        }
        unsafe extern "system" fn AddTargetBrush<Identity: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lightid: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlLightStatics_Impl::AddTargetBrush(this, core::mem::transmute(&lightid), core::mem::transmute_copy(&brush)).into()
            }
        }
        unsafe extern "system" fn RemoveTargetBrush<Identity: IXamlLightStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lightid: *mut core::ffi::c_void, brush: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlLightStatics_Impl::RemoveTargetBrush(this, core::mem::transmute(&lightid), core::mem::transmute_copy(&brush)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlLightStatics, OFFSET>(),
            AddTargetElement: AddTargetElement::<Identity, OFFSET>,
            RemoveTargetElement: RemoveTargetElement::<Identity, OFFSET>,
            AddTargetBrush: AddTargetBrush::<Identity, OFFSET>,
            RemoveTargetBrush: RemoveTargetBrush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlLightStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlLightStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub AddTargetElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    AddTargetElement: usize,
    #[cfg(feature = "UI_Composition")]
    pub RemoveTargetElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    RemoveTargetElement: usize,
    #[cfg(feature = "UI_Composition")]
    pub AddTargetBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    AddTargetBrush: usize,
    #[cfg(feature = "UI_Composition")]
    pub RemoveTargetBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    RemoveTargetBrush: usize,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ImageSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ImageSource, super::DependencyObject);
impl ImageSource {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ImageSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IImageSource>();
}
unsafe impl windows_core::Interface for ImageSource {
    type Vtable = <IImageSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IImageSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ImageSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.ImageSource";
}
unsafe impl Send for ImageSource {}
unsafe impl Sync for ImageSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LineSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LineSegment, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LineSegment, PathSegment, super::DependencyObject);
impl LineSegment {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LineSegment, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Point(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Point)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPoint(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPoint)(windows_core::Interface::as_raw(self), value).ok() }
    }
    fn ILineSegmentStatics<R, F: FnOnce(&ILineSegmentStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<LineSegment, ILineSegmentStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for LineSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILineSegment>();
}
unsafe impl windows_core::Interface for LineSegment {
    type Vtable = <ILineSegment as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILineSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LineSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.LineSegment";
}
unsafe impl Send for LineSegment {}
unsafe impl Sync for LineSegment {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MicaBackdrop(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MicaBackdrop, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MicaBackdrop, SystemBackdrop, super::DependencyObject);
impl MicaBackdrop {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub fn Kind(&self) -> windows_core::Result<super::super::Composition::SystemBackdrops::MicaKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Kind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub fn SetKind(&self, value: super::super::Composition::SystemBackdrops::MicaKind) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKind)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IMicaBackdropFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IMicaBackdropFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub fn GetDefaultSystemBackdropConfiguration<P0, P1>(&self, target: P0, xamlroot: P1) -> windows_core::Result<super::super::Composition::SystemBackdrops::SystemBackdropConfiguration>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdrop>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetDefaultSystemBackdropConfiguration)(windows_core::Interface::as_raw(this), target.param().abi(), xamlroot.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetConnected<P0, P1>(&self, connectedtarget: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTargetConnected)(windows_core::Interface::as_raw(this), connectedtarget.param().abi(), xamlroot.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetDisconnected<P0>(&self, disconnectedtarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTargetDisconnected)(windows_core::Interface::as_raw(this), disconnectedtarget.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnDefaultSystemBackdropConfigurationChanged<P0, P1>(&self, target: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDefaultSystemBackdropConfigurationChanged)(windows_core::Interface::as_raw(this), target.param().abi(), xamlroot.param().abi()).ok() }
    }
    fn IMicaBackdropFactory<R, F: FnOnce(&IMicaBackdropFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MicaBackdrop, IMicaBackdropFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IMicaBackdropStatics<R, F: FnOnce(&IMicaBackdropStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MicaBackdrop, IMicaBackdropStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for MicaBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMicaBackdrop>();
}
unsafe impl windows_core::Interface for MicaBackdrop {
    type Vtable = <IMicaBackdrop as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMicaBackdrop as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MicaBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.MicaBackdrop";
}
unsafe impl Send for MicaBackdrop {}
unsafe impl Sync for MicaBackdrop {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathFigure(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PathFigure, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PathFigure, super::DependencyObject);
impl PathFigure {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PathFigure, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Segments(&self) -> windows_core::Result<PathSegmentCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Segments)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSegments<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PathSegmentCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSegments)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn StartPoint(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetStartPoint(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartPoint)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsClosed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsClosed(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsClosed)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsFilled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFilled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFilled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsFilled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    fn IPathFigureStatics<R, F: FnOnce(&IPathFigureStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PathFigure, IPathFigureStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PathFigure {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPathFigure>();
}
unsafe impl windows_core::Interface for PathFigure {
    type Vtable = <IPathFigure as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPathFigure as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PathFigure {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathFigure";
}
unsafe impl Send for PathFigure {}
unsafe impl Sync for PathFigure {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathFigureCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PathFigureCollection, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IVector<PathFigure>);
windows_core::imp::required_hierarchy!(PathFigureCollection, windows_collections::IIterable<PathFigure>);
impl PathFigureCollection {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PathFigureCollection, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<PathFigure>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<PathFigure>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<PathFigure> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<PathFigure>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<PathFigure>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IndexOf)(windows_core::Interface::as_raw(self), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<PathFigure>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<PathFigure>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PathFigure>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAtEnd)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<PathFigure>]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMany)(windows_core::Interface::as_raw(self), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<PathFigure>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReplaceAll)(windows_core::Interface::as_raw(self), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for PathFigureCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<PathFigure>>();
}
unsafe impl windows_core::Interface for PathFigureCollection {
    type Vtable = <windows_collections::IVector<PathFigure> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IVector<PathFigure> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PathFigureCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathFigureCollection";
}
unsafe impl Send for PathFigureCollection {}
unsafe impl Sync for PathFigureCollection {}
impl IntoIterator for PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PathFigureCollection {
    type Item = PathFigure;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        windows_collections::BufferedIterator::new(self.First().unwrap())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathGeometry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PathGeometry, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PathGeometry, Geometry, super::DependencyObject);
impl PathGeometry {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PathGeometry, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Bounds(&self) -> windows_core::Result<windows::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Figures(&self) -> windows_core::Result<PathFigureCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Figures)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetFigures<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PathFigureCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFigures)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    fn IPathGeometryStatics<R, F: FnOnce(&IPathGeometryStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PathGeometry, IPathGeometryStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PathGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPathGeometry>();
}
unsafe impl windows_core::Interface for PathGeometry {
    type Vtable = <IPathGeometry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPathGeometry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PathGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathGeometry";
}
unsafe impl Send for PathGeometry {}
unsafe impl Sync for PathGeometry {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathSegment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PathSegment, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PathSegment, super::DependencyObject);
impl PathSegment {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PathSegment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPathSegment>();
}
unsafe impl windows_core::Interface for PathSegment {
    type Vtable = <IPathSegment as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPathSegment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PathSegment {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathSegment";
}
unsafe impl Send for PathSegment {}
unsafe impl Sync for PathSegment {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathSegmentCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PathSegmentCollection, windows_core::IUnknown, windows_core::IInspectable, windows_collections::IVector<PathSegment>);
windows_core::imp::required_hierarchy!(PathSegmentCollection, windows_collections::IIterable<PathSegment>);
impl PathSegmentCollection {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PathSegmentCollection, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<PathSegment>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<PathSegment>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetAt(&self, index: u32) -> windows_core::Result<PathSegment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IVectorView<PathSegment>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<PathSegment>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IndexOf)(windows_core::Interface::as_raw(self), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    pub fn SetAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<PathSegment>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok() }
    }
    pub fn InsertAt<P1>(&self, index: u32, value: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<PathSegment>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAt)(windows_core::Interface::as_raw(self), index, value.param().abi()).ok() }
    }
    pub fn RemoveAt(&self, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAt)(windows_core::Interface::as_raw(self), index).ok() }
    }
    pub fn Append<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PathSegment>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn RemoveAtEnd(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAtEnd)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<PathSegment>]) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMany)(windows_core::Interface::as_raw(self), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn ReplaceAll(&self, items: &[Option<PathSegment>]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReplaceAll)(windows_core::Interface::as_raw(self), items.len().try_into().unwrap(), core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl windows_core::RuntimeType for PathSegmentCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, windows_collections::IVector<PathSegment>>();
}
unsafe impl windows_core::Interface for PathSegmentCollection {
    type Vtable = <windows_collections::IVector<PathSegment> as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <windows_collections::IVector<PathSegment> as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PathSegmentCollection {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.PathSegmentCollection";
}
unsafe impl Send for PathSegmentCollection {}
unsafe impl Sync for PathSegmentCollection {}
impl IntoIterator for PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &PathSegmentCollection {
    type Item = PathSegment;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        windows_collections::BufferedIterator::new(self.First().unwrap())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RectangleGeometry(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RectangleGeometry, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RectangleGeometry, Geometry, super::DependencyObject);
impl RectangleGeometry {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RectangleGeometry, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Bounds(&self) -> windows_core::Result<windows::Foundation::Rect> {
        let this = &windows_core::Interface::cast::<IGeometry>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Bounds)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Rect(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRect(&self, value: windows::Foundation::Rect) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRect)(windows_core::Interface::as_raw(self), value).ok() }
    }
    fn IRectangleGeometryStatics<R, F: FnOnce(&IRectangleGeometryStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RectangleGeometry, IRectangleGeometryStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RectangleGeometry {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRectangleGeometry>();
}
unsafe impl windows_core::Interface for RectangleGeometry {
    type Vtable = <IRectangleGeometry as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRectangleGeometry as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RectangleGeometry {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.RectangleGeometry";
}
unsafe impl Send for RectangleGeometry {}
unsafe impl Sync for RectangleGeometry {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Stretch(pub i32);
impl Stretch {
    pub const None: Self = Self(0);
    pub const Fill: Self = Self(1);
    pub const Uniform: Self = Self(2);
    pub const UniformToFill: Self = Self(3);
}
impl windows_core::imp::TypeKind for Stretch {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Stretch {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Media.Stretch;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.Stretch");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SweepDirection(pub i32);
impl SweepDirection {
    pub const Counterclockwise: Self = Self(0);
    pub const Clockwise: Self = Self(1);
}
impl windows_core::imp::TypeKind for SweepDirection {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for SweepDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Media.SweepDirection;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.SweepDirection");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemBackdrop(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SystemBackdrop, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SystemBackdrop, super::DependencyObject);
impl SystemBackdrop {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition_SystemBackdrops")]
    pub fn GetDefaultSystemBackdropConfiguration<P0, P1>(&self, target: P0, xamlroot: P1) -> windows_core::Result<super::super::Composition::SystemBackdrops::SystemBackdropConfiguration>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultSystemBackdropConfiguration)(windows_core::Interface::as_raw(self), target.param().abi(), xamlroot.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::ISystemBackdropFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::ISystemBackdropFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetConnected<P0, P1>(&self, connectedtarget: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTargetConnected)(windows_core::Interface::as_raw(this), connectedtarget.param().abi(), xamlroot.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnTargetDisconnected<P0>(&self, disconnectedtarget: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTargetDisconnected)(windows_core::Interface::as_raw(this), disconnectedtarget.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnDefaultSystemBackdropConfigurationChanged<P0, P1>(&self, target: P0, xamlroot: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Composition::ICompositionSupportsSystemBackdrop>,
        P1: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<ISystemBackdropOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDefaultSystemBackdropConfigurationChanged)(windows_core::Interface::as_raw(this), target.param().abi(), xamlroot.param().abi()).ok() }
    }
    fn ISystemBackdropFactory<R, F: FnOnce(&ISystemBackdropFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SystemBackdrop, ISystemBackdropFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SystemBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISystemBackdrop>();
}
unsafe impl windows_core::Interface for SystemBackdrop {
    type Vtable = <ISystemBackdrop as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISystemBackdrop as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SystemBackdrop {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.SystemBackdrop";
}
unsafe impl Send for SystemBackdrop {}
unsafe impl Sync for SystemBackdrop {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlLight(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XamlLight, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XamlLight, super::DependencyObject);
impl XamlLight {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IXamlLightFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IXamlLightFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn GetId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXamlLightOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnConnected<P0>(&self, newelement: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        let this = &windows_core::Interface::cast::<IXamlLightOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnConnected)(windows_core::Interface::as_raw(this), newelement.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn OnDisconnected<P0>(&self, oldelement: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        let this = &windows_core::Interface::cast::<IXamlLightOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnected)(windows_core::Interface::as_raw(this), oldelement.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn AddTargetElement<P1>(lightid: &windows_core::HSTRING, element: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::UIElement>,
    {
        Self::IXamlLightStatics(|this| unsafe { (windows_core::Interface::vtable(this).AddTargetElement)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(lightid), element.param().abi()).ok() })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn RemoveTargetElement<P1>(lightid: &windows_core::HSTRING, element: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::UIElement>,
    {
        Self::IXamlLightStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveTargetElement)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(lightid), element.param().abi()).ok() })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn AddTargetBrush<P1>(lightid: &windows_core::HSTRING, brush: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<Brush>,
    {
        Self::IXamlLightStatics(|this| unsafe { (windows_core::Interface::vtable(this).AddTargetBrush)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(lightid), brush.param().abi()).ok() })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn RemoveTargetBrush<P1>(lightid: &windows_core::HSTRING, brush: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<Brush>,
    {
        Self::IXamlLightStatics(|this| unsafe { (windows_core::Interface::vtable(this).RemoveTargetBrush)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(lightid), brush.param().abi()).ok() })
    }
    fn IXamlLightFactory<R, F: FnOnce(&IXamlLightFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XamlLight, IXamlLightFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IXamlLightStatics<R, F: FnOnce(&IXamlLightStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XamlLight, IXamlLightStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlLight {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXamlLight>();
}
unsafe impl windows_core::Interface for XamlLight {
    type Vtable = <IXamlLight as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlLight as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XamlLight {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.XamlLight";
}
unsafe impl Send for XamlLight {}
unsafe impl Sync for XamlLight {}
