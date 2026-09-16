#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitmapSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BitmapSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BitmapSource, super::ImageSource, super::super::DependencyObject);
impl BitmapSource {
    pub fn PixelWidth(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PixelWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PixelHeight(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PixelHeight)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetSource<P0>(&self, streamsource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Storage::Streams::IRandomAccessStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), streamsource.param().abi()).ok() }
    }
    pub fn SetSourceAsync<P0>(&self, streamsource: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<windows::Storage::Streams::IRandomAccessStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetSourceAsync)(windows_core::Interface::as_raw(self), streamsource.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IBitmapSourceFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IBitmapSourceFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IBitmapSourceFactory<R, F: FnOnce(&IBitmapSourceFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapSource, IBitmapSourceFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBitmapSourceStatics<R, F: FnOnce(&IBitmapSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<BitmapSource, IBitmapSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for BitmapSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBitmapSource>();
}
unsafe impl windows_core::Interface for BitmapSource {
    type Vtable = <IBitmapSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBitmapSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.BitmapSource";
}
unsafe impl Send for BitmapSource {}
unsafe impl Sync for BitmapSource {}
windows_core::imp::define_interface!(IBitmapSource, IBitmapSource_Vtbl, 0x8424269d_9b82_534f_8fea_af5b5ef96bf2);
impl windows_core::RuntimeType for IBitmapSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.Imaging.IBitmapSource");
}
impl windows_core::RuntimeName for IBitmapSource {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.IBitmapSource";
}
pub trait IBitmapSource_Impl: windows_core::IUnknownImpl {
    fn PixelWidth(&self) -> windows_core::Result<i32>;
    fn PixelHeight(&self) -> windows_core::Result<i32>;
    fn SetSource(&self, streamSource: windows_core::Ref<windows::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<()>;
    fn SetSourceAsync(&self, streamSource: windows_core::Ref<windows::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<windows_future::IAsyncAction>;
}
impl IBitmapSource_Vtbl {
    pub const fn new<Identity: IBitmapSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PixelWidth<Identity: IBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapSource_Impl::PixelWidth(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PixelHeight<Identity: IBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapSource_Impl::PixelHeight(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSource<Identity: IBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamsource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitmapSource_Impl::SetSource(this, core::mem::transmute_copy(&streamsource)).into()
            }
        }
        unsafe extern "system" fn SetSourceAsync<Identity: IBitmapSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamsource: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapSource_Impl::SetSourceAsync(this, core::mem::transmute_copy(&streamsource)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapSource, OFFSET>(),
            PixelWidth: PixelWidth::<Identity, OFFSET>,
            PixelHeight: PixelHeight::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
            SetSourceAsync: SetSourceAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSourceAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapSourceFactory, IBitmapSourceFactory_Vtbl, 0x0392f025_1868_5876_ad67_12e94a8da5bf);
impl windows_core::RuntimeType for IBitmapSourceFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.Imaging.IBitmapSourceFactory");
}
impl windows_core::RuntimeName for IBitmapSourceFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.IBitmapSourceFactory";
}
pub trait IBitmapSourceFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<BitmapSource>;
}
impl IBitmapSourceFactory_Vtbl {
    pub const fn new<Identity: IBitmapSourceFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IBitmapSourceFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapSourceFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IBitmapSourceFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapSourceFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBitmapSourceStatics, IBitmapSourceStatics_Vtbl, 0xefa3745e_4400_5f0b_bdc7_3f2911a3d719);
impl windows_core::RuntimeType for IBitmapSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.Imaging.IBitmapSourceStatics");
}
impl windows_core::RuntimeName for IBitmapSourceStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.IBitmapSourceStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IWriteableBitmap, IWriteableBitmap_Vtbl, 0x78c824a9_0e43_5f1e_93bc_d046cca82b7e);
impl windows_core::RuntimeType for IWriteableBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.Imaging.IWriteableBitmap");
}
impl windows_core::RuntimeName for IWriteableBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.IWriteableBitmap";
}
pub trait IWriteableBitmap_Impl: windows_core::IUnknownImpl {
    fn PixelBuffer(&self) -> windows_core::Result<windows::Storage::Streams::IBuffer>;
    fn Invalidate(&self) -> windows_core::Result<()>;
}
impl IWriteableBitmap_Vtbl {
    pub const fn new<Identity: IWriteableBitmap_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PixelBuffer<Identity: IWriteableBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteableBitmap_Impl::PixelBuffer(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Invalidate<Identity: IWriteableBitmap_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteableBitmap_Impl::Invalidate(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWriteableBitmap, OFFSET>(),
            PixelBuffer: PixelBuffer::<Identity, OFFSET>,
            Invalidate: Invalidate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteableBitmap as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PixelBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Invalidate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWriteableBitmapFactory, IWriteableBitmapFactory_Vtbl, 0x26e861d9_b080_512b_96c4_80050e7e08d1);
impl windows_core::RuntimeType for IWriteableBitmapFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Media.Imaging.IWriteableBitmapFactory");
}
impl windows_core::RuntimeName for IWriteableBitmapFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.IWriteableBitmapFactory";
}
pub trait IWriteableBitmapFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstanceWithDimensions(&self, pixelWidth: i32, pixelHeight: i32) -> windows_core::Result<WriteableBitmap>;
}
impl IWriteableBitmapFactory_Vtbl {
    pub const fn new<Identity: IWriteableBitmapFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstanceWithDimensions<Identity: IWriteableBitmapFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pixelwidth: i32, pixelheight: i32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteableBitmapFactory_Impl::CreateInstanceWithDimensions(this, pixelwidth, pixelheight) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWriteableBitmapFactory, OFFSET>(),
            CreateInstanceWithDimensions: CreateInstanceWithDimensions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteableBitmapFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWriteableBitmapFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstanceWithDimensions: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriteableBitmap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WriteableBitmap, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(WriteableBitmap, BitmapSource, super::ImageSource, super::super::DependencyObject);
impl WriteableBitmap {
    pub fn PixelWidth(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PixelHeight(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PixelHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSource<P0>(&self, streamsource: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<IBitmapSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), streamsource.param().abi()).ok() }
    }
    pub fn SetSourceAsync<P0>(&self, streamsource: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<windows::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<IBitmapSource>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetSourceAsync)(windows_core::Interface::as_raw(this), streamsource.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn PixelBuffer(&self) -> windows_core::Result<windows::Storage::Streams::IBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PixelBuffer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Invalidate(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Invalidate)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn CreateInstanceWithDimensions(pixelwidth: i32, pixelheight: i32) -> windows_core::Result<Self> {
        Self::IWriteableBitmapFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstanceWithDimensions)(windows_core::Interface::as_raw(this), pixelwidth, pixelheight, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IWriteableBitmapFactory<R, F: FnOnce(&IWriteableBitmapFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<WriteableBitmap, IWriteableBitmapFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for WriteableBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWriteableBitmap>();
}
unsafe impl windows_core::Interface for WriteableBitmap {
    type Vtable = <IWriteableBitmap as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWriteableBitmap as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WriteableBitmap {
    const NAME: &'static str = "Microsoft.UI.Xaml.Media.Imaging.WriteableBitmap";
}
unsafe impl Send for WriteableBitmap {}
unsafe impl Sync for WriteableBitmap {}
