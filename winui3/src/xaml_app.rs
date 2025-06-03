use std::{cell::RefCell, marker::PhantomData};

use windows::{
    Foundation::TypedEventHandler,
    Win32::Foundation::{CO_E_NOTCONSTRUCTED, E_ILLEGAL_METHOD_CALL},
};
use windows_core::{
    implement, Array, ComObject, IInspectable, Interface, Param, Ref, Result, HSTRING,
};

use crate::Microsoft::UI::Xaml::{
    Application, ApplicationHighContrastAdjustment, ApplicationTheme, DebugSettings,
    DispatcherShutdownMode, FocusVisualKind, IApplication2, IApplication2_Impl, IApplication3,
    IApplication3_Impl, IApplicationOverrides, IApplicationOverrides_Impl, IApplication_Impl,
    LaunchActivatedEventArgs,
    Markup::{IXamlMetadataProvider, IXamlMetadataProvider_Impl, IXamlType, XmlnsDefinition},
    ResourceDictionary, ResourceManagerRequestedEventArgs, UnhandledExceptionEventHandler,
    XamlTypeInfo::XamlControlsXamlMetaDataProvider,
};
use crate::Windows::UI::Xaml as WUX;

#[allow(non_snake_case)]
pub trait XamlAppOverrides {
    fn OnLaunched(&self, base: &Application, args: Option<&LaunchActivatedEventArgs>)
        -> Result<()>;

    #[cfg(feature = "XamlNavigation")]
    fn TryResolveXamlType(&self, full_name: &HSTRING) -> Result<IXamlType>;
}

#[implement(
    Application,
    IApplication2,
    IApplication3,
    IApplicationOverrides,
    IXamlMetadataProvider
)]
pub struct XamlApp<'a, T>
where
    T: XamlAppOverrides,
{
    base: RefCell<Option<IInspectable>>,
    provider: RefCell<Option<XamlControlsXamlMetaDataProvider>>,
    inner: T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: XamlAppOverrides> XamlApp<'a, T> {
    pub fn compose(inner: T) -> Result<Application> {
        Application::IApplicationFactory(|this| unsafe {
            let app = ComObject::new(Self {
                base: RefCell::new(None),
                provider: RefCell::new(None),
                inner,
                _phantom: PhantomData,
            });
            let inner__ = app.base.as_ptr();
            let outer: IInspectable = app.into_interface();
            let mut result__ = core::mem::zeroed();
            (Interface::vtable(this).CreateInstance)(
                Interface::as_raw(this),
                Interface::as_raw(&outer),
                inner__ as _,
                &mut result__,
            )
            .and_then(|| windows_core::Type::from_abi(result__))
        })
    }

    fn with_base<R, F: FnOnce(&Application) -> Result<R>>(&self, func: F) -> Result<R> {
        self.base
            .borrow()
            .as_ref()
            .ok_or_else(|| CO_E_NOTCONSTRUCTED.into())
            .and_then(|base| base.cast::<Application>())
            .and_then(|base| func(&base))
    }

    fn with_iface<I, R, F: FnOnce(&I) -> Result<R>>(&self, func: F) -> Result<R>
    where
        I: Interface,
    {
        self.base
            .borrow()
            .as_ref()
            .ok_or_else(|| CO_E_NOTCONSTRUCTED.into())
            .and_then(|base| base.cast::<I>())
            .and_then(|base| func(&base))
    }

    fn with_provider<R, F: FnOnce(&XamlControlsXamlMetaDataProvider) -> Result<R>>(
        &self,
        func: F,
    ) -> Result<R> {
        self.provider
            .borrow()
            .as_ref()
            .ok_or_else(|| E_ILLEGAL_METHOD_CALL.into())
            .and_then(|provider| func(provider))
    }
}

