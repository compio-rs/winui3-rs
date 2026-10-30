#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataReader, windows_core::IUnknown, windows_core::IInspectable, IDataReader);
windows_core::imp::required_hierarchy!(DataReader, super::super::Foundation::IClosable);
impl DataReader {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UnconsumedBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnconsumedBufferLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadByte(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadByte)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadBytes)(windows_core::Interface::as_raw(self), value.len().try_into().unwrap(), value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBuffer)(windows_core::Interface::as_raw(self), length, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReadBoolean(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBoolean)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt16(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt16)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt32(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt32)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt64(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt64)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt16(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt16)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt32(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt32)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt64(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt64)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadSingle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadSingle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadDouble(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDouble)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadString)(windows_core::Interface::as_raw(self), codeunitcount, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadDateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadTimeSpan(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadTimeSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadAsync)(windows_core::Interface::as_raw(self), count, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateDataReader<P0>(inputstream: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IInputStream>,
    {
        Self::IDataReaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDataReader)(windows_core::Interface::as_raw(this), inputstream.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn FromBuffer<P0>(buffer: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IBuffer>,
    {
        Self::IDataReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataReader>();
}
unsafe impl windows_core::Interface for DataReader {
    type Vtable = <IDataReader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataReader {
    const NAME: &'static str = "Windows.Storage.Streams.DataReader";
}
unsafe impl Send for DataReader {}
unsafe impl Sync for DataReader {}
pub type DataReaderLoadOperation = windows_future::IAsyncOperation<u32>;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataWriter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataWriter, windows_core::IUnknown, windows_core::IInspectable, IDataWriter);
windows_core::imp::required_hierarchy!(DataWriter, super::super::Foundation::IClosable);
impl DataWriter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataWriter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UnstoredBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnstoredBufferLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn WriteByte(&self, value: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteByte)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteBytes)(windows_core::Interface::as_raw(self), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteBuffer)(windows_core::Interface::as_raw(self), buffer.param().abi()).ok() }
    }
    pub fn WriteBufferRange<P0>(&self, buffer: P0, start: u32, count: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteBufferRange)(windows_core::Interface::as_raw(self), buffer.param().abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteBoolean)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteGuid(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteGuid)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteInt16)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteInt32)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteInt64)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteUInt16)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteUInt32)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteUInt64)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteSingle)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteDouble)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteDateTime(&self, value: windows_time::DateTime) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteDateTime)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteTimeSpan(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteTimeSpan)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn MeasureString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MeasureString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateDataWriter<P0>(outputstream: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IOutputStream>,
    {
        Self::IDataWriterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDataWriter)(windows_core::Interface::as_raw(this), outputstream.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IDataWriterFactory<R, F: FnOnce(&IDataWriterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataWriter, IDataWriterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataWriter>();
}
unsafe impl windows_core::Interface for DataWriter {
    type Vtable = <IDataWriter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataWriter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriter";
}
unsafe impl Send for DataWriter {}
unsafe impl Sync for DataWriter {}
windows_core::imp::define_interface!(IBuffer, IBuffer_Vtbl, 0x905a0fe0_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IBuffer");
}
windows_core::imp::interface_hierarchy!(IBuffer, windows_core::IUnknown, windows_core::IInspectable);
impl IBuffer {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLength)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeName for IBuffer {
    const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
}
pub trait IBuffer_Impl: windows_core::IUnknownImpl {
    fn Capacity(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn SetLength(&self, value: u32) -> windows_core::Result<()>;
}
impl IBuffer_Vtbl {
    pub const fn new<Identity: IBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capacity<Identity: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBuffer_Impl::Capacity(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Length<Identity: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBuffer_Impl::Length(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLength<Identity: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBuffer_Impl::SetLength(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBuffer, OFFSET>(),
            Capacity: Capacity::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBuffer as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
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
windows_core::imp::define_interface!(IDataReader, IDataReader_Vtbl, 0xe2b50029_b4c1_4314_a4b8_fb813a2f275e);
impl windows_core::RuntimeType for IDataReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataReader");
}
windows_core::imp::interface_hierarchy!(IDataReader, windows_core::IUnknown, windows_core::IInspectable);
impl IDataReader {
    pub fn UnconsumedBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnconsumedBufferLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadByte(&self) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadByte)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReadBytes)(windows_core::Interface::as_raw(self), value.len().try_into().unwrap(), value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBuffer)(windows_core::Interface::as_raw(self), length, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ReadBoolean(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadBoolean)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadGuid(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadGuid)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt16(&self) -> windows_core::Result<i16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt16)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt32(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt32)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt64(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadInt64)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt16(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt16)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt32(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt32)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt64(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadUInt64)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadSingle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadSingle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadDouble(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDouble)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadString)(windows_core::Interface::as_raw(self), codeunitcount, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadDateTime(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadDateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ReadTimeSpan(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReadTimeSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadAsync)(windows_core::Interface::as_raw(self), count, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IInputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IDataReader {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReader";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnconsumedBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    UnicodeEncoding: usize,
    SetUnicodeEncoding: usize,
    ByteOrder: usize,
    SetByteOrder: usize,
    InputStreamOptions: usize,
    SetInputStreamOptions: usize,
    pub ReadByte: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ReadBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ReadGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ReadInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ReadInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReadInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub ReadUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ReadUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub ReadSingle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub ReadDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ReadString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
    pub ReadTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataReaderFactory, IDataReaderFactory_Vtbl, 0xd7527847_57da_4e15_914c_06806699a098);
impl windows_core::RuntimeType for IDataReaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataReaderFactory");
}
impl windows_core::RuntimeName for IDataReaderFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReaderFactory";
}
pub trait IDataReaderFactory_Impl: windows_core::IUnknownImpl {
    fn CreateDataReader(&self, inputStream: windows_core::Ref<IInputStream>) -> windows_core::Result<DataReader>;
}
impl IDataReaderFactory_Vtbl {
    pub const fn new<Identity: IDataReaderFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDataReader<Identity: IDataReaderFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputstream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReaderFactory_Impl::CreateDataReader(this, core::mem::transmute_copy(&inputstream)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataReaderFactory, OFFSET>(), CreateDataReader: CreateDataReader::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataReaderFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataReaderStatics, IDataReaderStatics_Vtbl, 0x11fcbfc8_f93a_471b_b121_f379e349313c);
impl windows_core::RuntimeType for IDataReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataReaderStatics");
}
impl windows_core::RuntimeName for IDataReaderStatics {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReaderStatics";
}
pub trait IDataReaderStatics_Impl: windows_core::IUnknownImpl {
    fn FromBuffer(&self, buffer: windows_core::Ref<IBuffer>) -> windows_core::Result<DataReader>;
}
impl IDataReaderStatics_Vtbl {
    pub const fn new<Identity: IDataReaderStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FromBuffer<Identity: IDataReaderStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReaderStatics_Impl::FromBuffer(this, core::mem::transmute_copy(&buffer)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataReaderStatics, OFFSET>(), FromBuffer: FromBuffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataReaderStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataWriter, IDataWriter_Vtbl, 0x64b89265_d341_4922_b38a_dd4af8808c4e);
impl windows_core::RuntimeType for IDataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataWriter");
}
windows_core::imp::interface_hierarchy!(IDataWriter, windows_core::IUnknown, windows_core::IInspectable);
impl IDataWriter {
    pub fn UnstoredBufferLength(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UnstoredBufferLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn WriteByte(&self, value: u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteByte)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteBytes)(windows_core::Interface::as_raw(self), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteBuffer)(windows_core::Interface::as_raw(self), buffer.param().abi()).ok() }
    }
    pub fn WriteBufferRange<P0>(&self, buffer: P0, start: u32, count: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteBufferRange)(windows_core::Interface::as_raw(self), buffer.param().abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteBoolean)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteGuid(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteGuid)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteInt16)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteInt32)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteInt64)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteUInt16)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteUInt32)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteUInt64)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteSingle)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteDouble)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteDateTime(&self, value: windows_time::DateTime) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteDateTime)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteTimeSpan(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).WriteTimeSpan)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn WriteString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn MeasureString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MeasureString)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FlushAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IOutputStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetachStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IDataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriter";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnstoredBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    UnicodeEncoding: usize,
    SetUnicodeEncoding: usize,
    ByteOrder: usize,
    SetByteOrder: usize,
    pub WriteByte: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub WriteBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteBufferRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub WriteBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WriteGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub WriteInt16: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub WriteInt32: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WriteInt64: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WriteUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WriteUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub WriteSingle: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub WriteDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub WriteDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::DateTime) -> windows_core::HRESULT,
    pub WriteTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::TimeSpan) -> windows_core::HRESULT,
    pub WriteString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MeasureString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    StoreAsync: usize,
    pub FlushAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataWriterFactory, IDataWriterFactory_Vtbl, 0x338c67c2_8b84_4c2b_9c50_7b8767847a1f);
