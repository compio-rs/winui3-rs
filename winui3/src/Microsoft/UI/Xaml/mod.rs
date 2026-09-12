#[cfg(feature = "UI_Xaml_Controls")]
pub mod Controls;
#[cfg(feature = "UI_Xaml_Data")]
pub mod Data;
#[cfg(feature = "UI_Xaml_Documents")]
pub mod Documents;
#[cfg(feature = "UI_Xaml_Input")]
pub mod Input;
#[cfg(feature = "UI_Xaml_Interop")]
pub mod Interop;
#[cfg(feature = "UI_Xaml_Markup")]
pub mod Markup;
#[cfg(feature = "UI_Xaml_Media")]
pub mod Media;
#[cfg(feature = "UI_Xaml_XamlTypeInfo")]
pub mod XamlTypeInfo;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Application(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Application, windows_core::IUnknown, windows_core::IInspectable);
impl Application {
    pub fn Resources(&self) -> windows_core::Result<ResourceDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ResourceDictionary>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetResources)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn RequestedTheme(&self) -> windows_core::Result<ApplicationTheme> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedTheme)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRequestedTheme(&self, value: ApplicationTheme) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRequestedTheme)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn FocusVisualKind(&self) -> windows_core::Result<FocusVisualKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusVisualKind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualKind(&self, value: FocusVisualKind) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFocusVisualKind)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn UnhandledException<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<UnhandledExceptionEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <UnhandledExceptionEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).UnhandledException)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveUnhandledException))
        }
    }
    pub fn Exit(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Exit)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn ResourceManagerRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ResourceManagerRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IApplication2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<windows_core::IInspectable, ResourceManagerRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ResourceManagerRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveResourceManagerRequested))
        }
    }
    pub fn DispatcherShutdownMode(&self) -> windows_core::Result<DispatcherShutdownMode> {
        let this = &windows_core::Interface::cast::<IApplication3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherShutdownMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetDispatcherShutdownMode(&self, value: DispatcherShutdownMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IApplication3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDispatcherShutdownMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IApplicationFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IApplicationFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn OnLaunched<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LaunchActivatedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IApplicationOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnLaunched)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Current() -> windows_core::Result<Self> {
        Self::IApplicationStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn Start<P0>(callback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ApplicationInitializationCallback>,
    {
        Self::IApplicationStatics(|this| unsafe { (windows_core::Interface::vtable(this).Start)(windows_core::Interface::as_raw(this), callback.param().abi()).ok() })
    }
    pub fn LoadComponent<P0, P1>(component: P0, resourcelocator: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows::Foundation::Uri>,
    {
        Self::IApplicationStatics(|this| unsafe { (windows_core::Interface::vtable(this).LoadComponent)(windows_core::Interface::as_raw(this), component.param().abi(), resourcelocator.param().abi()).ok() })
    }
    fn IApplicationFactory<R, F: FnOnce(&IApplicationFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Application, IApplicationFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IApplicationStatics<R, F: FnOnce(&IApplicationStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Application, IApplicationStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Application {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplication>();
}
unsafe impl windows_core::Interface for Application {
    type Vtable = <IApplication as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IApplication as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Application {
    const NAME: &'static str = "Microsoft.UI.Xaml.Application";
}
unsafe impl Send for Application {}
unsafe impl Sync for Application {}
windows_core::imp::define_interface!(ApplicationInitializationCallback, ApplicationInitializationCallback_Vtbl, 0xd8eef1c9_1234_56f1_9963_45dd9c80a661);
impl windows_core::RuntimeType for ApplicationInitializationCallback {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ApplicationInitializationCallback {
    pub fn new<F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&ApplicationInitializationCallbackBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0>(&self, p: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ApplicationInitializationCallbackParams>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), p.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ApplicationInitializationCallback_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct ApplicationInitializationCallbackBox<F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<ApplicationInitializationCallbackParams>) -> windows_core::Result<()> + Send + 'static> ApplicationInitializationCallbackBox<F> {
    const VTABLE: ApplicationInitializationCallback_Vtbl = ApplicationInitializationCallback_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ApplicationInitializationCallback, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ApplicationInitializationCallback, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ApplicationInitializationCallback, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<ApplicationInitializationCallback, F>);
            (this.invoke)(core::mem::transmute_copy(&p)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationInitializationCallbackParams(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ApplicationInitializationCallbackParams, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for ApplicationInitializationCallbackParams {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IApplicationInitializationCallbackParams>();
}
unsafe impl windows_core::Interface for ApplicationInitializationCallbackParams {
    type Vtable = <IApplicationInitializationCallbackParams as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IApplicationInitializationCallbackParams as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ApplicationInitializationCallbackParams {
    const NAME: &'static str = "Microsoft.UI.Xaml.ApplicationInitializationCallbackParams";
}
unsafe impl Send for ApplicationInitializationCallbackParams {}
unsafe impl Sync for ApplicationInitializationCallbackParams {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ApplicationTheme(pub i32);
impl ApplicationTheme {
    pub const Light: Self = Self(0);
    pub const Dark: Self = Self(1);
}
impl windows_core::imp::TypeKind for ApplicationTheme {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ApplicationTheme {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.ApplicationTheme;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.ApplicationTheme");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BringIntoViewRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BringIntoViewRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BringIntoViewRequestedEventArgs, RoutedEventArgs);
impl BringIntoViewRequestedEventArgs {
    #[cfg(feature = "UI_Composition")]
    pub fn TargetElement(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetTargetElement<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTargetElement)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn AnimationDesired(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnimationDesired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAnimationDesired(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAnimationDesired)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn TargetRect(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTargetRect(&self, value: windows::Foundation::Rect) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetRect)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn HorizontalAlignmentRatio(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalAlignmentRatio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn VerticalAlignmentRatio(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalAlignmentRatio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn HorizontalOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHorizontalOffset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn VerticalOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVerticalOffset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BringIntoViewRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBringIntoViewRequestedEventArgs>();
}
unsafe impl windows_core::Interface for BringIntoViewRequestedEventArgs {
    type Vtable = <IBringIntoViewRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBringIntoViewRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BringIntoViewRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.BringIntoViewRequestedEventArgs";
}
unsafe impl Send for BringIntoViewRequestedEventArgs {}
unsafe impl Sync for BringIntoViewRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataContextChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataContextChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DataContextChangedEventArgs {
    pub fn NewValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for DataContextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataContextChangedEventArgs>();
}
unsafe impl windows_core::Interface for DataContextChangedEventArgs {
    type Vtable = <IDataContextChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataContextChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataContextChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.DataContextChangedEventArgs";
}
unsafe impl Send for DataContextChangedEventArgs {}
unsafe impl Sync for DataContextChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DependencyObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DependencyObject, windows_core::IUnknown, windows_core::IInspectable);
impl DependencyObject {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dispatcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IDependencyObjectFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IDependencyObjectFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    fn IDependencyObjectFactory<R, F: FnOnce(&IDependencyObjectFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DependencyObject, IDependencyObjectFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDependencyObject>();
}
unsafe impl windows_core::Interface for DependencyObject {
    type Vtable = <IDependencyObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDependencyObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DependencyObject {
    const NAME: &'static str = "Microsoft.UI.Xaml.DependencyObject";
}
unsafe impl Send for DependencyObject {}
unsafe impl Sync for DependencyObject {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DispatcherShutdownMode(pub i32);
impl DispatcherShutdownMode {
    pub const OnLastWindowClose: Self = Self(0);
    pub const OnExplicitShutdown: Self = Self(1);
}
impl windows_core::imp::TypeKind for DispatcherShutdownMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DispatcherShutdownMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.DispatcherShutdownMode;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.DispatcherShutdownMode");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DragStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DragStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DragStartingEventArgs, RoutedEventArgs);
impl DragStartingEventArgs {
    pub fn Cancel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCancel)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Data(&self) -> windows_core::Result<windows::ApplicationModel::DataTransfer::DataPackage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn AllowedOperations(&self) -> windows_core::Result<windows::ApplicationModel::DataTransfer::DataPackageOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowedOperations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowedOperations(&self, value: windows::ApplicationModel::DataTransfer::DataPackageOperation) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowedOperations)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn GetPosition<P0>(&self, relativeto: P0) -> windows_core::Result<windows::Foundation::Point>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), relativeto.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DragStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDragStartingEventArgs>();
}
unsafe impl windows_core::Interface for DragStartingEventArgs {
    type Vtable = <IDragStartingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDragStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DragStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.DragStartingEventArgs";
}
unsafe impl Send for DragStartingEventArgs {}
unsafe impl Sync for DragStartingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DropCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DropCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DropCompletedEventArgs, RoutedEventArgs);
impl DropCompletedEventArgs {
    pub fn DropResult(&self) -> windows_core::Result<windows::ApplicationModel::DataTransfer::DataPackageOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DropResult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DropCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDropCompletedEventArgs>();
}
unsafe impl windows_core::Interface for DropCompletedEventArgs {
    type Vtable = <IDropCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDropCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DropCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.DropCompletedEventArgs";
}
unsafe impl Send for DropCompletedEventArgs {}
unsafe impl Sync for DropCompletedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveViewportChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EffectiveViewportChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EffectiveViewportChangedEventArgs {
    pub fn EffectiveViewport(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EffectiveViewport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MaxViewport(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxViewport)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn BringIntoViewDistanceX(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BringIntoViewDistanceX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn BringIntoViewDistanceY(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BringIntoViewDistanceY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for EffectiveViewportChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEffectiveViewportChangedEventArgs>();
}
unsafe impl windows_core::Interface for EffectiveViewportChangedEventArgs {
    type Vtable = <IEffectiveViewportChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IEffectiveViewportChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EffectiveViewportChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.EffectiveViewportChangedEventArgs";
}
unsafe impl Send for EffectiveViewportChangedEventArgs {}
unsafe impl Sync for EffectiveViewportChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusState(pub i32);
impl FocusState {
    pub const Unfocused: Self = Self(0);
    pub const Pointer: Self = Self(1);
    pub const Keyboard: Self = Self(2);
    pub const Programmatic: Self = Self(3);
}
impl windows_core::imp::TypeKind for FocusState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.FocusState;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.FocusState");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusVisualKind(pub i32);
impl FocusVisualKind {
    pub const DottedLine: Self = Self(0);
    pub const HighVisibility: Self = Self(1);
    pub const Reveal: Self = Self(2);
}
impl windows_core::imp::TypeKind for FocusVisualKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusVisualKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.FocusVisualKind;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.FocusVisualKind");
}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrameworkElement(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(FrameworkElement, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(FrameworkElement, super::Composition::IAnimationObject, super::Composition::IVisualElement, super::Composition::IVisualElement2, UIElement, DependencyObject);
#[cfg(feature = "UI_Composition")]
impl FrameworkElement {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<ResourceDictionary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Resources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ResourceDictionary>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetResources)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tag)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Language)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Width)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetWidth)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Height)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHeight)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinWidth)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxWidth)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MinHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinHeight)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MaxHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxHeight)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<HorizontalAlignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: HorizontalAlignment) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHorizontalAlignment)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<VerticalAlignment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: VerticalAlignment) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVerticalAlignment)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<Thickness> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Margin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMargin)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BaseUri)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DataContext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDataContext)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowFocusOnInteraction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<Thickness> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusVisualMargin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFocusVisualMargin)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<Thickness> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<Thickness> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: Thickness) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<Media::Brush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Media::Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<Media::Brush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Media::Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<Style> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Style)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Style>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStyle)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLoaded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Loaded)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Unloaded)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DataContextChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDataContextChanged))
        }
    }
    pub fn LayoutUpdated<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).LayoutUpdated)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Loading)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ActualThemeChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).EffectiveViewportChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IFrameworkElementFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IFrameworkElementFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DeferTree<P0>(element: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        Self::IFrameworkElementStatics(|this| unsafe { (windows_core::Interface::vtable(this).DeferTree)(windows_core::Interface::as_raw(this), element.param().abi()).ok() })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<Visibility> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<Input::Pointer>> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<XamlRoot> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<FocusState> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<DependencyObject> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<UIElement>, windows_core::Ref<BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<UIElement, BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn IFrameworkElementFactory<R, F: FnOnce(&IFrameworkElementFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IFrameworkElementStatics<R, F: FnOnce(&IFrameworkElementStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FrameworkElement, IFrameworkElementStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for FrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFrameworkElement>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for FrameworkElement {
    type Vtable = <IFrameworkElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFrameworkElement as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for FrameworkElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.FrameworkElement";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for FrameworkElement {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for FrameworkElement {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GridLength {
    pub Value: f64,
    pub GridUnitType: GridUnitType,
}
impl windows_core::imp::TypeKind for GridLength {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for GridLength {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Xaml.GridLength;f8;enum(Microsoft.UI.Xaml.GridUnitType;i4))");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.GridLength");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GridUnitType(pub i32);
impl GridUnitType {
    pub const Auto: Self = Self(0);
    pub const Pixel: Self = Self(1);
    pub const Star: Self = Self(2);
}
impl windows_core::imp::TypeKind for GridUnitType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for GridUnitType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.GridUnitType;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.GridUnitType");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HorizontalAlignment(pub i32);
impl HorizontalAlignment {
    pub const Left: Self = Self(0);
    pub const Center: Self = Self(1);
    pub const Right: Self = Self(2);
    pub const Stretch: Self = Self(3);
}
impl windows_core::imp::TypeKind for HorizontalAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for HorizontalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.HorizontalAlignment;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.HorizontalAlignment");
}
windows_core::imp::define_interface!(IApplication, IApplication_Vtbl, 0x06a8f4e7_1146_55af_820d_ebd55643b021);
impl windows_core::RuntimeType for IApplication {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplication");
}
impl windows_core::RuntimeName for IApplication {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplication";
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplication_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    DebugSettings: usize,
    pub RequestedTheme: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ApplicationTheme) -> windows_core::HRESULT,
    pub SetRequestedTheme: unsafe extern "system" fn(*mut core::ffi::c_void, ApplicationTheme) -> windows_core::HRESULT,
    pub FocusVisualKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusVisualKind) -> windows_core::HRESULT,
    pub SetFocusVisualKind: unsafe extern "system" fn(*mut core::ffi::c_void, FocusVisualKind) -> windows_core::HRESULT,
    HighContrastAdjustment: usize,
    SetHighContrastAdjustment: usize,
    pub UnhandledException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUnhandledException: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Exit: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplication2, IApplication2_Vtbl, 0x469e6d36_2e11_5b06_9e0a_c5eef0cf8f12);
