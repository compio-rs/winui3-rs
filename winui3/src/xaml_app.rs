#![allow(deprecated)]

use windows_core::{
    imp::WeakRefCount, implement, Array, IInspectable_Vtbl, Interface, Ref, Result, HSTRING,
};

use crate::Microsoft::UI::Xaml::{
    Application, IApplicationFactory, IApplicationFactory_Vtbl, IApplicationOverrides,
    IApplicationOverrides_Impl, LaunchActivatedEventArgs,
    Markup::{IXamlMetadataProvider, IXamlMetadataProvider_Impl, IXamlType, XmlnsDefinition},
    XamlTypeInfo::XamlControlsXamlMetaDataProvider,
};
use crate::Windows::UI::Xaml as WUX;
use crate::{ChildClass, ChildClassImpl, Compose, CreateInstanceFn};

#[deprecated]
#[allow(non_snake_case)]
pub trait XamlAppOverrides {
    fn OnLaunched(&self, base: &Application, args: Option<&LaunchActivatedEventArgs>)
        -> Result<()>;
}

#[deprecated]
#[implement(IApplicationOverrides, IXamlMetadataProvider)]
pub struct XamlApp<T>
where
    T: XamlAppOverrides + 'static,
{
    inner: T,
    provider: XamlControlsXamlMetaDataProvider,
}

impl<T> ChildClass for XamlApp<T>
where
    T: XamlAppOverrides + 'static,
{
    type BaseType = Application;
    type FactoryInterface = IApplicationFactory;

    fn create_interface_fn(vtable: &IApplicationFactory_Vtbl) -> CreateInstanceFn {
        vtable.CreateInstance
    }

    fn identity_vtable(vtable: &mut Self::Outer) -> &mut &'static IInspectable_Vtbl {
        &mut vtable.identity
    }

    fn ref_count(vtable: &Self::Outer) -> &WeakRefCount {
        &vtable.count
    }

    fn into_outer(self) -> Self::Outer {
        Self::into_outer(self)
    }
}

impl<T: XamlAppOverrides> XamlApp<T> {
    pub fn compose(inner: T) -> Result<Application> {
        let app = Self {
            inner,
            provider: XamlControlsXamlMetaDataProvider::new()?,
        };
        Application::IApplicationFactory(|factory| Compose::compose_with(app, factory))
    }
}

impl<T: XamlAppOverrides> ChildClassImpl for XamlApp_Impl<T> {}

impl<T: XamlAppOverrides> IApplicationOverrides_Impl for XamlApp_Impl<T> {
    fn OnLaunched(&self, args: Ref<'_, LaunchActivatedEventArgs>) -> Result<()> {
        let base = self.base()?.cast::<Application>()?;
        self.inner.OnLaunched(&base, args.as_ref())
    }
}

impl<T: XamlAppOverrides> IXamlMetadataProvider_Impl for XamlApp_Impl<T> {
    fn GetXamlType(&self, type_name: &WUX::Interop::TypeName) -> Result<IXamlType> {
        self.provider.GetXamlType(type_name)
    }

    fn GetXamlTypeByFullName(&self, full_name: &HSTRING) -> Result<IXamlType> {
        self.provider.GetXamlTypeByFullName(full_name)
    }

    fn GetXmlnsDefinitions(&self) -> Result<Array<XmlnsDefinition>> {
        self.provider.GetXmlnsDefinitions()
    }
}
