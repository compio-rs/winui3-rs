#[cfg(feature = "Storage_FileProperties")]
pub mod FileProperties;
#[cfg(feature = "Storage_Search")]
pub mod Search;
#[cfg(feature = "Storage_Streams")]
pub mod Streams;
#[cfg(feature = "Storage_Streams")]
windows_core::imp::define_interface!(IStorageFile, IStorageFile_Vtbl, 0xfa3f6186_4214_428c_a64c_14c9ac7315ea);
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for IStorageFile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFile");
}
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(IStorageFile, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(IStorageFile, Streams::IInputStreamReference, Streams::IRandomAccessStreamReference, IStorageItem);
#[cfg(feature = "Storage_Streams")]
impl IStorageFile {
    pub fn FileType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn OpenTransactedWriteAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<StorageStreamTransaction>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenTransactedWriteAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CopyOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> windows_core::Result<windows_future::IAsyncOperation<StorageFile>>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyOverloadDefaultNameAndOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CopyOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFile>>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyOverloadDefaultOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CopyAndReplaceAsync<P0>(&self, filetoreplace: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyAndReplaceAsync)(windows_core::Interface::as_raw(self), filetoreplace.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MoveOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveOverloadDefaultNameAndOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MoveOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveOverloadDefaultOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MoveAndReplaceAsync<P0>(&self, filetoreplace: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveAndReplaceAsync)(windows_core::Interface::as_raw(self), filetoreplace.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OpenSequentialReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<Streams::IInputStream>> {
        let this = &windows_core::Interface::cast::<Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenSequentialReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>> {
        let this = &windows_core::Interface::cast::<Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<windows_time::DateTime> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IStorageFile {
    const NAME: &'static str = "Windows.Storage.IStorageFile";
}
#[cfg(feature = "Storage_Streams")]
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFile_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FileType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    OpenAsync: usize,
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyOverloadDefaultNameAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyOverloadDefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CopyOverload: usize,
    pub CopyAndReplaceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveOverloadDefaultNameAndOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveOverloadDefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    MoveOverload: usize,
    pub MoveAndReplaceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageFile2, IStorageFile2_Vtbl, 0x954e4bcf_0a77_42fb_b777_c2ed58a52e44);
impl windows_core::RuntimeType for IStorageFile2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFile2");
}
windows_core::imp::interface_hierarchy!(IStorageFile2, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IStorageFile2 {
    const NAME: &'static str = "Windows.Storage.IStorageFile2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFile2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IStorageFilePropertiesWithAvailability, IStorageFilePropertiesWithAvailability_Vtbl, 0xafcbbe9b_582b_4133_9648_e44ca46ee491);
impl windows_core::RuntimeType for IStorageFilePropertiesWithAvailability {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFilePropertiesWithAvailability");
}
windows_core::imp::interface_hierarchy!(IStorageFilePropertiesWithAvailability, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageFilePropertiesWithAvailability {
    pub fn IsAvailable(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAvailable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IStorageFilePropertiesWithAvailability {
    const NAME: &'static str = "Windows.Storage.IStorageFilePropertiesWithAvailability";
}
pub trait IStorageFilePropertiesWithAvailability_Impl: windows_core::IUnknownImpl {
    fn IsAvailable(&self) -> windows_core::Result<bool>;
}
impl IStorageFilePropertiesWithAvailability_Vtbl {
    pub const fn new<Identity: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsAvailable<Identity: IStorageFilePropertiesWithAvailability_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFilePropertiesWithAvailability_Impl::IsAvailable(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageFilePropertiesWithAvailability, OFFSET>(),
            IsAvailable: IsAvailable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageFilePropertiesWithAvailability as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFilePropertiesWithAvailability_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageFileStatics, IStorageFileStatics_Vtbl, 0x5984c710_daf2_43c8_8bb4_a4d3eacfd03f);
impl windows_core::RuntimeType for IStorageFileStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFileStatics");
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IStorageFileStatics {
    const NAME: &'static str = "Windows.Storage.IStorageFileStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub GetFileFromPathAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFileFromPathAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetFileFromApplicationUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFileFromApplicationUriAsync: usize,
    CreateStreamedFileAsync: usize,
    ReplaceWithStreamedFileAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateStreamedFileFromUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateStreamedFileFromUriAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ReplaceWithStreamedFileFromUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ReplaceWithStreamedFileFromUriAsync: usize,
}
windows_core::imp::define_interface!(IStorageFileStatics2, IStorageFileStatics2_Vtbl, 0x5c76a781_212e_4af9_8f04_740cae108974);
impl windows_core::RuntimeType for IStorageFileStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFileStatics2");
}
impl windows_core::RuntimeName for IStorageFileStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageFileStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IStorageFolder, IStorageFolder_Vtbl, 0x72d1cb78_b3ef_4f75_a80b_6fd9dae2944b);
impl windows_core::RuntimeType for IStorageFolder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFolder");
}
windows_core::imp::interface_hierarchy!(IStorageFolder, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IStorageFolder, IStorageItem);
impl IStorageFolder {
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFile>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFolder>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFolderAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFile>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFolderAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFolder>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolderAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetItemAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<IStorageItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<StorageFile>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<StorageFolder>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IStorageItem>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemsAsyncOverloadDefaultStartAndCount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<windows_time::DateTime> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IStorageFolder {
    const NAME: &'static str = "Windows.Storage.IStorageFolder";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFileAsyncOverloadDefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFileAsyncOverloadDefaultOptions: usize,
    CreateFileAsync: usize,
    #[cfg(feature = "Storage_Search")]
    pub CreateFolderAsyncOverloadDefaultOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    CreateFolderAsyncOverloadDefaultOptions: usize,
    CreateFolderAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub GetFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFileAsync: usize,
    #[cfg(feature = "Storage_Search")]
    pub GetFolderAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetFolderAsync: usize,
    pub GetItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetFilesAsyncOverloadDefaultOptionsStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetFilesAsyncOverloadDefaultOptionsStartAndCount: usize,
    #[cfg(feature = "Storage_Search")]
    pub GetFoldersAsyncOverloadDefaultOptionsStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetFoldersAsyncOverloadDefaultOptionsStartAndCount: usize,
    pub GetItemsAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageFolder2, IStorageFolder2_Vtbl, 0xe827e8b9_08d9_4a8e_a0ac_fe5ed3cbbbd3);
impl windows_core::RuntimeType for IStorageFolder2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFolder2");
}
windows_core::imp::interface_hierarchy!(IStorageFolder2, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageFolder2 {
    pub fn TryGetItemAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<IStorageItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryGetItemAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageFolder2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder2";
}
pub trait IStorageFolder2_Impl: windows_core::IUnknownImpl {
    fn TryGetItemAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<IStorageItem>>;
}
impl IStorageFolder2_Vtbl {
    pub const fn new<Identity: IStorageFolder2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TryGetItemAsync<Identity: IStorageFolder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolder2_Impl::TryGetItemAsync(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageFolder2, OFFSET>(), TryGetItemAsync: TryGetItemAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageFolder2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryGetItemAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageFolder3, IStorageFolder3_Vtbl, 0x9f617899_bde1_4124_aeb3_b06ad96f98d4);
impl windows_core::RuntimeType for IStorageFolder3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFolder3");
}
impl windows_core::RuntimeName for IStorageFolder3 {
    const NAME: &'static str = "Windows.Storage.IStorageFolder3";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolder3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IStorageFolderStatics, IStorageFolderStatics_Vtbl, 0x08f327ff_85d5_48b9_aee9_28511e339f9f);
impl windows_core::RuntimeType for IStorageFolderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFolderStatics");
}
#[cfg(feature = "Storage_Search")]
impl windows_core::RuntimeName for IStorageFolderStatics {
    const NAME: &'static str = "Windows.Storage.IStorageFolderStatics";
}
#[cfg(feature = "Storage_Search")]
pub trait IStorageFolderStatics_Impl: windows_core::IUnknownImpl {
    fn GetFolderFromPathAsync(&self, path: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFolder>>;
}
#[cfg(feature = "Storage_Search")]
impl IStorageFolderStatics_Vtbl {
    pub const fn new<Identity: IStorageFolderStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFolderFromPathAsync<Identity: IStorageFolderStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, path: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageFolderStatics_Impl::GetFolderFromPathAsync(this, core::mem::transmute(&path)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageFolderStatics, OFFSET>(),
            GetFolderFromPathAsync: GetFolderFromPathAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageFolderStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Search")]
    pub GetFolderFromPathAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetFolderFromPathAsync: usize,
}
windows_core::imp::define_interface!(IStorageFolderStatics2, IStorageFolderStatics2_Vtbl, 0xb4656dc3_71d2_467d_8b29_371f0f62bf6f);
impl windows_core::RuntimeType for IStorageFolderStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageFolderStatics2");
}
impl windows_core::RuntimeName for IStorageFolderStatics2 {
    const NAME: &'static str = "Windows.Storage.IStorageFolderStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
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
windows_core::imp::define_interface!(IStorageItem2, IStorageItem2_Vtbl, 0x53f926d2_083c_4283_b45b_81c007237e44);
impl windows_core::RuntimeType for IStorageItem2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageItem2");
}
windows_core::imp::interface_hierarchy!(IStorageItem2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IStorageItem2, IStorageItem);
impl IStorageItem2 {
    #[cfg(feature = "Storage_Search")]
    pub fn GetParentAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<StorageFolder>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsEqual<P0>(&self, item: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IStorageItem>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<windows_time::DateTime> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
#[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
impl windows_core::RuntimeName for IStorageItem2 {
    const NAME: &'static str = "Windows.Storage.IStorageItem2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItem2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Search")]
    pub GetParentAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Search"))]
    GetParentAsync: usize,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemProperties, IStorageItemProperties_Vtbl, 0x86664478_8029_46fe_a789_1c2f3e2ffb5c);
impl windows_core::RuntimeType for IStorageItemProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageItemProperties");
}
windows_core::imp::interface_hierarchy!(IStorageItemProperties, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageItemProperties {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderRelativeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FolderRelativeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageItemProperties {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetThumbnailAsyncOverloadDefaultSizeDefaultOptions: usize,
    GetThumbnailAsyncOverloadDefaultOptions: usize,
    GetThumbnailAsync: usize,
    pub DisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FolderRelativeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemProperties2, IStorageItemProperties2_Vtbl, 0x8e86a951_04b9_4bd2_929d_fef3f71621d0);
impl windows_core::RuntimeType for IStorageItemProperties2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageItemProperties2");
}
windows_core::imp::interface_hierarchy!(IStorageItemProperties2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IStorageItemProperties2, IStorageItemProperties);
impl IStorageItemProperties2 {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderRelativeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderRelativeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageItemProperties2 {
    const NAME: &'static str = "Windows.Storage.IStorageItemProperties2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemProperties2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IStorageItemPropertiesWithProvider, IStorageItemPropertiesWithProvider_Vtbl, 0x861bf39b_6368_4dee_b40e_74684a5ce714);
impl windows_core::RuntimeType for IStorageItemPropertiesWithProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageItemPropertiesWithProvider");
}
windows_core::imp::interface_hierarchy!(IStorageItemPropertiesWithProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IStorageItemPropertiesWithProvider, IStorageItemProperties);
impl IStorageItemPropertiesWithProvider {
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderRelativeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderRelativeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageItemPropertiesWithProvider {
    const NAME: &'static str = "Windows.Storage.IStorageItemPropertiesWithProvider";
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemPropertiesWithProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IStorageStreamTransaction, IStorageStreamTransaction_Vtbl, 0xf67cf363_a53d_4d94_ae2c_67232d93acdd);
impl windows_core::RuntimeType for IStorageStreamTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.IStorageStreamTransaction");
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IStorageStreamTransaction {
    const NAME: &'static str = "Windows.Storage.IStorageStreamTransaction";
}
#[cfg(feature = "Storage_Streams")]
pub trait IStorageStreamTransaction_Impl: super::Foundation::IClosable_Impl {
    fn Stream(&self) -> windows_core::Result<Streams::IRandomAccessStream>;
    fn CommitAsync(&self) -> windows_core::Result<windows_future::IAsyncAction>;
}
#[cfg(feature = "Storage_Streams")]
impl IStorageStreamTransaction_Vtbl {
    pub const fn new<Identity: IStorageStreamTransaction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Stream<Identity: IStorageStreamTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageStreamTransaction_Impl::Stream(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommitAsync<Identity: IStorageStreamTransaction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageStreamTransaction_Impl::CommitAsync(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageStreamTransaction, OFFSET>(),
            Stream: Stream::<Identity, OFFSET>,
            CommitAsync: CommitAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageStreamTransaction as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageStreamTransaction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    pub CommitAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageFile(windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::interface_hierarchy!(StorageFile, windows_core::IUnknown, windows_core::IInspectable, IStorageFile);
#[cfg(feature = "Storage_Streams")]
windows_core::imp::required_hierarchy!(StorageFile, Streams::IInputStreamReference, Streams::IRandomAccessStreamReference, IStorageFile2, IStorageFilePropertiesWithAvailability, IStorageItem, IStorageItem2, IStorageItemProperties, IStorageItemProperties2, IStorageItemPropertiesWithProvider);
#[cfg(feature = "Storage_Streams")]
impl StorageFile {
    pub fn OpenSequentialReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<Streams::IInputStream>> {
        let this = &windows_core::Interface::cast::<Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenSequentialReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<Streams::IRandomAccessStreamWithContentType>> {
        let this = &windows_core::Interface::cast::<Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn FileType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FileType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn OpenTransactedWriteAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<StorageStreamTransaction>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenTransactedWriteAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CopyOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyOverloadDefaultNameAndOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CopyOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyOverloadDefaultOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CopyAndReplaceAsync<P0>(&self, filetoreplace: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IStorageFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CopyAndReplaceAsync)(windows_core::Interface::as_raw(self), filetoreplace.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MoveOverloadDefaultNameAndOptions<P0>(&self, destinationfolder: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveOverloadDefaultNameAndOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MoveOverloadDefaultOptions<P0>(&self, destinationfolder: P0, desirednewname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IStorageFolder>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveOverloadDefaultOptions)(windows_core::Interface::as_raw(self), destinationfolder.param().abi(), core::mem::transmute_copy(desirednewname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MoveAndReplaceAsync<P0>(&self, filetoreplace: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<IStorageFile>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveAndReplaceAsync)(windows_core::Interface::as_raw(self), filetoreplace.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsAvailable(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAvailable)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetFileFromPathAsync(path: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Self>> {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileFromPathAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn GetFileFromApplicationUriAsync<P0>(uri: P0) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
    where
        P0: windows_core::Param<super::Foundation::Uri>,
    {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFileFromApplicationUriAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateStreamedFileFromUriAsync<P1, P2>(displaynamewithextension: &windows_core::HSTRING, uri: P1, thumbnail: P2) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
    where
        P1: windows_core::Param<super::Foundation::Uri>,
        P2: windows_core::Param<Streams::IRandomAccessStreamReference>,
    {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateStreamedFileFromUriAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(displaynamewithextension), uri.param().abi(), thumbnail.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn ReplaceWithStreamedFileFromUriAsync<P0, P1, P2>(filetoreplace: P0, uri: P1, thumbnail: P2) -> windows_core::Result<windows_future::IAsyncOperation<Self>>
    where
        P0: windows_core::Param<IStorageFile>,
        P1: windows_core::Param<super::Foundation::Uri>,
        P2: windows_core::Param<Streams::IRandomAccessStreamReference>,
    {
        Self::IStorageFileStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceWithStreamedFileFromUriAsync)(windows_core::Interface::as_raw(this), filetoreplace.param().abi(), uri.param().abi(), thumbnail.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<windows_time::DateTime> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Storage_Search")]
    pub fn GetParentAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<StorageFolder>> {
        let this = &windows_core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParentAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsEqual<P0>(&self, item: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderRelativeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderRelativeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IStorageFileStatics<R, F: FnOnce(&IStorageFileStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageFile, IStorageFileStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IStorageFileStatics2<R, F: FnOnce(&IStorageFileStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageFile, IStorageFileStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeType for StorageFile {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageFile>();
}
#[cfg(feature = "Storage_Streams")]
unsafe impl windows_core::Interface for StorageFile {
    type Vtable = <IStorageFile as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageFile as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for StorageFile {
    const NAME: &'static str = "Windows.Storage.StorageFile";
}
#[cfg(feature = "Storage_Search")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageFolder(windows_core::IUnknown);
#[cfg(feature = "Storage_Search")]
windows_core::imp::interface_hierarchy!(StorageFolder, windows_core::IUnknown, windows_core::IInspectable, IStorageFolder);
#[cfg(feature = "Storage_Search")]
windows_core::imp::required_hierarchy!(StorageFolder, IStorageFolder2, Search::IStorageFolderQueryOperations, IStorageItem, IStorageItem2, IStorageItemProperties, IStorageItemProperties2, IStorageItemPropertiesWithProvider);
#[cfg(feature = "Storage_Search")]
impl StorageFolder {
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFile>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFileAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Self>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFolderAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFileAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<StorageFile>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetFolderAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Self>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolderAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetItemAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<IStorageItem>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<StorageFile>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<Self>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IStorageItem>>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemsAsyncOverloadDefaultStartAndCount)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn TryGetItemAsync(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<IStorageItem>> {
        let this = &windows_core::Interface::cast::<IStorageFolder2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryGetItemAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetIndexedStateAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<Search::IndexedState>> {
        let this = &windows_core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIndexedStateAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IVectorView<IStorageItem>>> {
        let this = &windows_core::Interface::cast::<Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetItemsAsync)(windows_core::Interface::as_raw(this), startindex, maxitemstoretrieve, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetFolderFromPathAsync(path: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<Self>> {
        Self::IStorageFolderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetFolderFromPathAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(path), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(desiredname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_FileProperties")]
    pub fn GetBasicPropertiesAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<FileProperties::BasicProperties>> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetBasicPropertiesAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Path)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DateCreated(&self) -> windows_core::Result<windows_time::DateTime> {
        let this = &windows_core::Interface::cast::<IStorageItem>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DateCreated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GetParentAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<Self>> {
        let this = &windows_core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetParentAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsEqual<P0>(&self, item: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IStorageItem>,
    {
        let this = &windows_core::Interface::cast::<IStorageItem2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEqual)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn DisplayName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FolderRelativeId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FolderRelativeId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IStorageFolderStatics<R, F: FnOnce(&IStorageFolderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageFolder, IStorageFolderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IStorageFolderStatics2<R, F: FnOnce(&IStorageFolderStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<StorageFolder, IStorageFolderStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Storage_Search")]
impl windows_core::RuntimeType for StorageFolder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageFolder>();
}
#[cfg(feature = "Storage_Search")]
unsafe impl windows_core::Interface for StorageFolder {
    type Vtable = <IStorageFolder as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageFolder as windows_core::Interface>::IID;
}
#[cfg(feature = "Storage_Search")]
impl windows_core::RuntimeName for StorageFolder {
    const NAME: &'static str = "Windows.Storage.StorageFolder";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StorageStreamTransaction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(StorageStreamTransaction, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(StorageStreamTransaction, super::Foundation::IClosable);
impl StorageStreamTransaction {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> windows_core::Result<Streams::IRandomAccessStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CommitAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommitAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for StorageStreamTransaction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IStorageStreamTransaction>();
}
unsafe impl windows_core::Interface for StorageStreamTransaction {
    type Vtable = <IStorageStreamTransaction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IStorageStreamTransaction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for StorageStreamTransaction {
    const NAME: &'static str = "Windows.Storage.StorageStreamTransaction";
}
