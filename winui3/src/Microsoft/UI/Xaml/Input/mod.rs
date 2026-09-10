#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessKeyDisplayDismissedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccessKeyDisplayDismissedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccessKeyDisplayDismissedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessKeyDisplayDismissedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AccessKeyDisplayDismissedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccessKeyDisplayDismissedEventArgs>();
}
unsafe impl windows_core::Interface for AccessKeyDisplayDismissedEventArgs {
    type Vtable = <IAccessKeyDisplayDismissedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccessKeyDisplayDismissedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyDisplayDismissedEventArgs";
}
unsafe impl Send for AccessKeyDisplayDismissedEventArgs {}
unsafe impl Sync for AccessKeyDisplayDismissedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessKeyDisplayRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccessKeyDisplayRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccessKeyDisplayRequestedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessKeyDisplayRequestedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn PressedKeys(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PressedKeys)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for AccessKeyDisplayRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccessKeyDisplayRequestedEventArgs>();
}
unsafe impl windows_core::Interface for AccessKeyDisplayRequestedEventArgs {
    type Vtable = <IAccessKeyDisplayRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccessKeyDisplayRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyDisplayRequestedEventArgs";
}
unsafe impl Send for AccessKeyDisplayRequestedEventArgs {}
unsafe impl Sync for AccessKeyDisplayRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessKeyInvokedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccessKeyInvokedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl AccessKeyInvokedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessKeyInvokedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl windows_core::RuntimeType for AccessKeyInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccessKeyInvokedEventArgs>();
}
unsafe impl windows_core::Interface for AccessKeyInvokedEventArgs {
    type Vtable = <IAccessKeyInvokedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccessKeyInvokedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccessKeyInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyInvokedEventArgs";
}
unsafe impl Send for AccessKeyInvokedEventArgs {}
unsafe impl Sync for AccessKeyInvokedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccessKeyManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AccessKeyManager, windows_core::IUnknown, windows_core::IInspectable);
impl AccessKeyManager {
    pub fn IsDisplayModeEnabled() -> windows_core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDisplayModeEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn AreKeyTipsEnabled() -> windows_core::Result<bool> {
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AreKeyTipsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn SetAreKeyTipsEnabled(value: bool) -> windows_core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).SetAreKeyTipsEnabled)(windows_core::Interface::as_raw(this), value).ok() })
    }
    pub fn IsDisplayModeEnabledChanged<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<windows_core::IInspectable, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        Self::IAccessKeyManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).IsDisplayModeEnabledChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveIsDisplayModeEnabledChanged))
        })
    }
    pub fn ExitDisplayMode() -> windows_core::Result<()> {
        Self::IAccessKeyManagerStatics(|this| unsafe { (windows_core::Interface::vtable(this).ExitDisplayMode)(windows_core::Interface::as_raw(this)).ok() })
    }
    pub fn EnterDisplayModeForXamlRoot<P0>(xamlroot: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::XamlRoot>,
    {
        Self::IAccessKeyManagerStatics2(|this| unsafe { (windows_core::Interface::vtable(this).EnterDisplayModeForXamlRoot)(windows_core::Interface::as_raw(this), xamlroot.param().abi()).ok() })
    }
    fn IAccessKeyManagerStatics<R, F: FnOnce(&IAccessKeyManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IAccessKeyManagerStatics2<R, F: FnOnce(&IAccessKeyManagerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AccessKeyManager, IAccessKeyManagerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AccessKeyManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAccessKeyManager>();
}
unsafe impl windows_core::Interface for AccessKeyManager {
    type Vtable = <IAccessKeyManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAccessKeyManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AccessKeyManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.AccessKeyManager";
}
unsafe impl Send for AccessKeyManager {}
unsafe impl Sync for AccessKeyManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanExecuteRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CanExecuteRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CanExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanExecute(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanExecute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanExecute(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCanExecute)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for CanExecuteRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICanExecuteRequestedEventArgs>();
}
unsafe impl windows_core::Interface for CanExecuteRequestedEventArgs {
    type Vtable = <ICanExecuteRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICanExecuteRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CanExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.CanExecuteRequestedEventArgs";
}
unsafe impl Send for CanExecuteRequestedEventArgs {}
unsafe impl Sync for CanExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharacterReceivedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CharacterReceivedRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CharacterReceivedRoutedEventArgs, super::RoutedEventArgs);
impl CharacterReceivedRoutedEventArgs {
    pub fn Character(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Character)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<windows::UI::Core::CorePhysicalKeyStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CharacterReceivedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICharacterReceivedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for CharacterReceivedRoutedEventArgs {
    type Vtable = <ICharacterReceivedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICharacterReceivedRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.CharacterReceivedRoutedEventArgs";
}
unsafe impl Send for CharacterReceivedRoutedEventArgs {}
unsafe impl Sync for CharacterReceivedRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContextRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ContextRequestedEventArgs, super::RoutedEventArgs);
impl ContextRequestedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ContextRequestedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
    #[cfg(feature = "UI_Composition")]
    pub fn TryGetPosition<P0>(&self, relativeto: P0, point: &mut windows::Foundation::Point) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryGetPosition)(windows_core::Interface::as_raw(self), relativeto.param().abi(), point, &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ContextRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContextRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ContextRequestedEventArgs {
    type Vtable = <IContextRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContextRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContextRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ContextRequestedEventArgs";
}
unsafe impl Send for ContextRequestedEventArgs {}
unsafe impl Sync for ContextRequestedEventArgs {}
windows_core::imp::define_interface!(DoubleTappedEventHandler, DoubleTappedEventHandler_Vtbl, 0xf7a501b9_e277_5611_87b0_0e0607622183);
impl windows_core::RuntimeType for DoubleTappedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl DoubleTappedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&DoubleTappedEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<DoubleTappedRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct DoubleTappedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct DoubleTappedEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> DoubleTappedEventHandlerBox<F> {
    const VTABLE: DoubleTappedEventHandler_Vtbl = DoubleTappedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<DoubleTappedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<DoubleTappedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<DoubleTappedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<DoubleTappedEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DoubleTappedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DoubleTappedRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DoubleTappedRoutedEventArgs, super::RoutedEventArgs);
impl DoubleTappedRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DoubleTappedRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "UI_Composition")]
    pub fn GetPosition<P0>(&self, relativeto: P0) -> windows_core::Result<windows::Foundation::Point>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), relativeto.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for DoubleTappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDoubleTappedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for DoubleTappedRoutedEventArgs {
    type Vtable = <IDoubleTappedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDoubleTappedRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.DoubleTappedRoutedEventArgs";
}
unsafe impl Send for DoubleTappedRoutedEventArgs {}
unsafe impl Sync for DoubleTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecuteRequestedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExecuteRequestedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ExecuteRequestedEventArgs {
    pub fn Parameter(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Parameter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ExecuteRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExecuteRequestedEventArgs>();
}
unsafe impl windows_core::Interface for ExecuteRequestedEventArgs {
    type Vtable = <IExecuteRequestedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExecuteRequestedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ExecuteRequestedEventArgs";
}
unsafe impl Send for ExecuteRequestedEventArgs {}
unsafe impl Sync for ExecuteRequestedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FindNextElementOptions(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FindNextElementOptions, windows_core::IUnknown, windows_core::IInspectable);
impl FindNextElementOptions {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FindNextElementOptions, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn SearchRoot(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SearchRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSearchRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSearchRoot)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ExclusionRect(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExclusionRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetExclusionRect(&self, value: windows::Foundation::Rect) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExclusionRect)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn HintRect(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HintRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHintRect(&self, value: windows::Foundation::Rect) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHintRect)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn XYFocusNavigationStrategyOverride(&self) -> windows_core::Result<XYFocusNavigationStrategyOverride> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XYFocusNavigationStrategyOverride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetXYFocusNavigationStrategyOverride)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for FindNextElementOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFindNextElementOptions>();
}
unsafe impl windows_core::Interface for FindNextElementOptions {
    type Vtable = <IFindNextElementOptions as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFindNextElementOptions as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FindNextElementOptions {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FindNextElementOptions";
}
unsafe impl Send for FindNextElementOptions {}
unsafe impl Sync for FindNextElementOptions {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusInputDeviceKind(pub i32);
impl FocusInputDeviceKind {
    pub const None: Self = Self(0);
    pub const Mouse: Self = Self(1);
    pub const Touch: Self = Self(2);
    pub const Pen: Self = Self(3);
    pub const Keyboard: Self = Self(4);
    pub const GameController: Self = Self(5);
}
impl windows_core::imp::TypeKind for FocusInputDeviceKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusInputDeviceKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.FocusInputDeviceKind;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.FocusInputDeviceKind");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusManager(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusManager, windows_core::IUnknown, windows_core::IInspectable);
impl FocusManager {
    pub fn GotFocus<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<FocusManagerGotFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<FocusManagerGotFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        })
    }
    pub fn LostFocus<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<FocusManagerLostFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<FocusManagerLostFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        })
    }
    pub fn GettingFocus<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        })
    }
    pub fn LosingFocus<F>(handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        })
    }
    pub fn TryMoveFocusAsync(focusnavigationdirection: FocusNavigationDirection) -> windows_core::Result<windows_future::IAsyncOperation<FocusMovementResult>> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryMoveFocusAsync)(windows_core::Interface::as_raw(this), focusnavigationdirection, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn TryMoveFocusWithOptionsAsync<P1>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: P1) -> windows_core::Result<windows_future::IAsyncOperation<FocusMovementResult>>
    where
        P1: windows_core::Param<FindNextElementOptions>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryMoveFocusWithOptionsAsync)(windows_core::Interface::as_raw(this), focusnavigationdirection, focusnavigationoptions.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn TryMoveFocusWithOptions<P1>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: P1) -> windows_core::Result<bool>
    where
        P1: windows_core::Param<FindNextElementOptions>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryMoveFocusWithOptions)(windows_core::Interface::as_raw(this), focusnavigationdirection, focusnavigationoptions.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn FindNextElement(focusnavigationdirection: FocusNavigationDirection) -> windows_core::Result<super::DependencyObject> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindNextElement)(windows_core::Interface::as_raw(this), focusnavigationdirection, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn FindFirstFocusableElement<P0>(searchscope: P0) -> windows_core::Result<super::DependencyObject>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindFirstFocusableElement)(windows_core::Interface::as_raw(this), searchscope.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn FindLastFocusableElement<P0>(searchscope: P0) -> windows_core::Result<super::DependencyObject>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindLastFocusableElement)(windows_core::Interface::as_raw(this), searchscope.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn FindNextElementWithOptions<P1>(focusnavigationdirection: FocusNavigationDirection, focusnavigationoptions: P1) -> windows_core::Result<super::DependencyObject>
    where
        P1: windows_core::Param<FindNextElementOptions>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindNextElementWithOptions)(windows_core::Interface::as_raw(this), focusnavigationdirection, focusnavigationoptions.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn FindNextFocusableElement(focusnavigationdirection: FocusNavigationDirection) -> windows_core::Result<super::UIElement> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindNextFocusableElement)(windows_core::Interface::as_raw(this), focusnavigationdirection, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Composition")]
    pub fn FindNextFocusableElementWithHint(focusnavigationdirection: FocusNavigationDirection, hintrect: windows::Foundation::Rect) -> windows_core::Result<super::UIElement> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindNextFocusableElementWithHint)(windows_core::Interface::as_raw(this), focusnavigationdirection, hintrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn TryMoveFocus(focusnavigationdirection: FocusNavigationDirection) -> windows_core::Result<bool> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryMoveFocus)(windows_core::Interface::as_raw(this), focusnavigationdirection, &mut result__).map(|| result__)
        })
    }
    pub fn GetFocusedElement() -> windows_core::Result<windows_core::IInspectable> {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFocusedElement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetFocusedElementWithRoot<P0>(xamlroot: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<super::XamlRoot>,
    {
        Self::IFocusManagerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFocusedElementWithRoot)(windows_core::Interface::as_raw(this), xamlroot.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFocusManagerStatics<R, F: FnOnce(&IFocusManagerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FocusManager, IFocusManagerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FocusManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusManager>();
}
unsafe impl windows_core::Interface for FocusManager {
    type Vtable = <IFocusManager as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusManager as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManager";
}
unsafe impl Send for FocusManager {}
unsafe impl Sync for FocusManager {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusManagerGotFocusEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusManagerGotFocusEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FocusManagerGotFocusEventArgs {
    pub fn NewFocusedElement(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FocusManagerGotFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusManagerGotFocusEventArgs>();
}
unsafe impl windows_core::Interface for FocusManagerGotFocusEventArgs {
    type Vtable = <IFocusManagerGotFocusEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusManagerGotFocusEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManagerGotFocusEventArgs";
}
unsafe impl Send for FocusManagerGotFocusEventArgs {}
unsafe impl Sync for FocusManagerGotFocusEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusManagerLostFocusEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusManagerLostFocusEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FocusManagerLostFocusEventArgs {
    pub fn OldFocusedElement(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OldFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FocusManagerLostFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusManagerLostFocusEventArgs>();
}
unsafe impl windows_core::Interface for FocusManagerLostFocusEventArgs {
    type Vtable = <IFocusManagerLostFocusEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusManagerLostFocusEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusManagerLostFocusEventArgs";
}
unsafe impl Send for FocusManagerLostFocusEventArgs {}
unsafe impl Sync for FocusManagerLostFocusEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusMovementResult(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusMovementResult, windows_core::IUnknown, windows_core::IInspectable);
impl FocusMovementResult {
    pub fn Succeeded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Succeeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for FocusMovementResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusMovementResult>();
}
unsafe impl windows_core::Interface for FocusMovementResult {
    type Vtable = <IFocusMovementResult as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusMovementResult as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusMovementResult {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.FocusMovementResult";
}
unsafe impl Send for FocusMovementResult {}
unsafe impl Sync for FocusMovementResult {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusNavigationDirection(pub i32);
impl FocusNavigationDirection {
    pub const Next: Self = Self(0);
    pub const Previous: Self = Self(1);
    pub const Up: Self = Self(2);
    pub const Down: Self = Self(3);
    pub const Left: Self = Self(4);
    pub const Right: Self = Self(5);
    pub const None: Self = Self(6);
}
impl windows_core::imp::TypeKind for FocusNavigationDirection {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusNavigationDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.FocusNavigationDirection;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.FocusNavigationDirection");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GettingFocusEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GettingFocusEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(GettingFocusEventArgs, super::RoutedEventArgs);
impl GettingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OldFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn NewFocusedElement(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetNewFocusedElement<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNewFocusedElement)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Direction(&self) -> windows_core::Result<FocusNavigationDirection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub fn InputDevice(&self) -> windows_core::Result<FocusInputDeviceKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InputDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCancel)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TryCancel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryCancel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TrySetNewFocusedElement<P0>(&self, element: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrySetNewFocusedElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GettingFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGettingFocusEventArgs>();
}
unsafe impl windows_core::Interface for GettingFocusEventArgs {
    type Vtable = <IGettingFocusEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGettingFocusEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GettingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.GettingFocusEventArgs";
}
unsafe impl Send for GettingFocusEventArgs {}
unsafe impl Sync for GettingFocusEventArgs {}
windows_core::imp::define_interface!(HoldingEventHandler, HoldingEventHandler_Vtbl, 0xfe23c5bd_4984_56b6_b92b_fc9d1216b24e);
impl windows_core::RuntimeType for HoldingEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl HoldingEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&HoldingEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<HoldingRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct HoldingEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct HoldingEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> HoldingEventHandlerBox<F> {
    const VTABLE: HoldingEventHandler_Vtbl = HoldingEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<HoldingEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<HoldingEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<HoldingEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<HoldingEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoldingRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HoldingRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(HoldingRoutedEventArgs, super::RoutedEventArgs);
impl HoldingRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<HoldingRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn HoldingState(&self) -> windows_core::Result<super::super::Input::HoldingState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HoldingState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "UI_Composition")]
    pub fn GetPosition<P0>(&self, relativeto: P0) -> windows_core::Result<windows::Foundation::Point>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), relativeto.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for HoldingRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHoldingRoutedEventArgs>();
}
unsafe impl windows_core::Interface for HoldingRoutedEventArgs {
    type Vtable = <IHoldingRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHoldingRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HoldingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.HoldingRoutedEventArgs";
}
unsafe impl Send for HoldingRoutedEventArgs {}
unsafe impl Sync for HoldingRoutedEventArgs {}
windows_core::imp::define_interface!(IAccessKeyDisplayDismissedEventArgs, IAccessKeyDisplayDismissedEventArgs_Vtbl, 0x125a83d8_7f86_5ea9_9063_b9407e644587);
impl windows_core::RuntimeType for IAccessKeyDisplayDismissedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IAccessKeyDisplayDismissedEventArgs");
}
impl windows_core::RuntimeName for IAccessKeyDisplayDismissedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IAccessKeyDisplayDismissedEventArgs";
}
pub trait IAccessKeyDisplayDismissedEventArgs_Impl: windows_core::IUnknownImpl {}
impl IAccessKeyDisplayDismissedEventArgs_Vtbl {
    pub const fn new<Identity: IAccessKeyDisplayDismissedEventArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccessKeyDisplayDismissedEventArgs, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessKeyDisplayDismissedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayDismissedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IAccessKeyDisplayRequestedEventArgs, IAccessKeyDisplayRequestedEventArgs_Vtbl, 0xc4ed84d8_2b27_59b1_9cf0_7f9164de58cb);
impl windows_core::RuntimeType for IAccessKeyDisplayRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IAccessKeyDisplayRequestedEventArgs");
}
impl windows_core::RuntimeName for IAccessKeyDisplayRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IAccessKeyDisplayRequestedEventArgs";
}
pub trait IAccessKeyDisplayRequestedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PressedKeys(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IAccessKeyDisplayRequestedEventArgs_Vtbl {
    pub const fn new<Identity: IAccessKeyDisplayRequestedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PressedKeys<Identity: IAccessKeyDisplayRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessKeyDisplayRequestedEventArgs_Impl::PressedKeys(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccessKeyDisplayRequestedEventArgs, OFFSET>(),
            PressedKeys: PressedKeys::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessKeyDisplayRequestedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyDisplayRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PressedKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessKeyInvokedEventArgs, IAccessKeyInvokedEventArgs_Vtbl, 0xd00c11a4_f9fb_5707_9692_98b80bb8546d);
impl windows_core::RuntimeType for IAccessKeyInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IAccessKeyInvokedEventArgs");
}
impl windows_core::RuntimeName for IAccessKeyInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IAccessKeyInvokedEventArgs";
}
pub trait IAccessKeyInvokedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IAccessKeyInvokedEventArgs_Vtbl {
    pub const fn new<Identity: IAccessKeyInvokedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IAccessKeyInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessKeyInvokedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IAccessKeyInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessKeyInvokedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccessKeyInvokedEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessKeyInvokedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyInvokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessKeyManager, IAccessKeyManager_Vtbl, 0x8f2a4402_a635_53dc_bc17_da911eabaade);
impl windows_core::RuntimeType for IAccessKeyManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IAccessKeyManager");
}
impl windows_core::RuntimeName for IAccessKeyManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IAccessKeyManager";
}
pub trait IAccessKeyManager_Impl: windows_core::IUnknownImpl {}
impl IAccessKeyManager_Vtbl {
    pub const fn new<Identity: IAccessKeyManager_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccessKeyManager, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessKeyManager as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IAccessKeyManagerStatics, IAccessKeyManagerStatics_Vtbl, 0x3375aef7_742f_5e84_b76f_c187e08253bf);
