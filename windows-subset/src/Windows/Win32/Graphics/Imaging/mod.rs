pub const CLSID_WICImagingFactory: windows_core::GUID = windows_core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
pub const GUID_WICPixelFormat32bppPBGRA: windows_core::GUID = windows_core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
pub const GUID_WICPixelFormat32bppRGBA: windows_core::GUID = windows_core::GUID::from_u128(0xf5c7ad2d_6a8d_43dd_a7a8_a29935261ae9);
windows_core::imp::define_interface!(IWICBitmap, IWICBitmap_Vtbl, 0x00000121_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICBitmap {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmap, windows_core::IUnknown, IWICBitmapSource);
impl IWICBitmap {
    pub unsafe fn Lock(&self, prclock: *const WICRect, flags: u32) -> windows_core::Result<IWICBitmapLock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), prclock, flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetResolution)(windows_core::Interface::as_raw(self), dpix, dpiy).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmap_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    SetPalette: usize,
    pub SetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IWICBitmap {}
windows_core::imp::define_interface!(IWICBitmapLock, IWICBitmapLock_Vtbl, 0x00000123_a8f2_4877_ba0a_fd2b6645fb94);
windows_core::imp::interface_hierarchy!(IWICBitmapLock, windows_core::IUnknown);
impl IWICBitmapLock {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), puiwidth as _, puiheight as _).ok() }
    }
    pub unsafe fn GetStride(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDataPointer)(windows_core::Interface::as_raw(self), pcbbuffersize as _, ppbdata as _).ok() }
    }
    pub unsafe fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetStride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDataPointer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IWICBitmapLock_Impl: windows_core::IUnknownImpl {
    fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()>;
    fn GetStride(&self) -> windows_core::Result<u32>;
    fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IWICBitmapLock_Vtbl {
    pub const fn new<Identity: IWICBitmapLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSize<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapLock_Impl::GetSize(this, core::mem::transmute_copy(&puiwidth), core::mem::transmute_copy(&puiheight)).into()
            }
        }
        unsafe extern "system" fn GetStride<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbstride: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapLock_Impl::GetStride(this) {
                    Ok(ok__) => {
                        pcbstride.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataPointer<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWICBitmapLock_Impl::GetDataPointer(this, core::mem::transmute_copy(&pcbbuffersize), core::mem::transmute_copy(&ppbdata)).into()
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IWICBitmapLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppixelformat: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWICBitmapLock_Impl::GetPixelFormat(this) {
                    Ok(ok__) => {
                        ppixelformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSize: GetSize::<Identity, OFFSET>,
            GetStride: GetStride::<Identity, OFFSET>,
            GetDataPointer: GetDataPointer::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWICBitmapLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWICBitmapLock {}
windows_core::imp::define_interface!(IWICBitmapSource, IWICBitmapSource_Vtbl, 0x00000120_a8f2_4877_ba0a_fd2b6645fb94);
windows_core::imp::interface_hierarchy!(IWICBitmapSource, windows_core::IUnknown);
impl IWICBitmapSource {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), puiwidth as _, puiheight as _).ok() }
    }
    pub unsafe fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetResolution)(windows_core::Interface::as_raw(self), pdpix as _, pdpiy as _).ok() }
    }
    pub unsafe fn CopyPixels(&self, prc: Option<*const WICRect>, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyPixels)(windows_core::Interface::as_raw(self), prc.unwrap_or(core::mem::zeroed()) as _, cbstride, cbbuffersize, pbbuffer as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64) -> windows_core::HRESULT,
    CopyPalette: usize,
    pub CopyPixels: unsafe extern "system" fn(*mut core::ffi::c_void, *const WICRect, u32, u32, *mut u8) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IWICBitmapSource {}
