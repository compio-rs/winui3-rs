windows_core::imp::define_interface!(INotifyPropertyChanged, INotifyPropertyChanged_Vtbl, 0x90b17601_b065_586e_83d9_9adc3a695284);
impl windows_core::RuntimeType for INotifyPropertyChanged {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Data.INotifyPropertyChanged");
}
windows_core::imp::interface_hierarchy!(INotifyPropertyChanged, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for INotifyPropertyChanged {
    const NAME: &'static str = "Microsoft.UI.Xaml.Data.INotifyPropertyChanged";
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyPropertyChanged_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PropertyChanged: usize,
    pub RemovePropertyChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