impl windows_core::RuntimeType for IAccessKeyManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IAccessKeyManagerStatics");
}
impl windows_core::RuntimeName for IAccessKeyManagerStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IAccessKeyManagerStatics";
}
pub trait IAccessKeyManagerStatics_Impl: windows_core::IUnknownImpl {
    fn IsDisplayModeEnabled(&self) -> windows_core::Result<bool>;
    fn AreKeyTipsEnabled(&self) -> windows_core::Result<bool>;
    fn SetAreKeyTipsEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn IsDisplayModeEnabledChanged(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<windows_core::IInspectable, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveIsDisplayModeEnabledChanged(&self, token: i64) -> windows_core::Result<()>;
    fn ExitDisplayMode(&self) -> windows_core::Result<()>;
}
impl IAccessKeyManagerStatics_Vtbl {
    pub const fn new<Identity: IAccessKeyManagerStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDisplayModeEnabled<Identity: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessKeyManagerStatics_Impl::IsDisplayModeEnabled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AreKeyTipsEnabled<Identity: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessKeyManagerStatics_Impl::AreKeyTipsEnabled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAreKeyTipsEnabled<Identity: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessKeyManagerStatics_Impl::SetAreKeyTipsEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn IsDisplayModeEnabledChanged<Identity: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessKeyManagerStatics_Impl::IsDisplayModeEnabledChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveIsDisplayModeEnabledChanged<Identity: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessKeyManagerStatics_Impl::RemoveIsDisplayModeEnabledChanged(this, token).into()
            }
        }
        unsafe extern "system" fn ExitDisplayMode<Identity: IAccessKeyManagerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessKeyManagerStatics_Impl::ExitDisplayMode(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccessKeyManagerStatics, OFFSET>(),
            IsDisplayModeEnabled: IsDisplayModeEnabled::<Identity, OFFSET>,
            AreKeyTipsEnabled: AreKeyTipsEnabled::<Identity, OFFSET>,
            SetAreKeyTipsEnabled: SetAreKeyTipsEnabled::<Identity, OFFSET>,
            IsDisplayModeEnabledChanged: IsDisplayModeEnabledChanged::<Identity, OFFSET>,
            RemoveIsDisplayModeEnabledChanged: RemoveIsDisplayModeEnabledChanged::<Identity, OFFSET>,
            ExitDisplayMode: ExitDisplayMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessKeyManagerStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsDisplayModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AreKeyTipsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAreKeyTipsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsDisplayModeEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveIsDisplayModeEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ExitDisplayMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAccessKeyManagerStatics2, IAccessKeyManagerStatics2_Vtbl, 0x512c9f63_24ad_5df2_b8ed_472406db31c0);