windows_core::imp::define_interface!(IWICFormatConverter, IWICFormatConverter_Vtbl, 0x00000301_a8f2_4877_ba0a_fd2b6645fb94);
impl core::ops::Deref for IWICFormatConverter {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICFormatConverter, windows_core::IUnknown, IWICBitmapSource);
impl IWICFormatConverter {
    pub unsafe fn CanConvert(&self, srcpixelformat: *const windows_core::GUID, dstpixelformat: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanConvert)(windows_core::Interface::as_raw(self), srcpixelformat, dstpixelformat, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFormatConverter_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    Initialize: usize,
    pub CanConvert: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IWICFormatConverter {}
windows_core::imp::define_interface!(IWICImagingFactory, IWICImagingFactory_Vtbl, 0xec5ec8a9_c395_4314_9c77_54d7a935ff70);
windows_core::imp::interface_hierarchy!(IWICImagingFactory, windows_core::IUnknown);
impl IWICImagingFactory {
    pub unsafe fn CreateFormatConverter(&self) -> windows_core::Result<IWICFormatConverter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFormatConverter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, option, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromSource<P0>(&self, pibitmapsource: P0, option: WICBitmapCreateCacheOption) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromSource)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), option, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromSourceRect<P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> windows_core::Result<IWICBitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromSourceRect)(windows_core::Interface::as_raw(self), pibitmapsource.param().abi(), x, y, width, height, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const windows_core::GUID, cbstride: u32, pbbuffer: &[u8]) -> windows_core::Result<IWICBitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromMemory)(windows_core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, cbstride, pbbuffer.len().try_into().unwrap(), pbbuffer.as_ptr(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateDecoderFromFilename: usize,
    CreateDecoderFromStream: usize,
    CreateDecoderFromFileHandle: usize,
    CreateComponentInfo: usize,
    CreateDecoder: usize,
    CreateEncoder: usize,
    CreatePalette: usize,
    pub CreateFormatConverter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateBitmapScaler: usize,
    CreateBitmapClipper: usize,
    CreateBitmapFlipRotator: usize,
    CreateStream: usize,
    CreateColorContext: usize,
    CreateColorTransformer: usize,
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, WICBitmapCreateCacheOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, WICBitmapCreateCacheOption, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateBitmapFromMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const windows_core::GUID, u32, u32, *const u8, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateBitmapFromHBITMAP: usize,
    CreateBitmapFromHICON: usize,
    CreateComponentEnumerator: usize,
    CreateFastMetadataEncoderFromDecoder: usize,
    CreateFastMetadataEncoderFromFrameDecode: usize,
    CreateQueryWriter: usize,
    CreateQueryWriterFromReader: usize,
}
impl windows_core::RuntimeName for IWICImagingFactory {}
pub const WICBitmapCacheOnDemand: WICBitmapCreateCacheOption = 1;
pub const WICBitmapCacheOnLoad: WICBitmapCreateCacheOption = 2;
pub type WICBitmapCreateCacheOption = i32;
pub type WICBitmapDitherType = i32;
pub const WICBitmapDitherTypeDualSpiral4x4: WICBitmapDitherType = 6;
pub const WICBitmapDitherTypeDualSpiral8x8: WICBitmapDitherType = 7;
pub const WICBitmapDitherTypeErrorDiffusion: WICBitmapDitherType = 8;
pub const WICBitmapDitherTypeNone: WICBitmapDitherType = 0;
pub const WICBitmapDitherTypeOrdered16x16: WICBitmapDitherType = 3;
pub const WICBitmapDitherTypeOrdered4x4: WICBitmapDitherType = 1;
pub const WICBitmapDitherTypeOrdered8x8: WICBitmapDitherType = 2;
pub const WICBitmapDitherTypeSolid: WICBitmapDitherType = 0;
pub const WICBitmapDitherTypeSpiral4x4: WICBitmapDitherType = 4;
pub const WICBitmapDitherTypeSpiral8x8: WICBitmapDitherType = 5;
pub type WICBitmapLockFlags = i32;
pub const WICBitmapLockRead: WICBitmapLockFlags = 1;
pub const WICBitmapLockWrite: WICBitmapLockFlags = 2;
pub const WICBitmapNoCache: WICBitmapCreateCacheOption = 0;
pub type WICBitmapPaletteType = i32;
pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = 0;
pub const WICBitmapPaletteTypeFixedBW: WICBitmapPaletteType = 2;
pub const WICBitmapPaletteTypeFixedGray16: WICBitmapPaletteType = 11;
pub const WICBitmapPaletteTypeFixedGray256: WICBitmapPaletteType = 12;
pub const WICBitmapPaletteTypeFixedGray4: WICBitmapPaletteType = 10;
pub const WICBitmapPaletteTypeFixedHalftone125: WICBitmapPaletteType = 6;
pub const WICBitmapPaletteTypeFixedHalftone216: WICBitmapPaletteType = 7;
pub const WICBitmapPaletteTypeFixedHalftone252: WICBitmapPaletteType = 8;
pub const WICBitmapPaletteTypeFixedHalftone256: WICBitmapPaletteType = 9;
pub const WICBitmapPaletteTypeFixedHalftone27: WICBitmapPaletteType = 4;
pub const WICBitmapPaletteTypeFixedHalftone64: WICBitmapPaletteType = 5;
pub const WICBitmapPaletteTypeFixedHalftone8: WICBitmapPaletteType = 3;
pub const WICBitmapPaletteTypeFixedWebPalette: WICBitmapPaletteType = 7;
pub const WICBitmapPaletteTypeMedianCut: WICBitmapPaletteType = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WICRect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