impl windows_core::RuntimeType for IApplication2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplication2");
}
impl windows_core::RuntimeName for IApplication2 {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplication2";
}
pub trait IApplication2_Impl: windows_core::IUnknownImpl {
    fn ResourceManagerRequested(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<windows_core::IInspectable, ResourceManagerRequestedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveResourceManagerRequested(&self, token: i64) -> windows_core::Result<()>;
}
impl IApplication2_Vtbl {
    pub const fn new<Identity: IApplication2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResourceManagerRequested<Identity: IApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApplication2_Impl::ResourceManagerRequested(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveResourceManagerRequested<Identity: IApplication2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplication2_Impl::RemoveResourceManagerRequested(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplication2, OFFSET>(),
            ResourceManagerRequested: ResourceManagerRequested::<Identity, OFFSET>,
            RemoveResourceManagerRequested: RemoveResourceManagerRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplication2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplication2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ResourceManagerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveResourceManagerRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplication3, IApplication3_Vtbl, 0xbe941595_61fe_5b36_a3d3_962a647d7c6f);
impl windows_core::RuntimeType for IApplication3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplication3");
}
impl windows_core::RuntimeName for IApplication3 {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplication3";
}
pub trait IApplication3_Impl: windows_core::IUnknownImpl {
    fn DispatcherShutdownMode(&self) -> windows_core::Result<DispatcherShutdownMode>;
    fn SetDispatcherShutdownMode(&self, value: DispatcherShutdownMode) -> windows_core::Result<()>;
}
impl IApplication3_Vtbl {
    pub const fn new<Identity: IApplication3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DispatcherShutdownMode<Identity: IApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut DispatcherShutdownMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApplication3_Impl::DispatcherShutdownMode(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDispatcherShutdownMode<Identity: IApplication3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: DispatcherShutdownMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplication3_Impl::SetDispatcherShutdownMode(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplication3, OFFSET>(),
            DispatcherShutdownMode: DispatcherShutdownMode::<Identity, OFFSET>,
            SetDispatcherShutdownMode: SetDispatcherShutdownMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplication3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplication3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DispatcherShutdownMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DispatcherShutdownMode) -> windows_core::HRESULT,
    pub SetDispatcherShutdownMode: unsafe extern "system" fn(*mut core::ffi::c_void, DispatcherShutdownMode) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationFactory, IApplicationFactory_Vtbl, 0x9fd96657_5294_5a65_a1db_4fea143597da);
impl windows_core::RuntimeType for IApplicationFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplicationFactory");
}
impl windows_core::RuntimeName for IApplicationFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplicationFactory";
}
pub trait IApplicationFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<Application>;
}
impl IApplicationFactory_Vtbl {
    pub const fn new<Identity: IApplicationFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IApplicationFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IApplicationFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplicationFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationInitializationCallbackParams, IApplicationInitializationCallbackParams_Vtbl, 0x1b1906ea_5b7b_5876_81ab_7c2281ac3d20);
impl windows_core::RuntimeType for IApplicationInitializationCallbackParams {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplicationInitializationCallbackParams");
}
impl windows_core::RuntimeName for IApplicationInitializationCallbackParams {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplicationInitializationCallbackParams";
}
pub trait IApplicationInitializationCallbackParams_Impl: windows_core::IUnknownImpl {}
impl IApplicationInitializationCallbackParams_Vtbl {
    pub const fn new<Identity: IApplicationInitializationCallbackParams_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplicationInitializationCallbackParams, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationInitializationCallbackParams as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationInitializationCallbackParams_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IApplicationOverrides, IApplicationOverrides_Vtbl, 0xa33e81ef_c665_503b_8827_d27ef1720a06);