impl windows_core::RuntimeType for IDataWriterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IDataWriterFactory");
}
impl windows_core::RuntimeName for IDataWriterFactory {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriterFactory";
}
pub trait IDataWriterFactory_Impl: windows_core::IUnknownImpl {
    fn CreateDataWriter(&self, outputStream: windows_core::Ref<IOutputStream>) -> windows_core::Result<DataWriter>;
}
impl IDataWriterFactory_Vtbl {
    pub const fn new<Identity: IDataWriterFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateDataWriter<Identity: IDataWriterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputstream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataWriterFactory_Impl::CreateDataWriter(this, core::mem::transmute_copy(&outputstream)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataWriterFactory, OFFSET>(), CreateDataWriter: CreateDataWriter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataWriterFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDataWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
windows_core::imp::define_interface!(IInputStreamReference, IInputStreamReference_Vtbl, 0x43929d18_5ec9_4b5a_919c_4205b0c804b6);
impl windows_core::RuntimeType for IInputStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IInputStreamReference");
}
windows_core::imp::interface_hierarchy!(IInputStreamReference, windows_core::IUnknown, windows_core::IInspectable);
impl IInputStreamReference {
    pub fn OpenSequentialReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IInputStream>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenSequentialReadAsync)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IInputStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStreamReference";
}
pub trait IInputStreamReference_Impl: windows_core::IUnknownImpl {
    fn OpenSequentialReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IInputStream>>;
}
impl IInputStreamReference_Vtbl {
    pub const fn new<Identity: IInputStreamReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenSequentialReadAsync<Identity: IInputStreamReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputStreamReference_Impl::OpenSequentialReadAsync(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputStreamReference, OFFSET>(),
            OpenSequentialReadAsync: OpenSequentialReadAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputStreamReference as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStreamReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenSequentialReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOutputStream, IOutputStream_Vtbl, 0x905a0fe6_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IOutputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.Streams.IOutputStream");
}
windows_core::imp::interface_hierarchy!(IOutputStream, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IOutputStream, super::super::Foundation::IClosable);
impl IOutputStream {
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteAsync)(windows_core::Interface::as_raw(self), buffer.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
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
pub trait IOutputStream_Impl: super::super::Foundation::IClosable_Impl {
    fn WriteAsync(&self, buffer: windows_core::Ref<IBuffer>) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>>;
}
impl IOutputStream_Vtbl {
    pub const fn new<Identity: IOutputStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteAsync<Identity: IOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOutputStream_Impl::WriteAsync(this, core::mem::transmute_copy(&buffer)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: IOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOutputStream_Impl::FlushAsync(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IOutputStream, OFFSET>(),
            WriteAsync: WriteAsync::<Identity, OFFSET>,
            FlushAsync: FlushAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOutputStream as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOutputStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WriteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
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
pub trait IRandomAccessStreamReferenceStatics_Impl: windows_core::IUnknownImpl {
    fn CreateFromFile(&self, file: windows_core::Ref<super::IStorageFile>) -> windows_core::Result<RandomAccessStreamReference>;
    fn CreateFromUri(&self, uri: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<RandomAccessStreamReference>;
    fn CreateFromStream(&self, stream: windows_core::Ref<IRandomAccessStream>) -> windows_core::Result<RandomAccessStreamReference>;
}
impl IRandomAccessStreamReferenceStatics_Vtbl {
    pub const fn new<Identity: IRandomAccessStreamReferenceStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromFile<Identity: IRandomAccessStreamReferenceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, file: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStreamReferenceStatics_Impl::CreateFromFile(this, core::mem::transmute_copy(&file)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromUri<Identity: IRandomAccessStreamReferenceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStreamReferenceStatics_Impl::CreateFromUri(this, core::mem::transmute_copy(&uri)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromStream<Identity: IRandomAccessStreamReferenceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStreamReferenceStatics_Impl::CreateFromStream(this, core::mem::transmute_copy(&stream)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamReferenceStatics, OFFSET>(),
            CreateFromFile: CreateFromFile::<Identity, OFFSET>,
            CreateFromUri: CreateFromUri::<Identity, OFFSET>,
            CreateFromStream: CreateFromStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamReferenceStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
pub struct InMemoryRandomAccessStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InMemoryRandomAccessStream, windows_core::IUnknown, windows_core::IInspectable, IRandomAccessStream);
windows_core::imp::required_hierarchy!(InMemoryRandomAccessStream, super::super::Foundation::IClosable, IInputStream, IOutputStream);
impl InMemoryRandomAccessStream {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InMemoryRandomAccessStream, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
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
}
impl windows_core::RuntimeType for InMemoryRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStream>();
}
unsafe impl windows_core::Interface for InMemoryRandomAccessStream {
    type Vtable = <IRandomAccessStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InMemoryRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.InMemoryRandomAccessStream";
}
unsafe impl Send for InMemoryRandomAccessStream {}
unsafe impl Sync for InMemoryRandomAccessStream {}
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
    pub fn CreateFromFile<P0>(file: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromFile)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
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
