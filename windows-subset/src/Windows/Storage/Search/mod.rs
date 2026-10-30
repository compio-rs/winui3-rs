windows_core::imp::define_interface!(IStorageFolderQueryOperations, IStorageFolderQueryOperations_Vtbl, 0xcb43ccc9_446b_4a4f_be97_757771be5203);
impl windows_core::RuntimeType for IStorageFolderQueryOperations {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Search.IStorageFolderQueryOperations");
}
windows_core::imp::interface_hierarchy!(IStorageFolderQueryOperations, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageFolderQueryOperations {
    pub fn GetIndexedStateAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IndexedState>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIndexedStateAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<super::IStorageItem>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemsAsync)(windows_core::Interface::as_raw(self), startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageFolderQueryOperations {
    const NAME: &'static str = "Windows.Storage.Search.IStorageFolderQueryOperations";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryOperations_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetIndexedStateAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateFileQueryOverloadDefault: usize,
    CreateFileQuery: usize,
    CreateFileQueryWithOptions: usize,
    CreateFolderQueryOverloadDefault: usize,
    CreateFolderQuery: usize,
    CreateFolderQueryWithOptions: usize,
    CreateItemQuery: usize,
    CreateItemQueryWithOptions: usize,
    GetFilesAsync: usize,
    GetFilesAsyncOverloadDefaultStartAndCount: usize,
    GetFoldersAsync: usize,
    GetFoldersAsyncOverloadDefaultStartAndCount: usize,
    pub GetItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IndexedState(pub i32);
impl IndexedState {
    pub const Unknown: Self = Self(0);
    pub const NotIndexed: Self = Self(1);
    pub const PartiallyIndexed: Self = Self(2);
    pub const FullyIndexed: Self = Self(3);
}
impl windows_core::imp::TypeKind for IndexedState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for IndexedState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexedState;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Search.IndexedState");
}