impl windows_core::RuntimeType for IApplicationOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplicationOverrides");
}
impl IApplicationOverrides {
    pub fn OnLaunched<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<LaunchActivatedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnLaunched)(windows_core::Interface::as_raw(self), args.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplicationOverrides";
}
pub trait IApplicationOverrides_Impl: windows_core::IUnknownImpl {
    fn OnLaunched(&self, args: windows_core::Ref<LaunchActivatedEventArgs>) -> windows_core::Result<()>;
}
impl IApplicationOverrides_Vtbl {
    pub const fn new<Identity: IApplicationOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLaunched<Identity: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, args: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApplicationOverrides_Impl::OnLaunched(this, core::mem::transmute_copy(&args)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IApplicationOverrides, OFFSET>(), OnLaunched: OnLaunched::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApplicationOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnLaunched: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IApplicationStatics, IApplicationStatics_Vtbl, 0x4e0d09f5_4358_512c_a987_503b52848e95);
impl windows_core::RuntimeType for IApplicationStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IApplicationStatics");
}
impl windows_core::RuntimeName for IApplicationStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.IApplicationStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBringIntoViewRequestedEventArgs, IBringIntoViewRequestedEventArgs_Vtbl, 0x807de8f9_b1dc_5a63_8101_5ee966841a27);
impl windows_core::RuntimeType for IBringIntoViewRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IBringIntoViewRequestedEventArgs");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IBringIntoViewRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IBringIntoViewRequestedEventArgs";
}
#[cfg(feature = "UI_Composition")]
pub trait IBringIntoViewRequestedEventArgs_Impl: windows_core::IUnknownImpl {
    fn TargetElement(&self) -> windows_core::Result<UIElement>;
    fn SetTargetElement(&self, value: windows_core::Ref<UIElement>) -> windows_core::Result<()>;
    fn AnimationDesired(&self) -> windows_core::Result<bool>;
    fn SetAnimationDesired(&self, value: bool) -> windows_core::Result<()>;
    fn TargetRect(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn SetTargetRect(&self, value: &windows::Foundation::Rect) -> windows_core::Result<()>;
    fn HorizontalAlignmentRatio(&self) -> windows_core::Result<f64>;
    fn VerticalAlignmentRatio(&self) -> windows_core::Result<f64>;
    fn HorizontalOffset(&self) -> windows_core::Result<f64>;
    fn SetHorizontalOffset(&self, value: f64) -> windows_core::Result<()>;
    fn VerticalOffset(&self) -> windows_core::Result<f64>;
    fn SetVerticalOffset(&self, value: f64) -> windows_core::Result<()>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition")]
impl IBringIntoViewRequestedEventArgs_Vtbl {
    pub const fn new<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TargetElement<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::TargetElement(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetElement<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBringIntoViewRequestedEventArgs_Impl::SetTargetElement(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn AnimationDesired<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::AnimationDesired(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAnimationDesired<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBringIntoViewRequestedEventArgs_Impl::SetAnimationDesired(this, value).into()
            }
        }
        unsafe extern "system" fn TargetRect<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::TargetRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTargetRect<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBringIntoViewRequestedEventArgs_Impl::SetTargetRect(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn HorizontalAlignmentRatio<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::HorizontalAlignmentRatio(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VerticalAlignmentRatio<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::VerticalAlignmentRatio(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HorizontalOffset<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::HorizontalOffset(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHorizontalOffset<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBringIntoViewRequestedEventArgs_Impl::SetHorizontalOffset(this, value).into()
            }
        }
        unsafe extern "system" fn VerticalOffset<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::VerticalOffset(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVerticalOffset<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBringIntoViewRequestedEventArgs_Impl::SetVerticalOffset(this, value).into()
            }
        }
        unsafe extern "system" fn Handled<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBringIntoViewRequestedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IBringIntoViewRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBringIntoViewRequestedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBringIntoViewRequestedEventArgs, OFFSET>(),
            TargetElement: TargetElement::<Identity, OFFSET>,
            SetTargetElement: SetTargetElement::<Identity, OFFSET>,
            AnimationDesired: AnimationDesired::<Identity, OFFSET>,
            SetAnimationDesired: SetAnimationDesired::<Identity, OFFSET>,
            TargetRect: TargetRect::<Identity, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, OFFSET>,
            HorizontalAlignmentRatio: HorizontalAlignmentRatio::<Identity, OFFSET>,
            VerticalAlignmentRatio: VerticalAlignmentRatio::<Identity, OFFSET>,
            HorizontalOffset: HorizontalOffset::<Identity, OFFSET>,
            SetHorizontalOffset: SetHorizontalOffset::<Identity, OFFSET>,
            VerticalOffset: VerticalOffset::<Identity, OFFSET>,
            SetVerticalOffset: SetVerticalOffset::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBringIntoViewRequestedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBringIntoViewRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub TargetElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    TargetElement: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetTargetElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetTargetElement: usize,
    pub AnimationDesired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAnimationDesired: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub SetTargetRect: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect) -> windows_core::HRESULT,
    pub HorizontalAlignmentRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub VerticalAlignmentRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub HorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub VerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetVerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataContextChangedEventArgs, IDataContextChangedEventArgs_Vtbl, 0xa1be80f4_cf83_5022_b113_9233f1d4fafa);
impl windows_core::RuntimeType for IDataContextChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IDataContextChangedEventArgs");
}
impl windows_core::RuntimeName for IDataContextChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IDataContextChangedEventArgs";
}
pub trait IDataContextChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn NewValue(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IDataContextChangedEventArgs_Vtbl {
    pub const fn new<Identity: IDataContextChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewValue<Identity: IDataContextChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataContextChangedEventArgs_Impl::NewValue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IDataContextChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataContextChangedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IDataContextChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataContextChangedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataContextChangedEventArgs, OFFSET>(),
            NewValue: NewValue::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataContextChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataContextChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDependencyObject, IDependencyObject_Vtbl, 0xe7beaee7_160e_50f7_8789_d63463f979fa);
impl windows_core::RuntimeType for IDependencyObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IDependencyObject");
}
#[cfg(feature = "UI_Dispatching")]
impl windows_core::RuntimeName for IDependencyObject {
    const NAME: &'static str = "Microsoft.UI.Xaml.IDependencyObject";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetValue: usize,
    SetValue: usize,
    ClearValue: usize,
    ReadLocalValue: usize,
    GetAnimationBaseValue: usize,
    RegisterPropertyChangedCallback: usize,
    UnregisterPropertyChangedCallback: usize,
    pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
}
windows_core::imp::define_interface!(IDependencyObjectFactory, IDependencyObjectFactory_Vtbl, 0x936b614c_475f_5d7d_b3f7_bf1fbea28126);
impl windows_core::RuntimeType for IDependencyObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IDependencyObjectFactory");
}
impl windows_core::RuntimeName for IDependencyObjectFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IDependencyObjectFactory";
}
pub trait IDependencyObjectFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<DependencyObject>;
}
impl IDependencyObjectFactory_Vtbl {
    pub const fn new<Identity: IDependencyObjectFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IDependencyObjectFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDependencyObjectFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDependencyObjectFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDependencyObjectFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDependencyObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDragStartingEventArgs, IDragStartingEventArgs_Vtbl, 0xad17bace_9613_5666_a31b_79a73fba77cf);
impl windows_core::RuntimeType for IDragStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IDragStartingEventArgs");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IDragStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IDragStartingEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    DragUI: usize,
    pub AllowedOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::ApplicationModel::DataTransfer::DataPackageOperation) -> windows_core::HRESULT,
    pub SetAllowedOperations: unsafe extern "system" fn(*mut core::ffi::c_void, windows::ApplicationModel::DataTransfer::DataPackageOperation) -> windows_core::HRESULT,
    GetDeferral: usize,
    #[cfg(feature = "UI_Composition")]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPosition: usize,
}
windows_core::imp::define_interface!(IDropCompletedEventArgs, IDropCompletedEventArgs_Vtbl, 0xe700082d_c640_5d44_b23a_f213dfbeb245);
impl windows_core::RuntimeType for IDropCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IDropCompletedEventArgs");
}
impl windows_core::RuntimeName for IDropCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IDropCompletedEventArgs";
}
pub trait IDropCompletedEventArgs_Impl: windows_core::IUnknownImpl {
    fn DropResult(&self) -> windows_core::Result<windows::ApplicationModel::DataTransfer::DataPackageOperation>;
}
impl IDropCompletedEventArgs_Vtbl {
    pub const fn new<Identity: IDropCompletedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DropResult<Identity: IDropCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::ApplicationModel::DataTransfer::DataPackageOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropCompletedEventArgs_Impl::DropResult(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDropCompletedEventArgs, OFFSET>(), DropResult: DropResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DropResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::ApplicationModel::DataTransfer::DataPackageOperation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEffectiveViewportChangedEventArgs, IEffectiveViewportChangedEventArgs_Vtbl, 0x636e8159_2d82_538a_8483_cd576e41d0df);
impl windows_core::RuntimeType for IEffectiveViewportChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IEffectiveViewportChangedEventArgs");
}
impl windows_core::RuntimeName for IEffectiveViewportChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IEffectiveViewportChangedEventArgs";
}
pub trait IEffectiveViewportChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn EffectiveViewport(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn MaxViewport(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn BringIntoViewDistanceX(&self) -> windows_core::Result<f64>;
    fn BringIntoViewDistanceY(&self) -> windows_core::Result<f64>;
}
impl IEffectiveViewportChangedEventArgs_Vtbl {
    pub const fn new<Identity: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EffectiveViewport<Identity: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEffectiveViewportChangedEventArgs_Impl::EffectiveViewport(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MaxViewport<Identity: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEffectiveViewportChangedEventArgs_Impl::MaxViewport(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BringIntoViewDistanceX<Identity: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEffectiveViewportChangedEventArgs_Impl::BringIntoViewDistanceX(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BringIntoViewDistanceY<Identity: IEffectiveViewportChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEffectiveViewportChangedEventArgs_Impl::BringIntoViewDistanceY(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IEffectiveViewportChangedEventArgs, OFFSET>(),
            EffectiveViewport: EffectiveViewport::<Identity, OFFSET>,
            MaxViewport: MaxViewport::<Identity, OFFSET>,
            BringIntoViewDistanceX: BringIntoViewDistanceX::<Identity, OFFSET>,
            BringIntoViewDistanceY: BringIntoViewDistanceY::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEffectiveViewportChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEffectiveViewportChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EffectiveViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub MaxViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub BringIntoViewDistanceX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub BringIntoViewDistanceY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameworkElement, IFrameworkElement_Vtbl, 0xfe08f13d_dc6a_5495_ad44_c2d8d21863b0);
impl windows_core::RuntimeType for IFrameworkElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IFrameworkElement");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
impl windows_core::RuntimeName for IFrameworkElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.IFrameworkElement";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Triggers: usize,
    pub Resources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActualWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ActualHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Width: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Height: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MinHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMinHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub MaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub HorizontalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HorizontalAlignment) -> windows_core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, HorizontalAlignment) -> windows_core::HRESULT,
    pub VerticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VerticalAlignment) -> windows_core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, VerticalAlignment) -> windows_core::HRESULT,
    pub Margin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
    pub SetMargin: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BaseUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DataContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDataContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowFocusOnInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowFocusOnInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub FocusVisualMargin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
    pub SetFocusVisualMargin: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    pub FocusVisualSecondaryThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
    pub SetFocusVisualSecondaryThickness: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    pub FocusVisualPrimaryThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Thickness) -> windows_core::HRESULT,
    pub SetFocusVisualPrimaryThickness: unsafe extern "system" fn(*mut core::ffi::c_void, Thickness) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub FocusVisualSecondaryBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    FocusVisualSecondaryBrush: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub SetFocusVisualSecondaryBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    SetFocusVisualSecondaryBrush: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub FocusVisualPrimaryBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    FocusVisualPrimaryBrush: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub SetFocusVisualPrimaryBrush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    SetFocusVisualPrimaryBrush: usize,
    pub AllowFocusWhenDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowFocusWhenDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Style: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Parent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    FlowDirection: usize,
    SetFlowDirection: usize,
    RequestedTheme: usize,
    SetRequestedTheme: usize,
    pub IsLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    ActualTheme: usize,
    pub Loaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLoaded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Unloaded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUnloaded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub DataContextChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    DataContextChanged: usize,
    pub RemoveDataContextChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    SizeChanged: usize,
    pub RemoveSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LayoutUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLayoutUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub Loading: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Loading: usize,
    pub RemoveLoading: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub ActualThemeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    ActualThemeChanged: usize,
    pub RemoveActualThemeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub EffectiveViewportChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    EffectiveViewportChanged: usize,
    pub RemoveEffectiveViewportChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub FindName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameworkElementFactory, IFrameworkElementFactory_Vtbl, 0xbd3f2272_3efa_5f92_b759_90b1cc3e784c);
impl windows_core::RuntimeType for IFrameworkElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IFrameworkElementFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IFrameworkElementFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IFrameworkElementFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait IFrameworkElementFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<FrameworkElement>;
}
#[cfg(feature = "UI_Composition")]
impl IFrameworkElementFactory_Vtbl {
    pub const fn new<Identity: IFrameworkElementFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IFrameworkElementFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElementFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElementFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IFrameworkElementOverrides, IFrameworkElementOverrides_Vtbl, 0xffc6fd98_f38c_5904_9ce4_97a3427cf4ba);