impl windows_core::RuntimeType for IAccessKeyManagerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IAccessKeyManagerStatics2");
}
impl windows_core::RuntimeName for IAccessKeyManagerStatics2 {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IAccessKeyManagerStatics2";
}
pub trait IAccessKeyManagerStatics2_Impl: windows_core::IUnknownImpl {
    fn EnterDisplayModeForXamlRoot(&self, XamlRoot: windows_core::Ref<super::XamlRoot>) -> windows_core::Result<()>;
}
impl IAccessKeyManagerStatics2_Vtbl {
    pub const fn new<Identity: IAccessKeyManagerStatics2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnterDisplayModeForXamlRoot<Identity: IAccessKeyManagerStatics2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xamlroot: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessKeyManagerStatics2_Impl::EnterDisplayModeForXamlRoot(this, core::mem::transmute_copy(&xamlroot)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IAccessKeyManagerStatics2, OFFSET>(),
            EnterDisplayModeForXamlRoot: EnterDisplayModeForXamlRoot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessKeyManagerStatics2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessKeyManagerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnterDisplayModeForXamlRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICanExecuteRequestedEventArgs, ICanExecuteRequestedEventArgs_Vtbl, 0xe4bf6d7d_f6eb_53ca_a2d4_c741ec871e38);
impl windows_core::RuntimeType for ICanExecuteRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.ICanExecuteRequestedEventArgs");
}
impl windows_core::RuntimeName for ICanExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ICanExecuteRequestedEventArgs";
}
pub trait ICanExecuteRequestedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Parameter(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn CanExecute(&self) -> windows_core::Result<bool>;
    fn SetCanExecute(&self, value: bool) -> windows_core::Result<()>;
}
impl ICanExecuteRequestedEventArgs_Vtbl {
    pub const fn new<Identity: ICanExecuteRequestedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Parameter<Identity: ICanExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICanExecuteRequestedEventArgs_Impl::Parameter(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanExecute<Identity: ICanExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICanExecuteRequestedEventArgs_Impl::CanExecute(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCanExecute<Identity: ICanExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICanExecuteRequestedEventArgs_Impl::SetCanExecute(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICanExecuteRequestedEventArgs, OFFSET>(),
            Parameter: Parameter::<Identity, OFFSET>,
            CanExecute: CanExecute::<Identity, OFFSET>,
            SetCanExecute: SetCanExecute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICanExecuteRequestedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanExecuteRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCanExecute: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICharacterReceivedRoutedEventArgs, ICharacterReceivedRoutedEventArgs_Vtbl, 0xe26ca5bb_34c3_5c1e_9a16_00b80b07a899);
impl windows_core::RuntimeType for ICharacterReceivedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.ICharacterReceivedRoutedEventArgs");
}
impl windows_core::RuntimeName for ICharacterReceivedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ICharacterReceivedRoutedEventArgs";
}
pub trait ICharacterReceivedRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Character(&self) -> windows_core::Result<u16>;
    fn KeyStatus(&self) -> windows_core::Result<windows::UI::Core::CorePhysicalKeyStatus>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl ICharacterReceivedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Character<Identity: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICharacterReceivedRoutedEventArgs_Impl::Character(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyStatus<Identity: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::UI::Core::CorePhysicalKeyStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICharacterReceivedRoutedEventArgs_Impl::KeyStatus(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICharacterReceivedRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: ICharacterReceivedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICharacterReceivedRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICharacterReceivedRoutedEventArgs, OFFSET>(),
            Character: Character::<Identity, OFFSET>,
            KeyStatus: KeyStatus::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICharacterReceivedRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Character: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Core::CorePhysicalKeyStatus) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICommand, ICommand_Vtbl, 0xe5af3542_ca67_4081_995b_709dd13792df);
impl windows_core::RuntimeType for ICommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.ICommand");
}
windows_core::imp::interface_hierarchy!(ICommand, windows_core::IUnknown, windows_core::IInspectable);
impl ICommand {
    pub fn CanExecuteChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CanExecuteChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCanExecuteChanged))
        }
    }
    pub fn CanExecute<P0>(&self, parameter: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanExecute)(windows_core::Interface::as_raw(self), parameter.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Execute<P0>(&self, parameter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).Execute)(windows_core::Interface::as_raw(self), parameter.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for ICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ICommand";
}
pub trait ICommand_Impl: windows_core::IUnknownImpl {
    fn CanExecuteChanged(&self, handler: windows_core::Ref<windows::Foundation::EventHandler<windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveCanExecuteChanged(&self, token: i64) -> windows_core::Result<()>;
    fn CanExecute(&self, parameter: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<bool>;
    fn Execute(&self, parameter: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>;
}
impl ICommand_Vtbl {
    pub const fn new<Identity: ICommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanExecuteChanged<Identity: ICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICommand_Impl::CanExecuteChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveCanExecuteChanged<Identity: ICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommand_Impl::RemoveCanExecuteChanged(this, token).into()
            }
        }
        unsafe extern "system" fn CanExecute<Identity: ICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameter: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICommand_Impl::CanExecute(this, core::mem::transmute_copy(&parameter)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Execute<Identity: ICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parameter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICommand_Impl::Execute(this, core::mem::transmute_copy(&parameter)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICommand, OFFSET>(),
            CanExecuteChanged: CanExecuteChanged::<Identity, OFFSET>,
            RemoveCanExecuteChanged: RemoveCanExecuteChanged::<Identity, OFFSET>,
            CanExecute: CanExecute::<Identity, OFFSET>,
            Execute: Execute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICommand as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CanExecuteChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCanExecuteChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CanExecute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Execute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextRequestedEventArgs, IContextRequestedEventArgs_Vtbl, 0xbcedcb98_77b5_53c0_802e_fd52f3806e51);
impl windows_core::RuntimeType for IContextRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IContextRequestedEventArgs");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IContextRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IContextRequestedEventArgs";
}
#[cfg(feature = "UI_Composition")]
pub trait IContextRequestedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn TryGetPosition(&self, relativeTo: windows_core::Ref<super::UIElement>, point: &mut windows::Foundation::Point) -> windows_core::Result<bool>;
}
#[cfg(feature = "UI_Composition")]
impl IContextRequestedEventArgs_Vtbl {
    pub const fn new<Identity: IContextRequestedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContextRequestedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextRequestedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn TryGetPosition<Identity: IContextRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeto: *mut core::ffi::c_void, point: *mut windows::Foundation::Point, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContextRequestedEventArgs_Impl::TryGetPosition(this, core::mem::transmute_copy(&relativeto), core::mem::transmute_copy(&point)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContextRequestedEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            TryGetPosition: TryGetPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextRequestedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub TryGetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows::Foundation::Point, *mut bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    TryGetPosition: usize,
}
windows_core::imp::define_interface!(IDoubleTappedRoutedEventArgs, IDoubleTappedRoutedEventArgs_Vtbl, 0x32b9549d_11d8_53a5_a953_02409537a11f);
impl windows_core::RuntimeType for IDoubleTappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IDoubleTappedRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IDoubleTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IDoubleTappedRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IDoubleTappedRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn GetPosition(&self, relativeTo: windows_core::Ref<super::UIElement>) -> windows_core::Result<windows::Foundation::Point>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IDoubleTappedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerDeviceType<Identity: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDoubleTappedRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDoubleTappedRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDoubleTappedRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IDoubleTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeto: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDoubleTappedRoutedEventArgs_Impl::GetPosition(this, core::mem::transmute_copy(&relativeto)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDoubleTappedRoutedEventArgs, OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDoubleTappedRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDoubleTappedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPosition: usize,
}
windows_core::imp::define_interface!(IExecuteRequestedEventArgs, IExecuteRequestedEventArgs_Vtbl, 0xe1a9fd0c_34d0_5ae2_8f5d_377e7a8a2708);
impl windows_core::RuntimeType for IExecuteRequestedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IExecuteRequestedEventArgs");
}
impl windows_core::RuntimeName for IExecuteRequestedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IExecuteRequestedEventArgs";
}
pub trait IExecuteRequestedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Parameter(&self) -> windows_core::Result<windows_core::IInspectable>;
}
impl IExecuteRequestedEventArgs_Vtbl {
    pub const fn new<Identity: IExecuteRequestedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Parameter<Identity: IExecuteRequestedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExecuteRequestedEventArgs_Impl::Parameter(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IExecuteRequestedEventArgs, OFFSET>(), Parameter: Parameter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExecuteRequestedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExecuteRequestedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Parameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFindNextElementOptions, IFindNextElementOptions_Vtbl, 0x7f88e76b_7417_5447_aed4_2fabd291bdc6);
impl windows_core::RuntimeType for IFindNextElementOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IFindNextElementOptions");
}
impl windows_core::RuntimeName for IFindNextElementOptions {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IFindNextElementOptions";
}
pub trait IFindNextElementOptions_Impl: windows_core::IUnknownImpl {
    fn SearchRoot(&self) -> windows_core::Result<super::DependencyObject>;
    fn SetSearchRoot(&self, value: windows_core::Ref<super::DependencyObject>) -> windows_core::Result<()>;
    fn ExclusionRect(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn SetExclusionRect(&self, value: &windows::Foundation::Rect) -> windows_core::Result<()>;
    fn HintRect(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn SetHintRect(&self, value: &windows::Foundation::Rect) -> windows_core::Result<()>;
    fn XYFocusNavigationStrategyOverride(&self) -> windows_core::Result<XYFocusNavigationStrategyOverride>;
    fn SetXYFocusNavigationStrategyOverride(&self, value: XYFocusNavigationStrategyOverride) -> windows_core::Result<()>;
}
impl IFindNextElementOptions_Vtbl {
    pub const fn new<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SearchRoot<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFindNextElementOptions_Impl::SearchRoot(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSearchRoot<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFindNextElementOptions_Impl::SetSearchRoot(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ExclusionRect<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFindNextElementOptions_Impl::ExclusionRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExclusionRect<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFindNextElementOptions_Impl::SetExclusionRect(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn HintRect<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFindNextElementOptions_Impl::HintRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHintRect<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFindNextElementOptions_Impl::SetHintRect(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn XYFocusNavigationStrategyOverride<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut XYFocusNavigationStrategyOverride) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFindNextElementOptions_Impl::XYFocusNavigationStrategyOverride(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetXYFocusNavigationStrategyOverride<Identity: IFindNextElementOptions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: XYFocusNavigationStrategyOverride) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFindNextElementOptions_Impl::SetXYFocusNavigationStrategyOverride(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFindNextElementOptions, OFFSET>(),
            SearchRoot: SearchRoot::<Identity, OFFSET>,
            SetSearchRoot: SetSearchRoot::<Identity, OFFSET>,
            ExclusionRect: ExclusionRect::<Identity, OFFSET>,
            SetExclusionRect: SetExclusionRect::<Identity, OFFSET>,
            HintRect: HintRect::<Identity, OFFSET>,
            SetHintRect: SetHintRect::<Identity, OFFSET>,
            XYFocusNavigationStrategyOverride: XYFocusNavigationStrategyOverride::<Identity, OFFSET>,
            SetXYFocusNavigationStrategyOverride: SetXYFocusNavigationStrategyOverride::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFindNextElementOptions as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindNextElementOptions_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SearchRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSearchRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExclusionRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub SetExclusionRect: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect) -> windows_core::HRESULT,
    pub HintRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub SetHintRect: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect) -> windows_core::HRESULT,
    pub XYFocusNavigationStrategyOverride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut XYFocusNavigationStrategyOverride) -> windows_core::HRESULT,
    pub SetXYFocusNavigationStrategyOverride: unsafe extern "system" fn(*mut core::ffi::c_void, XYFocusNavigationStrategyOverride) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusManager, IFocusManager_Vtbl, 0x9fd07bc5_d2d4_53fe_a31a_846de8b7a257);
impl windows_core::RuntimeType for IFocusManager {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IFocusManager");
}
impl windows_core::RuntimeName for IFocusManager {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IFocusManager";
}
pub trait IFocusManager_Impl: windows_core::IUnknownImpl {}
impl IFocusManager_Vtbl {
    pub const fn new<Identity: IFocusManager_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusManager, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusManager as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManager_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IFocusManagerGotFocusEventArgs, IFocusManagerGotFocusEventArgs_Vtbl, 0x50aca341_4519_59cf_83b1_c9c45cfdb816);
impl windows_core::RuntimeType for IFocusManagerGotFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IFocusManagerGotFocusEventArgs");
}
impl windows_core::RuntimeName for IFocusManagerGotFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IFocusManagerGotFocusEventArgs";
}
pub trait IFocusManagerGotFocusEventArgs_Impl: windows_core::IUnknownImpl {
    fn NewFocusedElement(&self) -> windows_core::Result<super::DependencyObject>;
    fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IFocusManagerGotFocusEventArgs_Vtbl {
    pub const fn new<Identity: IFocusManagerGotFocusEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewFocusedElement<Identity: IFocusManagerGotFocusEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusManagerGotFocusEventArgs_Impl::NewFocusedElement(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: IFocusManagerGotFocusEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusManagerGotFocusEventArgs_Impl::CorrelationId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusManagerGotFocusEventArgs, OFFSET>(),
            NewFocusedElement: NewFocusedElement::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusManagerGotFocusEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerGotFocusEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusManagerLostFocusEventArgs, IFocusManagerLostFocusEventArgs_Vtbl, 0xfdaf2c3f_a22e_5902_abce_b60758fbed1e);
impl windows_core::RuntimeType for IFocusManagerLostFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IFocusManagerLostFocusEventArgs");
}
impl windows_core::RuntimeName for IFocusManagerLostFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IFocusManagerLostFocusEventArgs";
}
pub trait IFocusManagerLostFocusEventArgs_Impl: windows_core::IUnknownImpl {
    fn OldFocusedElement(&self) -> windows_core::Result<super::DependencyObject>;
    fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IFocusManagerLostFocusEventArgs_Vtbl {
    pub const fn new<Identity: IFocusManagerLostFocusEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OldFocusedElement<Identity: IFocusManagerLostFocusEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusManagerLostFocusEventArgs_Impl::OldFocusedElement(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CorrelationId<Identity: IFocusManagerLostFocusEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusManagerLostFocusEventArgs_Impl::CorrelationId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusManagerLostFocusEventArgs, OFFSET>(),
            OldFocusedElement: OldFocusedElement::<Identity, OFFSET>,
            CorrelationId: CorrelationId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusManagerLostFocusEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerLostFocusEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OldFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusManagerStatics, IFocusManagerStatics_Vtbl, 0xe73dce04_e23a_5fb3_96ab_7df04c51dff2);
impl windows_core::RuntimeType for IFocusManagerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IFocusManagerStatics");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IFocusManagerStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IFocusManagerStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusManagerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GettingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGettingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LosingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLosingFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    TryFocusAsync: usize,
    pub TryMoveFocusAsync: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryMoveFocusWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryMoveFocusWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub FindNextElement: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstFocusableElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindLastFocusableElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindNextElementWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub FindNextFocusableElement: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    FindNextFocusableElement: usize,
    #[cfg(feature = "UI_Composition")]
    pub FindNextFocusableElementWithHint: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, windows::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    FindNextFocusableElementWithHint: usize,
    pub TryMoveFocus: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationDirection, *mut bool) -> windows_core::HRESULT,
    pub GetFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocusedElementWithRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusMovementResult, IFocusMovementResult_Vtbl, 0xa46259fd_3edd_554b_a188_0a47b71e4e1a);
impl windows_core::RuntimeType for IFocusMovementResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IFocusMovementResult");
}
impl windows_core::RuntimeName for IFocusMovementResult {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IFocusMovementResult";
}
pub trait IFocusMovementResult_Impl: windows_core::IUnknownImpl {
    fn Succeeded(&self) -> windows_core::Result<bool>;
}
impl IFocusMovementResult_Vtbl {
    pub const fn new<Identity: IFocusMovementResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Succeeded<Identity: IFocusMovementResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusMovementResult_Impl::Succeeded(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusMovementResult, OFFSET>(), Succeeded: Succeeded::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusMovementResult as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusMovementResult_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Succeeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGettingFocusEventArgs, IGettingFocusEventArgs_Vtbl, 0x37fd3af0_bd3c_5bf5_a9cd_71a1e87af950);
impl windows_core::RuntimeType for IGettingFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IGettingFocusEventArgs");
}
impl windows_core::RuntimeName for IGettingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IGettingFocusEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IGettingFocusEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OldFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    FocusState: usize,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusNavigationDirection) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusInputDeviceKind) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHoldingRoutedEventArgs, IHoldingRoutedEventArgs_Vtbl, 0x8272a4b2_2221_551e_b0bb_16e29138ab20);
impl windows_core::RuntimeType for IHoldingRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IHoldingRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IHoldingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IHoldingRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IHoldingRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn HoldingState(&self) -> windows_core::Result<super::super::Input::HoldingState>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn GetPosition(&self, relativeTo: windows_core::Ref<super::UIElement>) -> windows_core::Result<windows::Foundation::Point>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IHoldingRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerDeviceType<Identity: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHoldingRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HoldingState<Identity: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::HoldingState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHoldingRoutedEventArgs_Impl::HoldingState(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHoldingRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IHoldingRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IHoldingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeto: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHoldingRoutedEventArgs_Impl::GetPosition(this, core::mem::transmute_copy(&relativeto)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHoldingRoutedEventArgs, OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            HoldingState: HoldingState::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHoldingRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "UI_Input")]
    pub HoldingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::HoldingState) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    HoldingState: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPosition: usize,
}
windows_core::imp::define_interface!(IInertiaExpansionBehavior, IInertiaExpansionBehavior_Vtbl, 0xd60029b7_f0cd_5aea_abe5_7410d09118c6);
impl windows_core::RuntimeType for IInertiaExpansionBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IInertiaExpansionBehavior");
}
impl windows_core::RuntimeName for IInertiaExpansionBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IInertiaExpansionBehavior";
}
pub trait IInertiaExpansionBehavior_Impl: windows_core::IUnknownImpl {
    fn DesiredDeceleration(&self) -> windows_core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> windows_core::Result<()>;
    fn DesiredExpansion(&self) -> windows_core::Result<f64>;
    fn SetDesiredExpansion(&self, value: f64) -> windows_core::Result<()>;
}
impl IInertiaExpansionBehavior_Vtbl {
    pub const fn new<Identity: IInertiaExpansionBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DesiredDeceleration<Identity: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaExpansionBehavior_Impl::DesiredDeceleration(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaExpansionBehavior_Impl::SetDesiredDeceleration(this, value).into()
            }
        }
        unsafe extern "system" fn DesiredExpansion<Identity: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaExpansionBehavior_Impl::DesiredExpansion(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredExpansion<Identity: IInertiaExpansionBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaExpansionBehavior_Impl::SetDesiredExpansion(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInertiaExpansionBehavior, OFFSET>(),
            DesiredDeceleration: DesiredDeceleration::<Identity, OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Identity, OFFSET>,
            DesiredExpansion: DesiredExpansion::<Identity, OFFSET>,
            SetDesiredExpansion: SetDesiredExpansion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInertiaExpansionBehavior as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaExpansionBehavior_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DesiredExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDesiredExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInertiaRotationBehavior, IInertiaRotationBehavior_Vtbl, 0x27b4bd03_9149_5691_bce5_fa33b32c4a81);
impl windows_core::RuntimeType for IInertiaRotationBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IInertiaRotationBehavior");
}
impl windows_core::RuntimeName for IInertiaRotationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IInertiaRotationBehavior";
}
pub trait IInertiaRotationBehavior_Impl: windows_core::IUnknownImpl {
    fn DesiredDeceleration(&self) -> windows_core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> windows_core::Result<()>;
    fn DesiredRotation(&self) -> windows_core::Result<f64>;
    fn SetDesiredRotation(&self, value: f64) -> windows_core::Result<()>;
}
impl IInertiaRotationBehavior_Vtbl {
    pub const fn new<Identity: IInertiaRotationBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DesiredDeceleration<Identity: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaRotationBehavior_Impl::DesiredDeceleration(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaRotationBehavior_Impl::SetDesiredDeceleration(this, value).into()
            }
        }
        unsafe extern "system" fn DesiredRotation<Identity: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaRotationBehavior_Impl::DesiredRotation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredRotation<Identity: IInertiaRotationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaRotationBehavior_Impl::SetDesiredRotation(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInertiaRotationBehavior, OFFSET>(),
            DesiredDeceleration: DesiredDeceleration::<Identity, OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Identity, OFFSET>,
            DesiredRotation: DesiredRotation::<Identity, OFFSET>,
            SetDesiredRotation: SetDesiredRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInertiaRotationBehavior as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaRotationBehavior_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DesiredRotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDesiredRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInertiaTranslationBehavior, IInertiaTranslationBehavior_Vtbl, 0xd4f91cf5_3317_5914_b25a_ea6ee55b96d0);
