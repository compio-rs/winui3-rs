#[cfg(feature = "Storage_FileProperties")]
pub mod FileProperties;
#[cfg(feature = "Storage_Streams")]
pub mod Streams;
windows_core::imp::define_interface!(IStorageItem, IStorageItem_Vtbl, 0x4207a996_ca2f_42f7_bde8_8b10457a7f30);
impl windows_core::RuntimeType for IStorageItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageItem");
}
windows_core::imp::interface_hierarchy!(IStorageItem, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageItem {
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<FileProperties::BasicProperties>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DateCreated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Storage_FileProperties")]
impl windows_core::RuntimeName for IStorageItem {
    const NAME: &'static str = "Windows.Storage.IStorageItem";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RenameAsyncOverloadDefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    RenameAsync: usize,
    pub DeleteAsyncOverloadDefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    DeleteAsync: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub GetBasicPropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    GetBasicPropertiesAsync: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    Attributes: usize,
    pub DateCreated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
}