impl windows_core::RuntimeType for IFrameworkElementOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IFrameworkElementOverrides");
}
impl IFrameworkElementOverrides {
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MeasureOverride)(windows_core::Interface::as_raw(self), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ArrangeOverride)(windows_core::Interface::as_raw(self), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnApplyTemplate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GoToElementStateCore)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IFrameworkElementOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IFrameworkElementOverrides";
}
pub trait IFrameworkElementOverrides_Impl: windows_core::IUnknownImpl {
    fn MeasureOverride(&self, availableSize: &windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size>;
    fn ArrangeOverride(&self, finalSize: &windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size>;
    fn OnApplyTemplate(&self) -> windows_core::Result<()>;
    fn GoToElementStateCore(&self, stateName: &windows_core::HSTRING, useTransitions: bool) -> windows_core::Result<bool>;
}
impl IFrameworkElementOverrides_Vtbl {
    pub const fn new<Identity: IFrameworkElementOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MeasureOverride<Identity: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availablesize: windows::Foundation::Size, result__: *mut windows::Foundation::Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElementOverrides_Impl::MeasureOverride(this, core::mem::transmute(&availablesize)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ArrangeOverride<Identity: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, finalsize: windows::Foundation::Size, result__: *mut windows::Foundation::Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElementOverrides_Impl::ArrangeOverride(this, core::mem::transmute(&finalsize)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnApplyTemplate<Identity: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElementOverrides_Impl::OnApplyTemplate(this).into()
            }
        }
        unsafe extern "system" fn GoToElementStateCore<Identity: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statename: *mut core::ffi::c_void, usetransitions: bool, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFrameworkElementOverrides_Impl::GoToElementStateCore(this, core::mem::transmute(&statename), usetransitions) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElementOverrides, OFFSET>(),
            MeasureOverride: MeasureOverride::<Identity, OFFSET>,
            ArrangeOverride: ArrangeOverride::<Identity, OFFSET>,
            OnApplyTemplate: OnApplyTemplate::<Identity, OFFSET>,
            GoToElementStateCore: GoToElementStateCore::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MeasureOverride: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Size, *mut windows::Foundation::Size) -> windows_core::HRESULT,
    pub ArrangeOverride: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Size, *mut windows::Foundation::Size) -> windows_core::HRESULT,
    pub OnApplyTemplate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GoToElementStateCore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameworkElementProtected, IFrameworkElementProtected_Vtbl, 0xe59a3db0_91e5_5903_9caf_d1bb9f458bf2);
impl windows_core::RuntimeType for IFrameworkElementProtected {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IFrameworkElementProtected");
}
impl windows_core::RuntimeName for IFrameworkElementProtected {
    const NAME: &'static str = "Microsoft.UI.Xaml.IFrameworkElementProtected";
}
pub trait IFrameworkElementProtected_Impl: windows_core::IUnknownImpl {
    fn InvalidateViewport(&self) -> windows_core::Result<()>;
}
impl IFrameworkElementProtected_Vtbl {
    pub const fn new<Identity: IFrameworkElementProtected_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvalidateViewport<Identity: IFrameworkElementProtected_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFrameworkElementProtected_Impl::InvalidateViewport(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFrameworkElementProtected, OFFSET>(),
            InvalidateViewport: InvalidateViewport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFrameworkElementProtected as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementProtected_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub InvalidateViewport: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFrameworkElementStatics, IFrameworkElementStatics_Vtbl, 0x894e2704_14e7_569a_b21e_afc7df7145a1);
impl windows_core::RuntimeType for IFrameworkElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IFrameworkElementStatics");
}
impl windows_core::RuntimeName for IFrameworkElementStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.IFrameworkElementStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFrameworkElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    TagProperty: usize,
    LanguageProperty: usize,
    ActualWidthProperty: usize,
    ActualHeightProperty: usize,
    WidthProperty: usize,
    HeightProperty: usize,
    MinWidthProperty: usize,
    MaxWidthProperty: usize,
    MinHeightProperty: usize,
    MaxHeightProperty: usize,
    HorizontalAlignmentProperty: usize,
    VerticalAlignmentProperty: usize,
    MarginProperty: usize,
    NameProperty: usize,
    DataContextProperty: usize,
    AllowFocusOnInteractionProperty: usize,
    FocusVisualMarginProperty: usize,
    FocusVisualSecondaryThicknessProperty: usize,
    FocusVisualPrimaryThicknessProperty: usize,
    FocusVisualSecondaryBrushProperty: usize,
    FocusVisualPrimaryBrushProperty: usize,
    AllowFocusWhenDisabledProperty: usize,
    StyleProperty: usize,
    FlowDirectionProperty: usize,
    RequestedThemeProperty: usize,
    ActualThemeProperty: usize,
    pub DeferTree: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILaunchActivatedEventArgs, ILaunchActivatedEventArgs_Vtbl, 0xd505cea9_1bcb_5b29_a8be_944e00f06f78);
impl windows_core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.ILaunchActivatedEventArgs");
}
impl windows_core::RuntimeName for ILaunchActivatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.ILaunchActivatedEventArgs";
}
pub trait ILaunchActivatedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn UWPLaunchActivatedEventArgs(&self) -> windows_core::Result<windows::ApplicationModel::Activation::LaunchActivatedEventArgs>;
}
impl ILaunchActivatedEventArgs_Vtbl {
    pub const fn new<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Arguments<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILaunchActivatedEventArgs_Impl::Arguments(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UWPLaunchActivatedEventArgs<Identity: ILaunchActivatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILaunchActivatedEventArgs_Impl::UWPLaunchActivatedEventArgs(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ILaunchActivatedEventArgs, OFFSET>(),
            Arguments: Arguments::<Identity, OFFSET>,
            UWPLaunchActivatedEventArgs: UWPLaunchActivatedEventArgs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILaunchActivatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UWPLaunchActivatedEventArgs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceDictionary, IResourceDictionary_Vtbl, 0x1b690975_a710_5783_a6e1_15836f6186c2);
impl windows_core::RuntimeType for IResourceDictionary {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IResourceDictionary");
}
impl windows_core::RuntimeName for IResourceDictionary {
    const NAME: &'static str = "Microsoft.UI.Xaml.IResourceDictionary";
}
pub trait IResourceDictionary_Impl: windows_core::IUnknownImpl {
    fn Source(&self) -> windows_core::Result<windows::Foundation::Uri>;
    fn SetSource(&self, value: windows_core::Ref<windows::Foundation::Uri>) -> windows_core::Result<()>;
    fn MergedDictionaries(&self) -> windows_core::Result<windows_collections::IVector<ResourceDictionary>>;
    fn ThemeDictionaries(&self) -> windows_core::Result<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>;
}
impl IResourceDictionary_Vtbl {
    pub const fn new<Identity: IResourceDictionary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Source<Identity: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceDictionary_Impl::Source(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSource<Identity: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IResourceDictionary_Impl::SetSource(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn MergedDictionaries<Identity: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceDictionary_Impl::MergedDictionaries(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ThemeDictionaries<Identity: IResourceDictionary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceDictionary_Impl::ThemeDictionaries(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IResourceDictionary, OFFSET>(),
            Source: Source::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
            MergedDictionaries: MergedDictionaries::<Identity, OFFSET>,
            ThemeDictionaries: ThemeDictionaries::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceDictionary as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceDictionary_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MergedDictionaries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ThemeDictionaries: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceDictionaryFactory, IResourceDictionaryFactory_Vtbl, 0xea22a48f_ab71_56f6_a392_d82310c8aa7b);
impl windows_core::RuntimeType for IResourceDictionaryFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IResourceDictionaryFactory");
}
impl windows_core::RuntimeName for IResourceDictionaryFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IResourceDictionaryFactory";
}
pub trait IResourceDictionaryFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<ResourceDictionary>;
}
impl IResourceDictionaryFactory_Vtbl {
    pub const fn new<Identity: IResourceDictionaryFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IResourceDictionaryFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IResourceDictionaryFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IResourceDictionaryFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IResourceDictionaryFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceDictionaryFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IResourceManagerRequestedEventArgs, IResourceManagerRequestedEventArgs_Vtbl, 0xc35f4cf1_fcd6_5c6b_9be2_4cfaefb68b2a);
impl windows_core::RuntimeType for IResourceManagerRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IResourceManagerRequestedEventArgs");
}
impl windows_core::RuntimeName for IResourceManagerRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IResourceManagerRequestedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IRoutedEventArgs, IRoutedEventArgs_Vtbl, 0x0908c407_1c7d_5de3_9c50_d971c62ec8ec);
impl windows_core::RuntimeType for IRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IRoutedEventArgs");
}
impl windows_core::RuntimeName for IRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IRoutedEventArgs";
}
pub trait IRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl IRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OriginalSource<Identity: IRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRoutedEventArgs_Impl::OriginalSource(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IRoutedEventArgs, OFFSET>(), OriginalSource: OriginalSource::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OriginalSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRoutedEventArgsFactory, IRoutedEventArgsFactory_Vtbl, 0x914b02c7_076b_5b89_98e7_6c373379e9af);
impl windows_core::RuntimeType for IRoutedEventArgsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IRoutedEventArgsFactory");
}
impl windows_core::RuntimeName for IRoutedEventArgsFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IRoutedEventArgsFactory";
}
pub trait IRoutedEventArgsFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<RoutedEventArgs>;
}
impl IRoutedEventArgsFactory_Vtbl {
    pub const fn new<Identity: IRoutedEventArgsFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IRoutedEventArgsFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRoutedEventArgsFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IRoutedEventArgsFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRoutedEventArgsFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoutedEventArgsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStyle, IStyle_Vtbl, 0x65e1d164_572f_5b0e_a80f_9c02441fac49);
impl windows_core::RuntimeType for IStyle {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IStyle");
}
impl windows_core::RuntimeName for IStyle {
    const NAME: &'static str = "Microsoft.UI.Xaml.IStyle";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStyle_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSealed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    Setters: usize,
    pub TargetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows::UI::Xaml::Interop::TypeName>) -> windows_core::HRESULT,
    pub SetTargetType: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows::UI::Xaml::Interop::TypeName>) -> windows_core::HRESULT,
    pub BasedOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBasedOn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Seal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStyleFactory, IStyleFactory_Vtbl, 0xc2d924a2_3862_517c_b083_9a9120d7302d);