impl windows_core::RuntimeType for IInertiaTranslationBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IInertiaTranslationBehavior");
}
impl windows_core::RuntimeName for IInertiaTranslationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IInertiaTranslationBehavior";
}
pub trait IInertiaTranslationBehavior_Impl: windows_core::IUnknownImpl {
    fn DesiredDeceleration(&self) -> windows_core::Result<f64>;
    fn SetDesiredDeceleration(&self, value: f64) -> windows_core::Result<()>;
    fn DesiredDisplacement(&self) -> windows_core::Result<f64>;
    fn SetDesiredDisplacement(&self, value: f64) -> windows_core::Result<()>;
}
impl IInertiaTranslationBehavior_Vtbl {
    pub const fn new<Identity: IInertiaTranslationBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DesiredDeceleration<Identity: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaTranslationBehavior_Impl::DesiredDeceleration(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredDeceleration<Identity: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaTranslationBehavior_Impl::SetDesiredDeceleration(this, value).into()
            }
        }
        unsafe extern "system" fn DesiredDisplacement<Identity: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInertiaTranslationBehavior_Impl::DesiredDisplacement(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDesiredDisplacement<Identity: IInertiaTranslationBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInertiaTranslationBehavior_Impl::SetDesiredDisplacement(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInertiaTranslationBehavior, OFFSET>(),
            DesiredDeceleration: DesiredDeceleration::<Identity, OFFSET>,
            SetDesiredDeceleration: SetDesiredDeceleration::<Identity, OFFSET>,
            DesiredDisplacement: DesiredDisplacement::<Identity, OFFSET>,
            SetDesiredDisplacement: SetDesiredDisplacement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInertiaTranslationBehavior as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInertiaTranslationBehavior_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDesiredDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DesiredDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDesiredDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputScope, IInputScope_Vtbl, 0x76ea58b1_e910_5176_9147_695cc95e7da2);
impl windows_core::RuntimeType for IInputScope {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IInputScope");
}
impl windows_core::RuntimeName for IInputScope {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IInputScope";
}
pub trait IInputScope_Impl: windows_core::IUnknownImpl {
    fn Names(&self) -> windows_core::Result<windows_collections::IVector<InputScopeName>>;
}
impl IInputScope_Vtbl {
    pub const fn new<Identity: IInputScope_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Names<Identity: IInputScope_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputScope_Impl::Names(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputScope, OFFSET>(), Names: Names::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputScope as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScope_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Names: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputScopeName, IInputScopeName_Vtbl, 0xee99a66d_28d0_53cb_82ee_1b6ee58bcc35);
impl windows_core::RuntimeType for IInputScopeName {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IInputScopeName");
}
impl windows_core::RuntimeName for IInputScopeName {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IInputScopeName";
}
pub trait IInputScopeName_Impl: windows_core::IUnknownImpl {
    fn NameValue(&self) -> windows_core::Result<InputScopeNameValue>;
    fn SetNameValue(&self, value: InputScopeNameValue) -> windows_core::Result<()>;
}
impl IInputScopeName_Vtbl {
    pub const fn new<Identity: IInputScopeName_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NameValue<Identity: IInputScopeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputScopeNameValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputScopeName_Impl::NameValue(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNameValue<Identity: IInputScopeName_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: InputScopeNameValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputScopeName_Impl::SetNameValue(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputScopeName, OFFSET>(),
            NameValue: NameValue::<Identity, OFFSET>,
            SetNameValue: SetNameValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputScopeName as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeName_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NameValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputScopeNameValue) -> windows_core::HRESULT,
    pub SetNameValue: unsafe extern "system" fn(*mut core::ffi::c_void, InputScopeNameValue) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputScopeNameFactory, IInputScopeNameFactory_Vtbl, 0xfeec2efd_bc09_5cd6_9b47_6d35d1d87c61);
impl windows_core::RuntimeType for IInputScopeNameFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IInputScopeNameFactory");
}
impl windows_core::RuntimeName for IInputScopeNameFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IInputScopeNameFactory";
}
pub trait IInputScopeNameFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, nameValue: InputScopeNameValue) -> windows_core::Result<InputScopeName>;
}
impl IInputScopeNameFactory_Vtbl {
    pub const fn new<Identity: IInputScopeNameFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IInputScopeNameFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, namevalue: InputScopeNameValue, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputScopeNameFactory_Impl::CreateInstance(this, namevalue) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputScopeNameFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputScopeNameFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputScopeNameFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, InputScopeNameValue, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyRoutedEventArgs, IKeyRoutedEventArgs_Vtbl, 0xee357007_a2d6_5c75_9431_05fd66ec7915);
impl windows_core::RuntimeType for IKeyRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IKeyRoutedEventArgs");
}
impl windows_core::RuntimeName for IKeyRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IKeyRoutedEventArgs";
}
pub trait IKeyRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Key(&self) -> windows_core::Result<windows::System::VirtualKey>;
    fn KeyStatus(&self) -> windows_core::Result<windows::UI::Core::CorePhysicalKeyStatus>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn OriginalKey(&self) -> windows_core::Result<windows::System::VirtualKey>;
    fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IKeyRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IKeyRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Key<Identity: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKey) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyRoutedEventArgs_Impl::Key(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyStatus<Identity: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::UI::Core::CorePhysicalKeyStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyRoutedEventArgs_Impl::KeyStatus(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn OriginalKey<Identity: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKey) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyRoutedEventArgs_Impl::OriginalKey(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceId<Identity: IKeyRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyRoutedEventArgs_Impl::DeviceId(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKeyRoutedEventArgs, OFFSET>(),
            Key: Key::<Identity, OFFSET>,
            KeyStatus: KeyStatus::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            OriginalKey: OriginalKey::<Identity, OFFSET>,
            DeviceId: DeviceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKey) -> windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::UI::Core::CorePhysicalKeyStatus) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub OriginalKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKey) -> windows_core::HRESULT,
    pub DeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyboardAccelerator, IKeyboardAccelerator_Vtbl, 0x6f8bf1e2_4e91_5cf9_a6be_4770caf3d770);
impl windows_core::RuntimeType for IKeyboardAccelerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IKeyboardAccelerator");
}
impl windows_core::RuntimeName for IKeyboardAccelerator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IKeyboardAccelerator";
}
pub trait IKeyboardAccelerator_Impl: windows_core::IUnknownImpl {
    fn Key(&self) -> windows_core::Result<windows::System::VirtualKey>;
    fn SetKey(&self, value: windows::System::VirtualKey) -> windows_core::Result<()>;
    fn Modifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers>;
    fn SetModifiers(&self, value: windows::System::VirtualKeyModifiers) -> windows_core::Result<()>;
    fn IsEnabled(&self) -> windows_core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()>;
    fn ScopeOwner(&self) -> windows_core::Result<super::DependencyObject>;
    fn SetScopeOwner(&self, value: windows_core::Ref<super::DependencyObject>) -> windows_core::Result<()>;
    fn Invoked(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<KeyboardAccelerator, KeyboardAcceleratorInvokedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveInvoked(&self, token: i64) -> windows_core::Result<()>;
}
impl IKeyboardAccelerator_Vtbl {
    pub const fn new<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Key<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKey) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAccelerator_Impl::Key(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKey<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::System::VirtualKey) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyboardAccelerator_Impl::SetKey(this, value).into()
            }
        }
        unsafe extern "system" fn Modifiers<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAccelerator_Impl::Modifiers(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetModifiers<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::System::VirtualKeyModifiers) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyboardAccelerator_Impl::SetModifiers(this, value).into()
            }
        }
        unsafe extern "system" fn IsEnabled<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAccelerator_Impl::IsEnabled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsEnabled<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyboardAccelerator_Impl::SetIsEnabled(this, value).into()
            }
        }
        unsafe extern "system" fn ScopeOwner<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAccelerator_Impl::ScopeOwner(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScopeOwner<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyboardAccelerator_Impl::SetScopeOwner(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Invoked<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAccelerator_Impl::Invoked(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveInvoked<Identity: IKeyboardAccelerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyboardAccelerator_Impl::RemoveInvoked(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKeyboardAccelerator, OFFSET>(),
            Key: Key::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            Modifiers: Modifiers::<Identity, OFFSET>,
            SetModifiers: SetModifiers::<Identity, OFFSET>,
            IsEnabled: IsEnabled::<Identity, OFFSET>,
            SetIsEnabled: SetIsEnabled::<Identity, OFFSET>,
            ScopeOwner: ScopeOwner::<Identity, OFFSET>,
            SetScopeOwner: SetScopeOwner::<Identity, OFFSET>,
            Invoked: Invoked::<Identity, OFFSET>,
            RemoveInvoked: RemoveInvoked::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyboardAccelerator as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAccelerator_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKey) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows::System::VirtualKey) -> windows_core::HRESULT,
    pub Modifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    pub SetModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, windows::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    pub IsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ScopeOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetScopeOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Invoked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveInvoked: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyboardAcceleratorFactory, IKeyboardAcceleratorFactory_Vtbl, 0xca1d410a_af2a_51b9_a1de_6c0af9f3b598);
impl windows_core::RuntimeType for IKeyboardAcceleratorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IKeyboardAcceleratorFactory");
}
impl windows_core::RuntimeName for IKeyboardAcceleratorFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IKeyboardAcceleratorFactory";
}
pub trait IKeyboardAcceleratorFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<KeyboardAccelerator>;
}
impl IKeyboardAcceleratorFactory_Vtbl {
    pub const fn new<Identity: IKeyboardAcceleratorFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IKeyboardAcceleratorFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAcceleratorFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKeyboardAcceleratorFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyboardAcceleratorFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyboardAcceleratorInvokedEventArgs, IKeyboardAcceleratorInvokedEventArgs_Vtbl, 0x62c9fdb0_b574_527d_97eb_5c7f674441e0);
