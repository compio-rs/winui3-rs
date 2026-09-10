#[cfg(feature = "UI_Composition")]
pub mod Composition;
#[cfg(feature = "UI_Dispatching")]
pub mod Dispatching;
#[cfg(feature = "UI_Input")]
pub mod Input;
#[cfg(feature = "UI_Text")]
pub mod Text;
#[cfg(feature = "UI_Windowing")]
pub mod Windowing;
#[cfg(feature = "UI_Xaml")]
pub mod Xaml;
windows_core::imp::define_interface!(IClosableNotifier, IClosableNotifier_Vtbl, 0x2989e93b_ed0f_5e79_90f2_eac592fc6e6a);
impl windows_core::RuntimeType for IClosableNotifier {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.IClosableNotifier");
}
windows_core::imp::interface_hierarchy!(IClosableNotifier, windows_core::IUnknown, windows_core::IInspectable);
impl IClosableNotifier {
    pub fn IsClosed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IClosableNotifier {
    const NAME: &'static str = "Microsoft.UI.IClosableNotifier";
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosableNotifier_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    Closed: usize,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    FrameworkClosed: usize,
    pub RemoveFrameworkClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IconId {
    pub Value: u64,
}
impl windows_core::imp::TypeKind for IconId {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for IconId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.IconId;u8)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.IconId");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WindowId {
    pub Value: u64,
}
impl windows_core::imp::TypeKind for WindowId {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for WindowId {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.WindowId;u8)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.WindowId");
}