impl windows_core::RuntimeType for IStyleFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IStyleFactory");
}
impl windows_core::RuntimeName for IStyleFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IStyleFactory";
}
pub trait IStyleFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, targetType: &windows::UI::Xaml::Interop::TypeName) -> windows_core::Result<Style>;
}
impl IStyleFactory_Vtbl {
    pub const fn new<Identity: IStyleFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IStyleFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targettype: core::mem::MaybeUninit<windows::UI::Xaml::Interop::TypeName>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStyleFactory_Impl::CreateInstance(this, core::mem::transmute(&targettype)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IStyleFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStyleFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStyleFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows::UI::Xaml::Interop::TypeName>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIElement, IUIElement_Vtbl, 0xc3c01020_320c_5cf6_9d24_d396bbfa4d8b);
impl windows_core::RuntimeType for IUIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IUIElement");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input", feature = "UI_Xaml_Input", feature = "UI_Xaml_Media"))]
impl windows_core::RuntimeName for IUIElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.IUIElement";
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Size) -> windows_core::HRESULT,
    pub AllowDrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowDrop: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Opacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    Clip: usize,
    SetClip: usize,
    RenderTransform: usize,
    SetRenderTransform: usize,
    Projection: usize,
    SetProjection: usize,
    Transform3D: usize,
    SetTransform3D: usize,
    pub RenderTransformOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetRenderTransformOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
    pub IsHitTestVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHitTestVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Visibility: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Visibility) -> windows_core::HRESULT,
    pub SetVisibility: unsafe extern "system" fn(*mut core::ffi::c_void, Visibility) -> windows_core::HRESULT,
    pub RenderSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Size) -> windows_core::HRESULT,
    pub UseLayoutRounding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseLayoutRounding: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    Transitions: usize,
    SetTransitions: usize,
    CacheMode: usize,
    SetCacheMode: usize,
    pub IsTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDoubleTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsDoubleTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanDrag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanDrag: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsRightTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsRightTapEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsHoldingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsHoldingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ManipulationMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::ManipulationModes) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ManipulationMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetManipulationMode: unsafe extern "system" fn(*mut core::ffi::c_void, Input::ManipulationModes) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetManipulationMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerCaptures: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerCaptures: usize,
    ContextFlyout: usize,
    SetContextFlyout: usize,
    CompositeMode: usize,
    SetCompositeMode: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Lights: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Lights: usize,
    pub CanBeScrollAnchor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanBeScrollAnchor: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetExitDisplayModeOnAccessKeyInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsAccessKeyScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsAccessKeyScope: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AccessKeyScopeOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAccessKeyScopeOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyTipPlacementMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::KeyTipPlacementMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyTipPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyTipPlacementMode: unsafe extern "system" fn(*mut core::ffi::c_void, Input::KeyTipPlacementMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyTipPlacementMode: usize,
    pub KeyTipHorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetKeyTipHorizontalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub KeyTipVerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetKeyTipVerticalOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub KeyTipTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetKeyTipTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusKeyboardNavigation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::XYFocusKeyboardNavigationMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusKeyboardNavigation: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusKeyboardNavigation: unsafe extern "system" fn(*mut core::ffi::c_void, Input::XYFocusKeyboardNavigationMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusKeyboardNavigation: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusUpNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusUpNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusUpNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusDownNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusDownNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusDownNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusLeftNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusLeftNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusLeftNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub XYFocusRightNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    XYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetXYFocusRightNavigationStrategy: unsafe extern "system" fn(*mut core::ffi::c_void, Input::XYFocusNavigationStrategy) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetXYFocusRightNavigationStrategy: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyboardAccelerators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyboardAccelerators: usize,
    pub KeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetKeyboardAcceleratorPlacementTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyboardAcceleratorPlacementMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::KeyboardAcceleratorPlacementMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyboardAcceleratorPlacementMode: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetKeyboardAcceleratorPlacementMode: unsafe extern "system" fn(*mut core::ffi::c_void, Input::KeyboardAcceleratorPlacementMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetKeyboardAcceleratorPlacementMode: usize,
    HighContrastAdjustment: usize,
    SetHighContrastAdjustment: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TabFocusNavigation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Input::KeyboardNavigationMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TabFocusNavigation: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetTabFocusNavigation: unsafe extern "system" fn(*mut core::ffi::c_void, Input::KeyboardNavigationMode) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetTabFocusNavigation: usize,
    OpacityTransition: usize,
    SetOpacityTransition: usize,
    pub Translation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub SetTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
    TranslationTransition: usize,
    SetTranslationTransition: usize,
    pub Rotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    RotationTransition: usize,
    SetRotationTransition: usize,
    pub Scale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub SetScale: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
    ScaleTransition: usize,
    SetScaleTransition: usize,
    pub TransformMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub SetTransformMatrix: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Matrix4x4) -> windows_core::HRESULT,
    pub CenterPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub SetCenterPoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
    pub RotationAxis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub SetRotationAxis: unsafe extern "system" fn(*mut core::ffi::c_void, windows_numerics::Vector3) -> windows_core::HRESULT,
    pub ActualOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector3) -> windows_core::HRESULT,
    pub ActualSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_numerics::Vector2) -> windows_core::HRESULT,
    pub XamlRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXamlRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    Shadow: usize,
    SetShadow: usize,
    pub RasterizationScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRasterizationScale: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub FocusState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusState) -> windows_core::HRESULT,
    pub UseSystemFocusVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetUseSystemFocusVisuals: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub XYFocusLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXYFocusLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XYFocusRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXYFocusRight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XYFocusUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXYFocusUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub XYFocusDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetXYFocusDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TabIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetTabIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyUp: usize,
    pub RemoveKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    KeyDown: usize,
    pub RemoveKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub DragStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    DragStarting: usize,
    pub RemoveDragStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub DropCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    DropCompleted: usize,
    pub RemoveDropCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub CharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    CharacterReceived: usize,
    pub RemoveCharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DragEnter: usize,
    pub RemoveDragEnter: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DragLeave: usize,
    pub RemoveDragLeave: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    DragOver: usize,
    pub RemoveDragOver: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    Drop: usize,
    pub RemoveDrop: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerPressed: usize,
    pub RemovePointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerMoved: usize,
    pub RemovePointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerReleased: usize,
    pub RemovePointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerEntered: usize,
    pub RemovePointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerExited: usize,
    pub RemovePointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerCaptureLost: usize,
    pub RemovePointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerCanceled: usize,
    pub RemovePointerCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PointerWheelChanged: usize,
    pub RemovePointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub Tapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    Tapped: usize,
    pub RemoveTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub DoubleTapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    DoubleTapped: usize,
    pub RemoveDoubleTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub Holding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    Holding: usize,
    pub RemoveHolding: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub ContextRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    ContextRequested: usize,
    pub RemoveContextRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub ContextCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    ContextCanceled: usize,
    pub RemoveContextCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub RightTapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    RightTapped: usize,
    pub RemoveRightTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ManipulationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ManipulationStarting: usize,
    pub RemoveManipulationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ManipulationInertiaStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ManipulationInertiaStarting: usize,
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ManipulationStarted: usize,
    pub RemoveManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ManipulationDelta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ManipulationDelta: usize,
    pub RemoveManipulationDelta: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ManipulationCompleted: usize,
    pub RemoveManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayRequested: usize,
    pub RemoveAccessKeyDisplayRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub AccessKeyDisplayDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    AccessKeyDisplayDismissed: usize,
    pub RemoveAccessKeyDisplayDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub AccessKeyInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    AccessKeyInvoked: usize,
    pub RemoveAccessKeyInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub ProcessKeyboardAccelerators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    ProcessKeyboardAccelerators: usize,
    pub RemoveProcessKeyboardAccelerators: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub GettingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    GettingFocus: usize,
    pub RemoveGettingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub LosingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    LosingFocus: usize,
    pub RemoveLosingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
    pub NoFocusCandidateFound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Input")))]
    NoFocusCandidateFound: usize,
    pub RemoveNoFocusCandidateFound: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PreviewKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PreviewKeyDown: usize,
    pub RemovePreviewKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub PreviewKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    PreviewKeyUp: usize,
    pub RemovePreviewKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub BringIntoViewRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    BringIntoViewRequested: usize,
    pub RemoveBringIntoViewRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Measure: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Size) -> windows_core::HRESULT,
    pub Arrange: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub CapturePointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    CapturePointer: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub ReleasePointerCapture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    ReleasePointerCapture: usize,
    pub ReleasePointerCaptures: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    AddHandler: usize,
    RemoveHandler: usize,
    TransformToVisual: usize,
    pub InvalidateMeasure: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InvalidateArrange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateLayout: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CancelDirectManipulations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub StartDragAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    StartDragAsync: usize,
    pub StartBringIntoView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    StartBringIntoViewWithOptions: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TryInvokeKeyboardAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TryInvokeKeyboardAccelerator: usize,
    pub Focus: unsafe extern "system" fn(*mut core::ffi::c_void, FocusState, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIElementFactory, IUIElementFactory_Vtbl, 0x14d1d309_add0_5ccb_b946_77488cd70f87);
impl windows_core::RuntimeType for IUIElementFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IUIElementFactory");
}
impl windows_core::RuntimeName for IUIElementFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IUIElementFactory";
}
pub trait IUIElementFactory_Impl: windows_core::IUnknownImpl {}
impl IUIElementFactory_Vtbl {
    pub const fn new<Identity: IUIElementFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IUIElementOverrides, IUIElementOverrides_Vtbl, 0x9034f41e_ab7b_59e7_8168_50de6b689dde);
impl windows_core::RuntimeType for IUIElementOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IUIElementOverrides");
}
impl IUIElementOverrides {
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(self), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(self), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(self), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BringIntoViewRequestedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnBringIntoViewRequested)(windows_core::Interface::as_raw(self), e.param().abi()).ok() }
    }
}
#[cfg(feature = "UI_Xaml_Input")]
impl windows_core::RuntimeName for IUIElementOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.IUIElementOverrides";
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    OnCreateAutomationPeer: usize,
    pub OnDisconnectVisualChildren: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindSubElementsForTouchTargeting: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point, windows::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChildrenInTabFocusOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnKeyboardAcceleratorInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnKeyboardAcceleratorInvoked: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub OnProcessKeyboardAccelerators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    OnProcessKeyboardAccelerators: usize,
    pub OnBringIntoViewRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUIElementProtected, IUIElementProtected_Vtbl, 0x8f69b9e9_1f00_5834_9bf1_a9257bed39f0);