impl windows_core::RuntimeType for IKeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs");
}
impl windows_core::RuntimeName for IKeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IKeyboardAcceleratorInvokedEventArgs";
}
pub trait IKeyboardAcceleratorInvokedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn Element(&self) -> windows_core::Result<super::DependencyObject>;
    fn KeyboardAccelerator(&self) -> windows_core::Result<KeyboardAccelerator>;
}
impl IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub const fn new<Identity: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAcceleratorInvokedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyboardAcceleratorInvokedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn Element<Identity: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAcceleratorInvokedEventArgs_Impl::Element(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyboardAccelerator<Identity: IKeyboardAcceleratorInvokedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyboardAcceleratorInvokedEventArgs_Impl::KeyboardAccelerator(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKeyboardAcceleratorInvokedEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            Element: Element::<Identity, OFFSET>,
            KeyboardAccelerator: KeyboardAccelerator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyboardAcceleratorInvokedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorInvokedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Element: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeyboardAccelerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IKeyboardAcceleratorStatics, IKeyboardAcceleratorStatics_Vtbl, 0x73e674ca_73f4_5e77_b8d6_ff7852a63b0b);
impl windows_core::RuntimeType for IKeyboardAcceleratorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IKeyboardAcceleratorStatics");
}
impl windows_core::RuntimeName for IKeyboardAcceleratorStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IKeyboardAcceleratorStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyboardAcceleratorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ILosingFocusEventArgs, ILosingFocusEventArgs_Vtbl, 0xfa0e5ffa_2b1b_52f8_bb66_e35f51e73cf3);
impl windows_core::RuntimeType for ILosingFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.ILosingFocusEventArgs");
}
impl windows_core::RuntimeName for ILosingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ILosingFocusEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct ILosingFocusEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OldFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NewFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNewFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    FocusState: usize,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusNavigationDirection) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusInputDeviceKind) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub TryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TrySetNewFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationCompletedRoutedEventArgs, IManipulationCompletedRoutedEventArgs_Vtbl, 0xe3be9e4e_c5fb_5859_a81d_ce12fc3a2f4d);
impl windows_core::RuntimeType for IManipulationCompletedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationCompletedRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationCompletedRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IManipulationCompletedRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Container(&self) -> windows_core::Result<super::UIElement>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn IsInertial(&self) -> windows_core::Result<bool>;
    fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> windows_core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IManipulationCompletedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Container<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedRoutedEventArgs_Impl::Container(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedRoutedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInertial<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedRoutedEventArgs_Impl::IsInertial(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cumulative<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedRoutedEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Velocities<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedRoutedEventArgs_Impl::Velocities(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationCompletedRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationCompletedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationCompletedRoutedEventArgs, OFFSET>(),
            Container: Container::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            IsInertial: IsInertial::<Identity, OFFSET>,
            Cumulative: Cumulative::<Identity, OFFSET>,
            Velocities: Velocities::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationCompletedRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Container: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Cumulative: usize,
    #[cfg(feature = "UI_Input")]
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationVelocities) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
}
windows_core::imp::define_interface!(IManipulationDeltaRoutedEventArgs, IManipulationDeltaRoutedEventArgs_Vtbl, 0x51369745_960f_54ac_93fa_763d22910dea);
impl windows_core::RuntimeType for IManipulationDeltaRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationDeltaRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationDeltaRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IManipulationDeltaRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Container(&self) -> windows_core::Result<super::UIElement>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn IsInertial(&self) -> windows_core::Result<bool>;
    fn Delta(&self) -> windows_core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> windows_core::Result<super::super::Input::ManipulationVelocities>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn Complete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IManipulationDeltaRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Container<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::Container(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInertial<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::IsInertial(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delta<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::Delta(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cumulative<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Velocities<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::Velocities(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationDeltaRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationDeltaRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Complete<Identity: IManipulationDeltaRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationDeltaRoutedEventArgs_Impl::Complete(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationDeltaRoutedEventArgs, OFFSET>(),
            Container: Container::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            IsInertial: IsInertial::<Identity, OFFSET>,
            Delta: Delta::<Identity, OFFSET>,
            Cumulative: Cumulative::<Identity, OFFSET>,
            Velocities: Velocities::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationDeltaRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationDeltaRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Container: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub Delta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Delta: usize,
    #[cfg(feature = "UI_Input")]
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Cumulative: usize,
    #[cfg(feature = "UI_Input")]
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationVelocities) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Velocities: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationInertiaStartingRoutedEventArgs, IManipulationInertiaStartingRoutedEventArgs_Vtbl, 0x17d510be_5514_5952_9afd_959b60ab9394);
impl windows_core::RuntimeType for IManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationInertiaStartingRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationInertiaStartingRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IManipulationInertiaStartingRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Container(&self) -> windows_core::Result<super::UIElement>;
    fn ExpansionBehavior(&self) -> windows_core::Result<InertiaExpansionBehavior>;
    fn SetExpansionBehavior(&self, value: windows_core::Ref<InertiaExpansionBehavior>) -> windows_core::Result<()>;
    fn RotationBehavior(&self) -> windows_core::Result<InertiaRotationBehavior>;
    fn SetRotationBehavior(&self, value: windows_core::Ref<InertiaRotationBehavior>) -> windows_core::Result<()>;
    fn TranslationBehavior(&self) -> windows_core::Result<InertiaTranslationBehavior>;
    fn SetTranslationBehavior(&self, value: windows_core::Ref<InertiaTranslationBehavior>) -> windows_core::Result<()>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn Delta(&self) -> windows_core::Result<super::super::Input::ManipulationDelta>;
    fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta>;
    fn Velocities(&self) -> windows_core::Result<super::super::Input::ManipulationVelocities>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IManipulationInertiaStartingRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Container<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::Container(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpansionBehavior<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::ExpansionBehavior(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetExpansionBehavior<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationInertiaStartingRoutedEventArgs_Impl::SetExpansionBehavior(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RotationBehavior<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::RotationBehavior(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotationBehavior<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationInertiaStartingRoutedEventArgs_Impl::SetRotationBehavior(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn TranslationBehavior<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::TranslationBehavior(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTranslationBehavior<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationInertiaStartingRoutedEventArgs_Impl::SetTranslationBehavior(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Handled<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationInertiaStartingRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delta<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::Delta(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cumulative<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Velocities<Identity: IManipulationInertiaStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationVelocities) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingRoutedEventArgs_Impl::Velocities(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationInertiaStartingRoutedEventArgs, OFFSET>(),
            Container: Container::<Identity, OFFSET>,
            ExpansionBehavior: ExpansionBehavior::<Identity, OFFSET>,
            SetExpansionBehavior: SetExpansionBehavior::<Identity, OFFSET>,
            RotationBehavior: RotationBehavior::<Identity, OFFSET>,
            SetRotationBehavior: SetRotationBehavior::<Identity, OFFSET>,
            TranslationBehavior: TranslationBehavior::<Identity, OFFSET>,
            SetTranslationBehavior: SetTranslationBehavior::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Delta: Delta::<Identity, OFFSET>,
            Cumulative: Cumulative::<Identity, OFFSET>,
            Velocities: Velocities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationInertiaStartingRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Container: usize,
    pub ExpansionBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetExpansionBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RotationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRotationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TranslationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTranslationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "UI_Input")]
    pub Delta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Delta: usize,
    #[cfg(feature = "UI_Input")]
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Cumulative: usize,
    #[cfg(feature = "UI_Input")]
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationVelocities) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Velocities: usize,
}
windows_core::imp::define_interface!(IManipulationPivot, IManipulationPivot_Vtbl, 0x286baba4_313d_507c_adc5_f739732cea27);
impl windows_core::RuntimeType for IManipulationPivot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationPivot");
}
impl windows_core::RuntimeName for IManipulationPivot {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationPivot";
}
pub trait IManipulationPivot_Impl: windows_core::IUnknownImpl {
    fn Center(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn SetCenter(&self, value: &windows::Foundation::Point) -> windows_core::Result<()>;
    fn Radius(&self) -> windows_core::Result<f64>;
    fn SetRadius(&self, value: f64) -> windows_core::Result<()>;
}
impl IManipulationPivot_Vtbl {
    pub const fn new<Identity: IManipulationPivot_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Center<Identity: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationPivot_Impl::Center(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCenter<Identity: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationPivot_Impl::SetCenter(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Radius<Identity: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationPivot_Impl::Radius(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRadius<Identity: IManipulationPivot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationPivot_Impl::SetRadius(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationPivot, OFFSET>(),
            Center: Center::<Identity, OFFSET>,
            SetCenter: SetCenter::<Identity, OFFSET>,
            Radius: Radius::<Identity, OFFSET>,
            SetRadius: SetRadius::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationPivot as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivot_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Center: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
    pub Radius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationPivotFactory, IManipulationPivotFactory_Vtbl, 0x67143ccd_ea6c_5fe2_bef2_adcbd7af52fd);
impl windows_core::RuntimeType for IManipulationPivotFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationPivotFactory");
}
impl windows_core::RuntimeName for IManipulationPivotFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationPivotFactory";
}
pub trait IManipulationPivotFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstanceWithCenterAndRadius(&self, center: &windows::Foundation::Point, radius: f64) -> windows_core::Result<ManipulationPivot>;
}
impl IManipulationPivotFactory_Vtbl {
    pub const fn new<Identity: IManipulationPivotFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstanceWithCenterAndRadius<Identity: IManipulationPivotFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, center: windows::Foundation::Point, radius: f64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationPivotFactory_Impl::CreateInstanceWithCenterAndRadius(this, core::mem::transmute(&center), radius) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationPivotFactory, OFFSET>(),
            CreateInstanceWithCenterAndRadius: CreateInstanceWithCenterAndRadius::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationPivotFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationPivotFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithCenterAndRadius: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationStartedRoutedEventArgs, IManipulationStartedRoutedEventArgs_Vtbl, 0x61857950_5821_5652_9fdf_c6277c5886f5);
impl windows_core::RuntimeType for IManipulationStartedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationStartedRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationStartedRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IManipulationStartedRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Container(&self) -> windows_core::Result<super::UIElement>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta>;
    fn Complete(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IManipulationStartedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Container<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedRoutedEventArgs_Impl::Container(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedRoutedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationStartedRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Cumulative<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedRoutedEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Complete<Identity: IManipulationStartedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationStartedRoutedEventArgs_Impl::Complete(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationStartedRoutedEventArgs, OFFSET>(),
            Container: Container::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Cumulative: Cumulative::<Identity, OFFSET>,
            Complete: Complete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationStartedRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Container: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    #[cfg(feature = "UI_Input")]
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::ManipulationDelta) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    Cumulative: usize,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationStartedRoutedEventArgsFactory, IManipulationStartedRoutedEventArgsFactory_Vtbl, 0x5681b0de_3fa7_503e_9c46_a80339760292);
impl windows_core::RuntimeType for IManipulationStartedRoutedEventArgsFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationStartedRoutedEventArgsFactory");
}
impl windows_core::RuntimeName for IManipulationStartedRoutedEventArgsFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationStartedRoutedEventArgsFactory";
}
pub trait IManipulationStartedRoutedEventArgsFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<ManipulationStartedRoutedEventArgs>;
}
impl IManipulationStartedRoutedEventArgsFactory_Vtbl {
    pub const fn new<Identity: IManipulationStartedRoutedEventArgsFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IManipulationStartedRoutedEventArgsFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedRoutedEventArgsFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationStartedRoutedEventArgsFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationStartedRoutedEventArgsFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedRoutedEventArgsFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationStartingRoutedEventArgs, IManipulationStartingRoutedEventArgs_Vtbl, 0x93a99f86_f5a0_5326_91b0_851c897af79f);
impl windows_core::RuntimeType for IManipulationStartingRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IManipulationStartingRoutedEventArgs");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IManipulationStartingRoutedEventArgs";
}
#[cfg(feature = "UI_Composition")]
pub trait IManipulationStartingRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Mode(&self) -> windows_core::Result<ManipulationModes>;
    fn SetMode(&self, value: ManipulationModes) -> windows_core::Result<()>;
    fn Container(&self) -> windows_core::Result<super::UIElement>;
    fn SetContainer(&self, value: windows_core::Ref<super::UIElement>) -> windows_core::Result<()>;
    fn Pivot(&self) -> windows_core::Result<ManipulationPivot>;
    fn SetPivot(&self, value: windows_core::Ref<ManipulationPivot>) -> windows_core::Result<()>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Composition")]
impl IManipulationStartingRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Mode<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationModes) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartingRoutedEventArgs_Impl::Mode(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMode<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ManipulationModes) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationStartingRoutedEventArgs_Impl::SetMode(this, value).into()
            }
        }
        unsafe extern "system" fn Container<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartingRoutedEventArgs_Impl::Container(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContainer<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationStartingRoutedEventArgs_Impl::SetContainer(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Pivot<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartingRoutedEventArgs_Impl::Pivot(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPivot<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationStartingRoutedEventArgs_Impl::SetPivot(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Handled<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartingRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IManipulationStartingRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IManipulationStartingRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationStartingRoutedEventArgs, OFFSET>(),
            Mode: Mode::<Identity, OFFSET>,
            SetMode: SetMode::<Identity, OFFSET>,
            Container: Container::<Identity, OFFSET>,
            SetContainer: SetContainer::<Identity, OFFSET>,
            Pivot: Pivot::<Identity, OFFSET>,
            SetPivot: SetPivot::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationStartingRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartingRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationModes) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, ManipulationModes) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub Container: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    Container: usize,
    #[cfg(feature = "UI_Composition")]
    pub SetContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    SetContainer: usize,
    pub Pivot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPivot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INoFocusCandidateFoundEventArgs, INoFocusCandidateFoundEventArgs_Vtbl, 0xa2d7153a_cd2a_59cb_a574_ac82e30b9201);
impl windows_core::RuntimeType for INoFocusCandidateFoundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.INoFocusCandidateFoundEventArgs");
}
impl windows_core::RuntimeName for INoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.INoFocusCandidateFoundEventArgs";
}
pub trait INoFocusCandidateFoundEventArgs_Impl: windows_core::IUnknownImpl {
    fn Direction(&self) -> windows_core::Result<FocusNavigationDirection>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn InputDevice(&self) -> windows_core::Result<FocusInputDeviceKind>;
}
impl INoFocusCandidateFoundEventArgs_Vtbl {
    pub const fn new<Identity: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Direction<Identity: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FocusNavigationDirection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INoFocusCandidateFoundEventArgs_Impl::Direction(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INoFocusCandidateFoundEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                INoFocusCandidateFoundEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn InputDevice<Identity: INoFocusCandidateFoundEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FocusInputDeviceKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INoFocusCandidateFoundEventArgs_Impl::InputDevice(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INoFocusCandidateFoundEventArgs, OFFSET>(),
            Direction: Direction::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            InputDevice: InputDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INoFocusCandidateFoundEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INoFocusCandidateFoundEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusNavigationDirection) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub InputDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusInputDeviceKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointer, IPointer_Vtbl, 0x1f9afbf5_11a3_5e68_aa1b_72febfa0ab23);
impl windows_core::RuntimeType for IPointer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IPointer");
}
#[cfg(feature = "UI_Input")]
impl windows_core::RuntimeName for IPointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IPointer";
}
#[cfg(feature = "UI_Input")]
pub trait IPointer_Impl: windows_core::IUnknownImpl {
    fn PointerId(&self) -> windows_core::Result<u32>;
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn IsInContact(&self) -> windows_core::Result<bool>;
    fn IsInRange(&self) -> windows_core::Result<bool>;
}
#[cfg(feature = "UI_Input")]
impl IPointer_Vtbl {
    pub const fn new<Identity: IPointer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerId<Identity: IPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointer_Impl::PointerId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointer_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInContact<Identity: IPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointer_Impl::IsInContact(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInRange<Identity: IPointer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointer_Impl::IsInRange(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointer, OFFSET>(),
            PointerId: PointerId::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            IsInContact: IsInContact::<Identity, OFFSET>,
            IsInRange: IsInRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointer as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    pub IsInContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerRoutedEventArgs, IPointerRoutedEventArgs_Vtbl, 0x66e78a9a_1bec_5f92_b1a1_ea6334ee511c);
impl windows_core::RuntimeType for IPointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IPointerRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IPointerRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IPointerRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IPointerRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Pointer(&self) -> windows_core::Result<Pointer>;
    fn KeyModifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn IsGenerated(&self) -> windows_core::Result<bool>;
    fn GetCurrentPoint(&self, relativeTo: windows_core::Ref<super::UIElement>) -> windows_core::Result<super::super::Input::PointerPoint>;
    fn GetIntermediatePoints(&self, relativeTo: windows_core::Ref<super::UIElement>) -> windows_core::Result<windows_collections::IVector<super::super::Input::PointerPoint>>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IPointerRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Pointer<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::Pointer(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyModifiers<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::KeyModifiers(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPointerRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn IsGenerated<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::IsGenerated(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPoint<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeto: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::GetCurrentPoint(this, core::mem::transmute_copy(&relativeto)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<Identity: IPointerRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeto: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerRoutedEventArgs_Impl::GetIntermediatePoints(this, core::mem::transmute_copy(&relativeto)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerRoutedEventArgs, OFFSET>(),
            Pointer: Pointer::<Identity, OFFSET>,
            KeyModifiers: KeyModifiers::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            IsGenerated: IsGenerated::<Identity, OFFSET>,
            GetCurrentPoint: GetCurrentPoint::<Identity, OFFSET>,
            GetIntermediatePoints: GetIntermediatePoints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Pointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsGenerated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
    pub GetCurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Input")))]
    GetCurrentPoint: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
    pub GetIntermediatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Input")))]
    GetIntermediatePoints: usize,
}
windows_core::imp::define_interface!(IProcessKeyboardAcceleratorEventArgs, IProcessKeyboardAcceleratorEventArgs_Vtbl, 0x9be0d058_3d26_5811_b50a_3bb80ca766c9);
impl windows_core::RuntimeType for IProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IProcessKeyboardAcceleratorEventArgs");
}
impl windows_core::RuntimeName for IProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IProcessKeyboardAcceleratorEventArgs";
}
pub trait IProcessKeyboardAcceleratorEventArgs_Impl: windows_core::IUnknownImpl {
    fn Key(&self) -> windows_core::Result<windows::System::VirtualKey>;
    fn Modifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IProcessKeyboardAcceleratorEventArgs_Vtbl {
    pub const fn new<Identity: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Key<Identity: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKey) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessKeyboardAcceleratorEventArgs_Impl::Key(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Modifiers<Identity: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessKeyboardAcceleratorEventArgs_Impl::Modifiers(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IProcessKeyboardAcceleratorEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IProcessKeyboardAcceleratorEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessKeyboardAcceleratorEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IProcessKeyboardAcceleratorEventArgs, OFFSET>(),
            Key: Key::<Identity, OFFSET>,
            Modifiers: Modifiers::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessKeyboardAcceleratorEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessKeyboardAcceleratorEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Key: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKey) -> windows_core::HRESULT,
    pub Modifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRightTappedRoutedEventArgs, IRightTappedRoutedEventArgs_Vtbl, 0x3972fafb_2915_5c62_bb6b_54ad84ff400d);
impl windows_core::RuntimeType for IRightTappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IRightTappedRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for IRightTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IRightTappedRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait IRightTappedRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn GetPosition(&self, relativeTo: windows_core::Ref<super::UIElement>) -> windows_core::Result<windows::Foundation::Point>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl IRightTappedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerDeviceType<Identity: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRightTappedRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRightTappedRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRightTappedRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IRightTappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeto: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRightTappedRoutedEventArgs_Impl::GetPosition(this, core::mem::transmute_copy(&relativeto)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRightTappedRoutedEventArgs, OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRightTappedRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPosition: usize,
}
windows_core::imp::define_interface!(IStandardUICommand, IStandardUICommand_Vtbl, 0x5f395d50_5449_59ab_9cb2_4e3700033f03);
impl windows_core::RuntimeType for IStandardUICommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IStandardUICommand");
}
impl windows_core::RuntimeName for IStandardUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IStandardUICommand";
}
pub trait IStandardUICommand_Impl: windows_core::IUnknownImpl {
    fn Kind(&self) -> windows_core::Result<StandardUICommandKind>;
    fn SetKind(&self, value: StandardUICommandKind) -> windows_core::Result<()>;
}
impl IStandardUICommand_Vtbl {
    pub const fn new<Identity: IStandardUICommand_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Kind<Identity: IStandardUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut StandardUICommandKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStandardUICommand_Impl::Kind(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKind<Identity: IStandardUICommand_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: StandardUICommandKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStandardUICommand_Impl::SetKind(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStandardUICommand, OFFSET>(),
            Kind: Kind::<Identity, OFFSET>,
            SetKind: SetKind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStandardUICommand as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut StandardUICommandKind) -> windows_core::HRESULT,
    pub SetKind: unsafe extern "system" fn(*mut core::ffi::c_void, StandardUICommandKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStandardUICommandFactory, IStandardUICommandFactory_Vtbl, 0x5800f099_3746_5bcf_b1ce_af3d6bf8e83f);
impl windows_core::RuntimeType for IStandardUICommandFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IStandardUICommandFactory");
}
impl windows_core::RuntimeName for IStandardUICommandFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IStandardUICommandFactory";
}
pub trait IStandardUICommandFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<StandardUICommand>;
    fn CreateInstanceWithKind(&self, kind: StandardUICommandKind, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<StandardUICommand>;
}
impl IStandardUICommandFactory_Vtbl {
    pub const fn new<Identity: IStandardUICommandFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IStandardUICommandFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStandardUICommandFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateInstanceWithKind<Identity: IStandardUICommandFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kind: StandardUICommandKind, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStandardUICommandFactory_Impl::CreateInstanceWithKind(this, kind, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStandardUICommandFactory, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            CreateInstanceWithKind: CreateInstanceWithKind::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStandardUICommandFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateInstanceWithKind: unsafe extern "system" fn(*mut core::ffi::c_void, StandardUICommandKind, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStandardUICommandStatics, IStandardUICommandStatics_Vtbl, 0xab80c197_85cc_5d36_81aa_156cd63be31a);
impl windows_core::RuntimeType for IStandardUICommandStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IStandardUICommandStatics");
}
impl windows_core::RuntimeName for IStandardUICommandStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IStandardUICommandStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStandardUICommandStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ITappedRoutedEventArgs, ITappedRoutedEventArgs_Vtbl, 0x73f74b8c_3709_547e_8e0c_51c03c89126a);
impl windows_core::RuntimeType for ITappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.ITappedRoutedEventArgs");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl windows_core::RuntimeName for ITappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ITappedRoutedEventArgs";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
pub trait ITappedRoutedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn GetPosition(&self, relativeTo: windows_core::Ref<super::UIElement>) -> windows_core::Result<windows::Foundation::Point>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
impl ITappedRoutedEventArgs_Vtbl {
    pub const fn new<Identity: ITappedRoutedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerDeviceType<Identity: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITappedRoutedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITappedRoutedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITappedRoutedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: ITappedRoutedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relativeto: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITappedRoutedEventArgs_Impl::GetPosition(this, core::mem::transmute_copy(&relativeto)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITappedRoutedEventArgs, OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITappedRoutedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedRoutedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Input")]
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Input::PointerDeviceType) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Input"))]
    PointerDeviceType: usize,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Composition")]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    GetPosition: usize,
}
windows_core::imp::define_interface!(IXamlUICommand, IXamlUICommand_Vtbl, 0xa457f2cb_51e0_541c_9c42_dd1dcbdf58fb);
impl windows_core::RuntimeType for IXamlUICommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IXamlUICommand");
}
impl windows_core::RuntimeName for IXamlUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IXamlUICommand";
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommand_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    IconSource: usize,
    SetIconSource: usize,
    pub KeyboardAccelerators: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExecuteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveExecuteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CanExecuteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCanExecuteRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub NotifyCanExecuteChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXamlUICommandFactory, IXamlUICommandFactory_Vtbl, 0xf1f80a20_0e31_5505_8bc3_cdd1f0947f1d);
impl windows_core::RuntimeType for IXamlUICommandFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IXamlUICommandFactory");
}
impl windows_core::RuntimeName for IXamlUICommandFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IXamlUICommandFactory";
}
pub trait IXamlUICommandFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<XamlUICommand>;
}
impl IXamlUICommandFactory_Vtbl {
    pub const fn new<Identity: IXamlUICommandFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IXamlUICommandFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlUICommandFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IXamlUICommandFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlUICommandFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXamlUICommandStatics, IXamlUICommandStatics_Vtbl, 0x981dbda6_cdcb_5e35_b24b_c4f60ba148d9);
