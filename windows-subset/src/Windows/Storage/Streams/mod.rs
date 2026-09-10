windows_core::imp::define_interface!(IContentTypeProvider, IContentTypeProvider_Vtbl, 0x97d098a5_3b99_4de9_88a5_e11d2f50c795);
impl windows_core::RuntimeType for IContentTypeProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IContentTypeProvider");
}
windows_core::imp::interface_hierarchy!(IContentTypeProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IContentTypeProvider {
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IContentTypeProvider {
    const NAME: &'static str = "Windows.Storage.Streams.IContentTypeProvider";
}
pub trait IContentTypeProvider_Impl: windows_core::IUnknownImpl {
    fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IContentTypeProvider_Vtbl {
    pub const fn new<Identity: IContentTypeProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContentType<Identity: IContentTypeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContentTypeProvider_Impl::ContentType(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentTypeProvider, OFFSET>(), ContentType: ContentType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentTypeProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentTypeProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputStream, IInputStream_Vtbl, 0x905a0fe2_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IInputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IInputStream");
}
windows_core::imp::interface_hierarchy!(IInputStream, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IInputStream, super::super::Foundation::IClosable);
impl IInputStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStream";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IOutputStream, IOutputStream_Vtbl, 0x905a0fe6_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IOutputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IOutputStream");
}
windows_core::imp::interface_hierarchy!(IOutputStream, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IOutputStream, super::super::Foundation::IClosable);
impl IOutputStream {
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
}
#[repr(C)]
#[doc(hidden)]
pub struct IOutputStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    WriteAsync: usize,
    pub FlushAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStream, IRandomAccessStream_Vtbl, 0x905a0fe1_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IRandomAccessStream");
}
windows_core::imp::interface_hierarchy!(IRandomAccessStream, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IRandomAccessStream, super::super::Foundation::IClosable, IInputStream, IOutputStream);
impl IRandomAccessStream {
    pub fn Size(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputStreamAt)(windows_core::Interface::as_raw(self), position, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputStreamAt)(windows_core::Interface::as_raw(self), position, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Seek)(windows_core::Interface::as_raw(self), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRead)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanWrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStream";
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetInputStreamAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputStreamAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub CloneStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamReference, IRandomAccessStreamReference_Vtbl, 0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
impl windows_core::RuntimeType for IRandomAccessStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IRandomAccessStreamReference");
}
windows_core::imp::interface_hierarchy!(IRandomAccessStreamReference, windows_core::IUnknown, windows_core::IInspectable);
impl IRandomAccessStreamReference {
    pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenReadAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IRandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
}
pub trait IRandomAccessStreamReference_Impl: windows_core::IUnknownImpl {
    fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
impl IRandomAccessStreamReference_Vtbl {
    pub const fn new<Identity: IRandomAccessStreamReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenReadAsync<Identity: IRandomAccessStreamReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStreamReference_Impl::OpenReadAsync(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamReference, OFFSET>(),
            OpenReadAsync: OpenReadAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamReference as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamReferenceStatics, IRandomAccessStreamReferenceStatics_Vtbl, 0x857309dc_3fbf_4e7d_986f_ef3b1a07a964);
impl windows_core::RuntimeType for IRandomAccessStreamReferenceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IRandomAccessStreamReferenceStatics");
}
impl windows_core::RuntimeName for IRandomAccessStreamReferenceStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReferenceStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateFromFile: usize,
    pub CreateFromUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamWithContentType, IRandomAccessStreamWithContentType_Vtbl, 0xcc254827_4b3d_438f_9232_10c76bc7e038);
impl windows_core::RuntimeType for IRandomAccessStreamWithContentType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IRandomAccessStreamWithContentType");
}
windows_core::imp::interface_hierarchy!(IRandomAccessStreamWithContentType, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IRandomAccessStreamWithContentType, super::super::Foundation::IClosable, IContentTypeProvider, IInputStream, IOutputStream, IRandomAccessStream);
impl IRandomAccessStreamWithContentType {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IRandomAccessStreamWithContentType {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamWithContentType";
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamWithContentType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomAccessStreamReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RandomAccessStreamReference, windows_core::IUnknown, windows_core::IInspectable, IRandomAccessStreamReference);
impl RandomAccessStreamReference {
    pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenReadAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateFromUri<P0>(uri: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUri)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromStream<P0>(stream: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IRandomAccessStream>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStream)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IRandomAccessStreamReferenceStatics<R, F: FnOnce(&IRandomAccessStreamReferenceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RandomAccessStreamReference, IRandomAccessStreamReferenceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RandomAccessStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStreamReference>();
}
unsafe impl windows_core::Interface for RandomAccessStreamReference {
    type Vtable = <IRandomAccessStreamReference as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStreamReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
unsafe impl Send for RandomAccessStreamReference {}
unsafe impl Sync for RandomAccessStreamReference {}