impl windows_core::RuntimeType for IUIElementProtected {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IUIElementProtected");
}
#[cfg(feature = "UI_Input")]
impl windows_core::RuntimeName for IUIElementProtected {
    const NAME: &'static str = "Microsoft.UI.Xaml.IUIElementProtected";
}
#[cfg(feature = "UI_Input")]
pub trait IUIElementProtected_Impl: windows_core::IUnknownImpl {
    fn ProtectedCursor(&self) -> windows_core::Result<super::Input::InputCursor>;
    fn SetProtectedCursor(&self, value: windows_core::Ref<super::Input::InputCursor>) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Input")]
impl IUIElementProtected_Vtbl {
    pub const fn new<Identity: IUIElementProtected_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProtectedCursor<Identity: IUIElementProtected_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIElementProtected_Impl::ProtectedCursor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProtectedCursor<Identity: IUIElementProtected_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIElementProtected_Impl::SetProtectedCursor(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUIElementProtected, OFFSET>(),
            ProtectedCursor: ProtectedCursor::<Identity, OFFSET>,
            SetProtectedCursor: SetProtectedCursor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIElementProtected as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementProtected_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input")]
    pub ProtectedCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    ProtectedCursor: usize,
    #[cfg(feature = "UI_Input")]
    pub SetProtectedCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    SetProtectedCursor: usize,
}
windows_core::imp::define_interface!(IUIElementStatics, IUIElementStatics_Vtbl, 0xd2921d87_3584_5e22_8a3a_c2c78dab4f6e);
impl windows_core::RuntimeType for IUIElementStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IUIElementStatics");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Input"))]
impl windows_core::RuntimeName for IUIElementStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.IUIElementStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIElementStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    KeyDownEvent: usize,
    KeyUpEvent: usize,
    PointerEnteredEvent: usize,
    PointerPressedEvent: usize,
    PointerMovedEvent: usize,
    PointerReleasedEvent: usize,
    PointerExitedEvent: usize,
    PointerCaptureLostEvent: usize,
    PointerCanceledEvent: usize,
    PointerWheelChangedEvent: usize,
    TappedEvent: usize,
    DoubleTappedEvent: usize,
    HoldingEvent: usize,
    RightTappedEvent: usize,
    ManipulationStartingEvent: usize,
    ManipulationInertiaStartingEvent: usize,
    ManipulationStartedEvent: usize,
    ManipulationDeltaEvent: usize,
    ManipulationCompletedEvent: usize,
    DragEnterEvent: usize,
    DragLeaveEvent: usize,
    DragOverEvent: usize,
    DropEvent: usize,
    GettingFocusEvent: usize,
    LosingFocusEvent: usize,
    NoFocusCandidateFoundEvent: usize,
    PreviewKeyDownEvent: usize,
    CharacterReceivedEvent: usize,
    PreviewKeyUpEvent: usize,
    BringIntoViewRequestedEvent: usize,
    ContextRequestedEvent: usize,
    AllowDropProperty: usize,
    OpacityProperty: usize,
    ClipProperty: usize,
    RenderTransformProperty: usize,
    ProjectionProperty: usize,
    Transform3DProperty: usize,
    RenderTransformOriginProperty: usize,
    IsHitTestVisibleProperty: usize,
    VisibilityProperty: usize,
    UseLayoutRoundingProperty: usize,
    TransitionsProperty: usize,
    CacheModeProperty: usize,
    IsTapEnabledProperty: usize,
    IsDoubleTapEnabledProperty: usize,
    CanDragProperty: usize,
    IsRightTapEnabledProperty: usize,
    IsHoldingEnabledProperty: usize,
    ManipulationModeProperty: usize,
    PointerCapturesProperty: usize,
    ContextFlyoutProperty: usize,
    CompositeModeProperty: usize,
    LightsProperty: usize,
    CanBeScrollAnchorProperty: usize,
    ExitDisplayModeOnAccessKeyInvokedProperty: usize,
    IsAccessKeyScopeProperty: usize,
    AccessKeyScopeOwnerProperty: usize,
    AccessKeyProperty: usize,
    KeyTipPlacementModeProperty: usize,
    KeyTipHorizontalOffsetProperty: usize,
    KeyTipVerticalOffsetProperty: usize,
    KeyTipTargetProperty: usize,
    XYFocusKeyboardNavigationProperty: usize,
    XYFocusUpNavigationStrategyProperty: usize,
    XYFocusDownNavigationStrategyProperty: usize,
    XYFocusLeftNavigationStrategyProperty: usize,
    XYFocusRightNavigationStrategyProperty: usize,
    KeyboardAcceleratorPlacementTargetProperty: usize,
    KeyboardAcceleratorPlacementModeProperty: usize,
    HighContrastAdjustmentProperty: usize,
    TabFocusNavigationProperty: usize,
    ShadowProperty: usize,
    FocusStateProperty: usize,
    UseSystemFocusVisualsProperty: usize,
    XYFocusLeftProperty: usize,
    XYFocusRightProperty: usize,
    XYFocusUpProperty: usize,
    XYFocusDownProperty: usize,
    IsTabStopProperty: usize,
    TabIndexProperty: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub TryStartDirectManipulation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    TryStartDirectManipulation: usize,
    #[cfg(feature = "UI_Composition")]
    pub RegisterAsScrollPort: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    RegisterAsScrollPort: usize,
}
windows_core::imp::define_interface!(IUnhandledExceptionEventArgs, IUnhandledExceptionEventArgs_Vtbl, 0x59eaeba9_8f9c_5be7_9b3b_820960faa220);
impl windows_core::RuntimeType for IUnhandledExceptionEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IUnhandledExceptionEventArgs");
}
impl windows_core::RuntimeName for IUnhandledExceptionEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IUnhandledExceptionEventArgs";
}
pub trait IUnhandledExceptionEventArgs_Impl: windows_core::IUnknownImpl {
    fn Exception(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn Message(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IUnhandledExceptionEventArgs_Vtbl {
    pub const fn new<Identity: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Exception<Identity: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUnhandledExceptionEventArgs_Impl::Exception(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Message<Identity: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUnhandledExceptionEventArgs_Impl::Message(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUnhandledExceptionEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IUnhandledExceptionEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUnhandledExceptionEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUnhandledExceptionEventArgs, OFFSET>(),
            Exception: Exception::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUnhandledExceptionEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnhandledExceptionEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Exception: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub Message: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindow, IWindow_Vtbl, 0x61f0ec79_5d52_56b5_86fb_40fa4af288b0);
impl windows_core::RuntimeType for IWindow {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindow");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Dispatching"))]
impl windows_core::RuntimeName for IWindow {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindow";
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Bounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Content: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetContent: usize,
    pub CoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    Compositor: usize,
    pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExtendsContentIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetExtendsContentIntoTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Activated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveActivated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSizeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub VisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVisibilityChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub SetTitleBar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetTitleBar: usize,
}
windows_core::imp::define_interface!(IWindow2, IWindow2_Vtbl, 0x42febaa5_1c32_522a_a591_57618c6f665d);
impl windows_core::RuntimeType for IWindow2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindow2");
}
#[cfg(all(feature = "UI_Windowing", feature = "UI_Xaml_Media"))]
impl windows_core::RuntimeName for IWindow2 {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindow2";
}
#[cfg(all(feature = "UI_Windowing", feature = "UI_Xaml_Media"))]
pub trait IWindow2_Impl: windows_core::IUnknownImpl {
    fn SystemBackdrop(&self) -> windows_core::Result<Media::SystemBackdrop>;
    fn SetSystemBackdrop(&self, value: windows_core::Ref<Media::SystemBackdrop>) -> windows_core::Result<()>;
    fn AppWindow(&self) -> windows_core::Result<super::Windowing::AppWindow>;
}
#[cfg(all(feature = "UI_Windowing", feature = "UI_Xaml_Media"))]
impl IWindow2_Vtbl {
    pub const fn new<Identity: IWindow2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SystemBackdrop<Identity: IWindow2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindow2_Impl::SystemBackdrop(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSystemBackdrop<Identity: IWindow2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindow2_Impl::SetSystemBackdrop(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn AppWindow<Identity: IWindow2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindow2_Impl::AppWindow(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindow2, OFFSET>(),
            SystemBackdrop: SystemBackdrop::<Identity, OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Identity, OFFSET>,
            AppWindow: AppWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindow2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindow2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SystemBackdrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SystemBackdrop: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetSystemBackdrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetSystemBackdrop: usize,
    #[cfg(feature = "UI_Windowing")]
    pub AppWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Windowing"))]
    AppWindow: usize,
}
windows_core::imp::define_interface!(IWindowActivatedEventArgs, IWindowActivatedEventArgs_Vtbl, 0xc723a5ea_82c4_5dd6_861b_70ef573b88d6);
impl windows_core::RuntimeType for IWindowActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindowActivatedEventArgs");
}
impl windows_core::RuntimeName for IWindowActivatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindowActivatedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowEventArgs, IWindowEventArgs_Vtbl, 0x1140827c_fe0a_5268_bc2b_f4492c2ccb49);
impl windows_core::RuntimeType for IWindowEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindowEventArgs");
}
impl windows_core::RuntimeName for IWindowEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindowEventArgs";
}
pub trait IWindowEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IWindowEventArgs_Vtbl {
    pub const fn new<Identity: IWindowEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IWindowEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IWindowEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowFactory, IWindowFactory_Vtbl, 0xf0441536_afef_5222_918f_324a9b2dec75);
impl windows_core::RuntimeType for IWindowFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindowFactory");
}
impl windows_core::RuntimeName for IWindowFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindowFactory";
}
pub trait IWindowFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<Window>;
}
impl IWindowFactory_Vtbl {
    pub const fn new<Identity: IWindowFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IWindowFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowNative, IWindowNative_Vtbl, 0xeecdbf0e_bae9_4cb6_a68e_9598e1cb57bb);
windows_core::imp::interface_hierarchy!(IWindowNative, windows_core::IUnknown);
impl IWindowNative {
    pub unsafe fn WindowHandle(&self) -> windows_core::Result<windows::Win32::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Win32::Foundation::HWND) -> windows_core::HRESULT,
}
pub trait IWindowNative_Impl: windows_core::IUnknownImpl {
    fn WindowHandle(&self) -> windows_core::Result<windows::Win32::Foundation::HWND>;
}
impl IWindowNative_Vtbl {
    pub const fn new<Identity: IWindowNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowHandle<Identity: IWindowNative_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut windows::Win32::Foundation::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowNative_Impl::WindowHandle(this) {
                    Ok(ok__) => {
                        hwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), WindowHandle: WindowHandle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowNative as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowNative {}
windows_core::imp::define_interface!(IWindowSizeChangedEventArgs, IWindowSizeChangedEventArgs_Vtbl, 0x542f6f2c_4b64_5c72_a7a5_3a7e0664b8ff);
impl windows_core::RuntimeType for IWindowSizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindowSizeChangedEventArgs");
}
impl windows_core::RuntimeName for IWindowSizeChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindowSizeChangedEventArgs";
}
pub trait IWindowSizeChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn Size(&self) -> windows_core::Result<windows::Foundation::Size>;
}
impl IWindowSizeChangedEventArgs_Vtbl {
    pub const fn new<Identity: IWindowSizeChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IWindowSizeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowSizeChangedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IWindowSizeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowSizeChangedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn Size<Identity: IWindowSizeChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowSizeChangedEventArgs_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowSizeChangedEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowSizeChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowSizeChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowStatics, IWindowStatics_Vtbl, 0x8cc985e3_a41a_5df4_b531_d3a1788d86c5);
