pub type D2D1_ALPHA_MODE = i32;
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = 3;
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = 1;
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = 2;
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
    pub point3: D2D_POINT_2F,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
pub type D2D1_FIGURE_BEGIN = i32;
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = 0;
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = 1;
pub type D2D1_FIGURE_END = i32;
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = 1;
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: D2D1_COLOR_F,
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: super::super::Dxgi::Common::DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_3X2_F {
    pub Anonymous: D2D_MATRIX_3X2_F_0,
}
impl Default for D2D_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_3X2_F_0 {
    pub Anonymous1: D2D_MATRIX_3X2_F_0_0,
    pub Anonymous2: D2D_MATRIX_3X2_F_0_1,
    pub m: [f32; 6],
}
impl Default for D2D_MATRIX_3X2_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_MATRIX_3X2_F_0_0 {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_MATRIX_3X2_F_0_1 {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_RECT_F {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_SIZE_F {
    pub width: f32,
    pub height: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_SIZE_U {
    pub width: u32,
    pub height: u32,
}
windows_core::imp::define_interface!(ID2D1SimplifiedGeometrySink, ID2D1SimplifiedGeometrySink_Vtbl, 0x2cd9069e_12e2_11dc_9fed_001143a055f9);
windows_core::imp::interface_hierarchy!(ID2D1SimplifiedGeometrySink, windows_core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    pub unsafe fn BeginFigure(&self, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginFigure)(windows_core::Interface::as_raw(self), startpoint, figurebegin);
        }
    }
    pub unsafe fn AddLines(&self, points: &[D2D_POINT_2F]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddLines)(windows_core::Interface::as_raw(self), points.as_ptr(), points.len().try_into().unwrap());
        }
    }
    pub unsafe fn AddBeziers(&self, beziers: &[D2D1_BEZIER_SEGMENT]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddBeziers)(windows_core::Interface::as_raw(self), beziers.as_ptr(), beziers.len().try_into().unwrap());
        }
    }
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        unsafe {
            (windows_core::Interface::vtable(self).EndFigure)(windows_core::Interface::as_raw(self), figureend);
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID2D1SimplifiedGeometrySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    SetFillMode: usize,
    SetSegmentFlags: usize,
    pub BeginFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D_POINT_2F, D2D1_FIGURE_BEGIN),
    pub AddLines: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_POINT_2F, u32),
    pub AddBeziers: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BEZIER_SEGMENT, u32),
    pub EndFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FIGURE_END),
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1SimplifiedGeometrySink {}
unsafe impl Sync for ID2D1SimplifiedGeometrySink {}
impl windows_core::RuntimeName for ID2D1SimplifiedGeometrySink {}
