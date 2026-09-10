#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpInputSite(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExpInputSite, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ExpInputSite, windows::Foundation::IClosable);
impl ExpInputSite {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for ExpInputSite {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExpInputSite>();
}
unsafe impl windows_core::Interface for ExpInputSite {
    type Vtable = <IExpInputSite as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExpInputSite as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExpInputSite {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.ExpInputSite";
}
unsafe impl Send for ExpInputSite {}
unsafe impl Sync for ExpInputSite {}
windows_core::imp::define_interface!(IExpInputSite, IExpInputSite_Vtbl, 0x6b707b95_bbe8_5131_a6d7_b11c26cb7cb6);
impl windows_core::RuntimeType for IExpInputSite {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.Experimental.IExpInputSite");
}
impl windows_core::RuntimeName for IExpInputSite {
    const NAME: &'static str = "Microsoft.UI.Input.Experimental.IExpInputSite";
}
pub trait IExpInputSite_Impl: windows_core::IUnknownImpl {}
impl IExpInputSite_Vtbl {
    pub const fn new<Identity: IExpInputSite_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IExpInputSite, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExpInputSite as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpInputSite_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