impl windows_core::RuntimeType for IWindowStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindowStatics");
}
impl windows_core::RuntimeName for IWindowStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindowStatics";
}
pub trait IWindowStatics_Impl: windows_core::IUnknownImpl {
    fn Current(&self) -> windows_core::Result<Window>;
}
impl IWindowStatics_Vtbl {
    pub const fn new<Identity: IWindowStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Current<Identity: IWindowStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowStatics_Impl::Current(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowStatics, OFFSET>(), Current: Current::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowVisibilityChangedEventArgs, IWindowVisibilityChangedEventArgs_Vtbl, 0x7bb24a6d_070c_5cb6_8e9c_547905be8265);
impl windows_core::RuntimeType for IWindowVisibilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IWindowVisibilityChangedEventArgs");
}
impl windows_core::RuntimeName for IWindowVisibilityChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IWindowVisibilityChangedEventArgs";
}
pub trait IWindowVisibilityChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn Visible(&self) -> windows_core::Result<bool>;
}
impl IWindowVisibilityChangedEventArgs_Vtbl {
    pub const fn new<Identity: IWindowVisibilityChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IWindowVisibilityChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowVisibilityChangedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IWindowVisibilityChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowVisibilityChangedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn Visible<Identity: IWindowVisibilityChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowVisibilityChangedEventArgs_Impl::Visible(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowVisibilityChangedEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            Visible: Visible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowVisibilityChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowVisibilityChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Visible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXamlRoot, IXamlRoot_Vtbl, 0x60cb215a_ad15_520a_8b01_4416824f0441);
impl windows_core::RuntimeType for IXamlRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IXamlRoot");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IXamlRoot {
    const NAME: &'static str = "Microsoft.UI.Xaml.IXamlRoot";
}
#[cfg(feature = "UI_Composition")]
pub trait IXamlRoot_Impl: windows_core::IUnknownImpl {
    fn Content(&self) -> windows_core::Result<UIElement>;
    fn Size(&self) -> windows_core::Result<windows::Foundation::Size>;
    fn RasterizationScale(&self) -> windows_core::Result<f64>;
    fn IsHostVisible(&self) -> windows_core::Result<bool>;
    fn Changed(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<XamlRoot, XamlRootChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveChanged(&self, token: i64) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition")]
impl IXamlRoot_Vtbl {
    pub const fn new<Identity: IXamlRoot_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Content<Identity: IXamlRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlRoot_Impl::Content(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Size<Identity: IXamlRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Size) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlRoot_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RasterizationScale<Identity: IXamlRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlRoot_Impl::RasterizationScale(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsHostVisible<Identity: IXamlRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlRoot_Impl::IsHostVisible(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Changed<Identity: IXamlRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlRoot_Impl::Changed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveChanged<Identity: IXamlRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlRoot_Impl::RemoveChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlRoot, OFFSET>(),
            Content: Content::<Identity, OFFSET>,
            Size: Size::<Identity, OFFSET>,
            RasterizationScale: RasterizationScale::<Identity, OFFSET>,
            IsHostVisible: IsHostVisible::<Identity, OFFSET>,
            Changed: Changed::<Identity, OFFSET>,
            RemoveChanged: RemoveChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlRoot as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRoot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub Content: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Content: usize,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Size) -> windows_core::HRESULT,
    pub RasterizationScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsHostVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Changed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXamlRoot2, IXamlRoot2_Vtbl, 0xbdee0f42_71cb_50c5_829b_4614d98c5794);
impl windows_core::RuntimeType for IXamlRoot2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IXamlRoot2");
}
impl windows_core::RuntimeName for IXamlRoot2 {
    const NAME: &'static str = "Microsoft.UI.Xaml.IXamlRoot2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRoot2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXamlRoot3, IXamlRoot3_Vtbl, 0xb71dbf3b_2e0f_5de0_ac68_f0c1f65114c8);
impl windows_core::RuntimeType for IXamlRoot3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IXamlRoot3");
}
impl windows_core::RuntimeName for IXamlRoot3 {
    const NAME: &'static str = "Microsoft.UI.Xaml.IXamlRoot3";
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRoot3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXamlRoot4, IXamlRoot4_Vtbl, 0x377bec22_632b_52be_b26f_5edf7838e5ca);
impl windows_core::RuntimeType for IXamlRoot4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IXamlRoot4");
}
impl windows_core::RuntimeName for IXamlRoot4 {
    const NAME: &'static str = "Microsoft.UI.Xaml.IXamlRoot4";
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRoot4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXamlRootChangedEventArgs, IXamlRootChangedEventArgs_Vtbl, 0x61d2c719_f8a1_515a_902c_cfa498ba7a7f);
impl windows_core::RuntimeType for IXamlRootChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.IXamlRootChangedEventArgs");
}
impl windows_core::RuntimeName for IXamlRootChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.IXamlRootChangedEventArgs";
}
pub trait IXamlRootChangedEventArgs_Impl: windows_core::IUnknownImpl {}
impl IXamlRootChangedEventArgs_Vtbl {
    pub const fn new<Identity: IXamlRootChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlRootChangedEventArgs, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlRootChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlRootChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl LaunchActivatedEventArgs {
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UWPLaunchActivatedEventArgs(&self) -> windows_core::Result<windows::ApplicationModel::Activation::LaunchActivatedEventArgs> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UWPLaunchActivatedEventArgs)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILaunchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LaunchActivatedEventArgs {
    type Vtable = <ILaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILaunchActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.LaunchActivatedEventArgs";
}
unsafe impl Send for LaunchActivatedEventArgs {}
unsafe impl Sync for LaunchActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceDictionary(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceDictionary, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ResourceDictionary, windows_collections::IIterable < windows_collections::IKeyValuePair < windows_core::IInspectable, windows_core::IInspectable > >, windows_collections::IMap < windows_core::IInspectable, windows_core::IInspectable >, DependencyObject);
impl ResourceDictionary {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn First(&self) -> windows_core::Result<windows_collections::IIterator<windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>>> {
        let this = &windows_core::Interface::cast::<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Lookup<P0>(&self, key: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lookup)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn HasKey<P0>(&self, key: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasKey)(windows_core::Interface::as_raw(this), key.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetView(&self) -> windows_core::Result<windows_collections::IMapView<windows_core::IInspectable, windows_core::IInspectable>> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Insert<P0, P1>(&self, key: P0, value: P1) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Insert)(windows_core::Interface::as_raw(this), key.param().abi(), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Remove<P0>(&self, key: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this), key.param().abi()).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Clear)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Source(&self) -> windows_core::Result<windows::Foundation::Uri> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Source)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn MergedDictionaries(&self) -> windows_core::Result<windows_collections::IVector<Self>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MergedDictionaries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ThemeDictionaries(&self) -> windows_core::Result<windows_collections::IMap<windows_core::IInspectable, windows_core::IInspectable>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ThemeDictionaries)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IResourceDictionaryFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IResourceDictionaryFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    fn IResourceDictionaryFactory<R, F: FnOnce(&IResourceDictionaryFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ResourceDictionary, IResourceDictionaryFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ResourceDictionary {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceDictionary>();
}
unsafe impl windows_core::Interface for ResourceDictionary {
    type Vtable = <IResourceDictionary as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IResourceDictionary as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceDictionary {
    const NAME: &'static str = "Microsoft.UI.Xaml.ResourceDictionary";
}
unsafe impl Send for ResourceDictionary {}
unsafe impl Sync for ResourceDictionary {}
impl IntoIterator for ResourceDictionary {
    type Item = windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
impl IntoIterator for &ResourceDictionary {
    type Item = windows_collections::IKeyValuePair<windows_core::IInspectable, windows_core::IInspectable>;
    type IntoIter = windows_collections::BufferedIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        windows_collections::BufferedIterator::new(self.First().unwrap())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceManagerRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ResourceManagerRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for ResourceManagerRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IResourceManagerRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ResourceManagerRequestedEventArgs {
    type Vtable = <IResourceManagerRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IResourceManagerRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ResourceManagerRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.ResourceManagerRequestedEventArgs";
}
unsafe impl Send for ResourceManagerRequestedEventArgs {}
unsafe impl Sync for ResourceManagerRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RoutedEventArgs {
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OriginalSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IRoutedEventArgsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IRoutedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    fn IRoutedEventArgsFactory<R, F: FnOnce(&IRoutedEventArgsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RoutedEventArgs, IRoutedEventArgsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRoutedEventArgs>();
}
unsafe impl windows_core::Interface for RoutedEventArgs {
    type Vtable = <IRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.RoutedEventArgs";
}
unsafe impl Send for RoutedEventArgs {}
unsafe impl Sync for RoutedEventArgs {}
windows_core::imp::define_interface!(RoutedEventHandler, RoutedEventHandler_Vtbl, 0xdae23d85_69ca_5bdf_805b_6161a3a215cc);
impl windows_core::RuntimeType for RoutedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl RoutedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&RoutedEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<RoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RoutedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct RoutedEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> RoutedEventHandlerBox<F> {
    const VTABLE: RoutedEventHandler_Vtbl = RoutedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<RoutedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<RoutedEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Style(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Style, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Style, DependencyObject);
impl Style {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Style, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsSealed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSealed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TargetType(&self) -> windows_core::Result<windows::UI::Xaml::Interop::TypeName> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TargetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTargetType(&self, value: &windows::UI::Xaml::Interop::TypeName) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTargetType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BasedOn(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BasedOn)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetBasedOn<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBasedOn)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Seal(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Seal)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn CreateInstance(targettype: &windows::UI::Xaml::Interop::TypeName) -> windows_core::Result<Self> {
        Self::IStyleFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(targettype), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IStyleFactory<R, F: FnOnce(&IStyleFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Style, IStyleFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Style {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStyle>();
}
unsafe impl windows_core::Interface for Style {
    type Vtable = <IStyle as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStyle as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Style {
    const NAME: &'static str = "Microsoft.UI.Xaml.Style";
}
unsafe impl Send for Style {}
unsafe impl Sync for Style {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextAlignment(pub i32);
impl TextAlignment {
    pub const Center: Self = Self(0);
    pub const Left: Self = Self(1);
    pub const Start: Self = Self(1);
    pub const Right: Self = Self(2);
    pub const End: Self = Self(2);
    pub const Justify: Self = Self(3);
    pub const DetectFromContent: Self = Self(4);
}
impl windows_core::imp::TypeKind for TextAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.TextAlignment;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.TextAlignment");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextWrapping(pub i32);
impl TextWrapping {
    pub const NoWrap: Self = Self(1);
    pub const Wrap: Self = Self(2);
    pub const WrapWholeWords: Self = Self(3);
}
impl windows_core::imp::TypeKind for TextWrapping {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextWrapping {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.TextWrapping;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.TextWrapping");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Thickness {
    pub Left: f64,
    pub Top: f64,
    pub Right: f64,
    pub Bottom: f64,
}
impl windows_core::imp::TypeKind for Thickness {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Thickness {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Xaml.Thickness;f8;f8;f8;f8)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Thickness");
}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UIElement(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(UIElement, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(UIElement, super::Composition::IAnimationObject, super::Composition::IVisualElement, super::Composition::IVisualElement2, DependencyObject);
#[cfg(feature = "UI_Composition")]
impl UIElement {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowDrop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowDrop)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Opacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RenderTransformOrigin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRenderTransformOrigin)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHitTestVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsHitTestVisible)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<Visibility> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visibility)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: Visibility) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVisibility)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RenderSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseLayoutRounding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseLayoutRounding)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTapEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsTapEnabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsDoubleTapEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanDrag)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCanDrag)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRightTapEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsRightTapEnabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHoldingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsHoldingEnabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<Input::ManipulationModes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManipulationMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: Input::ManipulationModes) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetManipulationMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<Input::Pointer>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerCaptures)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<Media::XamlLight>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lights)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanBeScrollAnchor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAccessKeyScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsAccessKeyScope)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccessKeyScopeOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccessKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAccessKey)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<Input::KeyTipPlacementMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyTipPlacementMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyTipVerticalOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyTipTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetKeyTipTarget)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<Input::XYFocusKeyboardNavigationMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<Input::XYFocusNavigationStrategy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<Input::KeyboardAccelerator>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyboardAccelerators)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<Input::KeyboardAcceleratorPlacementMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<Input::KeyboardNavigationMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TabFocusNavigation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTabFocusNavigation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Translation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTranslation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Rotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Scale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetScale)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransformMatrix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTransformMatrix)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CenterPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenterPoint)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RotationAxis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRotationAxis)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActualSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<XamlRoot> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XamlRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<XamlRoot>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetXamlRoot)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RasterizationScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRasterizationScale)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<FocusState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FocusState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseSystemFocusVisuals)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusLeft)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusLeft)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusRight)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusRight)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusUp)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusUp)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusDown)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusDown)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTabStop)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsTabStop)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TabIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTabIndex)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).KeyUp)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).KeyDown)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).GotFocus)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).LostFocus)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DragStarting)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DropCompleted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CharacterReceived)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerPressed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerMoved)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerReleased)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerEntered)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerExited)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerCaptureLost)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerCanceled)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerWheelChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Tapped)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DoubleTapped)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Holding)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ContextRequested)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ContextCanceled)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).RightTapped)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationStarting)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationInertiaStarting)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationStarted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationDelta)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationCompleted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).AccessKeyInvoked)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).GettingFocus)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).LosingFocus)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).NoFocusCandidateFound)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PreviewKeyDown)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PreviewKeyUp)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BringIntoViewRequested)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Measure)(windows_core::Interface::as_raw(self), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Arrange)(windows_core::Interface::as_raw(self), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Input::Pointer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CapturePointer)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::Pointer>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReleasePointerCapture)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReleasePointerCaptures)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InvalidateMeasure)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).InvalidateArrange)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UpdateLayout)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CancelDirectManipulations)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::Input::PointerPoint>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartDragAsync)(windows_core::Interface::as_raw(self), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StartBringIntoView)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(self), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: FocusState) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Focus)(windows_core::Interface::as_raw(self), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<DependencyObject>> {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryStartDirectManipulation<P0>(value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Input::Pointer>,
    {
        Self::IUIElementStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryStartDirectManipulation)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn RegisterAsScrollPort<P0>(element: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        Self::IUIElementStatics(|this| unsafe { (windows_core::Interface::vtable(this).RegisterAsScrollPort)(windows_core::Interface::as_raw(this), element.param().abi()).ok() })
    }
    fn IUIElementStatics<R, F: FnOnce(&IUIElementStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UIElement, IUIElementStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for UIElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUIElement>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for UIElement {
    type Vtable = <IUIElement as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUIElement as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for UIElement {
    const NAME: &'static str = "Microsoft.UI.Xaml.UIElement";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for UIElement {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for UIElement {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnhandledExceptionEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UnhandledExceptionEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl UnhandledExceptionEventArgs {
    pub fn Exception(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Exception)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Message(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Message)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for UnhandledExceptionEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUnhandledExceptionEventArgs>();
}
unsafe impl windows_core::Interface for UnhandledExceptionEventArgs {
    type Vtable = <IUnhandledExceptionEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUnhandledExceptionEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UnhandledExceptionEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.UnhandledExceptionEventArgs";
}
unsafe impl Send for UnhandledExceptionEventArgs {}
unsafe impl Sync for UnhandledExceptionEventArgs {}
windows_core::imp::define_interface!(UnhandledExceptionEventHandler, UnhandledExceptionEventHandler_Vtbl, 0x3427c1b6_5eca_5631_84b8_5bae732fb67f);
impl windows_core::RuntimeType for UnhandledExceptionEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl UnhandledExceptionEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<UnhandledExceptionEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&UnhandledExceptionEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<UnhandledExceptionEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct UnhandledExceptionEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct UnhandledExceptionEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<UnhandledExceptionEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<UnhandledExceptionEventArgs>) -> windows_core::Result<()> + Send + 'static> UnhandledExceptionEventHandlerBox<F> {
    const VTABLE: UnhandledExceptionEventHandler_Vtbl = UnhandledExceptionEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<UnhandledExceptionEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<UnhandledExceptionEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<UnhandledExceptionEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<UnhandledExceptionEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerticalAlignment(pub i32);
impl VerticalAlignment {
    pub const Top: Self = Self(0);
    pub const Center: Self = Self(1);
    pub const Bottom: Self = Self(2);
    pub const Stretch: Self = Self(3);
}
impl windows_core::imp::TypeKind for VerticalAlignment {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for VerticalAlignment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.VerticalAlignment;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.VerticalAlignment");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Visibility(pub i32);
impl Visibility {
    pub const Visible: Self = Self(0);
    pub const Collapsed: Self = Self(1);
}
impl windows_core::imp::TypeKind for Visibility {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Visibility {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Visibility;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Visibility");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Window(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Window, windows_core::IUnknown, windows_core::IInspectable);
impl Window {
    pub fn Bounds(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Bounds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Visible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn Content(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContent)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn CoreWindow(&self) -> windows_core::Result<windows::UI::Core::CoreWindow> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoreWindow)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dispatcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Title)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTitle)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ExtendsContentIntoTitleBar(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendsContentIntoTitleBar)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetExtendsContentIntoTitleBar(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExtendsContentIntoTitleBar)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Activated<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<WindowActivatedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<windows_core::IInspectable, WindowActivatedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Activated)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveActivated))
        }
    }
    pub fn Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<WindowEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<windows_core::IInspectable, WindowEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Closed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveClosed))
        }
    }
    pub fn SizeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<WindowSizeChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<windows_core::IInspectable, WindowSizeChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SizeChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveSizeChanged))
        }
    }
    pub fn VisibilityChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<WindowVisibilityChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<windows_core::IInspectable, WindowVisibilityChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).VisibilityChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveVisibilityChanged))
        }
    }
    pub fn Activate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetTitleBar<P0>(&self, titlebar: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<UIElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTitleBar)(windows_core::Interface::as_raw(self), titlebar.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SystemBackdrop(&self) -> windows_core::Result<Media::SystemBackdrop> {
        let this = &windows_core::Interface::cast::<IWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemBackdrop)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetSystemBackdrop<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Media::SystemBackdrop>,
    {
        let this = &windows_core::Interface::cast::<IWindow2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSystemBackdrop)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Windowing")]
    pub fn AppWindow(&self) -> windows_core::Result<super::Windowing::AppWindow> {
        let this = &windows_core::Interface::cast::<IWindow2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppWindow)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IWindowFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IWindowFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn Current() -> windows_core::Result<Self> {
        Self::IWindowStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Current)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IWindowFactory<R, F: FnOnce(&IWindowFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Window, IWindowFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IWindowStatics<R, F: FnOnce(&IWindowStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Window, IWindowStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Window {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindow>();
}
unsafe impl windows_core::Interface for Window {
    type Vtable = <IWindow as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindow as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Window {
    const NAME: &'static str = "Microsoft.UI.Xaml.Window";
}
unsafe impl Send for Window {}
unsafe impl Sync for Window {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowActivatedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for WindowActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowActivatedEventArgs>();
}
unsafe impl windows_core::Interface for WindowActivatedEventArgs {
    type Vtable = <IWindowActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowActivatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.WindowActivatedEventArgs";
}
unsafe impl Send for WindowActivatedEventArgs {}
unsafe impl Sync for WindowActivatedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for WindowEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowEventArgs>();
}
unsafe impl windows_core::Interface for WindowEventArgs {
    type Vtable = <IWindowEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.WindowEventArgs";
}
unsafe impl Send for WindowEventArgs {}
unsafe impl Sync for WindowEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowSizeChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowSizeChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowSizeChangedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Size(&self) -> windows_core::Result<windows::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowSizeChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowSizeChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowSizeChangedEventArgs {
    type Vtable = <IWindowSizeChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowSizeChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowSizeChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.WindowSizeChangedEventArgs";
}
unsafe impl Send for WindowSizeChangedEventArgs {}
unsafe impl Sync for WindowSizeChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowVisibilityChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowVisibilityChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowVisibilityChangedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Visible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Visible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowVisibilityChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowVisibilityChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowVisibilityChangedEventArgs {
    type Vtable = <IWindowVisibilityChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowVisibilityChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowVisibilityChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.WindowVisibilityChangedEventArgs";
}
unsafe impl Send for WindowVisibilityChangedEventArgs {}
unsafe impl Sync for WindowVisibilityChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlRoot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XamlRoot, windows_core::IUnknown, windows_core::IInspectable);
impl XamlRoot {
    #[cfg(feature = "UI_Composition")]
    pub fn Content(&self) -> windows_core::Result<UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Content)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<windows::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RasterizationScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsHostVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHostVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Changed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<XamlRootChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, XamlRootChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Changed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveChanged))
        }
    }
}
impl windows_core::RuntimeType for XamlRoot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXamlRoot>();
}
unsafe impl windows_core::Interface for XamlRoot {
    type Vtable = <IXamlRoot as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlRoot as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XamlRoot {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlRoot";
}
unsafe impl Send for XamlRoot {}
unsafe impl Sync for XamlRoot {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlRootChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XamlRootChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for XamlRootChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXamlRootChangedEventArgs>();
}
unsafe impl windows_core::Interface for XamlRootChangedEventArgs {
    type Vtable = <IXamlRootChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlRootChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XamlRootChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.XamlRootChangedEventArgs";
}
unsafe impl Send for XamlRootChangedEventArgs {}
unsafe impl Sync for XamlRootChangedEventArgs {}
