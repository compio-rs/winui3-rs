#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub mod Common;
#[inline]
pub unsafe fn D2D1CreateFactory<T>(factorytype: D2D1_FACTORY_TYPE, pfactoryoptions: Option<*const D2D1_FACTORY_OPTIONS>) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("d2d1.dll" "system" fn D2D1CreateFactory(factorytype : D2D1_FACTORY_TYPE, riid : *const windows_core::GUID, pfactoryoptions : *const D2D1_FACTORY_OPTIONS, ppifactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { D2D1CreateFactory(factorytype, &T::IID, pfactoryoptions.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ARC_SEGMENT {
    pub point: Common::D2D_POINT_2F,
    pub size: Common::D2D_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: D2D1_SWEEP_DIRECTION,
    pub arcSize: D2D1_ARC_SIZE,
}
pub type D2D1_ARC_SIZE = i32;
pub const D2D1_ARC_SIZE_LARGE: D2D1_ARC_SIZE = 1;
pub const D2D1_ARC_SIZE_SMALL: D2D1_ARC_SIZE = 0;
pub type D2D1_BITMAP_INTERPOLATION_MODE = i32;
pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE = 1;
pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: D2D1_BITMAP_INTERPOLATION_MODE = 0;
pub type D2D1_BITMAP_OPTIONS = i32;
pub const D2D1_BITMAP_OPTIONS_CANNOT_DRAW: D2D1_BITMAP_OPTIONS = 2;
pub const D2D1_BITMAP_OPTIONS_CPU_READ: D2D1_BITMAP_OPTIONS = 4;
pub const D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE: D2D1_BITMAP_OPTIONS = 8;
pub const D2D1_BITMAP_OPTIONS_NONE: D2D1_BITMAP_OPTIONS = 0;
pub const D2D1_BITMAP_OPTIONS_TARGET: D2D1_BITMAP_OPTIONS = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: Common::D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: core::mem::ManuallyDrop<Option<ID2D1ColorContext>>,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: Common::D2D_MATRIX_3X2_F,
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl Default for D2D1_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D2D1_DEBUG_LEVEL = i32;
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25;
pub type D2D1_DEVICE_CONTEXT_OPTIONS = i32;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS: D2D1_DEVICE_CONTEXT_OPTIONS = 1;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS = 0;
pub type D2D1_DRAW_TEXT_OPTIONS = i32;
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = 2;
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = 8;
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = 4;
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0;
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = 1;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ELLIPSE {
    pub point: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
pub type D2D1_EXTEND_MODE = i32;
pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = 0;
pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = 2;
pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_FACTORY_OPTIONS {
    pub debugLevel: D2D1_DEBUG_LEVEL,
}
pub type D2D1_FACTORY_TYPE = i32;
pub const D2D1_FACTORY_TYPE_MULTI_THREADED: D2D1_FACTORY_TYPE = 1;
pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE = 0;
pub type D2D1_FEATURE_LEVEL = i32;
pub const D2D1_FEATURE_LEVEL_10: D2D1_FEATURE_LEVEL = 40960;
pub const D2D1_FEATURE_LEVEL_9: D2D1_FEATURE_LEVEL = 37120;
pub const D2D1_FEATURE_LEVEL_DEFAULT: D2D1_FEATURE_LEVEL = 0;
pub type D2D1_GAMMA = i32;
pub const D2D1_GAMMA_1_0: D2D1_GAMMA = 1;
pub const D2D1_GAMMA_2_2: D2D1_GAMMA = 0;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: Common::D2D_POINT_2F,
    pub endPoint: Common::D2D_POINT_2F,
}
pub type D2D1_PRESENT_OPTIONS = i32;
pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS = 2;
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = 0;
pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS = 1;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: Common::D2D_POINT_2F,
    pub gradientOriginOffset: Common::D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
pub type D2D1_RENDER_TARGET_TYPE = i32;
pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE = 0;
pub const D2D1_RENDER_TARGET_TYPE_HARDWARE: D2D1_RENDER_TARGET_TYPE = 2;
pub const D2D1_RENDER_TARGET_TYPE_SOFTWARE: D2D1_RENDER_TARGET_TYPE = 1;
pub type D2D1_RENDER_TARGET_USAGE = i32;
pub const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING: D2D1_RENDER_TARGET_USAGE = 1;
pub const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE: D2D1_RENDER_TARGET_USAGE = 2;
pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE = 0;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ROUNDED_RECT {
    pub rect: Common::D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
pub type D2D1_SWEEP_DIRECTION = i32;
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: D2D1_SWEEP_DIRECTION = 1;
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: D2D1_SWEEP_DIRECTION = 0;
windows_core::imp::define_interface!(ID2D1Bitmap, ID2D1Bitmap_Vtbl, 0xa2296057_ea42_4099_983b_539fb6505426);
impl core::ops::Deref for ID2D1Bitmap {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Bitmap, windows_core::IUnknown, ID2D1Resource, ID2D1Image);
impl ID2D1Bitmap {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix as _, dpiy as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Bitmap_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    CopyFromBitmap: usize,
    CopyFromRenderTarget: usize,
    CopyFromMemory: usize,
}
unsafe impl Send for ID2D1Bitmap {}
unsafe impl Sync for ID2D1Bitmap {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1Bitmap {}
windows_core::imp::define_interface!(ID2D1Bitmap1, ID2D1Bitmap1_Vtbl, 0xa898a84c_3873_4588_b08b_ebbf978df041);
impl core::ops::Deref for ID2D1Bitmap1 {
    type Target = ID2D1Bitmap;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Bitmap1, windows_core::IUnknown, ID2D1Resource, ID2D1Image, ID2D1Bitmap);
impl ID2D1Bitmap1 {
    pub unsafe fn GetColorContext(&self) -> windows_core::Result<ID2D1ColorContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorContext)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetOptions(&self) -> D2D1_BITMAP_OPTIONS {
        unsafe { (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetSurface(&self) -> windows_core::Result<super::Dxgi::IDXGISurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurface)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Bitmap1_Vtbl {
    pub base__: ID2D1Bitmap_Vtbl,
    pub GetColorContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BITMAP_OPTIONS,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub GetSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    GetSurface: usize,
    Map: usize,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1Bitmap1 {}
unsafe impl Sync for ID2D1Bitmap1 {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1Bitmap1 {}
windows_core::imp::define_interface!(ID2D1Brush, ID2D1Brush_Vtbl, 0x2cd906a8_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Brush {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Brush, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Brush {
    pub unsafe fn SetOpacity(&self, opacity: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetOpacity)(windows_core::Interface::as_raw(self), opacity);
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetTransform(&self, transform: *const Common::D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform);
        }
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetTransform(&self, transform: *mut Common::D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Brush_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_MATRIX_3X2_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform: usize,
    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_MATRIX_3X2_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetTransform: usize,
}
unsafe impl Send for ID2D1Brush {}
unsafe impl Sync for ID2D1Brush {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
pub trait ID2D1Brush_Impl: ID2D1Resource_Impl {
    fn SetOpacity(&self, opacity: f32);
    fn SetTransform(&self, transform: *const Common::D2D_MATRIX_3X2_F);
    fn GetOpacity(&self) -> f32;
    fn GetTransform(&self, transform: *mut Common::D2D_MATRIX_3X2_F);
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1Brush_Vtbl {
    pub const fn new<Identity: ID2D1Brush_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOpacity<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, opacity: f32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::SetOpacity(this, core::mem::transmute_copy(&opacity));
            }
        }
        unsafe extern "system" fn SetTransform<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *const Common::D2D_MATRIX_3X2_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::SetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        unsafe extern "system" fn GetOpacity<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::GetOpacity(this)
            }
        }
        unsafe extern "system" fn GetTransform<Identity: ID2D1Brush_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut Common::D2D_MATRIX_3X2_F) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Brush_Impl::GetTransform(this, core::mem::transmute_copy(&transform));
            }
        }
        Self {
            base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>(),
            SetOpacity: SetOpacity::<Identity, OFFSET>,
            SetTransform: SetTransform::<Identity, OFFSET>,
            GetOpacity: GetOpacity::<Identity, OFFSET>,
            GetTransform: GetTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Brush as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1Brush {}
windows_core::imp::define_interface!(ID2D1ColorContext, ID2D1ColorContext_Vtbl, 0x1c4820bb_5771_4518_a581_2fe4dd0ec657);
impl core::ops::Deref for ID2D1ColorContext {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ColorContext, windows_core::IUnknown, ID2D1Resource);
impl ID2D1ColorContext {
    pub unsafe fn GetProfileSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetProfileSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProfile(&self, profile: *mut u8, profilesize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), profile as _, profilesize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1ColorContext_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetColorSpace: usize,
    pub GetProfileSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1ColorContext {}
unsafe impl Sync for ID2D1ColorContext {}
impl windows_core::RuntimeName for ID2D1ColorContext {}
windows_core::imp::define_interface!(ID2D1Device, ID2D1Device_Vtbl, 0x47dd575d_ac05_4cdd_8049_9b02cd16f44c);
impl core::ops::Deref for ID2D1Device {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Device {
    pub unsafe fn CreateDeviceContext(&self, options: D2D1_DEVICE_CONTEXT_OPTIONS) -> windows_core::Result<ID2D1DeviceContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximumTextureMemory)(windows_core::Interface::as_raw(self), maximuminbytes);
        }
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumTextureMemory)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearResources)(windows_core::Interface::as_raw(self), millisecondssinceuse);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Device_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_DEVICE_CONTEXT_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreatePrintControl: usize,
    pub SetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
    pub GetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub ClearResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
unsafe impl Send for ID2D1Device {}
unsafe impl Sync for ID2D1Device {}
impl windows_core::RuntimeName for ID2D1Device {}
windows_core::imp::define_interface!(ID2D1DeviceContext, ID2D1DeviceContext_Vtbl, 0xe8f7fe7a_191c_466d_ad95_975678bda998);
impl core::ops::Deref for ID2D1DeviceContext {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1DeviceContext, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1DeviceContext {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, sourcedata: Option<*const core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES1) -> windows_core::Result<ID2D1Bitmap1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), size, sourcedata.unwrap_or(core::mem::zeroed()) as _, pitch, bitmapproperties, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(&self, filename: P0) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContextFromFilename)(windows_core::Interface::as_raw(self), filename.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(&self, surface: P0, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES1>) -> windows_core::Result<ID2D1Bitmap1>
    where
        P0: windows_core::Param<super::Dxgi::IDXGISurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromDxgiSurface)(windows_core::Interface::as_raw(self), surface.param().abi(), bitmapproperties.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IsDxgiFormatSupported(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsDxgiFormatSupported)(windows_core::Interface::as_raw(self), format) }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> windows_core::Result<Common::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageLocalBounds)(windows_core::Interface::as_raw(self), image.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> windows_core::Result<Common::D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageWorldBounds)(windows_core::Interface::as_raw(self), image.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID2D1Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTarget)(windows_core::Interface::as_raw(self), image.param().abi());
        }
    }
    pub unsafe fn GetTarget(&self) -> windows_core::Result<ID2D1Image> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTarget)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::imp::Type::from_abi(result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillOpacityMask<P0, P1>(&self, opacitymask: P0, brush: P1, destinationrectangle: Option<*const Common::D2D_RECT_F>, sourcerectangle: Option<*const Common::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillOpacityMask)(windows_core::Interface::as_raw(self), opacitymask.param().abi(), brush.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, sourcerectangle.unwrap_or(core::mem::zeroed()) as _);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1DeviceContext_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_SIZE_U, *const core::ffi::c_void, u32, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap: usize,
    CreateBitmapFromWicBitmap: usize,
    CreateColorContext: usize,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateColorContextFromWicColorContext: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmapFromDxgiSurface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES1, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmapFromDxgiSurface: usize,
    CreateEffect: usize,
    CreateGradientStopCollection: usize,
    CreateImageBrush: usize,
    CreateBitmapBrush: usize,
    CreateCommandList: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub IsDxgiFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT) -> windows_core::BOOL,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    IsDxgiFormatSupported: usize,
    IsBufferPrecisionSupported: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageLocalBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageLocalBounds: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetImageWorldBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetImageWorldBounds: usize,
    GetGlyphRunWorldBounds: usize,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    SetRenderingControls: usize,
    GetRenderingControls: usize,
    SetPrimitiveBlend: usize,
    GetPrimitiveBlend: usize,
    SetUnitMode: usize,
    GetUnitMode: usize,
    DrawGlyphRun: usize,
    DrawImage: usize,
    DrawGdiMetafile: usize,
    DrawBitmap: usize,
    PushLayer: usize,
    InvalidateEffectInputRectangle: usize,
    GetEffectInvalidRectangleCount: usize,
    GetEffectInvalidRectangles: usize,
    GetEffectRequiredInputRectangles: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillOpacityMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillOpacityMask: usize,
}
unsafe impl Send for ID2D1DeviceContext {}
unsafe impl Sync for ID2D1DeviceContext {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1DeviceContext {}
windows_core::imp::define_interface!(ID2D1Factory, ID2D1Factory_Vtbl, 0x06152247_6f50_465a_9245_118bfd3b6007);
windows_core::imp::interface_hierarchy!(ID2D1Factory, windows_core::IUnknown);
impl ID2D1Factory {
    pub unsafe fn ReloadSystemMetrics(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReloadSystemMetrics)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesktopDpi)(windows_core::Interface::as_raw(self), dpix as _, dpiy as _);
        }
    }
    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePathGeometry)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReloadSystemMetrics: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesktopDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    CreateRectangleGeometry: usize,
    CreateRoundedRectangleGeometry: usize,
    CreateEllipseGeometry: usize,
    CreateGeometryGroup: usize,
    CreateTransformedGeometry: usize,
    pub CreatePathGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateStrokeStyle: usize,
    CreateDrawingStateBlock: usize,
    CreateWicBitmapRenderTarget: usize,
    CreateHwndRenderTarget: usize,
    CreateDxgiSurfaceRenderTarget: usize,
    CreateDCRenderTarget: usize,
}
unsafe impl Send for ID2D1Factory {}
unsafe impl Sync for ID2D1Factory {}
impl windows_core::RuntimeName for ID2D1Factory {}
windows_core::imp::define_interface!(ID2D1Factory1, ID2D1Factory1_Vtbl, 0xbb12d362_daee_4b9a_aa1d_14ba401cfa1f);
impl core::ops::Deref for ID2D1Factory1 {
    type Target = ID2D1Factory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory1, windows_core::IUnknown, ID2D1Factory);
