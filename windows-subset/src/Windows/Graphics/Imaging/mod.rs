windows_core::imp::define_interface!(ISoftwareBitmap, ISoftwareBitmap_Vtbl, 0x689e0708_7eef_483f_963f_da938818e073);
impl windows_core::RuntimeType for ISoftwareBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Graphics.Imaging.ISoftwareBitmap");
}
impl windows_core::RuntimeName for ISoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmap";
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    BitmapPixelFormat: usize,
    BitmapAlphaMode: usize,
    pub PixelWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetDpiX: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    LockBuffer: usize,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    CopyFromBuffer: usize,
    CopyToBuffer: usize,
    pub GetReadOnlyView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISoftwareBitmapFactory, ISoftwareBitmapFactory_Vtbl, 0xc99feb69_2d62_4d47_a6b3_4fdb6a07fdf8);
impl windows_core::RuntimeType for ISoftwareBitmapFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Graphics.Imaging.ISoftwareBitmapFactory");
}
impl windows_core::RuntimeName for ISoftwareBitmapFactory {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmapFactory";
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISoftwareBitmapStatics, ISoftwareBitmapStatics_Vtbl, 0xdf0385db_672f_4a9d_806e_c2442f343e86);
impl windows_core::RuntimeType for ISoftwareBitmapStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Graphics.Imaging.ISoftwareBitmapStatics");
}
impl windows_core::RuntimeName for ISoftwareBitmapStatics {
    const NAME: &'static str = "Windows.Graphics.Imaging.ISoftwareBitmapStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SoftwareBitmap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(SoftwareBitmap, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(SoftwareBitmap, super::super::Foundation::IClosable);
impl SoftwareBitmap {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
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
    pub fn IsReadOnly(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDpiX(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDpiX)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn DpiX(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DpiX)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDpiY(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDpiY)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn DpiY(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DpiY)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CopyTo<P0>(&self, bitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), bitmap.param().abi()).ok() }
    }
    pub fn GetReadOnlyView(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReadOnlyView)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Copy<P0>(source: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Copy)(windows_core::Interface::as_raw(this), source.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn ISoftwareBitmapFactory<R, F: FnOnce(&ISoftwareBitmapFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SoftwareBitmap, ISoftwareBitmapFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISoftwareBitmapStatics<R, F: FnOnce(&ISoftwareBitmapStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SoftwareBitmap, ISoftwareBitmapStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for SoftwareBitmap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISoftwareBitmap>();
}
unsafe impl windows_core::Interface for SoftwareBitmap {
    type Vtable = <ISoftwareBitmap as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISoftwareBitmap as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for SoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.SoftwareBitmap";
}
unsafe impl Send for SoftwareBitmap {}
unsafe impl Sync for SoftwareBitmap {}