impl<T: XamlAppOverrides> IApplication_Impl for XamlApp_Impl<'_, T> {
    fn Resources(&self) -> Result<ResourceDictionary> {
        self.with_base(|base| base.Resources())
    }

    fn SetResources(&self, value: Ref<'_, ResourceDictionary>) -> Result<()> {
        self.with_base(|base| base.SetResources(value.as_ref()))
    }

    fn DebugSettings(&self) -> Result<DebugSettings> {
        self.with_base(|base| base.DebugSettings())
    }

    fn RequestedTheme(&self) -> Result<ApplicationTheme> {
        self.with_base(|base| base.RequestedTheme())
    }

    fn SetRequestedTheme(&self, value: ApplicationTheme) -> Result<()> {
        self.with_base(|base| base.SetRequestedTheme(value))
    }

    fn FocusVisualKind(&self) -> Result<FocusVisualKind> {
        self.with_base(|base| base.FocusVisualKind())
    }

    fn SetFocusVisualKind(&self, value: FocusVisualKind) -> Result<()> {
        self.with_base(|base| base.SetFocusVisualKind(value))
    }

    fn HighContrastAdjustment(&self) -> Result<ApplicationHighContrastAdjustment> {
        self.with_base(|base| base.HighContrastAdjustment())
    }

    fn SetHighContrastAdjustment(&self, value: ApplicationHighContrastAdjustment) -> Result<()> {
        self.with_base(|base| base.SetHighContrastAdjustment(value))
    }

    fn UnhandledException(&self, handler: Ref<'_, UnhandledExceptionEventHandler>) -> Result<i64> {
        self.with_base(|base| base.UnhandledException(handler.as_ref()))
    }

    fn RemoveUnhandledException(&self, token: i64) -> Result<()> {
        self.with_base(|base| base.RemoveUnhandledException(token))
    }

    fn Exit(&self) -> Result<()> {
        self.with_base(|base| base.Exit())
    }
}

impl<T: XamlAppOverrides> IApplication2_Impl for XamlApp_Impl<'_, T> {
    fn ResourceManagerRequested(
        &self,
        handler: Ref<'_, TypedEventHandler<IInspectable, ResourceManagerRequestedEventArgs>>,
    ) -> Result<i64> {
        self.with_iface(|this: &IApplication2| unsafe {
            let mut result = Default::default();
            (this.vtable().ResourceManagerRequested)(
                this.as_raw(),
                handler.as_ref().param().abi(),
                &mut result,
            )
            .map(|| result)
        })
    }

    fn RemoveResourceManagerRequested(&self, token: i64) -> Result<()> {
        self.with_iface(|this: &IApplication2| unsafe {
            (this.vtable().RemoveResourceManagerRequested)(this.as_raw(), token).ok()
        })
    }
}

impl<T: XamlAppOverrides> IApplication3_Impl for XamlApp_Impl<'_, T> {
    fn DispatcherShutdownMode(&self) -> Result<DispatcherShutdownMode> {
        self.with_iface(|this: &IApplication3| unsafe {
            let mut result = Default::default();
            (this.vtable().DispatcherShutdownMode)(this.as_raw(), &mut result).map(|| result)
        })
    }

    fn SetDispatcherShutdownMode(&self, value: DispatcherShutdownMode) -> Result<()> {
        self.with_iface(|this: &IApplication3| unsafe {
            (this.vtable().SetDispatcherShutdownMode)(this.as_raw(), value).ok()
        })
    }
}

impl<T: XamlAppOverrides> IApplicationOverrides_Impl for XamlApp_Impl<'_, T> {
    fn OnLaunched(&self, args: Ref<'_, LaunchActivatedEventArgs>) -> Result<()> {
        XamlControlsXamlMetaDataProvider::Initialize()?;

        self.provider
            .borrow_mut()
            .replace(XamlControlsXamlMetaDataProvider::new()?);

        self.with_base(|base| self.inner.OnLaunched(base, args.as_ref()))
    }
}

impl<T: XamlAppOverrides> IXamlMetadataProvider_Impl for XamlApp_Impl<'_, T> {
    #[cfg(feature = "XamlNavigation")]
    fn GetXamlType(&self, type_name: &WUX::Interop::TypeName) -> Result<IXamlType> {
        let page_resolve = match type_name.Kind {
            WUX::Interop::TypeKind::Custom => self.inner.TryResolveXamlType(&type_name.Name),
            _ => Err(windows_core::Error::empty()),
        };
        page_resolve.or_else(|_| self.with_provider(|provider| provider.GetXamlType(type_name)))
    }

    #[cfg(not(feature = "XamlNavigation"))]
    fn GetXamlType(&self, type_name: &WUX::Interop::TypeName) -> Result<IXamlType> {
        self.with_provider(|provider| provider.GetXamlType(type_name))
    }

    #[cfg(feature = "XamlNavigation")]
    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        self.inner
            .TryResolveXamlType(full_name)
            .or_else(|_| self.with_provider(|provider| provider.GetXamlTypeByFullName(full_name)))
    }

    #[cfg(not(feature = "XamlNavigation"))]
    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        self.with_provider(|provider| provider.GetXamlTypeByFullName(full_name))
    }

    fn GetXmlnsDefinitions(&self) -> Result<Array<XmlnsDefinition>> {
        self.with_provider(|provider| provider.GetXmlnsDefinitions())
    }
}