impl ID2D1Factory1 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device>
    where
        P0: windows_core::Param<super::Dxgi::IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(windows_core::Interface::as_raw(self), dxgidevice.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn UnregisterEffect(&self, classid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnregisterEffect)(windows_core::Interface::as_raw(self), classid).ok() }
    }
    pub unsafe fn GetRegisteredEffects(&self, effects: Option<*mut windows_core::GUID>, effectscount: u32, effectsreturned: Option<*mut u32>, effectsregistered: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetRegisteredEffects)(windows_core::Interface::as_raw(self), effects.unwrap_or(core::mem::zeroed()) as _, effectscount, effectsreturned.unwrap_or(core::mem::zeroed()) as _, effectsregistered.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory1_Vtbl {
    pub base__: ID2D1Factory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub CreateDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))]
    CreateDevice: usize,
    CreateStrokeStyle: usize,
    CreatePathGeometry: usize,
    CreateDrawingStateBlock: usize,
    CreateGdiMetafile: usize,
    RegisterEffectFromStream: usize,
    RegisterEffectFromString: usize,
    pub UnregisterEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetRegisteredEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    GetEffectProperties: usize,
}
unsafe impl Send for ID2D1Factory1 {}
unsafe impl Sync for ID2D1Factory1 {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl windows_core::RuntimeName for ID2D1Factory1 {}
windows_core::imp::define_interface!(ID2D1Factory2, ID2D1Factory2_Vtbl, 0x94f81a73_9212_4376_9c58_b16a3a0d3992);
impl core::ops::Deref for ID2D1Factory2 {
    type Target = ID2D1Factory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory2, windows_core::IUnknown, ID2D1Factory, ID2D1Factory1);
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Factory2_Vtbl {
    pub base__: ID2D1Factory1_Vtbl,
    CreateDevice: usize,
}
unsafe impl Send for ID2D1Factory2 {}
unsafe impl Sync for ID2D1Factory2 {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl windows_core::RuntimeName for ID2D1Factory2 {}
windows_core::imp::define_interface!(ID2D1Geometry, ID2D1Geometry_Vtbl, 0x2cd906a1_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1Geometry {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Geometry, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Geometry {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetBounds(&self, worldtransform: Option<*const Common::D2D_MATRIX_3X2_F>) -> windows_core::Result<Common::D2D_RECT_F> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBounds)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillContainsPoint(&self, point: Common::D2D_POINT_2F, worldtransform: Option<*const Common::D2D_MATRIX_3X2_F>, flatteningtolerance: f32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillContainsPoint)(windows_core::Interface::as_raw(self), point, worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Outline<P2>(&self, worldtransform: Option<*const Common::D2D_MATRIX_3X2_F>, flatteningtolerance: f32, geometrysink: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<Common::ID2D1SimplifiedGeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Outline)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, geometrysink.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn ComputeArea(&self, worldtransform: Option<*const Common::D2D_MATRIX_3X2_F>, flatteningtolerance: f32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeArea)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn ComputeLength(&self, worldtransform: Option<*const Common::D2D_MATRIX_3X2_F>, flatteningtolerance: f32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeLength)(windows_core::Interface::as_raw(self), worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn ComputePointAtLength(&self, length: f32, worldtransform: Option<*const Common::D2D_MATRIX_3X2_F>, flatteningtolerance: f32, point: Option<*mut Common::D2D_POINT_2F>, unittangentvector: Option<*mut Common::D2D_POINT_2F>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ComputePointAtLength)(windows_core::Interface::as_raw(self), length, worldtransform.unwrap_or(core::mem::zeroed()) as _, flatteningtolerance, point.unwrap_or(core::mem::zeroed()) as _, unittangentvector.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Geometry_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetBounds: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_MATRIX_3X2_F, *mut Common::D2D_RECT_F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetBounds: usize,
    GetWidenedBounds: usize,
    StrokeContainsPoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillContainsPoint: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *const Common::D2D_MATRIX_3X2_F, f32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillContainsPoint: usize,
    CompareWithGeometry: usize,
    Simplify: usize,
    Tessellate: usize,
    CombineWithGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Outline: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_MATRIX_3X2_F, f32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Outline: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub ComputeArea: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_MATRIX_3X2_F, f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    ComputeArea: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub ComputeLength: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_MATRIX_3X2_F, f32, *mut f32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    ComputeLength: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub ComputePointAtLength: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const Common::D2D_MATRIX_3X2_F, f32, *mut Common::D2D_POINT_2F, *mut Common::D2D_POINT_2F) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    ComputePointAtLength: usize,
    Widen: usize,
}
unsafe impl Send for ID2D1Geometry {}
unsafe impl Sync for ID2D1Geometry {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1Geometry {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
windows_core::imp::define_interface!(ID2D1GeometrySink, ID2D1GeometrySink_Vtbl, 0x2cd9069f_12e2_11dc_9fed_001143a055f9);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl core::ops::Deref for ID2D1GeometrySink {
    type Target = Common::ID2D1SimplifiedGeometrySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
windows_core::imp::interface_hierarchy!(ID2D1GeometrySink, windows_core::IUnknown, Common::ID2D1SimplifiedGeometrySink);
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl ID2D1GeometrySink {
    pub unsafe fn AddLine(&self, point: Common::D2D_POINT_2F) {
        unsafe {
            (windows_core::Interface::vtable(self).AddLine)(windows_core::Interface::as_raw(self), point);
        }
    }
    pub unsafe fn AddBezier(&self, bezier: *const Common::D2D1_BEZIER_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddBezier)(windows_core::Interface::as_raw(self), bezier);
        }
    }
    pub unsafe fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddArc)(windows_core::Interface::as_raw(self), arc);
        }
    }
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1GeometrySink_Vtbl {
    pub base__: Common::ID2D1SimplifiedGeometrySink_Vtbl,
    pub AddLine: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F),
    pub AddBezier: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_BEZIER_SEGMENT),
    AddQuadraticBezier: usize,
    AddQuadraticBeziers: usize,
    pub AddArc: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ARC_SEGMENT),
}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl Send for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
unsafe impl Sync for ID2D1GeometrySink {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1GeometrySink {}
windows_core::imp::define_interface!(ID2D1HwndRenderTarget, ID2D1HwndRenderTarget_Vtbl, 0x2cd90698_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1HwndRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1HwndRenderTarget, windows_core::IUnknown, ID2D1Resource, ID2D1RenderTarget);
impl ID2D1HwndRenderTarget {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Resize(&self, pixelsize: *const Common::D2D_SIZE_U) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), pixelsize).ok() }
    }
    pub unsafe fn GetHwnd(&self) -> super::super::Foundation::HWND {
        unsafe { (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1HwndRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    CheckWindowState: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_SIZE_U) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Resize: usize,
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HWND,
}
unsafe impl Send for ID2D1HwndRenderTarget {}
unsafe impl Sync for ID2D1HwndRenderTarget {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1HwndRenderTarget {}
windows_core::imp::define_interface!(ID2D1Image, ID2D1Image_Vtbl, 0x65019f75_8da2_497c_b32c_dfa34e48ede6);
impl core::ops::Deref for ID2D1Image {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Image, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Image_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
unsafe impl Send for ID2D1Image {}
unsafe impl Sync for ID2D1Image {}
pub trait ID2D1Image_Impl: ID2D1Resource_Impl {}
impl ID2D1Image_Vtbl {
    pub const fn new<Identity: ID2D1Image_Impl, const OFFSET: isize>() -> Self {
        Self { base__: ID2D1Resource_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Image as windows_core::Interface>::IID || iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Image {}
windows_core::imp::define_interface!(ID2D1PathGeometry, ID2D1PathGeometry_Vtbl, 0x2cd906a5_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1PathGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1PathGeometry, windows_core::IUnknown, ID2D1Resource, ID2D1Geometry);
impl ID2D1PathGeometry {
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Open(&self) -> windows_core::Result<ID2D1GeometrySink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Stream<P0>(&self, geometrysink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1GeometrySink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Stream)(windows_core::Interface::as_raw(self), geometrysink.param().abi()).ok() }
    }
    pub unsafe fn GetSegmentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSegmentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFigureCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFigureCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1PathGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Open: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Stream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Stream: usize,
    pub GetSegmentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFigureCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1PathGeometry {}
unsafe impl Sync for ID2D1PathGeometry {}
#[cfg(feature = "Win32_Graphics_Direct2D_Common")]
impl windows_core::RuntimeName for ID2D1PathGeometry {}
windows_core::imp::define_interface!(ID2D1RenderTarget, ID2D1RenderTarget_Vtbl, 0x2cd90694_12e2_11dc_9fed_001143a055f9);
impl core::ops::Deref for ID2D1RenderTarget {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RenderTarget, windows_core::IUnknown, ID2D1Resource);
impl ID2D1RenderTarget {
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateBitmap(&self, size: Common::D2D_SIZE_U, srcdata: Option<*const core::ffi::c_void>, pitch: u32, bitmapproperties: *const D2D1_BITMAP_PROPERTIES) -> windows_core::Result<ID2D1Bitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(windows_core::Interface::as_raw(self), size, srcdata.unwrap_or(core::mem::zeroed()) as _, pitch, bitmapproperties, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSharedBitmap(&self, riid: *const windows_core::GUID, data: *mut core::ffi::c_void, bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>, bitmap: *mut Option<ID2D1Bitmap>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateSharedBitmap)(windows_core::Interface::as_raw(self), riid, data as _, bitmapproperties.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(bitmap)).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRectangle<P1>(&self, rect: *const Common::D2D_RECT_F, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRectangle)(windows_core::Interface::as_raw(self), rect, brush.param().abi());
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillRoundedRectangle<P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRoundedRectangle)(windows_core::Interface::as_raw(self), roundedrect, brush.param().abi());
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn FillEllipse<P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillEllipse)(windows_core::Interface::as_raw(self), ellipse, brush.param().abi());
        }
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillGeometry)(windows_core::Interface::as_raw(self), geometry.param().abi(), brush.param().abi(), opacitybrush.param().abi());
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn DrawBitmap<P0>(&self, bitmap: P0, destinationrectangle: Option<*const Common::D2D_RECT_F>, opacity: f32, interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE, sourcerectangle: Option<*const Common::D2D_RECT_F>)
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawBitmap)(windows_core::Interface::as_raw(self), bitmap.param().abi(), destinationrectangle.unwrap_or(core::mem::zeroed()) as _, opacity, interpolationmode, sourcerectangle.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub unsafe fn DrawTextLayout<P1, P2>(&self, origin: Common::D2D_POINT_2F, textlayout: P1, defaultfillbrush: P2, options: D2D1_DRAW_TEXT_OPTIONS)
    where
        P1: windows_core::Param<super::DirectWrite::IDWriteTextLayout>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawTextLayout)(windows_core::Interface::as_raw(self), origin, textlayout.param().abi(), defaultfillbrush.param().abi(), options);
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetTransform(&self, transform: *const Common::D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(windows_core::Interface::as_raw(self), transform);
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetTransform(&self, transform: *mut Common::D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(windows_core::Interface::as_raw(self), transform as _);
        }
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTags)(windows_core::Interface::as_raw(self), tag1, tag2);
        }
    }
    pub unsafe fn GetTags(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTags)(windows_core::Interface::as_raw(self), tag1.unwrap_or(core::mem::zeroed()) as _, tag2.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn PopLayer(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Flush(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self), tag1.unwrap_or(core::mem::zeroed()) as _, tag2.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopAxisAlignedClip)(windows_core::Interface::as_raw(self));
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn Clear(&self, clearcolor: Option<*const Common::D2D1_COLOR_F>) {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self), clearcolor.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn BeginDraw(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn EndDraw(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndDraw)(windows_core::Interface::as_raw(self), tag1.unwrap_or(core::mem::zeroed()) as _, tag2.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetPixelFormat(&self) -> Common::D2D1_PIXEL_FORMAT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDpi)(windows_core::Interface::as_raw(self), dpix, dpiy);
        }
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDpi)(windows_core::Interface::as_raw(self), dpix as _, dpiy as _);
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetSize(&self) -> Common::D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn GetPixelSize(&self) -> Common::D2D_SIZE_U {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelSize)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetMaximumBitmapSize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1RenderTarget_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_SIZE_U, *const core::ffi::c_void, u32, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateBitmap: usize,
    CreateBitmapFromWicBitmap: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub CreateSharedBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const D2D1_BITMAP_PROPERTIES, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    CreateSharedBitmap: usize,
    CreateBitmapBrush: usize,
    CreateSolidColorBrush: usize,
    CreateGradientStopCollection: usize,
    CreateLinearGradientBrush: usize,
    CreateRadialGradientBrush: usize,
    CreateCompatibleRenderTarget: usize,
    CreateLayer: usize,
    CreateMesh: usize,
    DrawLine: usize,
    DrawRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_RECT_F, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRectangle: usize,
    DrawRoundedRectangle: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillRoundedRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ROUNDED_RECT, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillRoundedRectangle: usize,
    DrawEllipse: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub FillEllipse: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ELLIPSE, *mut core::ffi::c_void),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    FillEllipse: usize,
    DrawGeometry: usize,
    pub FillGeometry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void),
    FillMesh: usize,
    FillOpacityMask: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub DrawBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const Common::D2D_RECT_F, f32, D2D1_BITMAP_INTERPOLATION_MODE, *const Common::D2D_RECT_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    DrawBitmap: usize,
    DrawText: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite"))]
    pub DrawTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, Common::D2D_POINT_2F, *mut core::ffi::c_void, *mut core::ffi::c_void, D2D1_DRAW_TEXT_OPTIONS),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite")))]
    DrawTextLayout: usize,
    DrawGlyphRun: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D_MATRIX_3X2_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_MATRIX_3X2_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetTransform: usize,
    SetAntialiasMode: usize,
    GetAntialiasMode: usize,
    SetTextAntialiasMode: usize,
    GetTextAntialiasMode: usize,
    SetTextRenderingParams: usize,
    GetTextRenderingParams: usize,
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64),
    pub GetTags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64),
    PushLayer: usize,
    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    SaveDrawingState: usize,
    RestoreDrawingState: usize,
    PushAxisAlignedClip: usize,
    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const Common::D2D1_COLOR_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    Clear: usize,
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EndDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D1_PIXEL_FORMAT),
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    GetPixelFormat: usize,
    pub SetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32),
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_F),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetSize: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut Common::D2D_SIZE_U),
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    GetPixelSize: usize,
    pub GetMaximumBitmapSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    IsSupported: usize,
}
unsafe impl Send for ID2D1RenderTarget {}
unsafe impl Sync for ID2D1RenderTarget {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_DirectWrite", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID2D1RenderTarget {}
windows_core::imp::define_interface!(ID2D1Resource, ID2D1Resource_Vtbl, 0x2cd90691_12e2_11dc_9fed_001143a055f9);
windows_core::imp::interface_hierarchy!(ID2D1Resource, windows_core::IUnknown);
impl ID2D1Resource {
    pub unsafe fn GetFactory(&self) -> windows_core::Result<ID2D1Factory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFactory)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::imp::Type::from_abi(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1Resource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
}
unsafe impl Send for ID2D1Resource {}
unsafe impl Sync for ID2D1Resource {}
pub trait ID2D1Resource_Impl: windows_core::IUnknownImpl {
    fn GetFactory(&self, factory: windows_core::OutRef<ID2D1Factory>);
}
impl ID2D1Resource_Vtbl {
    pub const fn new<Identity: ID2D1Resource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFactory<Identity: ID2D1Resource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Resource_Impl::GetFactory(this, core::mem::transmute_copy(&factory));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetFactory: GetFactory::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Resource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Resource {}