impl windows_core::RuntimeType for IXamlUICommandStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.IXamlUICommandStatics");
}
impl windows_core::RuntimeName for IXamlUICommandStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.IXamlUICommandStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlUICommandStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InertiaExpansionBehavior(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InertiaExpansionBehavior, windows_core::IUnknown, windows_core::IInspectable);
impl InertiaExpansionBehavior {
    pub fn DesiredDeceleration(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredDeceleration)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn DesiredExpansion(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredExpansion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredExpansion(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredExpansion)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for InertiaExpansionBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInertiaExpansionBehavior>();
}
unsafe impl windows_core::Interface for InertiaExpansionBehavior {
    type Vtable = <IInertiaExpansionBehavior as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInertiaExpansionBehavior as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InertiaExpansionBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaExpansionBehavior";
}
unsafe impl Send for InertiaExpansionBehavior {}
unsafe impl Sync for InertiaExpansionBehavior {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InertiaRotationBehavior(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InertiaRotationBehavior, windows_core::IUnknown, windows_core::IInspectable);
impl InertiaRotationBehavior {
    pub fn DesiredDeceleration(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredDeceleration)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn DesiredRotation(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredRotation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredRotation(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredRotation)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for InertiaRotationBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInertiaRotationBehavior>();
}
unsafe impl windows_core::Interface for InertiaRotationBehavior {
    type Vtable = <IInertiaRotationBehavior as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInertiaRotationBehavior as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InertiaRotationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaRotationBehavior";
}
unsafe impl Send for InertiaRotationBehavior {}
unsafe impl Sync for InertiaRotationBehavior {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InertiaTranslationBehavior(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InertiaTranslationBehavior, windows_core::IUnknown, windows_core::IInspectable);
impl InertiaTranslationBehavior {
    pub fn DesiredDeceleration(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredDeceleration(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredDeceleration)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn DesiredDisplacement(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DesiredDisplacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDesiredDisplacement(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDesiredDisplacement)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for InertiaTranslationBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInertiaTranslationBehavior>();
}
unsafe impl windows_core::Interface for InertiaTranslationBehavior {
    type Vtable = <IInertiaTranslationBehavior as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInertiaTranslationBehavior as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InertiaTranslationBehavior {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InertiaTranslationBehavior";
}
unsafe impl Send for InertiaTranslationBehavior {}
unsafe impl Sync for InertiaTranslationBehavior {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputScope(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputScope, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputScope, super::DependencyObject);
impl InputScope {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputScope, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
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
    pub fn Names(&self) -> windows_core::Result<windows_collections::IVector<InputScopeName>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Names)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InputScope {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputScope>();
}
unsafe impl windows_core::Interface for InputScope {
    type Vtable = <IInputScope as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputScope as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputScope {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InputScope";
}
unsafe impl Send for InputScope {}
unsafe impl Sync for InputScope {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputScopeName(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputScopeName, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputScopeName, super::DependencyObject);
impl InputScopeName {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputScopeName, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
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
    pub fn NameValue(&self) -> windows_core::Result<InputScopeNameValue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NameValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetNameValue(&self, value: InputScopeNameValue) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNameValue)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CreateInstance(namevalue: InputScopeNameValue) -> windows_core::Result<Self> {
        Self::IInputScopeNameFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), namevalue, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputScopeNameFactory<R, F: FnOnce(&IInputScopeNameFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputScopeName, IInputScopeNameFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputScopeName {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputScopeName>();
}
unsafe impl windows_core::Interface for InputScopeName {
    type Vtable = <IInputScopeName as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputScopeName as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputScopeName {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.InputScopeName";
}
unsafe impl Send for InputScopeName {}
unsafe impl Sync for InputScopeName {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputScopeNameValue(pub i32);
impl InputScopeNameValue {
    pub const Default: Self = Self(0);
    pub const Url: Self = Self(1);
    pub const EmailSmtpAddress: Self = Self(5);
    pub const PersonalFullName: Self = Self(7);
    pub const CurrencyAmountAndSymbol: Self = Self(20);
    pub const CurrencyAmount: Self = Self(21);
    pub const DateMonthNumber: Self = Self(23);
    pub const DateDayNumber: Self = Self(24);
    pub const DateYear: Self = Self(25);
    pub const Digits: Self = Self(28);
    pub const Number: Self = Self(29);
    pub const Password: Self = Self(31);
    pub const TelephoneNumber: Self = Self(32);
    pub const TelephoneCountryCode: Self = Self(33);
    pub const TelephoneAreaCode: Self = Self(34);
    pub const TelephoneLocalNumber: Self = Self(35);
    pub const TimeHour: Self = Self(37);
    pub const TimeMinutesOrSeconds: Self = Self(38);
    pub const NumberFullWidth: Self = Self(39);
    pub const AlphanumericHalfWidth: Self = Self(40);
    pub const AlphanumericFullWidth: Self = Self(41);
    pub const Hiragana: Self = Self(44);
    pub const KatakanaHalfWidth: Self = Self(45);
    pub const KatakanaFullWidth: Self = Self(46);
    pub const Hanja: Self = Self(47);
    pub const HangulHalfWidth: Self = Self(48);
    pub const HangulFullWidth: Self = Self(49);
    pub const Search: Self = Self(50);
    pub const Formula: Self = Self(51);
    pub const SearchIncremental: Self = Self(52);
    pub const ChineseHalfWidth: Self = Self(53);
    pub const ChineseFullWidth: Self = Self(54);
    pub const NativeScript: Self = Self(55);
    pub const Text: Self = Self(57);
    pub const Chat: Self = Self(58);
    pub const NameOrPhoneNumber: Self = Self(59);
    pub const EmailNameOrAddress: Self = Self(60);
    pub const Maps: Self = Self(62);
    pub const NumericPassword: Self = Self(63);
    pub const NumericPin: Self = Self(64);
    pub const AlphanumericPin: Self = Self(65);
    pub const FormulaNumber: Self = Self(67);
    pub const ChatWithoutEmoji: Self = Self(68);
}
impl windows_core::imp::TypeKind for InputScopeNameValue {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputScopeNameValue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.InputScopeNameValue;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.InputScopeNameValue");
}
windows_core::imp::define_interface!(KeyEventHandler, KeyEventHandler_Vtbl, 0xdb68e7cc_9a2b_527d_9989_25284daccc03);
impl windows_core::RuntimeType for KeyEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl KeyEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&KeyEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<KeyRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct KeyEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct KeyEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> KeyEventHandlerBox<F> {
    const VTABLE: KeyEventHandler_Vtbl = KeyEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<KeyEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<KeyEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<KeyEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<KeyEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(KeyRoutedEventArgs, super::RoutedEventArgs);
impl KeyRoutedEventArgs {
    pub fn Key(&self) -> windows_core::Result<windows::System::VirtualKey> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<windows::UI::Core::CorePhysicalKeyStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub fn OriginalKey(&self) -> windows_core::Result<windows::System::VirtualKey> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OriginalKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn DeviceId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for KeyRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyRoutedEventArgs>();
}
unsafe impl windows_core::Interface for KeyRoutedEventArgs {
    type Vtable = <IKeyRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyRoutedEventArgs";
}
unsafe impl Send for KeyRoutedEventArgs {}
unsafe impl Sync for KeyRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyTipPlacementMode(pub i32);
impl KeyTipPlacementMode {
    pub const Auto: Self = Self(0);
    pub const Bottom: Self = Self(1);
    pub const Top: Self = Self(2);
    pub const Left: Self = Self(3);
    pub const Right: Self = Self(4);
    pub const Center: Self = Self(5);
    pub const Hidden: Self = Self(6);
}
impl windows_core::imp::TypeKind for KeyTipPlacementMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for KeyTipPlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.KeyTipPlacementMode;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.KeyTipPlacementMode");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyboardAccelerator(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyboardAccelerator, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(KeyboardAccelerator, super::DependencyObject);
impl KeyboardAccelerator {
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
    pub fn Key(&self) -> windows_core::Result<windows::System::VirtualKey> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetKey(&self, value: windows::System::VirtualKey) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Modifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modifiers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetModifiers(&self, value: windows::System::VirtualKeyModifiers) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetModifiers)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsEnabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ScopeOwner(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScopeOwner)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScopeOwner)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Invoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<KeyboardAcceleratorInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, KeyboardAcceleratorInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Invoked)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveInvoked))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IKeyboardAcceleratorFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    fn IKeyboardAcceleratorFactory<R, F: FnOnce(&IKeyboardAcceleratorFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IKeyboardAcceleratorStatics<R, F: FnOnce(&IKeyboardAcceleratorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<KeyboardAccelerator, IKeyboardAcceleratorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for KeyboardAccelerator {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyboardAccelerator>();
}
unsafe impl windows_core::Interface for KeyboardAccelerator {
    type Vtable = <IKeyboardAccelerator as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyboardAccelerator as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyboardAccelerator {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAccelerator";
}
unsafe impl Send for KeyboardAccelerator {}
unsafe impl Sync for KeyboardAccelerator {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyboardAcceleratorInvokedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyboardAcceleratorInvokedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl KeyboardAcceleratorInvokedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Element(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Element)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAccelerator(&self) -> windows_core::Result<KeyboardAccelerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyboardAccelerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for KeyboardAcceleratorInvokedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyboardAcceleratorInvokedEventArgs>();
}
unsafe impl windows_core::Interface for KeyboardAcceleratorInvokedEventArgs {
    type Vtable = <IKeyboardAcceleratorInvokedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyboardAcceleratorInvokedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyboardAcceleratorInvokedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.KeyboardAcceleratorInvokedEventArgs";
}
unsafe impl Send for KeyboardAcceleratorInvokedEventArgs {}
unsafe impl Sync for KeyboardAcceleratorInvokedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyboardAcceleratorPlacementMode(pub i32);
impl KeyboardAcceleratorPlacementMode {
    pub const Auto: Self = Self(0);
    pub const Hidden: Self = Self(1);
}
impl windows_core::imp::TypeKind for KeyboardAcceleratorPlacementMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for KeyboardAcceleratorPlacementMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.KeyboardAcceleratorPlacementMode;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.KeyboardAcceleratorPlacementMode");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KeyboardNavigationMode(pub i32);
impl KeyboardNavigationMode {
    pub const Local: Self = Self(0);
    pub const Cycle: Self = Self(1);
    pub const Once: Self = Self(2);
}
impl windows_core::imp::TypeKind for KeyboardNavigationMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for KeyboardNavigationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.KeyboardNavigationMode;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.KeyboardNavigationMode");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LosingFocusEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LosingFocusEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(LosingFocusEventArgs, super::RoutedEventArgs);
impl LosingFocusEventArgs {
    pub fn OldFocusedElement(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OldFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn NewFocusedElement(&self) -> windows_core::Result<super::DependencyObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetNewFocusedElement<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNewFocusedElement)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Direction(&self) -> windows_core::Result<FocusNavigationDirection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub fn InputDevice(&self) -> windows_core::Result<FocusInputDeviceKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InputDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Cancel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCancel(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCancel)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TryCancel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryCancel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TrySetNewFocusedElement<P0>(&self, element: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrySetNewFocusedElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for LosingFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILosingFocusEventArgs>();
}
unsafe impl windows_core::Interface for LosingFocusEventArgs {
    type Vtable = <ILosingFocusEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILosingFocusEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LosingFocusEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.LosingFocusEventArgs";
}
unsafe impl Send for LosingFocusEventArgs {}
unsafe impl Sync for LosingFocusEventArgs {}
windows_core::imp::define_interface!(ManipulationCompletedEventHandler, ManipulationCompletedEventHandler_Vtbl, 0xd51df8db_71cd_5bfd_8426_767218ee55ec);
impl windows_core::RuntimeType for ManipulationCompletedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ManipulationCompletedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&ManipulationCompletedEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<ManipulationCompletedRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationCompletedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct ManipulationCompletedEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> ManipulationCompletedEventHandlerBox<F> {
    const VTABLE: ManipulationCompletedEventHandler_Vtbl = ManipulationCompletedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ManipulationCompletedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ManipulationCompletedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ManipulationCompletedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<ManipulationCompletedEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationCompletedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationCompletedRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ManipulationCompletedRoutedEventArgs, super::RoutedEventArgs);
impl ManipulationCompletedRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ManipulationCompletedRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "UI_Composition")]
    pub fn Container(&self) -> windows_core::Result<super::UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInertial(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInertial)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Velocities(&self) -> windows_core::Result<super::super::Input::ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ManipulationCompletedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationCompletedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationCompletedRoutedEventArgs {
    type Vtable = <IManipulationCompletedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationCompletedRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationCompletedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationCompletedRoutedEventArgs";
}
unsafe impl Send for ManipulationCompletedRoutedEventArgs {}
unsafe impl Sync for ManipulationCompletedRoutedEventArgs {}
windows_core::imp::define_interface!(ManipulationDeltaEventHandler, ManipulationDeltaEventHandler_Vtbl, 0x83f2d4ce_105f_5392_a38a_b7467b7c2ea5);
impl windows_core::RuntimeType for ManipulationDeltaEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ManipulationDeltaEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&ManipulationDeltaEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<ManipulationDeltaRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationDeltaEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct ManipulationDeltaEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> ManipulationDeltaEventHandlerBox<F> {
    const VTABLE: ManipulationDeltaEventHandler_Vtbl = ManipulationDeltaEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ManipulationDeltaEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ManipulationDeltaEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ManipulationDeltaEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<ManipulationDeltaEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationDeltaRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationDeltaRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ManipulationDeltaRoutedEventArgs, super::RoutedEventArgs);
impl ManipulationDeltaRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ManipulationDeltaRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "UI_Composition")]
    pub fn Container(&self) -> windows_core::Result<super::UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInertial(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInertial)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Delta(&self) -> windows_core::Result<super::super::Input::ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Velocities(&self) -> windows_core::Result<super::super::Input::ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Complete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ManipulationDeltaRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationDeltaRoutedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationDeltaRoutedEventArgs {
    type Vtable = <IManipulationDeltaRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationDeltaRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationDeltaRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationDeltaRoutedEventArgs";
}
unsafe impl Send for ManipulationDeltaRoutedEventArgs {}
unsafe impl Sync for ManipulationDeltaRoutedEventArgs {}
windows_core::imp::define_interface!(ManipulationInertiaStartingEventHandler, ManipulationInertiaStartingEventHandler_Vtbl, 0x5de296bd_6f1c_5f60_9180_10705282576c);
impl windows_core::RuntimeType for ManipulationInertiaStartingEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ManipulationInertiaStartingEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&ManipulationInertiaStartingEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<ManipulationInertiaStartingRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationInertiaStartingEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct ManipulationInertiaStartingEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> ManipulationInertiaStartingEventHandlerBox<F> {
    const VTABLE: ManipulationInertiaStartingEventHandler_Vtbl = ManipulationInertiaStartingEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ManipulationInertiaStartingEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ManipulationInertiaStartingEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ManipulationInertiaStartingEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<ManipulationInertiaStartingEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationInertiaStartingRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationInertiaStartingRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ManipulationInertiaStartingRoutedEventArgs, super::RoutedEventArgs);
impl ManipulationInertiaStartingRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ManipulationInertiaStartingRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "UI_Composition")]
    pub fn Container(&self) -> windows_core::Result<super::UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ExpansionBehavior(&self) -> windows_core::Result<InertiaExpansionBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpansionBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetExpansionBehavior<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InertiaExpansionBehavior>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetExpansionBehavior)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn RotationBehavior(&self) -> windows_core::Result<InertiaRotationBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RotationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetRotationBehavior<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InertiaRotationBehavior>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRotationBehavior)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn TranslationBehavior(&self) -> windows_core::Result<InertiaTranslationBehavior> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TranslationBehavior)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTranslationBehavior<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InertiaTranslationBehavior>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTranslationBehavior)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
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
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Delta(&self) -> windows_core::Result<super::super::Input::ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Velocities(&self) -> windows_core::Result<super::super::Input::ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ManipulationInertiaStartingRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationInertiaStartingRoutedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationInertiaStartingRoutedEventArgs {
    type Vtable = <IManipulationInertiaStartingRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationInertiaStartingRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationInertiaStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationInertiaStartingRoutedEventArgs";
}
unsafe impl Send for ManipulationInertiaStartingRoutedEventArgs {}
unsafe impl Sync for ManipulationInertiaStartingRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ManipulationModes(pub u32);
impl ManipulationModes {
    pub const None: Self = Self(0);
    pub const TranslateX: Self = Self(1);
    pub const TranslateY: Self = Self(2);
    pub const TranslateRailsX: Self = Self(4);
    pub const TranslateRailsY: Self = Self(8);
    pub const Rotate: Self = Self(16);
    pub const Scale: Self = Self(32);
    pub const TranslateInertia: Self = Self(64);
    pub const RotateInertia: Self = Self(128);
    pub const ScaleInertia: Self = Self(256);
    pub const All: Self = Self(65535);
    pub const System: Self = Self(65536);
}
impl windows_core::imp::TypeKind for ManipulationModes {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ManipulationModes {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.ManipulationModes;u4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.ManipulationModes");
}
impl ManipulationModes {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for ManipulationModes {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for ManipulationModes {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for ManipulationModes {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for ManipulationModes {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for ManipulationModes {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationPivot(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationPivot, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationPivot {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ManipulationPivot, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Center(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Center)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenter(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCenter)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Radius(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Radius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetRadius(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRadius)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CreateInstanceWithCenterAndRadius(center: windows::Foundation::Point, radius: f64) -> windows_core::Result<Self> {
        Self::IManipulationPivotFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithCenterAndRadius)(windows_core::Interface::as_raw(this), center, radius, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IManipulationPivotFactory<R, F: FnOnce(&IManipulationPivotFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ManipulationPivot, IManipulationPivotFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ManipulationPivot {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationPivot>();
}
unsafe impl windows_core::Interface for ManipulationPivot {
    type Vtable = <IManipulationPivot as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationPivot as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationPivot {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationPivot";
}
unsafe impl Send for ManipulationPivot {}
unsafe impl Sync for ManipulationPivot {}
windows_core::imp::define_interface!(ManipulationStartedEventHandler, ManipulationStartedEventHandler_Vtbl, 0x41060669_304c_53ac_9d43_bc311235aae4);
impl windows_core::RuntimeType for ManipulationStartedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ManipulationStartedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&ManipulationStartedEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<ManipulationStartedRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct ManipulationStartedEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> ManipulationStartedEventHandlerBox<F> {
    const VTABLE: ManipulationStartedEventHandler_Vtbl = ManipulationStartedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ManipulationStartedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ManipulationStartedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ManipulationStartedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<ManipulationStartedEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationStartedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationStartedRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ManipulationStartedRoutedEventArgs, super::RoutedEventArgs);
impl ManipulationStartedRoutedEventArgs {
    #[cfg(feature = "UI_Composition")]
    pub fn Container(&self) -> windows_core::Result<super::UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn Cumulative(&self) -> windows_core::Result<super::super::Input::ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Complete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IManipulationStartedRoutedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IManipulationStartedRoutedEventArgsFactory<R, F: FnOnce(&IManipulationStartedRoutedEventArgsFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ManipulationStartedRoutedEventArgs, IManipulationStartedRoutedEventArgsFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for ManipulationStartedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationStartedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationStartedRoutedEventArgs {
    type Vtable = <IManipulationStartedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationStartedRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationStartedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationStartedRoutedEventArgs";
}
unsafe impl Send for ManipulationStartedRoutedEventArgs {}
unsafe impl Sync for ManipulationStartedRoutedEventArgs {}
windows_core::imp::define_interface!(ManipulationStartingEventHandler, ManipulationStartingEventHandler_Vtbl, 0x44f528f1_f0e4_505c_a0bb_0c4839b29df5);
impl windows_core::RuntimeType for ManipulationStartingEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ManipulationStartingEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&ManipulationStartingEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<ManipulationStartingRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ManipulationStartingEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct ManipulationStartingEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> ManipulationStartingEventHandlerBox<F> {
    const VTABLE: ManipulationStartingEventHandler_Vtbl = ManipulationStartingEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ManipulationStartingEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ManipulationStartingEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ManipulationStartingEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<ManipulationStartingEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationStartingRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationStartingRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ManipulationStartingRoutedEventArgs, super::RoutedEventArgs);
impl ManipulationStartingRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ManipulationStartingRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mode(&self) -> windows_core::Result<ManipulationModes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Mode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: ManipulationModes) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn Container(&self) -> windows_core::Result<super::UIElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Container)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Composition")]
    pub fn SetContainer<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContainer)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Pivot(&self) -> windows_core::Result<ManipulationPivot> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pivot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetPivot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ManipulationPivot>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPivot)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
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
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for ManipulationStartingRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationStartingRoutedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationStartingRoutedEventArgs {
    type Vtable = <IManipulationStartingRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationStartingRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationStartingRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ManipulationStartingRoutedEventArgs";
}
unsafe impl Send for ManipulationStartingRoutedEventArgs {}
unsafe impl Sync for ManipulationStartingRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NoFocusCandidateFoundEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NoFocusCandidateFoundEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(NoFocusCandidateFoundEventArgs, super::RoutedEventArgs);
impl NoFocusCandidateFoundEventArgs {
    pub fn Direction(&self) -> windows_core::Result<FocusNavigationDirection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Direction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub fn InputDevice(&self) -> windows_core::Result<FocusInputDeviceKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InputDevice)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for NoFocusCandidateFoundEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INoFocusCandidateFoundEventArgs>();
}
unsafe impl windows_core::Interface for NoFocusCandidateFoundEventArgs {
    type Vtable = <INoFocusCandidateFoundEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INoFocusCandidateFoundEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NoFocusCandidateFoundEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.NoFocusCandidateFoundEventArgs";
}
unsafe impl Send for NoFocusCandidateFoundEventArgs {}
unsafe impl Sync for NoFocusCandidateFoundEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pointer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Pointer, windows_core::IUnknown, windows_core::IInspectable);
impl Pointer {
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInContact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInContact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInRange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for Pointer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointer>();
}
unsafe impl windows_core::Interface for Pointer {
    type Vtable = <IPointer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Pointer {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.Pointer";
}
unsafe impl Send for Pointer {}
unsafe impl Sync for Pointer {}
windows_core::imp::define_interface!(PointerEventHandler, PointerEventHandler_Vtbl, 0xa48a71e1_8bb4_5597_9e31_903a3f6a04fb);
impl windows_core::RuntimeType for PointerEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl PointerEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&PointerEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<PointerRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PointerEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct PointerEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> PointerEventHandlerBox<F> {
    const VTABLE: PointerEventHandler_Vtbl = PointerEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<PointerEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<PointerEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<PointerEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<PointerEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PointerRoutedEventArgs, super::RoutedEventArgs);
impl PointerRoutedEventArgs {
    pub fn Pointer(&self) -> windows_core::Result<Pointer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pointer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyModifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyModifiers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub fn IsGenerated(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsGenerated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
    pub fn GetCurrentPoint<P0>(&self, relativeto: P0) -> windows_core::Result<super::super::Input::PointerPoint>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPoint)(windows_core::Interface::as_raw(self), relativeto.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Input"))]
    pub fn GetIntermediatePoints<P0>(&self, relativeto: P0) -> windows_core::Result<windows_collections::IVector<super::super::Input::PointerPoint>>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntermediatePoints)(windows_core::Interface::as_raw(self), relativeto.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PointerRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerRoutedEventArgs>();
}
unsafe impl windows_core::Interface for PointerRoutedEventArgs {
    type Vtable = <IPointerRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.PointerRoutedEventArgs";
}
unsafe impl Send for PointerRoutedEventArgs {}
unsafe impl Sync for PointerRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessKeyboardAcceleratorEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ProcessKeyboardAcceleratorEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ProcessKeyboardAcceleratorEventArgs {
    pub fn Key(&self) -> windows_core::Result<windows::System::VirtualKey> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Key)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Modifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Modifiers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
impl windows_core::RuntimeType for ProcessKeyboardAcceleratorEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IProcessKeyboardAcceleratorEventArgs>();
}
unsafe impl windows_core::Interface for ProcessKeyboardAcceleratorEventArgs {
    type Vtable = <IProcessKeyboardAcceleratorEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IProcessKeyboardAcceleratorEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ProcessKeyboardAcceleratorEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.ProcessKeyboardAcceleratorEventArgs";
}
unsafe impl Send for ProcessKeyboardAcceleratorEventArgs {}
unsafe impl Sync for ProcessKeyboardAcceleratorEventArgs {}
windows_core::imp::define_interface!(RightTappedEventHandler, RightTappedEventHandler_Vtbl, 0x5070e32f_3dc7_56cf_8fdd_de1b40d0b472);
impl windows_core::RuntimeType for RightTappedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl RightTappedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&RightTappedEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<RightTappedRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RightTappedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct RightTappedEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> RightTappedEventHandlerBox<F> {
    const VTABLE: RightTappedEventHandler_Vtbl = RightTappedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<RightTappedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<RightTappedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<RightTappedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<RightTappedEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RightTappedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RightTappedRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RightTappedRoutedEventArgs, super::RoutedEventArgs);
impl RightTappedRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RightTappedRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "UI_Composition")]
    pub fn GetPosition<P0>(&self, relativeto: P0) -> windows_core::Result<windows::Foundation::Point>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), relativeto.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RightTappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRightTappedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for RightTappedRoutedEventArgs {
    type Vtable = <IRightTappedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRightTappedRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RightTappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.RightTappedRoutedEventArgs";
}
unsafe impl Send for RightTappedRoutedEventArgs {}
unsafe impl Sync for RightTappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StandardUICommand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StandardUICommand, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StandardUICommand, ICommand, XamlUICommand, super::DependencyObject);
impl StandardUICommand {
    pub fn CanExecuteChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<ICommand>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CanExecuteChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCanExecuteChanged))
        }
    }
    pub fn CanExecute<P0>(&self, parameter: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanExecute)(windows_core::Interface::as_raw(this), parameter.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Execute<P0>(&self, parameter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<ICommand>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Execute)(windows_core::Interface::as_raw(this), parameter.param().abi()).ok() }
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
    pub fn Kind(&self) -> windows_core::Result<StandardUICommandKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Kind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetKind(&self, value: StandardUICommandKind) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetKind)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IStandardUICommandFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn CreateInstanceWithKind(kind: StandardUICommandKind) -> windows_core::Result<Self> {
        Self::IStandardUICommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithKind)(windows_core::Interface::as_raw(this), kind, core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateInstanceWithKind_compose<T>(kind: StandardUICommandKind, compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IStandardUICommandFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithKind)(windows_core::Interface::as_raw(this), kind, core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Description)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDescription)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Command(&self) -> windows_core::Result<ICommand> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Command)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICommand>,
    {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ExecuteRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<XamlUICommand>, windows_core::Ref<ExecuteRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<XamlUICommand, ExecuteRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ExecuteRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveExecuteRequested))
        }
    }
    pub fn CanExecuteRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<XamlUICommand>, windows_core::Ref<CanExecuteRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<XamlUICommand, CanExecuteRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CanExecuteRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCanExecuteRequested))
        }
    }
    pub fn NotifyCanExecuteChanged(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXamlUICommand>(self)?;
        unsafe { (windows_core::Interface::vtable(this).NotifyCanExecuteChanged)(windows_core::Interface::as_raw(this)).ok() }
    }
    fn IStandardUICommandFactory<R, F: FnOnce(&IStandardUICommandFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StandardUICommand, IStandardUICommandFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IStandardUICommandStatics<R, F: FnOnce(&IStandardUICommandStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StandardUICommand, IStandardUICommandStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for StandardUICommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStandardUICommand>();
}
unsafe impl windows_core::Interface for StandardUICommand {
    type Vtable = <IStandardUICommand as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStandardUICommand as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StandardUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.StandardUICommand";
}
unsafe impl Send for StandardUICommand {}
unsafe impl Sync for StandardUICommand {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct StandardUICommandKind(pub i32);
impl StandardUICommandKind {
    pub const None: Self = Self(0);
    pub const Cut: Self = Self(1);
    pub const Copy: Self = Self(2);
    pub const Paste: Self = Self(3);
    pub const SelectAll: Self = Self(4);
    pub const Delete: Self = Self(5);
    pub const Share: Self = Self(6);
    pub const Save: Self = Self(7);
    pub const Open: Self = Self(8);
    pub const Close: Self = Self(9);
    pub const Pause: Self = Self(10);
    pub const Play: Self = Self(11);
    pub const Stop: Self = Self(12);
    pub const Forward: Self = Self(13);
    pub const Backward: Self = Self(14);
    pub const Undo: Self = Self(15);
    pub const Redo: Self = Self(16);
}
impl windows_core::imp::TypeKind for StandardUICommandKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for StandardUICommandKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.StandardUICommandKind;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.StandardUICommandKind");
}
windows_core::imp::define_interface!(TappedEventHandler, TappedEventHandler_Vtbl, 0xb60074f3_125b_534e_8f9c_9769bd3f0f64);
impl windows_core::RuntimeType for TappedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl TappedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&TappedEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<TappedRoutedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TappedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct TappedEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static> TappedEventHandlerBox<F> {
    const VTABLE: TappedEventHandler_Vtbl = TappedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<TappedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<TappedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<TappedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<TappedEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TappedRoutedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TappedRoutedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TappedRoutedEventArgs, super::RoutedEventArgs);
impl TappedRoutedEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TappedRoutedEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn PointerDeviceType(&self) -> windows_core::Result<super::super::Input::PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "UI_Composition")]
    pub fn GetPosition<P0>(&self, relativeto: P0) -> windows_core::Result<windows::Foundation::Point>
    where
        P0: windows_core::Param<super::UIElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), relativeto.param().abi(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TappedRoutedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITappedRoutedEventArgs>();
}
unsafe impl windows_core::Interface for TappedRoutedEventArgs {
    type Vtable = <ITappedRoutedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITappedRoutedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TappedRoutedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.TappedRoutedEventArgs";
}
unsafe impl Send for TappedRoutedEventArgs {}
unsafe impl Sync for TappedRoutedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XYFocusKeyboardNavigationMode(pub i32);
impl XYFocusKeyboardNavigationMode {
    pub const Auto: Self = Self(0);
    pub const Enabled: Self = Self(1);
    pub const Disabled: Self = Self(2);
}
impl windows_core::imp::TypeKind for XYFocusKeyboardNavigationMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for XYFocusKeyboardNavigationMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.XYFocusKeyboardNavigationMode;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.XYFocusKeyboardNavigationMode");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XYFocusNavigationStrategy(pub i32);
impl XYFocusNavigationStrategy {
    pub const Auto: Self = Self(0);
    pub const Projection: Self = Self(1);
    pub const NavigationDirectionDistance: Self = Self(2);
    pub const RectilinearDistance: Self = Self(3);
}
impl windows_core::imp::TypeKind for XYFocusNavigationStrategy {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for XYFocusNavigationStrategy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.XYFocusNavigationStrategy;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.XYFocusNavigationStrategy");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct XYFocusNavigationStrategyOverride(pub i32);
impl XYFocusNavigationStrategyOverride {
    pub const None: Self = Self(0);
    pub const Auto: Self = Self(1);
    pub const Projection: Self = Self(2);
    pub const NavigationDirectionDistance: Self = Self(3);
    pub const RectilinearDistance: Self = Self(4);
}
impl windows_core::imp::TypeKind for XYFocusNavigationStrategyOverride {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for XYFocusNavigationStrategyOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Input.XYFocusNavigationStrategyOverride;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Input.XYFocusNavigationStrategyOverride");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct XamlUICommand(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XamlUICommand, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XamlUICommand, ICommand, super::DependencyObject);
impl XamlUICommand {
    pub fn CanExecuteChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<ICommand>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CanExecuteChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCanExecuteChanged))
        }
    }
    pub fn CanExecute<P0>(&self, parameter: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<ICommand>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanExecute)(windows_core::Interface::as_raw(this), parameter.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn Execute<P0>(&self, parameter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<ICommand>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Execute)(windows_core::Interface::as_raw(this), parameter.param().abi()).ok() }
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
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<KeyboardAccelerator>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyboardAccelerators)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
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
    pub fn Description(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDescription(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Command(&self) -> windows_core::Result<ICommand> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Command)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ICommand>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCommand)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ExecuteRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ExecuteRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, ExecuteRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ExecuteRequested)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveExecuteRequested))
        }
    }
    pub fn CanExecuteRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<CanExecuteRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, CanExecuteRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CanExecuteRequested)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCanExecuteRequested))
        }
    }
    pub fn NotifyCanExecuteChanged(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).NotifyCanExecuteChanged)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IXamlUICommandFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IXamlUICommandFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    fn IXamlUICommandFactory<R, F: FnOnce(&IXamlUICommandFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XamlUICommand, IXamlUICommandFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IXamlUICommandStatics<R, F: FnOnce(&IXamlUICommandStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XamlUICommand, IXamlUICommandStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XamlUICommand {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXamlUICommand>();
}
unsafe impl windows_core::Interface for XamlUICommand {
    type Vtable = <IXamlUICommand as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IXamlUICommand as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XamlUICommand {
    const NAME: &'static str = "Microsoft.UI.Xaml.Input.XamlUICommand";
}
unsafe impl Send for XamlUICommand {}
unsafe impl Sync for XamlUICommand {}
