#[inline]
pub unsafe fn AddDllDirectory<P0>(newdirectory: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddDllDirectory(newdirectory : windows_core::PCWSTR) -> *mut core::ffi::c_void);
    unsafe { AddDllDirectory(newdirectory.param().abi()) }
}
#[inline]
pub unsafe fn CoCreateInstance<P1, T>(
    rclsid: *const windows_core::GUID,
    punkouter: P1,
    dwclscontext: CLSCTX,
) -> windows_core::Result<T>
where
    P1: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : CLSCTX, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        CoCreateInstance(
            rclsid,
            punkouter.param().abi(),
            dwclscontext,
            &T::IID,
            &mut result__,
        )
        .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CoInitializeEx(
    pvreserved: Option<*const core::ffi::c_void>,
    dwcoinit: COINIT,
) -> windows_core::HRESULT {
    windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : COINIT) -> windows_core::HRESULT);
    unsafe { CoInitializeEx(pvreserved.unwrap_or(core::mem::zeroed()) as _, dwcoinit) }
}
#[inline]
pub unsafe fn CoTaskMemAlloc(cb: usize) -> *mut core::ffi::c_void {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
    unsafe { CoTaskMemAlloc(cb) }
}
#[inline]
pub unsafe fn CoTaskMemFree(pv: Option<*const core::ffi::c_void>) {
    windows_core::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
    unsafe { CoTaskMemFree(pv.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CoUninitialize() {
    windows_core::link!("ole32.dll" "system" fn CoUninitialize());
    unsafe { CoUninitialize() }
}
#[inline]
pub unsafe fn D2D1CreateFactory<T>(
    factorytype: D2D1_FACTORY_TYPE,
    pfactoryoptions: Option<*const D2D1_FACTORY_OPTIONS>,
) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("d2d1.dll" "system" fn D2D1CreateFactory(factorytype : D2D1_FACTORY_TYPE, riid : *const windows_core::GUID, pfactoryoptions : *const D2D1_FACTORY_OPTIONS, ppifactory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        D2D1CreateFactory(
            factorytype,
            &T::IID,
            pfactoryoptions.unwrap_or(core::mem::zeroed()) as _,
            &mut result__,
        )
        .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn D3D11CreateDevice<P0>(
    padapter: P0,
    drivertype: D3D_DRIVER_TYPE,
    software: Option<HMODULE>,
    flags: D3D11_CREATE_DEVICE_FLAG,
    pfeaturelevels: Option<&[D3D_FEATURE_LEVEL]>,
    sdkversion: u32,
    ppdevice: Option<*mut Option<ID3D11Device>>,
    pfeaturelevel: Option<*mut D3D_FEATURE_LEVEL>,
    ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>,
) -> windows_core::Result<()>
where
    P0: windows_core::Param<IDXGIAdapter>,
{
    windows_core::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : *mut core::ffi::c_void, drivertype : D3D_DRIVER_TYPE, software : HMODULE, flags : D3D11_CREATE_DEVICE_FLAG, pfeaturelevels : *const D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        D3D11CreateDevice(
            padapter.param().abi(),
            drivertype,
            software.unwrap_or(core::mem::zeroed()) as _,
            flags,
            pfeaturelevels.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            pfeaturelevels.map_or(0, |slice| slice.len().try_into().unwrap()),
            sdkversion,
            ppdevice.unwrap_or(core::mem::zeroed()) as _,
            pfeaturelevel.unwrap_or(core::mem::zeroed()) as _,
            ppimmediatecontext.unwrap_or(core::mem::zeroed()) as _,
        )
        .ok()
    }
}
#[inline]
pub unsafe fn DWriteCreateFactory<T>(factorytype: DWRITE_FACTORY_TYPE) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe {
        DWriteCreateFactory(factorytype, &T::IID, &mut result__)
            .and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn GetProcAddress<P1>(hmodule: HMODULE, lpprocname: P1) -> FARPROC
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : windows_core::PCSTR) -> FARPROC);
    unsafe { GetProcAddress(hmodule, lpprocname.param().abi()) }
}
#[inline]
pub unsafe fn LoadLibraryW<P0>(lplibfilename: P0) -> HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadLibraryW(lplibfilename : windows_core::PCWSTR) -> HMODULE);
    unsafe { LoadLibraryW(lplibfilename.param().abi()) }
}
#[inline]
pub unsafe fn MFCreateAttributes(
    ppmfattributes: *mut Option<IMFAttributes>,
    cinitialsize: u32,
) -> windows_core::Result<()> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateAttributes(ppmfattributes : *mut *mut core::ffi::c_void, cinitialsize : u32) -> windows_core::HRESULT);
    unsafe { MFCreateAttributes(core::mem::transmute(ppmfattributes), cinitialsize).ok() }
}
#[inline]
pub unsafe fn MFShutdown() -> windows_core::Result<()> {
    windows_core::link!("mfplat.dll" "system" fn MFShutdown() -> windows_core::HRESULT);
    unsafe { MFShutdown().ok() }
}
#[inline]
pub unsafe fn MFStartup(version: u32, dwflags: u32) -> windows_core::Result<()> {
    windows_core::link!("mfplat.dll" "system" fn MFStartup(version : u32, dwflags : u32) -> windows_core::HRESULT);
    unsafe { MFStartup(version, dwflags).ok() }
}
#[inline]
pub unsafe fn PathCchCombineEx<P2, P3>(
    pszpathout: windows_core::PWSTR,
    cchpathout: usize,
    pszpathin: P2,
    pszmore: P3,
    dwflags: PATHCCH_OPTIONS,
) -> windows_core::Result<()>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-path-l1-1-0.dll" "system" fn PathCchCombineEx(pszpathout : windows_core::PWSTR, cchpathout : usize, pszpathin : windows_core::PCWSTR, pszmore : windows_core::PCWSTR, dwflags : PATHCCH_OPTIONS) -> windows_core::HRESULT);
    unsafe {
        PathCchCombineEx(
            pszpathout,
            cchpathout,
            pszpathin.param().abi(),
            pszmore.param().abi(),
            dwflags,
        )
        .ok()
    }
}
#[inline]
pub unsafe fn SHGetSpecialFolderPathW(
    hwnd: Option<HWND>,
    pszpath: windows_core::PWSTR,
    csidl: i32,
    fcreate: bool,
) -> windows_core::BOOL {
    windows_core::link!("shell32.dll" "system" fn SHGetSpecialFolderPathW(hwnd : HWND, pszpath : windows_core::PWSTR, csidl : i32, fcreate : windows_core::BOOL) -> windows_core::BOOL);
    unsafe {
        SHGetSpecialFolderPathW(
            hwnd.unwrap_or(core::mem::zeroed()) as _,
            pszpath,
            csidl,
            fcreate.into(),
        )
    }
}
#[inline]
pub unsafe fn SetDefaultDllDirectories(directoryflags: LOAD_LIBRARY_FLAGS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetDefaultDllDirectories(directoryflags : LOAD_LIBRARY_FLAGS) -> windows_core::BOOL);
    unsafe { SetDefaultDllDirectories(directoryflags) }
}
pub type ADVANCED_FEATURE_FLAGS = u16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BLOB {
    pub cbSize: u32,
    pub pBlobData: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BSTRBLOB {
    pub cbSize: u32,
    pub pData: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABOOL {
    pub cElems: u32,
    pub pElems: *mut VARIANT_BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::BSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CABSTRBLOB {
    pub cElems: u32,
    pub pElems: *mut BSTRBLOB,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAC {
    pub cElems: u32,
    pub pElems: windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CACLIPDATA {
    pub cElems: u32,
    pub pElems: *mut CLIPDATA,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CACLSID {
    pub cElems: u32,
    pub pElems: *mut windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CACY {
    pub cElems: u32,
    pub pElems: *mut CY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CADATE {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CADBL {
    pub cElems: u32,
    pub pElems: *mut f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAFILETIME {
    pub cElems: u32,
    pub pElems: *mut FILETIME,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAFLT {
    pub cElems: u32,
    pub pElems: *mut f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAH {
    pub cElems: u32,
    pub pElems: *mut i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAI {
    pub cElems: u32,
    pub pElems: *mut i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAL {
    pub cElems: u32,
    pub pElems: *mut i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALPSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CALPWSTR {
    pub cElems: u32,
    pub pElems: *mut windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAPROPVARIANT {
    pub cElems: u32,
    pub pElems: *mut PROPVARIANT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CASCODE {
    pub cElems: u32,
    pub pElems: *mut i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUB {
    pub cElems: u32,
    pub pElems: *mut u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUH {
    pub cElems: u32,
    pub pElems: *mut u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUI {
    pub cElems: u32,
    pub pElems: *mut u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CAUL {
    pub cElems: u32,
    pub pElems: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIPDATA {
    pub cbSize: u32,
    pub ulClipFmt: i32,
    pub pClipData: *mut u8,
}
pub type CLSCTX = u32;
pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = 262144;
pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = 524288;
pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = 8388608;
pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = 33554432;
pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = 262144;
pub const CLSCTX_ALL: CLSCTX = 23;
pub const CLSCTX_ALLOW_LOWER_TRUST_REGISTRATION: CLSCTX = 67108864;
pub const CLSCTX_APPCONTAINER: CLSCTX = 4194304;
pub const CLSCTX_DISABLE_AAA: CLSCTX = 32768;
pub const CLSCTX_DO_NOT_ELEVATE_SERVER: CLSCTX = 268435456;
pub const CLSCTX_ENABLE_AAA: CLSCTX = 65536;
pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = 1048576;
pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = 8192;
pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = 131072;
pub const CLSCTX_INPROC_HANDLER: CLSCTX = 2;
pub const CLSCTX_INPROC_HANDLER16: CLSCTX = 32;
pub const CLSCTX_INPROC_SERVER: CLSCTX = 1;
pub const CLSCTX_INPROC_SERVER16: CLSCTX = 8;
pub const CLSCTX_LOCAL_SERVER: CLSCTX = 4;
pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = 1024;
pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = 4096;
pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = 16384;
pub const CLSCTX_PS_DLL: CLSCTX = 2147483648;
pub const CLSCTX_REMOTE_SERVER: CLSCTX = 16;
pub const CLSCTX_RESERVED1: CLSCTX = 64;
pub const CLSCTX_RESERVED2: CLSCTX = 128;
pub const CLSCTX_RESERVED3: CLSCTX = 256;
pub const CLSCTX_RESERVED4: CLSCTX = 512;
pub const CLSCTX_RESERVED5: CLSCTX = 2048;
pub const CLSCTX_RESERVED6: CLSCTX = 16777216;
pub const CLSCTX_SERVER: CLSCTX = 21;
pub const CLSCTX_SERVER_MUST_BE_EQUAL_OR_GREATER_PRIVILEGE: CLSCTX = 134217728;
pub const CLSID_MFMediaEngineClassFactory: windows_core::GUID =
    windows_core::GUID::from_u128(0xb44392da_499b_446b_a4cb_005fead0e6d5);
pub type COINIT = i32;
pub const COINIT_APARTMENTTHREADED: COINIT = 2;
pub const COINIT_DISABLE_OLE1DDE: COINIT = 4;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub const COINIT_SPEED_OVER_MEMORY: COINIT = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMDLG_FILTERSPEC {
    pub pszName: windows_core::PWSTR,
    pub pszSpec: windows_core::PWSTR,
}
pub const CSIDL_WINDOWS: u32 = 36;
#[repr(C)]
#[derive(Clone, Copy)]
pub union CY {
    pub Anonymous: CY_0,
    pub int64: i64,
}
impl Default for CY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CY_0 {
    pub Lo: u32,
    pub Hi: i32,
}
pub type D2D1_ALPHA_MODE = i32;
pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = 3;
pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = 1;
pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = 2;
pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = 0;
pub type D2D1_ANTIALIAS_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ARC_SEGMENT {
    pub point: D2D_POINT_2F,
    pub size: D2D_SIZE_F,
    pub rotationAngle: f32,
    pub sweepDirection: D2D1_SWEEP_DIRECTION,
    pub arcSize: D2D1_ARC_SIZE,
}
pub type D2D1_ARC_SIZE = i32;
pub const D2D1_ARC_SIZE_LARGE: D2D1_ARC_SIZE = 1;
pub const D2D1_ARC_SIZE_SMALL: D2D1_ARC_SIZE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
    pub point3: D2D_POINT_2F,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_BITMAP_BRUSH_PROPERTIES1 {
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES {
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_BITMAP_PROPERTIES1 {
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub bitmapOptions: D2D1_BITMAP_OPTIONS,
    pub colorContext: core::mem::ManuallyDrop<Option<ID2D1ColorContext>>,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D1_BRUSH_PROPERTIES {
    pub opacity: f32,
    pub transform: D2D_MATRIX_3X2_F,
}
impl Default for D2D1_BRUSH_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D2D1_BUFFER_PRECISION = i32;
pub type D2D1_CAP_STYLE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_COLOR_F {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
pub type D2D1_COLOR_INTERPOLATION_MODE = i32;
pub type D2D1_COLOR_SPACE = i32;
pub type D2D1_COMBINE_MODE = i32;
pub type D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS = i32;
pub type D2D1_COMPOSITE_MODE = i32;
pub type D2D1_DASH_STYLE = i32;
pub type D2D1_DEBUG_LEVEL = i32;
pub const D2D1_DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25;
pub type D2D1_DEVICE_CONTEXT_OPTIONS = i32;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS:
    D2D1_DEVICE_CONTEXT_OPTIONS = 1;
pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D2D1_DRAWING_STATE_DESCRIPTION {
    pub antialiasMode: D2D1_ANTIALIAS_MODE,
    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
    pub tag1: u64,
    pub tag2: u64,
    pub transform: D2D_MATRIX_3X2_F,
}
impl Default for D2D1_DRAWING_STATE_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D2D1_DRAW_TEXT_OPTIONS = i32;
pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS = 2;
pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING: D2D1_DRAW_TEXT_OPTIONS = 8;
pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS = 4;
pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS = 0;
pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS = 1;
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct D2D1_EFFECT_INPUT_DESCRIPTION {
    pub effect: core::mem::ManuallyDrop<Option<ID2D1Effect>>,
    pub inputIndex: u32,
    pub inputRectangle: D2D_RECT_F,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ELLIPSE {
    pub point: D2D_POINT_2F,
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
pub type D2D1_FIGURE_BEGIN = i32;
pub const D2D1_FIGURE_BEGIN_FILLED: D2D1_FIGURE_BEGIN = 0;
pub const D2D1_FIGURE_BEGIN_HOLLOW: D2D1_FIGURE_BEGIN = 1;
pub type D2D1_FIGURE_END = i32;
pub const D2D1_FIGURE_END_CLOSED: D2D1_FIGURE_END = 1;
pub const D2D1_FIGURE_END_OPEN: D2D1_FIGURE_END = 0;
pub type D2D1_FILL_MODE = i32;
pub type D2D1_GAMMA = i32;
pub const D2D1_GAMMA_1_0: D2D1_GAMMA = 1;
pub const D2D1_GAMMA_2_2: D2D1_GAMMA = 0;
pub type D2D1_GEOMETRY_RELATION = i32;
pub type D2D1_GEOMETRY_SIMPLIFICATION_OPTION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_GRADIENT_STOP {
    pub position: f32,
    pub color: D2D1_COLOR_F,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
    pub hwnd: HWND,
    pub pixelSize: D2D_SIZE_U,
    pub presentOptions: D2D1_PRESENT_OPTIONS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_IMAGE_BRUSH_PROPERTIES {
    pub sourceRectangle: D2D_RECT_F,
    pub extendModeX: D2D1_EXTEND_MODE,
    pub extendModeY: D2D1_EXTEND_MODE,
    pub interpolationMode: D2D1_INTERPOLATION_MODE,
}
pub type D2D1_INTERPOLATION_MODE = i32;
pub type D2D1_LAYER_OPTIONS = i32;
pub type D2D1_LAYER_OPTIONS1 = i32;
#[repr(C)]
pub struct D2D1_LAYER_PARAMETERS {
    pub contentBounds: D2D_RECT_F,
    pub geometricMask: core::mem::ManuallyDrop<Option<ID2D1Geometry>>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: D2D_MATRIX_3X2_F,
    pub opacity: f32,
    pub opacityBrush: core::mem::ManuallyDrop<Option<ID2D1Brush>>,
    pub layerOptions: D2D1_LAYER_OPTIONS,
}
impl Clone for D2D1_LAYER_PARAMETERS {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for D2D1_LAYER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct D2D1_LAYER_PARAMETERS1 {
    pub contentBounds: D2D_RECT_F,
    pub geometricMask: core::mem::ManuallyDrop<Option<ID2D1Geometry>>,
    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
    pub maskTransform: D2D_MATRIX_3X2_F,
    pub opacity: f32,
    pub opacityBrush: core::mem::ManuallyDrop<Option<ID2D1Brush>>,
    pub layerOptions: D2D1_LAYER_OPTIONS1,
}
impl Clone for D2D1_LAYER_PARAMETERS1 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for D2D1_LAYER_PARAMETERS1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
    pub startPoint: D2D_POINT_2F,
    pub endPoint: D2D_POINT_2F,
}
pub type D2D1_LINE_JOIN = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_MAPPED_RECT {
    pub pitch: u32,
    pub bits: *mut u8,
}
pub type D2D1_MAP_OPTIONS = i32;
pub type D2D1_OPACITY_MASK_CONTENT = i32;
pub type D2D1_PATH_SEGMENT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_PIXEL_FORMAT {
    pub format: DXGI_FORMAT,
    pub alphaMode: D2D1_ALPHA_MODE,
}
pub type D2D1_PRESENT_OPTIONS = i32;
pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS = 2;
pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS = 0;
pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS = 1;
pub type D2D1_PRIMITIVE_BLEND = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_PRINT_CONTROL_PROPERTIES {
    pub fontSubset: D2D1_PRINT_FONT_SUBSET_MODE,
    pub rasterDPI: f32,
    pub colorSpace: D2D1_COLOR_SPACE,
}
pub type D2D1_PRINT_FONT_SUBSET_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_QUADRATIC_BEZIER_SEGMENT {
    pub point1: D2D_POINT_2F,
    pub point2: D2D_POINT_2F,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
    pub center: D2D_POINT_2F,
    pub gradientOriginOffset: D2D_POINT_2F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D1_RENDERING_CONTROLS {
    pub bufferPrecision: D2D1_BUFFER_PRECISION,
    pub tileSize: D2D_SIZE_U,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_RENDER_TARGET_PROPERTIES {
    pub r#type: D2D1_RENDER_TARGET_TYPE,
    pub pixelFormat: D2D1_PIXEL_FORMAT,
    pub dpiX: f32,
    pub dpiY: f32,
    pub usage: D2D1_RENDER_TARGET_USAGE,
    pub minLevel: D2D1_FEATURE_LEVEL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_ROUNDED_RECT {
    pub rect: D2D_RECT_F,
    pub radiusX: f32,
    pub radiusY: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D1_STROKE_STYLE_PROPERTIES {
    pub startCap: D2D1_CAP_STYLE,
    pub endCap: D2D1_CAP_STYLE,
    pub dashCap: D2D1_CAP_STYLE,
    pub lineJoin: D2D1_LINE_JOIN,
    pub miterLimit: f32,
    pub dashStyle: D2D1_DASH_STYLE,
    pub dashOffset: f32,
}
pub type D2D1_SWEEP_DIRECTION = i32;
pub const D2D1_SWEEP_DIRECTION_CLOCKWISE: D2D1_SWEEP_DIRECTION = 1;
pub const D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE: D2D1_SWEEP_DIRECTION = 0;
pub type D2D1_TEXT_ANTIALIAS_MODE = i32;
pub type D2D1_UNIT_MODE = i32;
pub type D2D1_WINDOW_STATE = i32;
pub const D2DERR_RECREATE_TARGET: windows_core::HRESULT =
    windows_core::HRESULT(0x8899000C_u32 as _);
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
#[derive(Clone, Copy)]
pub struct D2D_MATRIX_4X4_F {
    pub Anonymous: D2D_MATRIX_4X4_F_0,
}
impl Default for D2D_MATRIX_4X4_F {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D2D_MATRIX_4X4_F_0 {
    pub Anonymous: D2D_MATRIX_4X4_F_0_0,
    pub m: [f32; 16],
}
impl Default for D2D_MATRIX_4X4_F_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_MATRIX_4X4_F_0_0 {
    pub _11: f32,
    pub _12: f32,
    pub _13: f32,
    pub _14: f32,
    pub _21: f32,
    pub _22: f32,
    pub _23: f32,
    pub _24: f32,
    pub _31: f32,
    pub _32: f32,
    pub _33: f32,
    pub _34: f32,
    pub _41: f32,
    pub _42: f32,
    pub _43: f32,
    pub _44: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D2D_POINT_2F {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_POINT_2U {
    pub x: u32,
    pub y: u32,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D2D_RECT_U {
    pub left: u32,
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
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
pub type D3D11_BIND_FLAG = i32;
pub type D3D11_BLEND = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: windows_core::BOOL,
    pub IndependentBlendEnable: windows_core::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}
impl Default for D3D11_BLEND_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_BLEND_OP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BUFFEREX_SRV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D11_USAGE,
    pub BindFlags: D3D11_BIND_FLAG,
    pub CPUAccessFlags: D3D11_CPU_ACCESS_FLAG,
    pub MiscFlags: D3D11_RESOURCE_MISC_FLAG,
    pub StructureByteStride: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_RTV {
    pub Anonymous1: D3D11_BUFFER_RTV_0,
    pub Anonymous2: D3D11_BUFFER_RTV_1,
}
impl Default for D3D11_BUFFER_RTV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D11_BUFFER_RTV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D11_BUFFER_RTV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_BUFFER_SRV {
    pub Anonymous1: D3D11_BUFFER_SRV_0,
    pub Anonymous2: D3D11_BUFFER_SRV_1,
}
impl Default for D3D11_BUFFER_SRV {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl Default for D3D11_BUFFER_SRV_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl Default for D3D11_BUFFER_SRV_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_BUFFER_UAV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
pub type D3D11_COMPARISON_FUNC = i32;
pub type D3D11_COUNTER = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_COUNTER_DESC {
    pub Counter: D3D11_COUNTER,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D11_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
pub type D3D11_COUNTER_TYPE = i32;
pub type D3D11_CPU_ACCESS_FLAG = i32;
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 32;
pub const D3D11_CREATE_DEVICE_DEBUG: D3D11_CREATE_DEVICE_FLAG = 2;
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: D3D11_CREATE_DEVICE_FLAG = 64;
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: D3D11_CREATE_DEVICE_FLAG = 256;
pub type D3D11_CREATE_DEVICE_FLAG = u32;
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY:
    D3D11_CREATE_DEVICE_FLAG = 128;
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D11_CREATE_DEVICE_FLAG =
    8;
pub const D3D11_CREATE_DEVICE_SINGLETHREADED: D3D11_CREATE_DEVICE_FLAG = 1;
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: D3D11_CREATE_DEVICE_FLAG = 4;
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 2048;
pub type D3D11_CULL_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: windows_core::BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: windows_core::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: u32,
    pub Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0,
}
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D11_TEX1D_DSV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D11_TEX2D_DSV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D11_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_DSV,
}
impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_DEPTH_WRITE_MASK = i32;
pub type D3D11_DEVICE_CONTEXT_TYPE = i32;
pub type D3D11_DSV_DIMENSION = i32;
pub type D3D11_FEATURE = i32;
pub type D3D11_FILL_MODE = i32;
pub type D3D11_FILTER = i32;
pub type D3D11_INPUT_CLASSIFICATION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: windows_core::PSTR,
    pub SemanticIndex: u32,
    pub Format: DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
pub type D3D11_MAP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
pub type D3D11_QUERY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_QUERY_DESC {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: windows_core::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: windows_core::BOOL,
    pub ScissorEnable: windows_core::BOOL,
    pub MultisampleEnable: windows_core::BOOL,
    pub AntialiasedLineEnable: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: windows_core::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0,
}
impl Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
impl Default for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_RESOURCE_MISC_FLAG = i32;
pub type D3D11_RTV_DIMENSION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: D3D11_FILTER,
    pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D11_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl Default for D3D11_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3D11_SDK_VERSION: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0,
}
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: windows_core::PSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
pub type D3D11_STENCIL_OP = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *mut core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX1D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX2D_UAV {
    pub MipSlice: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: D3D11_BIND_FLAG,
    pub CPUAccessFlags: D3D11_CPU_ACCESS_FLAG,
    pub MiscFlags: D3D11_RESOURCE_MISC_FLAG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: D3D11_BIND_FLAG,
    pub CPUAccessFlags: D3D11_CPU_ACCESS_FLAG,
    pub MiscFlags: D3D11_RESOURCE_MISC_FLAG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: D3D11_BIND_FLAG,
    pub CPUAccessFlags: D3D11_CPU_ACCESS_FLAG,
    pub MiscFlags: D3D11_RESOURCE_MISC_FLAG,
}
pub type D3D11_TEXTURE_ADDRESS_MODE = i32;
pub type D3D11_UAV_DIMENSION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
}
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D11_TEX3D_UAV,
}
impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3D11_USAGE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
pub type D3D_DRIVER_TYPE = i32;
pub const D3D_DRIVER_TYPE_HARDWARE: D3D_DRIVER_TYPE = 1;
pub const D3D_DRIVER_TYPE_NULL: D3D_DRIVER_TYPE = 3;
pub const D3D_DRIVER_TYPE_REFERENCE: D3D_DRIVER_TYPE = 2;
pub const D3D_DRIVER_TYPE_SOFTWARE: D3D_DRIVER_TYPE = 4;
pub const D3D_DRIVER_TYPE_UNKNOWN: D3D_DRIVER_TYPE = 0;
pub const D3D_DRIVER_TYPE_WARP: D3D_DRIVER_TYPE = 5;
pub type D3D_FEATURE_LEVEL = i32;
pub const D3D_FEATURE_LEVEL_10_0: D3D_FEATURE_LEVEL = 40960;
pub const D3D_FEATURE_LEVEL_10_1: D3D_FEATURE_LEVEL = 41216;
pub const D3D_FEATURE_LEVEL_11_0: D3D_FEATURE_LEVEL = 45056;
pub const D3D_FEATURE_LEVEL_11_1: D3D_FEATURE_LEVEL = 45312;
pub const D3D_FEATURE_LEVEL_12_0: D3D_FEATURE_LEVEL = 49152;
pub const D3D_FEATURE_LEVEL_12_1: D3D_FEATURE_LEVEL = 49408;
pub const D3D_FEATURE_LEVEL_12_2: D3D_FEATURE_LEVEL = 49664;
pub const D3D_FEATURE_LEVEL_1_0_CORE: D3D_FEATURE_LEVEL = 4096;
pub const D3D_FEATURE_LEVEL_1_0_GENERIC: D3D_FEATURE_LEVEL = 256;
pub const D3D_FEATURE_LEVEL_9_1: D3D_FEATURE_LEVEL = 37120;
pub const D3D_FEATURE_LEVEL_9_2: D3D_FEATURE_LEVEL = 37376;
pub const D3D_FEATURE_LEVEL_9_3: D3D_FEATURE_LEVEL = 37632;
pub type D3D_PRIMITIVE_TOPOLOGY = i32;
pub type D3D_SRV_DIMENSION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DECIMAL {
    pub wReserved: u16,
    pub Anonymous1: DECIMAL_0,
    pub Hi32: u32,
    pub Anonymous2: DECIMAL_1,
}
impl Default for DECIMAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_0 {
    pub Anonymous: DECIMAL_0_0,
    pub signscale: u16,
}
impl Default for DECIMAL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_0_0 {
    pub scale: u8,
    pub sign: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DECIMAL_1 {
    pub Anonymous: DECIMAL_1_0,
    pub Lo64: u64,
}
impl Default for DECIMAL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DECIMAL_1_0 {
    pub Lo32: u32,
    pub Mid32: u32,
}
pub const DONT_RESOLVE_DLL_REFERENCES: LOAD_LIBRARY_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_CLUSTER_METRICS {
    pub width: f32,
    pub length: u16,
    pub _bitfield: u16,
}
impl DWRITE_CLUSTER_METRICS {
    pub fn canWrapLineAfter(&self) -> bool {
        self._bitfield & 1 != 0
    }
    pub fn set_canWrapLineAfter(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !1) | (value as u16);
    }
    pub fn isWhitespace(&self) -> bool {
        (self._bitfield >> 1) & 1 != 0
    }
    pub fn set_isWhitespace(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value as u16) << 1);
    }
    pub fn isNewline(&self) -> bool {
        (self._bitfield >> 2) & 1 != 0
    }
    pub fn set_isNewline(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value as u16) << 2);
    }
    pub fn isSoftHyphen(&self) -> bool {
        (self._bitfield >> 3) & 1 != 0
    }
    pub fn set_isSoftHyphen(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value as u16) << 3);
    }
    pub fn isRightToLeft(&self) -> bool {
        (self._bitfield >> 4) & 1 != 0
    }
    pub fn set_isRightToLeft(&mut self, value: bool) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value as u16) << 4);
    }
    pub fn padding(&self) -> u16 {
        self._bitfield >> 5
    }
    pub fn set_padding(&mut self, value: u16) {
        self._bitfield = (self._bitfield & !(2047 << 5)) | ((value & 2047) << 5);
    }
}
pub type DWRITE_FACTORY_TYPE = i32;
pub const DWRITE_FACTORY_TYPE_ISOLATED: DWRITE_FACTORY_TYPE = 1;
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0;
pub type DWRITE_FONT_FACE_TYPE = i32;
pub type DWRITE_FONT_SIMULATIONS = i32;
pub type DWRITE_FONT_STRETCH = i32;
pub const DWRITE_FONT_STRETCH_CONDENSED: DWRITE_FONT_STRETCH = 3;
pub const DWRITE_FONT_STRETCH_EXPANDED: DWRITE_FONT_STRETCH = 7;
pub const DWRITE_FONT_STRETCH_EXTRA_CONDENSED: DWRITE_FONT_STRETCH = 2;
pub const DWRITE_FONT_STRETCH_EXTRA_EXPANDED: DWRITE_FONT_STRETCH = 8;
pub const DWRITE_FONT_STRETCH_MEDIUM: DWRITE_FONT_STRETCH = 5;
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = 5;
pub const DWRITE_FONT_STRETCH_SEMI_CONDENSED: DWRITE_FONT_STRETCH = 4;
pub const DWRITE_FONT_STRETCH_SEMI_EXPANDED: DWRITE_FONT_STRETCH = 6;
pub const DWRITE_FONT_STRETCH_ULTRA_CONDENSED: DWRITE_FONT_STRETCH = 1;
pub const DWRITE_FONT_STRETCH_ULTRA_EXPANDED: DWRITE_FONT_STRETCH = 9;
pub const DWRITE_FONT_STRETCH_UNDEFINED: DWRITE_FONT_STRETCH = 0;
pub type DWRITE_FONT_STYLE = i32;
pub const DWRITE_FONT_STYLE_ITALIC: DWRITE_FONT_STYLE = 2;
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = 0;
pub const DWRITE_FONT_STYLE_OBLIQUE: DWRITE_FONT_STYLE = 1;
pub type DWRITE_FONT_WEIGHT = i32;
pub const DWRITE_FONT_WEIGHT_BLACK: DWRITE_FONT_WEIGHT = 900;
pub const DWRITE_FONT_WEIGHT_BOLD: DWRITE_FONT_WEIGHT = 700;
pub const DWRITE_FONT_WEIGHT_DEMI_BOLD: DWRITE_FONT_WEIGHT = 600;
pub const DWRITE_FONT_WEIGHT_EXTRA_BLACK: DWRITE_FONT_WEIGHT = 950;
pub const DWRITE_FONT_WEIGHT_EXTRA_BOLD: DWRITE_FONT_WEIGHT = 800;
pub const DWRITE_FONT_WEIGHT_EXTRA_LIGHT: DWRITE_FONT_WEIGHT = 200;
pub const DWRITE_FONT_WEIGHT_HEAVY: DWRITE_FONT_WEIGHT = 900;
pub const DWRITE_FONT_WEIGHT_LIGHT: DWRITE_FONT_WEIGHT = 300;
pub const DWRITE_FONT_WEIGHT_MEDIUM: DWRITE_FONT_WEIGHT = 500;
pub const DWRITE_FONT_WEIGHT_NORMAL: DWRITE_FONT_WEIGHT = 400;
pub const DWRITE_FONT_WEIGHT_REGULAR: DWRITE_FONT_WEIGHT = 400;
pub const DWRITE_FONT_WEIGHT_SEMI_BOLD: DWRITE_FONT_WEIGHT = 600;
pub const DWRITE_FONT_WEIGHT_SEMI_LIGHT: DWRITE_FONT_WEIGHT = 350;
pub const DWRITE_FONT_WEIGHT_THIN: DWRITE_FONT_WEIGHT = 100;
pub const DWRITE_FONT_WEIGHT_ULTRA_BLACK: DWRITE_FONT_WEIGHT = 950;
pub const DWRITE_FONT_WEIGHT_ULTRA_BOLD: DWRITE_FONT_WEIGHT = 800;
pub const DWRITE_FONT_WEIGHT_ULTRA_LIGHT: DWRITE_FONT_WEIGHT = 200;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_OFFSET {
    pub advanceOffset: f32,
    pub ascenderOffset: f32,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DWRITE_GLYPH_RUN {
    pub fontFace: core::mem::ManuallyDrop<Option<IDWriteFontFace>>,
    pub fontEmSize: f32,
    pub glyphCount: u32,
    pub glyphIndices: *mut u16,
    pub glyphAdvances: *mut f32,
    pub glyphOffsets: *mut DWRITE_GLYPH_OFFSET,
    pub isSideways: windows_core::BOOL,
    pub bidiLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_GLYPH_RUN_DESCRIPTION {
    pub localeName: windows_core::PWSTR,
    pub string: windows_core::PWSTR,
    pub stringLength: u32,
    pub clusterMap: *mut u16,
    pub textPosition: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidiLevel: u32,
    pub isText: windows_core::BOOL,
    pub isTrimmed: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_LINE_METRICS {
    pub length: u32,
    pub trailingWhitespaceLength: u32,
    pub newlineLength: u32,
    pub height: f32,
    pub baseline: f32,
    pub isTrimmed: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
pub type DWRITE_MEASURING_MODE = i32;
pub type DWRITE_NUMBER_SUBSTITUTION_METHOD = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_OVERHANG_METRICS {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type DWRITE_PIXEL_GEOMETRY = i32;
pub type DWRITE_RENDERING_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_METRICS {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub widthIncludingTrailingWhitespace: f32,
    pub height: f32,
    pub layoutWidth: f32,
    pub layoutHeight: f32,
    pub maxBidiReorderingDepth: u32,
    pub lineCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DWRITE_TEXT_RANGE {
    pub startPosition: u32,
    pub length: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DXGI_ADAPTER_DESC {
    pub Description: [u16; 128],
    pub VendorId: u32,
    pub DeviceId: u32,
    pub SubSysId: u32,
    pub Revision: u32,
    pub DedicatedVideoMemory: usize,
    pub DedicatedSystemMemory: usize,
    pub SharedSystemMemory: usize,
    pub AdapterLuid: LUID,
}
impl Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_ALPHA_MODE = i32;
pub const DXGI_ALPHA_MODE_IGNORE: DXGI_ALPHA_MODE = 3;
pub const DXGI_ALPHA_MODE_PREMULTIPLIED: DXGI_ALPHA_MODE = 1;
pub const DXGI_ALPHA_MODE_STRAIGHT: DXGI_ALPHA_MODE = 2;
pub const DXGI_ALPHA_MODE_UNSPECIFIED: DXGI_ALPHA_MODE = 0;
pub const DXGI_ERROR_DEVICE_REMOVED: windows_core::HRESULT =
    windows_core::HRESULT(0x887A0005_u32 as _);
pub const DXGI_ERROR_DEVICE_RESET: windows_core::HRESULT =
    windows_core::HRESULT(0x887A0007_u32 as _);
pub type DXGI_FORMAT = i32;
pub const DXGI_FORMAT_420_OPAQUE: DXGI_FORMAT = 106;
pub const DXGI_FORMAT_A4B4G4R4_UNORM: DXGI_FORMAT = 191;
pub const DXGI_FORMAT_A8P8: DXGI_FORMAT = 114;
pub const DXGI_FORMAT_A8_UNORM: DXGI_FORMAT = 65;
pub const DXGI_FORMAT_AI44: DXGI_FORMAT = 111;
pub const DXGI_FORMAT_AYUV: DXGI_FORMAT = 100;
pub const DXGI_FORMAT_B4G4R4A4_UNORM: DXGI_FORMAT = 115;
pub const DXGI_FORMAT_B5G5R5A1_UNORM: DXGI_FORMAT = 86;
pub const DXGI_FORMAT_B5G6R5_UNORM: DXGI_FORMAT = 85;
pub const DXGI_FORMAT_B8G8R8A8_TYPELESS: DXGI_FORMAT = 90;
pub const DXGI_FORMAT_B8G8R8A8_UNORM: DXGI_FORMAT = 87;
pub const DXGI_FORMAT_B8G8R8A8_UNORM_SRGB: DXGI_FORMAT = 91;
pub const DXGI_FORMAT_B8G8R8X8_TYPELESS: DXGI_FORMAT = 92;
pub const DXGI_FORMAT_B8G8R8X8_UNORM: DXGI_FORMAT = 88;
pub const DXGI_FORMAT_B8G8R8X8_UNORM_SRGB: DXGI_FORMAT = 93;
pub const DXGI_FORMAT_BC1_TYPELESS: DXGI_FORMAT = 70;
pub const DXGI_FORMAT_BC1_UNORM: DXGI_FORMAT = 71;
pub const DXGI_FORMAT_BC1_UNORM_SRGB: DXGI_FORMAT = 72;
pub const DXGI_FORMAT_BC2_TYPELESS: DXGI_FORMAT = 73;
pub const DXGI_FORMAT_BC2_UNORM: DXGI_FORMAT = 74;
pub const DXGI_FORMAT_BC2_UNORM_SRGB: DXGI_FORMAT = 75;
pub const DXGI_FORMAT_BC3_TYPELESS: DXGI_FORMAT = 76;
pub const DXGI_FORMAT_BC3_UNORM: DXGI_FORMAT = 77;
pub const DXGI_FORMAT_BC3_UNORM_SRGB: DXGI_FORMAT = 78;
pub const DXGI_FORMAT_BC4_SNORM: DXGI_FORMAT = 81;
pub const DXGI_FORMAT_BC4_TYPELESS: DXGI_FORMAT = 79;
pub const DXGI_FORMAT_BC4_UNORM: DXGI_FORMAT = 80;
pub const DXGI_FORMAT_BC5_SNORM: DXGI_FORMAT = 84;
pub const DXGI_FORMAT_BC5_TYPELESS: DXGI_FORMAT = 82;
pub const DXGI_FORMAT_BC5_UNORM: DXGI_FORMAT = 83;
pub const DXGI_FORMAT_BC6H_SF16: DXGI_FORMAT = 96;
pub const DXGI_FORMAT_BC6H_TYPELESS: DXGI_FORMAT = 94;
pub const DXGI_FORMAT_BC6H_UF16: DXGI_FORMAT = 95;
pub const DXGI_FORMAT_BC7_TYPELESS: DXGI_FORMAT = 97;
pub const DXGI_FORMAT_BC7_UNORM: DXGI_FORMAT = 98;
pub const DXGI_FORMAT_BC7_UNORM_SRGB: DXGI_FORMAT = 99;
pub const DXGI_FORMAT_D16_UNORM: DXGI_FORMAT = 55;
pub const DXGI_FORMAT_D24_UNORM_S8_UINT: DXGI_FORMAT = 45;
pub const DXGI_FORMAT_D32_FLOAT: DXGI_FORMAT = 40;
pub const DXGI_FORMAT_D32_FLOAT_S8X24_UINT: DXGI_FORMAT = 20;
pub const DXGI_FORMAT_G8R8_G8B8_UNORM: DXGI_FORMAT = 69;
pub const DXGI_FORMAT_IA44: DXGI_FORMAT = 112;
pub const DXGI_FORMAT_NV11: DXGI_FORMAT = 110;
pub const DXGI_FORMAT_NV12: DXGI_FORMAT = 103;
pub const DXGI_FORMAT_P010: DXGI_FORMAT = 104;
pub const DXGI_FORMAT_P016: DXGI_FORMAT = 105;
pub const DXGI_FORMAT_P208: DXGI_FORMAT = 130;
pub const DXGI_FORMAT_P8: DXGI_FORMAT = 113;
pub const DXGI_FORMAT_R10G10B10A2_TYPELESS: DXGI_FORMAT = 23;
pub const DXGI_FORMAT_R10G10B10A2_UINT: DXGI_FORMAT = 25;
pub const DXGI_FORMAT_R10G10B10A2_UNORM: DXGI_FORMAT = 24;
pub const DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM: DXGI_FORMAT = 89;
pub const DXGI_FORMAT_R11G11B10_FLOAT: DXGI_FORMAT = 26;
pub const DXGI_FORMAT_R16G16B16A16_FLOAT: DXGI_FORMAT = 10;
pub const DXGI_FORMAT_R16G16B16A16_SINT: DXGI_FORMAT = 14;
pub const DXGI_FORMAT_R16G16B16A16_SNORM: DXGI_FORMAT = 13;
pub const DXGI_FORMAT_R16G16B16A16_TYPELESS: DXGI_FORMAT = 9;
pub const DXGI_FORMAT_R16G16B16A16_UINT: DXGI_FORMAT = 12;
pub const DXGI_FORMAT_R16G16B16A16_UNORM: DXGI_FORMAT = 11;
pub const DXGI_FORMAT_R16G16_FLOAT: DXGI_FORMAT = 34;
pub const DXGI_FORMAT_R16G16_SINT: DXGI_FORMAT = 38;
pub const DXGI_FORMAT_R16G16_SNORM: DXGI_FORMAT = 37;
pub const DXGI_FORMAT_R16G16_TYPELESS: DXGI_FORMAT = 33;
pub const DXGI_FORMAT_R16G16_UINT: DXGI_FORMAT = 36;
pub const DXGI_FORMAT_R16G16_UNORM: DXGI_FORMAT = 35;
pub const DXGI_FORMAT_R16_FLOAT: DXGI_FORMAT = 54;
pub const DXGI_FORMAT_R16_SINT: DXGI_FORMAT = 59;
pub const DXGI_FORMAT_R16_SNORM: DXGI_FORMAT = 58;
pub const DXGI_FORMAT_R16_TYPELESS: DXGI_FORMAT = 53;
pub const DXGI_FORMAT_R16_UINT: DXGI_FORMAT = 57;
pub const DXGI_FORMAT_R16_UNORM: DXGI_FORMAT = 56;
pub const DXGI_FORMAT_R1_UNORM: DXGI_FORMAT = 66;
pub const DXGI_FORMAT_R24G8_TYPELESS: DXGI_FORMAT = 44;
pub const DXGI_FORMAT_R24_UNORM_X8_TYPELESS: DXGI_FORMAT = 46;
pub const DXGI_FORMAT_R32G32B32A32_FLOAT: DXGI_FORMAT = 2;
pub const DXGI_FORMAT_R32G32B32A32_SINT: DXGI_FORMAT = 4;
pub const DXGI_FORMAT_R32G32B32A32_TYPELESS: DXGI_FORMAT = 1;
pub const DXGI_FORMAT_R32G32B32A32_UINT: DXGI_FORMAT = 3;
pub const DXGI_FORMAT_R32G32B32_FLOAT: DXGI_FORMAT = 6;
pub const DXGI_FORMAT_R32G32B32_SINT: DXGI_FORMAT = 8;
pub const DXGI_FORMAT_R32G32B32_TYPELESS: DXGI_FORMAT = 5;
pub const DXGI_FORMAT_R32G32B32_UINT: DXGI_FORMAT = 7;
pub const DXGI_FORMAT_R32G32_FLOAT: DXGI_FORMAT = 16;
pub const DXGI_FORMAT_R32G32_SINT: DXGI_FORMAT = 18;
pub const DXGI_FORMAT_R32G32_TYPELESS: DXGI_FORMAT = 15;
pub const DXGI_FORMAT_R32G32_UINT: DXGI_FORMAT = 17;
pub const DXGI_FORMAT_R32G8X24_TYPELESS: DXGI_FORMAT = 19;
pub const DXGI_FORMAT_R32_FLOAT: DXGI_FORMAT = 41;
pub const DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS: DXGI_FORMAT = 21;
pub const DXGI_FORMAT_R32_SINT: DXGI_FORMAT = 43;
pub const DXGI_FORMAT_R32_TYPELESS: DXGI_FORMAT = 39;
pub const DXGI_FORMAT_R32_UINT: DXGI_FORMAT = 42;
pub const DXGI_FORMAT_R8G8B8A8_SINT: DXGI_FORMAT = 32;
pub const DXGI_FORMAT_R8G8B8A8_SNORM: DXGI_FORMAT = 31;
pub const DXGI_FORMAT_R8G8B8A8_TYPELESS: DXGI_FORMAT = 27;
pub const DXGI_FORMAT_R8G8B8A8_UINT: DXGI_FORMAT = 30;
pub const DXGI_FORMAT_R8G8B8A8_UNORM: DXGI_FORMAT = 28;
pub const DXGI_FORMAT_R8G8B8A8_UNORM_SRGB: DXGI_FORMAT = 29;
pub const DXGI_FORMAT_R8G8_B8G8_UNORM: DXGI_FORMAT = 68;
pub const DXGI_FORMAT_R8G8_SINT: DXGI_FORMAT = 52;
pub const DXGI_FORMAT_R8G8_SNORM: DXGI_FORMAT = 51;
pub const DXGI_FORMAT_R8G8_TYPELESS: DXGI_FORMAT = 48;
pub const DXGI_FORMAT_R8G8_UINT: DXGI_FORMAT = 50;
pub const DXGI_FORMAT_R8G8_UNORM: DXGI_FORMAT = 49;
pub const DXGI_FORMAT_R8_SINT: DXGI_FORMAT = 64;
pub const DXGI_FORMAT_R8_SNORM: DXGI_FORMAT = 63;
pub const DXGI_FORMAT_R8_TYPELESS: DXGI_FORMAT = 60;
pub const DXGI_FORMAT_R8_UINT: DXGI_FORMAT = 62;
pub const DXGI_FORMAT_R8_UNORM: DXGI_FORMAT = 61;
pub const DXGI_FORMAT_R9G9B9E5_SHAREDEXP: DXGI_FORMAT = 67;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE: DXGI_FORMAT = 189;
pub const DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE: DXGI_FORMAT = 190;
pub const DXGI_FORMAT_UNKNOWN: DXGI_FORMAT = 0;
pub const DXGI_FORMAT_V208: DXGI_FORMAT = 131;
pub const DXGI_FORMAT_V408: DXGI_FORMAT = 132;
pub const DXGI_FORMAT_X24_TYPELESS_G8_UINT: DXGI_FORMAT = 47;
pub const DXGI_FORMAT_X32_TYPELESS_G8X24_UINT: DXGI_FORMAT = 22;
pub const DXGI_FORMAT_Y210: DXGI_FORMAT = 108;
pub const DXGI_FORMAT_Y216: DXGI_FORMAT = 109;
pub const DXGI_FORMAT_Y410: DXGI_FORMAT = 101;
pub const DXGI_FORMAT_Y416: DXGI_FORMAT = 102;
pub const DXGI_FORMAT_YUY2: DXGI_FORMAT = 107;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_FRAME_STATISTICS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_MAPPED_RECT {
    pub Pitch: i32,
    pub pBits: *mut u8,
}
pub type DXGI_MAP_FLAGS = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_MODE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub RefreshRate: DXGI_RATIONAL,
    pub Format: DXGI_FORMAT,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
}
pub type DXGI_MODE_ROTATION = i32;
pub type DXGI_MODE_SCALING = i32;
pub type DXGI_MODE_SCANLINE_ORDER = i32;
pub type DXGI_PRESENT = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_PRESENT_PARAMETERS {
    pub DirtyRectsCount: u32,
    pub pDirtyRects: *mut RECT,
    pub pScrollRect: *mut RECT,
    pub pScrollOffset: *mut POINT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub type DXGI_RESIDENCY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SAMPLE_DESC {
    pub Count: u32,
    pub Quality: u32,
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SHARED_RESOURCE {
    pub Handle: HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SURFACE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub SampleDesc: DXGI_SAMPLE_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC {
    pub BufferDesc: DXGI_MODE_DESC,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub OutputWindow: HWND,
    pub Windowed: windows_core::BOOL,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub Flags: DXGI_SWAP_CHAIN_FLAG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: DXGI_FORMAT,
    pub Stereo: windows_core::BOOL,
    pub SampleDesc: DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: DXGI_ALPHA_MODE,
    pub Flags: DXGI_SWAP_CHAIN_FLAG,
}
pub type DXGI_SWAP_CHAIN_FLAG = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    pub RefreshRate: DXGI_RATIONAL,
    pub ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER,
    pub Scaling: DXGI_MODE_SCALING,
    pub Windowed: windows_core::BOOL,
}
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
pub type DXGI_USAGE = u32;
pub const DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = 64;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = 512;
pub const DXGI_USAGE_READ_ONLY: DXGI_USAGE = 256;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = 32;
pub const DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = 16;
pub const DXGI_USAGE_SHARED: DXGI_USAGE = 128;
pub const DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = 1024;
pub const ERROR_CANCELLED: WIN32_ERROR = 1223;
pub const ERROR_MOD_NOT_FOUND: WIN32_ERROR = 126;
pub const E_FAIL: windows_core::HRESULT = windows_core::HRESULT(0x80004005_u32 as _);
pub const E_INVALIDARG: windows_core::HRESULT = windows_core::HRESULT(0x80070057_u32 as _);
pub const E_NOINTERFACE: windows_core::HRESULT = windows_core::HRESULT(0x80004002_u32 as _);
pub const E_POINTER: windows_core::HRESULT = windows_core::HRESULT(0x80004003_u32 as _);
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
pub type FDAP = i32;
pub type FILEOPENDIALOGOPTIONS = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
pub const FOS_ALLNONSTORAGEITEMS: FILEOPENDIALOGOPTIONS = 128;
pub const FOS_ALLOWMULTISELECT: FILEOPENDIALOGOPTIONS = 512;
pub const FOS_CREATEPROMPT: FILEOPENDIALOGOPTIONS = 8192;
pub const FOS_DEFAULTNOMINIMODE: FILEOPENDIALOGOPTIONS = 536870912;
pub const FOS_DONTADDTORECENT: FILEOPENDIALOGOPTIONS = 33554432;
pub const FOS_FILEMUSTEXIST: FILEOPENDIALOGOPTIONS = 4096;
pub const FOS_FORCEFILESYSTEM: FILEOPENDIALOGOPTIONS = 64;
pub const FOS_FORCEPREVIEWPANEON: FILEOPENDIALOGOPTIONS = 1073741824;
pub const FOS_FORCESHOWHIDDEN: FILEOPENDIALOGOPTIONS = 268435456;
pub const FOS_HIDEMRUPLACES: FILEOPENDIALOGOPTIONS = 131072;
pub const FOS_HIDEPINNEDPLACES: FILEOPENDIALOGOPTIONS = 262144;
pub const FOS_NOCHANGEDIR: FILEOPENDIALOGOPTIONS = 8;
pub const FOS_NODEREFERENCELINKS: FILEOPENDIALOGOPTIONS = 1048576;
pub const FOS_NOREADONLYRETURN: FILEOPENDIALOGOPTIONS = 32768;
pub const FOS_NOTESTFILECREATE: FILEOPENDIALOGOPTIONS = 65536;
pub const FOS_NOVALIDATE: FILEOPENDIALOGOPTIONS = 256;
pub const FOS_OKBUTTONNEEDSINTERACTION: FILEOPENDIALOGOPTIONS = 2097152;
pub const FOS_OVERWRITEPROMPT: FILEOPENDIALOGOPTIONS = 2;
pub const FOS_PATHMUSTEXIST: FILEOPENDIALOGOPTIONS = 2048;
pub const FOS_PICKFOLDERS: FILEOPENDIALOGOPTIONS = 32;
pub const FOS_SHAREAWARE: FILEOPENDIALOGOPTIONS = 16384;
pub const FOS_STRICTFILETYPES: FILEOPENDIALOGOPTIONS = 4;
pub const FOS_SUPPORTSTREAMABLEITEMS: FILEOPENDIALOGOPTIONS = 2147483648;
pub const FileOpenDialog: windows_core::GUID =
    windows_core::GUID::from_u128(0xdc1c5a9c_e88a_4dde_a5a1_60f82a20aef7);
pub const FileSaveDialog: windows_core::GUID =
    windows_core::GUID::from_u128(0xc0b4e2f3_ba21_4773_8dba_335ec946eb8b);
pub type GETPROPERTYSTOREFLAGS = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HANDLE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMODULE(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HMONITOR(pub *mut core::ffi::c_void);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HWND(pub *mut core::ffi::c_void);
windows_core::imp::define_interface!(
    IBindCtx,
    IBindCtx_Vtbl,
    0x0000000e_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IBindCtx, windows_core::IUnknown);
#[repr(C)]
pub struct IBindCtx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    RegisterObjectBound: usize,
    RevokeObjectBound: usize,
    ReleaseBoundObjects: usize,
    SetBindOptions: usize,
    GetBindOptions: usize,
    GetRunningObjectTable: usize,
    RegisterObjectParam: usize,
    GetObjectParam: usize,
    EnumObjectParam: usize,
    RevokeObjectParam: usize,
}
impl windows_core::RuntimeName for IBindCtx {}
windows_core::imp::define_interface!(
    ID2D1Bitmap,
    ID2D1Bitmap_Vtbl,
    0xa2296057_ea42_4099_983b_539fb6505426
);
impl core::ops::Deref for ID2D1Bitmap {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1Bitmap,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Image
);
impl ID2D1Bitmap {
    pub unsafe fn GetSize(&self) -> D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn GetPixelSize(&self) -> D2D_SIZE_U {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn GetPixelFormat(&self) -> D2D1_PIXEL_FORMAT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDpi)(
                windows_core::Interface::as_raw(self),
                dpix as _,
                dpiy as _,
            );
        }
    }
    pub unsafe fn CopyFromBitmap<P1>(
        &self,
        destpoint: Option<*const D2D_POINT_2U>,
        bitmap: P1,
        srcrect: Option<*const D2D_RECT_U>,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyFromBitmap)(
                windows_core::Interface::as_raw(self),
                destpoint.unwrap_or(core::mem::zeroed()) as _,
                bitmap.param().abi(),
                srcrect.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CopyFromRenderTarget<P1>(
        &self,
        destpoint: Option<*const D2D_POINT_2U>,
        rendertarget: P1,
        srcrect: Option<*const D2D_RECT_U>,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ID2D1RenderTarget>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyFromRenderTarget)(
                windows_core::Interface::as_raw(self),
                destpoint.unwrap_or(core::mem::zeroed()) as _,
                rendertarget.param().abi(),
                srcrect.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CopyFromMemory(
        &self,
        dstrect: Option<*const D2D_RECT_U>,
        srcdata: *const core::ffi::c_void,
        pitch: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CopyFromMemory)(
                windows_core::Interface::as_raw(self),
                dstrect.unwrap_or(core::mem::zeroed()) as _,
                srcdata,
                pitch,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ID2D1Bitmap_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_SIZE_F),
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_SIZE_U),
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_PIXEL_FORMAT),
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    pub CopyFromBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_POINT_2U,
        *mut core::ffi::c_void,
        *const D2D_RECT_U,
    ) -> windows_core::HRESULT,
    pub CopyFromRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_POINT_2U,
        *mut core::ffi::c_void,
        *const D2D_RECT_U,
    ) -> windows_core::HRESULT,
    pub CopyFromMemory: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_U,
        *const core::ffi::c_void,
        u32,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1Bitmap {}
unsafe impl Sync for ID2D1Bitmap {}
impl windows_core::RuntimeName for ID2D1Bitmap {}
windows_core::imp::define_interface!(
    ID2D1Bitmap1,
    ID2D1Bitmap1_Vtbl,
    0xa898a84c_3873_4588_b08b_ebbf978df041
);
impl core::ops::Deref for ID2D1Bitmap1 {
    type Target = ID2D1Bitmap;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1Bitmap1,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Image,
    ID2D1Bitmap
);
impl ID2D1Bitmap1 {
    pub unsafe fn GetColorContext(&self) -> windows_core::Result<ID2D1ColorContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColorContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetOptions(&self) -> D2D1_BITMAP_OPTIONS {
        unsafe {
            (windows_core::Interface::vtable(self).GetOptions)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetSurface(&self) -> windows_core::Result<IDXGISurface> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurface)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Map(&self, options: D2D1_MAP_OPTIONS) -> windows_core::Result<D2D1_MAPPED_RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Map)(
                windows_core::Interface::as_raw(self),
                options,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct ID2D1Bitmap1_Vtbl {
    pub base__: ID2D1Bitmap_Vtbl,
    pub GetColorContext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetOptions: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_BITMAP_OPTIONS,
    pub GetSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Map: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D1_MAP_OPTIONS,
        *mut D2D1_MAPPED_RECT,
    ) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1Bitmap1 {}
unsafe impl Sync for ID2D1Bitmap1 {}
impl windows_core::RuntimeName for ID2D1Bitmap1 {}
windows_core::imp::define_interface!(
    ID2D1BitmapBrush,
    ID2D1BitmapBrush_Vtbl,
    0x2cd906aa_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1BitmapBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1BitmapBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1BitmapBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetExtendModeX: usize,
    SetExtendModeY: usize,
    SetInterpolationMode: usize,
    SetBitmap: usize,
    GetExtendModeX: usize,
    GetExtendModeY: usize,
    GetInterpolationMode: usize,
    GetBitmap: usize,
}
unsafe impl Send for ID2D1BitmapBrush {}
unsafe impl Sync for ID2D1BitmapBrush {}
impl windows_core::RuntimeName for ID2D1BitmapBrush {}
windows_core::imp::define_interface!(
    ID2D1BitmapBrush1,
    ID2D1BitmapBrush1_Vtbl,
    0x41343a53_e41a_49a2_91cd_21793bbb62e5
);
impl core::ops::Deref for ID2D1BitmapBrush1 {
    type Target = ID2D1BitmapBrush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1BitmapBrush1,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush,
    ID2D1BitmapBrush
);
#[repr(C)]
pub struct ID2D1BitmapBrush1_Vtbl {
    pub base__: ID2D1BitmapBrush_Vtbl,
    SetInterpolationMode1: usize,
    GetInterpolationMode1: usize,
}
unsafe impl Send for ID2D1BitmapBrush1 {}
unsafe impl Sync for ID2D1BitmapBrush1 {}
impl windows_core::RuntimeName for ID2D1BitmapBrush1 {}
windows_core::imp::define_interface!(
    ID2D1BitmapRenderTarget,
    ID2D1BitmapRenderTarget_Vtbl,
    0x2cd90695_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1BitmapRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1BitmapRenderTarget,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1RenderTarget
);
#[repr(C)]
pub struct ID2D1BitmapRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    GetBitmap: usize,
}
unsafe impl Send for ID2D1BitmapRenderTarget {}
unsafe impl Sync for ID2D1BitmapRenderTarget {}
impl windows_core::RuntimeName for ID2D1BitmapRenderTarget {}
windows_core::imp::define_interface!(
    ID2D1Brush,
    ID2D1Brush_Vtbl,
    0x2cd906a8_12e2_11dc_9fed_001143a055f9
);
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
            (windows_core::Interface::vtable(self).SetOpacity)(
                windows_core::Interface::as_raw(self),
                opacity,
            );
        }
    }
    pub unsafe fn SetTransform(&self, transform: *const D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(
                windows_core::Interface::as_raw(self),
                transform,
            );
        }
    }
    pub unsafe fn GetOpacity(&self) -> f32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetOpacity)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetTransform(&self, transform: *mut D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(
                windows_core::Interface::as_raw(self),
                transform as _,
            );
        }
    }
}
#[repr(C)]
pub struct ID2D1Brush_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void, f32),
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_MATRIX_3X2_F),
    pub GetOpacity: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_MATRIX_3X2_F),
}
unsafe impl Send for ID2D1Brush {}
unsafe impl Sync for ID2D1Brush {}
impl windows_core::RuntimeName for ID2D1Brush {}
windows_core::imp::define_interface!(
    ID2D1ColorContext,
    ID2D1ColorContext_Vtbl,
    0x1c4820bb_5771_4518_a581_2fe4dd0ec657
);
impl core::ops::Deref for ID2D1ColorContext {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1ColorContext, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1ColorContext_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetColorSpace: usize,
    GetProfileSize: usize,
    GetProfile: usize,
}
unsafe impl Send for ID2D1ColorContext {}
unsafe impl Sync for ID2D1ColorContext {}
impl windows_core::RuntimeName for ID2D1ColorContext {}
windows_core::imp::define_interface!(
    ID2D1CommandList,
    ID2D1CommandList_Vtbl,
    0xb4f34a19_2383_4d76_94f6_ec343657c3dc
);
impl core::ops::Deref for ID2D1CommandList {
    type Target = ID2D1Image;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1CommandList,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Image
);
#[repr(C)]
pub struct ID2D1CommandList_Vtbl {
    pub base__: ID2D1Image_Vtbl,
    Stream: usize,
    Close: usize,
}
unsafe impl Send for ID2D1CommandList {}
unsafe impl Sync for ID2D1CommandList {}
impl windows_core::RuntimeName for ID2D1CommandList {}
windows_core::imp::define_interface!(
    ID2D1DCRenderTarget,
    ID2D1DCRenderTarget_Vtbl,
    0x1c51bc64_de61_46fd_9899_63a5d8f03950
);
impl core::ops::Deref for ID2D1DCRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1DCRenderTarget,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1RenderTarget
);
#[repr(C)]
pub struct ID2D1DCRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    BindDC: usize,
}
unsafe impl Send for ID2D1DCRenderTarget {}
unsafe impl Sync for ID2D1DCRenderTarget {}
impl windows_core::RuntimeName for ID2D1DCRenderTarget {}
windows_core::imp::define_interface!(
    ID2D1Device,
    ID2D1Device_Vtbl,
    0x47dd575d_ac05_4cdd_8049_9b02cd16f44c
);
impl core::ops::Deref for ID2D1Device {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Device, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Device {
    pub unsafe fn CreateDeviceContext(
        &self,
        options: D2D1_DEVICE_CONTEXT_OPTIONS,
    ) -> windows_core::Result<ID2D1DeviceContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDeviceContext)(
                windows_core::Interface::as_raw(self),
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePrintControl<P0, P1>(
        &self,
        wicfactory: P0,
        documenttarget: P1,
        printcontrolproperties: Option<*const D2D1_PRINT_CONTROL_PROPERTIES>,
    ) -> windows_core::Result<ID2D1PrintControl>
    where
        P0: windows_core::Param<IWICImagingFactory>,
        P1: windows_core::Param<IPrintDocumentPackageTarget>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePrintControl)(
                windows_core::Interface::as_raw(self),
                wicfactory.param().abi(),
                documenttarget.param().abi(),
                printcontrolproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMaximumTextureMemory(&self, maximuminbytes: u64) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximumTextureMemory)(
                windows_core::Interface::as_raw(self),
                maximuminbytes,
            );
        }
    }
    pub unsafe fn GetMaximumTextureMemory(&self) -> u64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaximumTextureMemory)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn ClearResources(&self, millisecondssinceuse: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearResources)(
                windows_core::Interface::as_raw(self),
                millisecondssinceuse,
            );
        }
    }
}
#[repr(C)]
pub struct ID2D1Device_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub CreateDeviceContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D1_DEVICE_CONTEXT_OPTIONS,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePrintControl: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_PRINT_CONTROL_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
    pub GetMaximumTextureMemory: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub ClearResources: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
}
unsafe impl Send for ID2D1Device {}
unsafe impl Sync for ID2D1Device {}
impl windows_core::RuntimeName for ID2D1Device {}
windows_core::imp::define_interface!(
    ID2D1Device1,
    ID2D1Device1_Vtbl,
    0xd21768e1_23a4_4823_a14b_7c3eba85d658
);
impl core::ops::Deref for ID2D1Device1 {
    type Target = ID2D1Device;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1Device1,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Device
);
#[repr(C)]
pub struct ID2D1Device1_Vtbl {
    pub base__: ID2D1Device_Vtbl,
    GetRenderingPriority: usize,
    SetRenderingPriority: usize,
    CreateDeviceContext: usize,
}
unsafe impl Send for ID2D1Device1 {}
unsafe impl Sync for ID2D1Device1 {}
impl windows_core::RuntimeName for ID2D1Device1 {}
windows_core::imp::define_interface!(
    ID2D1DeviceContext,
    ID2D1DeviceContext_Vtbl,
    0xe8f7fe7a_191c_466d_ad95_975678bda998
);
impl core::ops::Deref for ID2D1DeviceContext {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1DeviceContext,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1RenderTarget
);
impl ID2D1DeviceContext {
    pub unsafe fn CreateBitmap(
        &self,
        size: D2D_SIZE_U,
        sourcedata: Option<*const core::ffi::c_void>,
        pitch: u32,
        bitmapproperties: *const D2D1_BITMAP_PROPERTIES1,
    ) -> windows_core::Result<ID2D1Bitmap1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(
                windows_core::Interface::as_raw(self),
                size,
                sourcedata.unwrap_or(core::mem::zeroed()) as _,
                pitch,
                bitmapproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(
        &self,
        wicbitmapsource: P0,
        bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES1>,
    ) -> windows_core::Result<ID2D1Bitmap1>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromWicBitmap)(
                windows_core::Interface::as_raw(self),
                wicbitmapsource.param().abi(),
                bitmapproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContext(
        &self,
        space: D2D1_COLOR_SPACE,
        profile: Option<&[u8]>,
    ) -> windows_core::Result<ID2D1ColorContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContext)(
                windows_core::Interface::as_raw(self),
                space,
                profile.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                profile.map_or(0, |slice| slice.len().try_into().unwrap()),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContextFromFilename<P0>(
        &self,
        filename: P0,
    ) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContextFromFilename)(
                windows_core::Interface::as_raw(self),
                filename.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateColorContextFromWicColorContext<P0>(
        &self,
        wiccolorcontext: P0,
    ) -> windows_core::Result<ID2D1ColorContext>
    where
        P0: windows_core::Param<IWICColorContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateColorContextFromWicColorContext)(
                windows_core::Interface::as_raw(self),
                wiccolorcontext.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromDxgiSurface<P0>(
        &self,
        surface: P0,
        bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES1>,
    ) -> windows_core::Result<ID2D1Bitmap1>
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromDxgiSurface)(
                windows_core::Interface::as_raw(self),
                surface.param().abi(),
                bitmapproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEffect(
        &self,
        effectid: *const windows_core::GUID,
    ) -> windows_core::Result<ID2D1Effect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEffect)(
                windows_core::Interface::as_raw(self),
                effectid,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGradientStopCollection(
        &self,
        straightalphagradientstops: &[D2D1_GRADIENT_STOP],
        preinterpolationspace: D2D1_COLOR_SPACE,
        postinterpolationspace: D2D1_COLOR_SPACE,
        bufferprecision: D2D1_BUFFER_PRECISION,
        extendmode: D2D1_EXTEND_MODE,
        colorinterpolationmode: D2D1_COLOR_INTERPOLATION_MODE,
    ) -> windows_core::Result<ID2D1GradientStopCollection1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGradientStopCollection)(
                windows_core::Interface::as_raw(self),
                straightalphagradientstops.as_ptr(),
                straightalphagradientstops.len().try_into().unwrap(),
                preinterpolationspace,
                postinterpolationspace,
                bufferprecision,
                extendmode,
                colorinterpolationmode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateImageBrush<P0>(
        &self,
        image: P0,
        imagebrushproperties: *const D2D1_IMAGE_BRUSH_PROPERTIES,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
    ) -> windows_core::Result<ID2D1ImageBrush>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateImageBrush)(
                windows_core::Interface::as_raw(self),
                image.param().abi(),
                imagebrushproperties,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapBrush<P0>(
        &self,
        bitmap: P0,
        bitmapbrushproperties: Option<*const D2D1_BITMAP_BRUSH_PROPERTIES1>,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
    ) -> windows_core::Result<ID2D1BitmapBrush1>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapBrush)(
                windows_core::Interface::as_raw(self),
                bitmap.param().abi(),
                bitmapbrushproperties.unwrap_or(core::mem::zeroed()) as _,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCommandList(&self) -> windows_core::Result<ID2D1CommandList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCommandList)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsDxgiFormatSupported(&self, format: DXGI_FORMAT) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsDxgiFormatSupported)(
                windows_core::Interface::as_raw(self),
                format,
            )
        }
    }
    pub unsafe fn IsBufferPrecisionSupported(
        &self,
        bufferprecision: D2D1_BUFFER_PRECISION,
    ) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsBufferPrecisionSupported)(
                windows_core::Interface::as_raw(self),
                bufferprecision,
            )
        }
    }
    pub unsafe fn GetImageLocalBounds<P0>(&self, image: P0) -> windows_core::Result<D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageLocalBounds)(
                windows_core::Interface::as_raw(self),
                image.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetImageWorldBounds<P0>(&self, image: P0) -> windows_core::Result<D2D_RECT_F>
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageWorldBounds)(
                windows_core::Interface::as_raw(self),
                image.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetGlyphRunWorldBounds(
        &self,
        baselineorigin: D2D_POINT_2F,
        glyphrun: *const DWRITE_GLYPH_RUN,
        measuringmode: DWRITE_MEASURING_MODE,
    ) -> windows_core::Result<D2D_RECT_F> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlyphRunWorldBounds)(
                windows_core::Interface::as_raw(self),
                baselineorigin,
                glyphrun,
                measuringmode,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID2D1Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetTarget<P0>(&self, image: P0)
    where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTarget)(
                windows_core::Interface::as_raw(self),
                image.param().abi(),
            );
        }
    }
    pub unsafe fn GetTarget(&self) -> windows_core::Result<ID2D1Image> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTarget)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetRenderingControls(&self, renderingcontrols: *const D2D1_RENDERING_CONTROLS) {
        unsafe {
            (windows_core::Interface::vtable(self).SetRenderingControls)(
                windows_core::Interface::as_raw(self),
                renderingcontrols,
            );
        }
    }
    pub unsafe fn GetRenderingControls(&self) -> D2D1_RENDERING_CONTROLS {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderingControls)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn SetPrimitiveBlend(&self, primitiveblend: D2D1_PRIMITIVE_BLEND) {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrimitiveBlend)(
                windows_core::Interface::as_raw(self),
                primitiveblend,
            );
        }
    }
    pub unsafe fn GetPrimitiveBlend(&self) -> D2D1_PRIMITIVE_BLEND {
        unsafe {
            (windows_core::Interface::vtable(self).GetPrimitiveBlend)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn SetUnitMode(&self, unitmode: D2D1_UNIT_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnitMode)(
                windows_core::Interface::as_raw(self),
                unitmode,
            );
        }
    }
    pub unsafe fn GetUnitMode(&self) -> D2D1_UNIT_MODE {
        unsafe {
            (windows_core::Interface::vtable(self).GetUnitMode)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn DrawGlyphRun<P3>(
        &self,
        baselineorigin: D2D_POINT_2F,
        glyphrun: *const DWRITE_GLYPH_RUN,
        glyphrundescription: Option<*const DWRITE_GLYPH_RUN_DESCRIPTION>,
        foregroundbrush: P3,
        measuringmode: DWRITE_MEASURING_MODE,
    ) where
        P3: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGlyphRun)(
                windows_core::Interface::as_raw(self),
                baselineorigin,
                glyphrun,
                glyphrundescription.unwrap_or(core::mem::zeroed()) as _,
                foregroundbrush.param().abi(),
                measuringmode,
            );
        }
    }
    pub unsafe fn DrawImage<P0>(
        &self,
        image: P0,
        targetoffset: Option<*const D2D_POINT_2F>,
        imagerectangle: Option<*const D2D_RECT_F>,
        interpolationmode: D2D1_INTERPOLATION_MODE,
        compositemode: D2D1_COMPOSITE_MODE,
    ) where
        P0: windows_core::Param<ID2D1Image>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawImage)(
                windows_core::Interface::as_raw(self),
                image.param().abi(),
                targetoffset.unwrap_or(core::mem::zeroed()) as _,
                imagerectangle.unwrap_or(core::mem::zeroed()) as _,
                interpolationmode,
                compositemode,
            );
        }
    }
    pub unsafe fn DrawGdiMetafile<P0>(
        &self,
        gdimetafile: P0,
        targetoffset: Option<*const D2D_POINT_2F>,
    ) where
        P0: windows_core::Param<ID2D1GdiMetafile>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGdiMetafile)(
                windows_core::Interface::as_raw(self),
                gdimetafile.param().abi(),
                targetoffset.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DrawBitmap<P0>(
        &self,
        bitmap: P0,
        destinationrectangle: Option<*const D2D_RECT_F>,
        opacity: f32,
        interpolationmode: D2D1_INTERPOLATION_MODE,
        sourcerectangle: Option<*const D2D_RECT_F>,
        perspectivetransform: Option<*const D2D_MATRIX_4X4_F>,
    ) where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawBitmap)(
                windows_core::Interface::as_raw(self),
                bitmap.param().abi(),
                destinationrectangle.unwrap_or(core::mem::zeroed()) as _,
                opacity,
                interpolationmode,
                sourcerectangle.unwrap_or(core::mem::zeroed()) as _,
                perspectivetransform.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn PushLayer<P1>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS1, layer: P1)
    where
        P1: windows_core::Param<ID2D1Layer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PushLayer)(
                windows_core::Interface::as_raw(self),
                layerparameters,
                layer.param().abi(),
            );
        }
    }
    pub unsafe fn InvalidateEffectInputRectangle<P0>(
        &self,
        effect: P0,
        input: u32,
        inputrectangle: *const D2D_RECT_F,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InvalidateEffectInputRectangle)(
                windows_core::Interface::as_raw(self),
                effect.param().abi(),
                input,
                inputrectangle,
            )
            .ok()
        }
    }
    pub unsafe fn GetEffectInvalidRectangleCount<P0>(&self, effect: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEffectInvalidRectangleCount)(
                windows_core::Interface::as_raw(self),
                effect.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetEffectInvalidRectangles<P0>(
        &self,
        effect: P0,
        rectangles: *mut D2D_RECT_F,
        rectanglescount: u32,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetEffectInvalidRectangles)(
                windows_core::Interface::as_raw(self),
                effect.param().abi(),
                rectangles as _,
                rectanglescount,
            )
            .ok()
        }
    }
    pub unsafe fn GetEffectRequiredInputRectangles<P0>(
        &self,
        rendereffect: P0,
        renderimagerectangle: Option<*const D2D_RECT_F>,
        inputdescriptions: *const D2D1_EFFECT_INPUT_DESCRIPTION,
        requiredinputrects: *mut D2D_RECT_F,
        inputcount: u32,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1Effect>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetEffectRequiredInputRectangles)(
                windows_core::Interface::as_raw(self),
                rendereffect.param().abi(),
                renderimagerectangle.unwrap_or(core::mem::zeroed()) as _,
                inputdescriptions,
                requiredinputrects as _,
                inputcount,
            )
            .ok()
        }
    }
    pub unsafe fn FillOpacityMask<P0, P1>(
        &self,
        opacitymask: P0,
        brush: P1,
        destinationrectangle: Option<*const D2D_RECT_F>,
        sourcerectangle: Option<*const D2D_RECT_F>,
    ) where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillOpacityMask)(
                windows_core::Interface::as_raw(self),
                opacitymask.param().abi(),
                brush.param().abi(),
                destinationrectangle.unwrap_or(core::mem::zeroed()) as _,
                sourcerectangle.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
}
#[repr(C)]
pub struct ID2D1DeviceContext_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub CreateBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_SIZE_U,
        *const core::ffi::c_void,
        u32,
        *const D2D1_BITMAP_PROPERTIES1,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_PROPERTIES1,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateColorContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D1_COLOR_SPACE,
        *const u8,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateColorContextFromFilename: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateColorContextFromWicColorContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub CreateBitmapFromDxgiSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_PROPERTIES1,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGradientStopCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_GRADIENT_STOP,
        u32,
        D2D1_COLOR_SPACE,
        D2D1_COLOR_SPACE,
        D2D1_BUFFER_PRECISION,
        D2D1_EXTEND_MODE,
        D2D1_COLOR_INTERPOLATION_MODE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateImageBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_IMAGE_BRUSH_PROPERTIES,
        *const D2D1_BRUSH_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBitmapBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_BRUSH_PROPERTIES1,
        *const D2D1_BRUSH_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCommandList: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsDxgiFormatSupported:
        unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_FORMAT) -> windows_core::BOOL,
    pub IsBufferPrecisionSupported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D1_BUFFER_PRECISION,
    ) -> windows_core::BOOL,
    pub GetImageLocalBounds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut D2D_RECT_F,
    ) -> windows_core::HRESULT,
    pub GetImageWorldBounds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut D2D_RECT_F,
    ) -> windows_core::HRESULT,
    pub GetGlyphRunWorldBounds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_POINT_2F,
        *const DWRITE_GLYPH_RUN,
        DWRITE_MEASURING_MODE,
        *mut D2D_RECT_F,
    ) -> windows_core::HRESULT,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetRenderingControls:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_RENDERING_CONTROLS),
    pub GetRenderingControls:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_RENDERING_CONTROLS),
    pub SetPrimitiveBlend: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PRIMITIVE_BLEND),
    pub GetPrimitiveBlend:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_PRIMITIVE_BLEND,
    pub SetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_UNIT_MODE),
    pub GetUnitMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_UNIT_MODE,
    pub DrawGlyphRun: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_POINT_2F,
        *const DWRITE_GLYPH_RUN,
        *const DWRITE_GLYPH_RUN_DESCRIPTION,
        *mut core::ffi::c_void,
        DWRITE_MEASURING_MODE,
    ),
    pub DrawImage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_POINT_2F,
        *const D2D_RECT_F,
        D2D1_INTERPOLATION_MODE,
        D2D1_COMPOSITE_MODE,
    ),
    pub DrawGdiMetafile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_POINT_2F,
    ),
    pub DrawBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        f32,
        D2D1_INTERPOLATION_MODE,
        *const D2D_RECT_F,
        *const D2D_MATRIX_4X4_F,
    ),
    pub PushLayer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_LAYER_PARAMETERS1,
        *mut core::ffi::c_void,
    ),
    pub InvalidateEffectInputRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const D2D_RECT_F,
    ) -> windows_core::HRESULT,
    pub GetEffectInvalidRectangleCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetEffectInvalidRectangles: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut D2D_RECT_F,
        u32,
    ) -> windows_core::HRESULT,
    pub GetEffectRequiredInputRectangles: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *const D2D1_EFFECT_INPUT_DESCRIPTION,
        *mut D2D_RECT_F,
        u32,
    ) -> windows_core::HRESULT,
    pub FillOpacityMask: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *const D2D_RECT_F,
    ),
}
unsafe impl Send for ID2D1DeviceContext {}
unsafe impl Sync for ID2D1DeviceContext {}
impl windows_core::RuntimeName for ID2D1DeviceContext {}
windows_core::imp::define_interface!(
    ID2D1DrawingStateBlock,
    ID2D1DrawingStateBlock_Vtbl,
    0x28506e39_ebf6_46a1_bb47_fd85565ab957
);
impl core::ops::Deref for ID2D1DrawingStateBlock {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1DrawingStateBlock,
    windows_core::IUnknown,
    ID2D1Resource
);
#[repr(C)]
pub struct ID2D1DrawingStateBlock_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetDescription: usize,
    SetDescription: usize,
    SetTextRenderingParams: usize,
    GetTextRenderingParams: usize,
}
unsafe impl Send for ID2D1DrawingStateBlock {}
unsafe impl Sync for ID2D1DrawingStateBlock {}
impl windows_core::RuntimeName for ID2D1DrawingStateBlock {}
windows_core::imp::define_interface!(
    ID2D1Effect,
    ID2D1Effect_Vtbl,
    0x28211a43_7d89_476f_8181_2d6159b220ad
);
impl core::ops::Deref for ID2D1Effect {
    type Target = ID2D1Properties;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Effect, windows_core::IUnknown, ID2D1Properties);
#[repr(C)]
pub struct ID2D1Effect_Vtbl {
    pub base__: ID2D1Properties_Vtbl,
    SetInput: usize,
    SetInputCount: usize,
    GetInput: usize,
    GetInputCount: usize,
    GetOutput: usize,
}
unsafe impl Send for ID2D1Effect {}
unsafe impl Sync for ID2D1Effect {}
impl windows_core::RuntimeName for ID2D1Effect {}
windows_core::imp::define_interface!(
    ID2D1EllipseGeometry,
    ID2D1EllipseGeometry_Vtbl,
    0x2cd906a4_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1EllipseGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1EllipseGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1EllipseGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetEllipse: usize,
}
unsafe impl Send for ID2D1EllipseGeometry {}
unsafe impl Sync for ID2D1EllipseGeometry {}
impl windows_core::RuntimeName for ID2D1EllipseGeometry {}
windows_core::imp::define_interface!(
    ID2D1Factory,
    ID2D1Factory_Vtbl,
    0x06152247_6f50_465a_9245_118bfd3b6007
);
windows_core::imp::interface_hierarchy!(ID2D1Factory, windows_core::IUnknown);
impl ID2D1Factory {
    pub unsafe fn ReloadSystemMetrics(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ReloadSystemMetrics)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDesktopDpi)(
                windows_core::Interface::as_raw(self),
                dpix as _,
                dpiy as _,
            );
        }
    }
    pub unsafe fn CreateRectangleGeometry(
        &self,
        rectangle: *const D2D_RECT_F,
    ) -> windows_core::Result<ID2D1RectangleGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRectangleGeometry)(
                windows_core::Interface::as_raw(self),
                rectangle,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRoundedRectangleGeometry(
        &self,
        roundedrectangle: *const D2D1_ROUNDED_RECT,
    ) -> windows_core::Result<ID2D1RoundedRectangleGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRoundedRectangleGeometry)(
                windows_core::Interface::as_raw(self),
                roundedrectangle,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEllipseGeometry(
        &self,
        ellipse: *const D2D1_ELLIPSE,
    ) -> windows_core::Result<ID2D1EllipseGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipseGeometry)(
                windows_core::Interface::as_raw(self),
                ellipse,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGeometryGroup(
        &self,
        fillmode: D2D1_FILL_MODE,
        geometries: &[Option<ID2D1Geometry>],
    ) -> windows_core::Result<ID2D1GeometryGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGeometryGroup)(
                windows_core::Interface::as_raw(self),
                fillmode,
                core::mem::transmute(geometries.as_ptr()),
                geometries.len().try_into().unwrap(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTransformedGeometry<P0>(
        &self,
        sourcegeometry: P0,
        transform: *const D2D_MATRIX_3X2_F,
    ) -> windows_core::Result<ID2D1TransformedGeometry>
    where
        P0: windows_core::Param<ID2D1Geometry>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransformedGeometry)(
                windows_core::Interface::as_raw(self),
                sourcegeometry.param().abi(),
                transform,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePathGeometry)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateStrokeStyle(
        &self,
        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
        dashes: Option<&[f32]>,
    ) -> windows_core::Result<ID2D1StrokeStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStrokeStyle)(
                windows_core::Interface::as_raw(self),
                strokestyleproperties,
                dashes.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                dashes.map_or(0, |slice| slice.len().try_into().unwrap()),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDrawingStateBlock<P1>(
        &self,
        drawingstatedescription: Option<*const D2D1_DRAWING_STATE_DESCRIPTION>,
        textrenderingparams: P1,
    ) -> windows_core::Result<ID2D1DrawingStateBlock>
    where
        P1: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDrawingStateBlock)(
                windows_core::Interface::as_raw(self),
                drawingstatedescription.unwrap_or(core::mem::zeroed()) as _,
                textrenderingparams.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateWicBitmapRenderTarget<P0>(
        &self,
        target: P0,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<IWICBitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateWicBitmapRenderTarget)(
                windows_core::Interface::as_raw(self),
                target.param().abi(),
                rendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateHwndRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1HwndRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateHwndRenderTarget)(
                windows_core::Interface::as_raw(self),
                rendertargetproperties,
                hwndrendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDxgiSurfaceRenderTarget<P0>(
        &self,
        dxgisurface: P0,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>
    where
        P0: windows_core::Param<IDXGISurface>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDxgiSurfaceRenderTarget)(
                windows_core::Interface::as_raw(self),
                dxgisurface.param().abi(),
                rendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateDCRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1DCRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDCRenderTarget)(
                windows_core::Interface::as_raw(self),
                rendertargetproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ID2D1Factory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReloadSystemMetrics:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDesktopDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    pub CreateRectangleGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRoundedRectangleGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ROUNDED_RECT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateEllipseGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ELLIPSE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGeometryGroup: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D1_FILL_MODE,
        *const *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTransformedGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePathGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateStrokeStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_STROKE_STYLE_PROPERTIES,
        *const f32,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDrawingStateBlock: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_DRAWING_STATE_DESCRIPTION,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateWicBitmapRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateHwndRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDxgiSurfaceRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDCRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1Factory {}
unsafe impl Sync for ID2D1Factory {}
pub trait ID2D1Factory_Impl: windows_core::IUnknownImpl {
    fn ReloadSystemMetrics(&self) -> windows_core::Result<()>;
    fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32);
    fn CreateRectangleGeometry(
        &self,
        rectangle: *const D2D_RECT_F,
    ) -> windows_core::Result<ID2D1RectangleGeometry>;
    fn CreateRoundedRectangleGeometry(
        &self,
        roundedrectangle: *const D2D1_ROUNDED_RECT,
    ) -> windows_core::Result<ID2D1RoundedRectangleGeometry>;
    fn CreateEllipseGeometry(
        &self,
        ellipse: *const D2D1_ELLIPSE,
    ) -> windows_core::Result<ID2D1EllipseGeometry>;
    fn CreateGeometryGroup(
        &self,
        fillmode: D2D1_FILL_MODE,
        geometries: *const Option<ID2D1Geometry>,
        geometriescount: u32,
    ) -> windows_core::Result<ID2D1GeometryGroup>;
    fn CreateTransformedGeometry(
        &self,
        sourcegeometry: windows_core::Ref<ID2D1Geometry>,
        transform: *const D2D_MATRIX_3X2_F,
    ) -> windows_core::Result<ID2D1TransformedGeometry>;
    fn CreatePathGeometry(&self) -> windows_core::Result<ID2D1PathGeometry>;
    fn CreateStrokeStyle(
        &self,
        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
        dashes: *const f32,
        dashescount: u32,
    ) -> windows_core::Result<ID2D1StrokeStyle>;
    fn CreateDrawingStateBlock(
        &self,
        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
        textrenderingparams: windows_core::Ref<IDWriteRenderingParams>,
    ) -> windows_core::Result<ID2D1DrawingStateBlock>;
    fn CreateWicBitmapRenderTarget(
        &self,
        target: windows_core::Ref<IWICBitmap>,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateHwndRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1HwndRenderTarget>;
    fn CreateDxgiSurfaceRenderTarget(
        &self,
        dxgisurface: windows_core::Ref<IDXGISurface>,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1RenderTarget>;
    fn CreateDCRenderTarget(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::Result<ID2D1DCRenderTarget>;
}
impl ID2D1Factory_Vtbl {
    pub const fn new<Identity: ID2D1Factory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReloadSystemMetrics<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory_Impl::ReloadSystemMetrics(this).into()
            }
        }
        unsafe extern "system" fn GetDesktopDpi<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dpix: *mut f32,
            dpiy: *mut f32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1Factory_Impl::GetDesktopDpi(
                    this,
                    core::mem::transmute_copy(&dpix),
                    core::mem::transmute_copy(&dpiy),
                );
            }
        }
        unsafe extern "system" fn CreateRectangleGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rectangle: *const D2D_RECT_F,
            rectanglegeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateRectangleGeometry(
                    this,
                    core::mem::transmute_copy(&rectangle),
                ) {
                    Ok(ok__) => {
                        rectanglegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRoundedRectangleGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            roundedrectangle: *const D2D1_ROUNDED_RECT,
            roundedrectanglegeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateRoundedRectangleGeometry(
                    this,
                    core::mem::transmute_copy(&roundedrectangle),
                ) {
                    Ok(ok__) => {
                        roundedrectanglegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEllipseGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ellipse: *const D2D1_ELLIPSE,
            ellipsegeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateEllipseGeometry(
                    this,
                    core::mem::transmute_copy(&ellipse),
                ) {
                    Ok(ok__) => {
                        ellipsegeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGeometryGroup<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fillmode: D2D1_FILL_MODE,
            geometries: *const *mut core::ffi::c_void,
            geometriescount: u32,
            geometrygroup: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateGeometryGroup(
                    this,
                    core::mem::transmute_copy(&fillmode),
                    core::mem::transmute_copy(&geometries),
                    core::mem::transmute_copy(&geometriescount),
                ) {
                    Ok(ok__) => {
                        geometrygroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTransformedGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            sourcegeometry: *mut core::ffi::c_void,
            transform: *const D2D_MATRIX_3X2_F,
            transformedgeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateTransformedGeometry(
                    this,
                    core::mem::transmute_copy(&sourcegeometry),
                    core::mem::transmute_copy(&transform),
                ) {
                    Ok(ok__) => {
                        transformedgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePathGeometry<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pathgeometry: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreatePathGeometry(this) {
                    Ok(ok__) => {
                        pathgeometry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStrokeStyle<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
            dashes: *const f32,
            dashescount: u32,
            strokestyle: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateStrokeStyle(
                    this,
                    core::mem::transmute_copy(&strokestyleproperties),
                    core::mem::transmute_copy(&dashes),
                    core::mem::transmute_copy(&dashescount),
                ) {
                    Ok(ok__) => {
                        strokestyle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDrawingStateBlock<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
            textrenderingparams: *mut core::ffi::c_void,
            drawingstateblock: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDrawingStateBlock(
                    this,
                    core::mem::transmute_copy(&drawingstatedescription),
                    core::mem::transmute_copy(&textrenderingparams),
                ) {
                    Ok(ok__) => {
                        drawingstateblock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateWicBitmapRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            target: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            rendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateWicBitmapRenderTarget(
                    this,
                    core::mem::transmute_copy(&target),
                    core::mem::transmute_copy(&rendertargetproperties),
                ) {
                    Ok(ok__) => {
                        rendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateHwndRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
            hwndrendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateHwndRenderTarget(
                    this,
                    core::mem::transmute_copy(&rendertargetproperties),
                    core::mem::transmute_copy(&hwndrendertargetproperties),
                ) {
                    Ok(ok__) => {
                        hwndrendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDxgiSurfaceRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dxgisurface: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            rendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDxgiSurfaceRenderTarget(
                    this,
                    core::mem::transmute_copy(&dxgisurface),
                    core::mem::transmute_copy(&rendertargetproperties),
                ) {
                    Ok(ok__) => {
                        rendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateDCRenderTarget<
            Identity: ID2D1Factory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
            dcrendertarget: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID2D1Factory_Impl::CreateDCRenderTarget(
                    this,
                    core::mem::transmute_copy(&rendertargetproperties),
                ) {
                    Ok(ok__) => {
                        dcrendertarget.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReloadSystemMetrics: ReloadSystemMetrics::<Identity, OFFSET>,
            GetDesktopDpi: GetDesktopDpi::<Identity, OFFSET>,
            CreateRectangleGeometry: CreateRectangleGeometry::<Identity, OFFSET>,
            CreateRoundedRectangleGeometry: CreateRoundedRectangleGeometry::<Identity, OFFSET>,
            CreateEllipseGeometry: CreateEllipseGeometry::<Identity, OFFSET>,
            CreateGeometryGroup: CreateGeometryGroup::<Identity, OFFSET>,
            CreateTransformedGeometry: CreateTransformedGeometry::<Identity, OFFSET>,
            CreatePathGeometry: CreatePathGeometry::<Identity, OFFSET>,
            CreateStrokeStyle: CreateStrokeStyle::<Identity, OFFSET>,
            CreateDrawingStateBlock: CreateDrawingStateBlock::<Identity, OFFSET>,
            CreateWicBitmapRenderTarget: CreateWicBitmapRenderTarget::<Identity, OFFSET>,
            CreateHwndRenderTarget: CreateHwndRenderTarget::<Identity, OFFSET>,
            CreateDxgiSurfaceRenderTarget: CreateDxgiSurfaceRenderTarget::<Identity, OFFSET>,
            CreateDCRenderTarget: CreateDCRenderTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1Factory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1Factory {}
windows_core::imp::define_interface!(
    ID2D1Factory1,
    ID2D1Factory1_Vtbl,
    0xbb12d362_daee_4b9a_aa1d_14ba401cfa1f
);
impl core::ops::Deref for ID2D1Factory1 {
    type Target = ID2D1Factory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Factory1, windows_core::IUnknown, ID2D1Factory);
#[repr(C)]
pub struct ID2D1Factory1_Vtbl {
    pub base__: ID2D1Factory_Vtbl,
    CreateDevice: usize,
    CreateStrokeStyle: usize,
    CreatePathGeometry: usize,
    CreateDrawingStateBlock: usize,
    CreateGdiMetafile: usize,
    RegisterEffectFromStream: usize,
    RegisterEffectFromString: usize,
    UnregisterEffect: usize,
    GetRegisteredEffects: usize,
    GetEffectProperties: usize,
}
unsafe impl Send for ID2D1Factory1 {}
unsafe impl Sync for ID2D1Factory1 {}
impl windows_core::RuntimeName for ID2D1Factory1 {}
windows_core::imp::define_interface!(
    ID2D1Factory2,
    ID2D1Factory2_Vtbl,
    0x94f81a73_9212_4376_9c58_b16a3a0d3992
);
impl core::ops::Deref for ID2D1Factory2 {
    type Target = ID2D1Factory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1Factory2,
    windows_core::IUnknown,
    ID2D1Factory,
    ID2D1Factory1
);
impl ID2D1Factory2 {
    pub unsafe fn CreateDevice<P0>(&self, dxgidevice: P0) -> windows_core::Result<ID2D1Device1>
    where
        P0: windows_core::Param<IDXGIDevice>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateDevice)(
                windows_core::Interface::as_raw(self),
                dxgidevice.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct ID2D1Factory2_Vtbl {
    pub base__: ID2D1Factory1_Vtbl,
    pub CreateDevice: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1Factory2 {}
unsafe impl Sync for ID2D1Factory2 {}
impl windows_core::RuntimeName for ID2D1Factory2 {}
windows_core::imp::define_interface!(
    ID2D1GdiMetafile,
    ID2D1GdiMetafile_Vtbl,
    0x2f543dc3_cfc1_4211_864f_cfd91c6f3395
);
impl core::ops::Deref for ID2D1GdiMetafile {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1GdiMetafile, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1GdiMetafile_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    Stream: usize,
    GetBounds: usize,
}
unsafe impl Send for ID2D1GdiMetafile {}
unsafe impl Sync for ID2D1GdiMetafile {}
impl windows_core::RuntimeName for ID2D1GdiMetafile {}
windows_core::imp::define_interface!(
    ID2D1Geometry,
    ID2D1Geometry_Vtbl,
    0x2cd906a1_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1Geometry {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Geometry, windows_core::IUnknown, ID2D1Resource);
impl ID2D1Geometry {
    pub unsafe fn GetBounds(
        &self,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
    ) -> windows_core::Result<D2D_RECT_F> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBounds)(
                windows_core::Interface::as_raw(self),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetWidenedBounds<P1>(
        &self,
        strokewidth: f32,
        strokestyle: P1,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
    ) -> windows_core::Result<D2D_RECT_F>
    where
        P1: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWidenedBounds)(
                windows_core::Interface::as_raw(self),
                strokewidth,
                strokestyle.param().abi(),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn StrokeContainsPoint<P2>(
        &self,
        point: D2D_POINT_2F,
        strokewidth: f32,
        strokestyle: P2,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
    ) -> windows_core::Result<windows_core::BOOL>
    where
        P2: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrokeContainsPoint)(
                windows_core::Interface::as_raw(self),
                point,
                strokewidth,
                strokestyle.param().abi(),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn FillContainsPoint(
        &self,
        point: D2D_POINT_2F,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillContainsPoint)(
                windows_core::Interface::as_raw(self),
                point,
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CompareWithGeometry<P0>(
        &self,
        inputgeometry: P0,
        inputgeometrytransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
    ) -> windows_core::Result<D2D1_GEOMETRY_RELATION>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareWithGeometry)(
                windows_core::Interface::as_raw(self),
                inputgeometry.param().abi(),
                inputgeometrytransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Simplify<P3>(
        &self,
        simplificationoption: D2D1_GEOMETRY_SIMPLIFICATION_OPTION,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
        geometrysink: P3,
    ) -> windows_core::Result<()>
    where
        P3: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Simplify)(
                windows_core::Interface::as_raw(self),
                simplificationoption,
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                geometrysink.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn Tessellate<P2>(
        &self,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
        tessellationsink: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID2D1TessellationSink>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Tessellate)(
                windows_core::Interface::as_raw(self),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                tessellationsink.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn CombineWithGeometry<P0, P4>(
        &self,
        inputgeometry: P0,
        combinemode: D2D1_COMBINE_MODE,
        inputgeometrytransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
        geometrysink: P4,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
        P4: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CombineWithGeometry)(
                windows_core::Interface::as_raw(self),
                inputgeometry.param().abi(),
                combinemode,
                inputgeometrytransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                geometrysink.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn Outline<P2>(
        &self,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
        geometrysink: P2,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Outline)(
                windows_core::Interface::as_raw(self),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                geometrysink.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn ComputeArea(
        &self,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
    ) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeArea)(
                windows_core::Interface::as_raw(self),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ComputeLength(
        &self,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
    ) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComputeLength)(
                windows_core::Interface::as_raw(self),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ComputePointAtLength(
        &self,
        length: f32,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
        point: Option<*mut D2D_POINT_2F>,
        unittangentvector: Option<*mut D2D_POINT_2F>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ComputePointAtLength)(
                windows_core::Interface::as_raw(self),
                length,
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                point.unwrap_or(core::mem::zeroed()) as _,
                unittangentvector.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn Widen<P1, P4>(
        &self,
        strokewidth: f32,
        strokestyle: P1,
        worldtransform: Option<*const D2D_MATRIX_3X2_F>,
        flatteningtolerance: f32,
        geometrysink: P4,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<ID2D1StrokeStyle>,
        P4: windows_core::Param<ID2D1SimplifiedGeometrySink>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Widen)(
                windows_core::Interface::as_raw(self),
                strokewidth,
                strokestyle.param().abi(),
                worldtransform.unwrap_or(core::mem::zeroed()) as _,
                flatteningtolerance,
                geometrysink.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ID2D1Geometry_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub GetBounds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        *mut D2D_RECT_F,
    ) -> windows_core::HRESULT,
    pub GetWidenedBounds: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut D2D_RECT_F,
    ) -> windows_core::HRESULT,
    pub StrokeContainsPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_POINT_2F,
        f32,
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub FillContainsPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_POINT_2F,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CompareWithGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut D2D1_GEOMETRY_RELATION,
    ) -> windows_core::HRESULT,
    pub Simplify: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D1_GEOMETRY_SIMPLIFICATION_OPTION,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Tessellate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CombineWithGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D2D1_COMBINE_MODE,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Outline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub ComputeArea: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut f32,
    ) -> windows_core::HRESULT,
    pub ComputeLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut f32,
    ) -> windows_core::HRESULT,
    pub ComputePointAtLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut D2D_POINT_2F,
        *mut D2D_POINT_2F,
    ) -> windows_core::HRESULT,
    pub Widen: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
        *const D2D_MATRIX_3X2_F,
        f32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1Geometry {}
unsafe impl Sync for ID2D1Geometry {}
impl windows_core::RuntimeName for ID2D1Geometry {}
windows_core::imp::define_interface!(
    ID2D1GeometryGroup,
    ID2D1GeometryGroup_Vtbl,
    0x2cd906a6_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1GeometryGroup {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1GeometryGroup,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1GeometryGroup_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetFillMode: usize,
    GetSourceGeometryCount: usize,
    GetSourceGeometries: usize,
}
unsafe impl Send for ID2D1GeometryGroup {}
unsafe impl Sync for ID2D1GeometryGroup {}
impl windows_core::RuntimeName for ID2D1GeometryGroup {}
windows_core::imp::define_interface!(
    ID2D1GeometrySink,
    ID2D1GeometrySink_Vtbl,
    0x2cd9069f_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1GeometrySink {
    type Target = ID2D1SimplifiedGeometrySink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1GeometrySink,
    windows_core::IUnknown,
    ID2D1SimplifiedGeometrySink
);
impl ID2D1GeometrySink {
    pub unsafe fn AddLine(&self, point: D2D_POINT_2F) {
        unsafe {
            (windows_core::Interface::vtable(self).AddLine)(
                windows_core::Interface::as_raw(self),
                point,
            );
        }
    }
    pub unsafe fn AddBezier(&self, bezier: *const D2D1_BEZIER_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddBezier)(
                windows_core::Interface::as_raw(self),
                bezier,
            );
        }
    }
    pub unsafe fn AddQuadraticBezier(&self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddQuadraticBezier)(
                windows_core::Interface::as_raw(self),
                bezier,
            );
        }
    }
    pub unsafe fn AddQuadraticBeziers(&self, beziers: &[D2D1_QUADRATIC_BEZIER_SEGMENT]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddQuadraticBeziers)(
                windows_core::Interface::as_raw(self),
                beziers.as_ptr(),
                beziers.len().try_into().unwrap(),
            );
        }
    }
    pub unsafe fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).AddArc)(
                windows_core::Interface::as_raw(self),
                arc,
            );
        }
    }
}
#[repr(C)]
pub struct ID2D1GeometrySink_Vtbl {
    pub base__: ID2D1SimplifiedGeometrySink_Vtbl,
    pub AddLine: unsafe extern "system" fn(*mut core::ffi::c_void, D2D_POINT_2F),
    pub AddBezier: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BEZIER_SEGMENT),
    pub AddQuadraticBezier:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_QUADRATIC_BEZIER_SEGMENT),
    pub AddQuadraticBeziers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_QUADRATIC_BEZIER_SEGMENT,
        u32,
    ),
    pub AddArc: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_ARC_SEGMENT),
}
unsafe impl Send for ID2D1GeometrySink {}
unsafe impl Sync for ID2D1GeometrySink {}
pub trait ID2D1GeometrySink_Impl: ID2D1SimplifiedGeometrySink_Impl {
    fn AddLine(&self, point: &D2D_POINT_2F);
    fn AddBezier(&self, bezier: *const D2D1_BEZIER_SEGMENT);
    fn AddQuadraticBezier(&self, bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT);
    fn AddQuadraticBeziers(&self, beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT, bezierscount: u32);
    fn AddArc(&self, arc: *const D2D1_ARC_SEGMENT);
}
impl ID2D1GeometrySink_Vtbl {
    pub const fn new<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddLine<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            point: D2D_POINT_2F,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddLine(this, core::mem::transmute(&point));
            }
        }
        unsafe extern "system" fn AddBezier<
            Identity: ID2D1GeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bezier: *const D2D1_BEZIER_SEGMENT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddBezier(this, core::mem::transmute_copy(&bezier));
            }
        }
        unsafe extern "system" fn AddQuadraticBezier<
            Identity: ID2D1GeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            bezier: *const D2D1_QUADRATIC_BEZIER_SEGMENT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddQuadraticBezier(
                    this,
                    core::mem::transmute_copy(&bezier),
                );
            }
        }
        unsafe extern "system" fn AddQuadraticBeziers<
            Identity: ID2D1GeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            beziers: *const D2D1_QUADRATIC_BEZIER_SEGMENT,
            bezierscount: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddQuadraticBeziers(
                    this,
                    core::mem::transmute_copy(&beziers),
                    core::mem::transmute_copy(&bezierscount),
                );
            }
        }
        unsafe extern "system" fn AddArc<Identity: ID2D1GeometrySink_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            arc: *const D2D1_ARC_SEGMENT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1GeometrySink_Impl::AddArc(this, core::mem::transmute_copy(&arc));
            }
        }
        Self {
            base__: ID2D1SimplifiedGeometrySink_Vtbl::new::<Identity, OFFSET>(),
            AddLine: AddLine::<Identity, OFFSET>,
            AddBezier: AddBezier::<Identity, OFFSET>,
            AddQuadraticBezier: AddQuadraticBezier::<Identity, OFFSET>,
            AddQuadraticBeziers: AddQuadraticBeziers::<Identity, OFFSET>,
            AddArc: AddArc::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1GeometrySink as windows_core::Interface>::IID
            || iid == &<ID2D1SimplifiedGeometrySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1GeometrySink {}
windows_core::imp::define_interface!(
    ID2D1GradientStopCollection,
    ID2D1GradientStopCollection_Vtbl,
    0x2cd906a7_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1GradientStopCollection {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1GradientStopCollection,
    windows_core::IUnknown,
    ID2D1Resource
);
#[repr(C)]
pub struct ID2D1GradientStopCollection_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetGradientStopCount: usize,
    GetGradientStops: usize,
    GetColorInterpolationGamma: usize,
    GetExtendMode: usize,
}
unsafe impl Send for ID2D1GradientStopCollection {}
unsafe impl Sync for ID2D1GradientStopCollection {}
impl windows_core::RuntimeName for ID2D1GradientStopCollection {}
windows_core::imp::define_interface!(
    ID2D1GradientStopCollection1,
    ID2D1GradientStopCollection1_Vtbl,
    0xae1572f4_5dd0_4777_998b_9279472ae63b
);
impl core::ops::Deref for ID2D1GradientStopCollection1 {
    type Target = ID2D1GradientStopCollection;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1GradientStopCollection1,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1GradientStopCollection
);
#[repr(C)]
pub struct ID2D1GradientStopCollection1_Vtbl {
    pub base__: ID2D1GradientStopCollection_Vtbl,
    GetGradientStops1: usize,
    GetPreInterpolationSpace: usize,
    GetPostInterpolationSpace: usize,
    GetBufferPrecision: usize,
    GetColorInterpolationMode: usize,
}
unsafe impl Send for ID2D1GradientStopCollection1 {}
unsafe impl Sync for ID2D1GradientStopCollection1 {}
impl windows_core::RuntimeName for ID2D1GradientStopCollection1 {}
windows_core::imp::define_interface!(
    ID2D1HwndRenderTarget,
    ID2D1HwndRenderTarget_Vtbl,
    0x2cd90698_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1HwndRenderTarget {
    type Target = ID2D1RenderTarget;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1HwndRenderTarget,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1RenderTarget
);
impl ID2D1HwndRenderTarget {
    pub unsafe fn CheckWindowState(&self) -> D2D1_WINDOW_STATE {
        unsafe {
            (windows_core::Interface::vtable(self).CheckWindowState)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn Resize(&self, pixelsize: *const D2D_SIZE_U) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Resize)(
                windows_core::Interface::as_raw(self),
                pixelsize,
            )
            .ok()
        }
    }
    pub unsafe fn GetHwnd(&self) -> HWND {
        unsafe {
            (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct ID2D1HwndRenderTarget_Vtbl {
    pub base__: ID2D1RenderTarget_Vtbl,
    pub CheckWindowState: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_WINDOW_STATE,
    pub Resize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_SIZE_U,
    ) -> windows_core::HRESULT,
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> HWND,
}
unsafe impl Send for ID2D1HwndRenderTarget {}
unsafe impl Sync for ID2D1HwndRenderTarget {}
impl windows_core::RuntimeName for ID2D1HwndRenderTarget {}
windows_core::imp::define_interface!(
    ID2D1Image,
    ID2D1Image_Vtbl,
    0x65019f75_8da2_497c_b32c_dfa34e48ede6
);
impl core::ops::Deref for ID2D1Image {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Image, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Image_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
}
unsafe impl Send for ID2D1Image {}
unsafe impl Sync for ID2D1Image {}
impl windows_core::RuntimeName for ID2D1Image {}
windows_core::imp::define_interface!(
    ID2D1ImageBrush,
    ID2D1ImageBrush_Vtbl,
    0xfe9e984d_3f95_407c_b5db_cb94d4e8f87c
);
impl core::ops::Deref for ID2D1ImageBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1ImageBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1ImageBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetImage: usize,
    SetExtendModeX: usize,
    SetExtendModeY: usize,
    SetInterpolationMode: usize,
    SetSourceRectangle: usize,
    GetImage: usize,
    GetExtendModeX: usize,
    GetExtendModeY: usize,
    GetInterpolationMode: usize,
    GetSourceRectangle: usize,
}
unsafe impl Send for ID2D1ImageBrush {}
unsafe impl Sync for ID2D1ImageBrush {}
impl windows_core::RuntimeName for ID2D1ImageBrush {}
windows_core::imp::define_interface!(
    ID2D1Layer,
    ID2D1Layer_Vtbl,
    0x2cd9069b_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1Layer {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Layer, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Layer_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetSize: usize,
}
unsafe impl Send for ID2D1Layer {}
unsafe impl Sync for ID2D1Layer {}
impl windows_core::RuntimeName for ID2D1Layer {}
windows_core::imp::define_interface!(
    ID2D1LinearGradientBrush,
    ID2D1LinearGradientBrush_Vtbl,
    0x2cd906ab_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1LinearGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1LinearGradientBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1LinearGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetStartPoint: usize,
    SetEndPoint: usize,
    GetStartPoint: usize,
    GetEndPoint: usize,
    GetGradientStopCollection: usize,
}
unsafe impl Send for ID2D1LinearGradientBrush {}
unsafe impl Sync for ID2D1LinearGradientBrush {}
impl windows_core::RuntimeName for ID2D1LinearGradientBrush {}
windows_core::imp::define_interface!(
    ID2D1Mesh,
    ID2D1Mesh_Vtbl,
    0x2cd906c2_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1Mesh {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1Mesh, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1Mesh_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    Open: usize,
}
unsafe impl Send for ID2D1Mesh {}
unsafe impl Sync for ID2D1Mesh {}
impl windows_core::RuntimeName for ID2D1Mesh {}
windows_core::imp::define_interface!(
    ID2D1PathGeometry,
    ID2D1PathGeometry_Vtbl,
    0x2cd906a5_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1PathGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1PathGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
impl ID2D1PathGeometry {
    pub unsafe fn Open(&self) -> windows_core::Result<ID2D1GeometrySink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Open)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Stream<P0>(&self, geometrysink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID2D1GeometrySink>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Stream)(
                windows_core::Interface::as_raw(self),
                geometrysink.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetSegmentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSegmentCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetFigureCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFigureCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct ID2D1PathGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    pub Open: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Stream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSegmentCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFigureCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1PathGeometry {}
unsafe impl Sync for ID2D1PathGeometry {}
impl windows_core::RuntimeName for ID2D1PathGeometry {}
windows_core::imp::define_interface!(
    ID2D1PrintControl,
    ID2D1PrintControl_Vtbl,
    0x2c1d867d_c290_41c8_ae7e_34a98702e9a5
);
windows_core::imp::interface_hierarchy!(ID2D1PrintControl, windows_core::IUnknown);
#[repr(C)]
pub struct ID2D1PrintControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AddPage: usize,
    Close: usize,
}
unsafe impl Send for ID2D1PrintControl {}
unsafe impl Sync for ID2D1PrintControl {}
impl windows_core::RuntimeName for ID2D1PrintControl {}
windows_core::imp::define_interface!(
    ID2D1Properties,
    ID2D1Properties_Vtbl,
    0x483473d7_cd46_4f9d_9d3a_3112aa80159d
);
windows_core::imp::interface_hierarchy!(ID2D1Properties, windows_core::IUnknown);
#[repr(C)]
pub struct ID2D1Properties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetPropertyCount: usize,
    GetPropertyName: usize,
    GetPropertyNameLength: usize,
    GetType: usize,
    GetPropertyIndex: usize,
    SetValueByName: usize,
    SetValue: usize,
    GetValueByName: usize,
    GetValue: usize,
    GetValueSize: usize,
    GetSubProperties: usize,
}
unsafe impl Send for ID2D1Properties {}
unsafe impl Sync for ID2D1Properties {}
impl windows_core::RuntimeName for ID2D1Properties {}
windows_core::imp::define_interface!(
    ID2D1RadialGradientBrush,
    ID2D1RadialGradientBrush_Vtbl,
    0x2cd906ac_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RadialGradientBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1RadialGradientBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1RadialGradientBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetCenter: usize,
    SetGradientOriginOffset: usize,
    SetRadiusX: usize,
    SetRadiusY: usize,
    GetCenter: usize,
    GetGradientOriginOffset: usize,
    GetRadiusX: usize,
    GetRadiusY: usize,
    GetGradientStopCollection: usize,
}
unsafe impl Send for ID2D1RadialGradientBrush {}
unsafe impl Sync for ID2D1RadialGradientBrush {}
impl windows_core::RuntimeName for ID2D1RadialGradientBrush {}
windows_core::imp::define_interface!(
    ID2D1RectangleGeometry,
    ID2D1RectangleGeometry_Vtbl,
    0x2cd906a2_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1RectangleGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1RectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetRect: usize,
}
unsafe impl Send for ID2D1RectangleGeometry {}
unsafe impl Sync for ID2D1RectangleGeometry {}
impl windows_core::RuntimeName for ID2D1RectangleGeometry {}
windows_core::imp::define_interface!(
    ID2D1RenderTarget,
    ID2D1RenderTarget_Vtbl,
    0x2cd90694_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RenderTarget {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1RenderTarget, windows_core::IUnknown, ID2D1Resource);
impl ID2D1RenderTarget {
    pub unsafe fn CreateBitmap(
        &self,
        size: D2D_SIZE_U,
        srcdata: Option<*const core::ffi::c_void>,
        pitch: u32,
        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
    ) -> windows_core::Result<ID2D1Bitmap> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmap)(
                windows_core::Interface::as_raw(self),
                size,
                srcdata.unwrap_or(core::mem::zeroed()) as _,
                pitch,
                bitmapproperties,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBitmapFromWicBitmap<P0>(
        &self,
        wicbitmapsource: P0,
        bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>,
    ) -> windows_core::Result<ID2D1Bitmap>
    where
        P0: windows_core::Param<IWICBitmapSource>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapFromWicBitmap)(
                windows_core::Interface::as_raw(self),
                wicbitmapsource.param().abi(),
                bitmapproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSharedBitmap(
        &self,
        riid: *const windows_core::GUID,
        data: *mut core::ffi::c_void,
        bitmapproperties: Option<*const D2D1_BITMAP_PROPERTIES>,
        bitmap: *mut Option<ID2D1Bitmap>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateSharedBitmap)(
                windows_core::Interface::as_raw(self),
                riid,
                data as _,
                bitmapproperties.unwrap_or(core::mem::zeroed()) as _,
                core::mem::transmute(bitmap),
            )
            .ok()
        }
    }
    pub unsafe fn CreateBitmapBrush<P0>(
        &self,
        bitmap: P0,
        bitmapbrushproperties: Option<*const D2D1_BITMAP_BRUSH_PROPERTIES>,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
    ) -> windows_core::Result<ID2D1BitmapBrush>
    where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateBitmapBrush)(
                windows_core::Interface::as_raw(self),
                bitmap.param().abi(),
                bitmapbrushproperties.unwrap_or(core::mem::zeroed()) as _,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSolidColorBrush(
        &self,
        color: *const D2D1_COLOR_F,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
    ) -> windows_core::Result<ID2D1SolidColorBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSolidColorBrush)(
                windows_core::Interface::as_raw(self),
                color,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGradientStopCollection(
        &self,
        gradientstops: &[D2D1_GRADIENT_STOP],
        colorinterpolationgamma: D2D1_GAMMA,
        extendmode: D2D1_EXTEND_MODE,
    ) -> windows_core::Result<ID2D1GradientStopCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGradientStopCollection)(
                windows_core::Interface::as_raw(self),
                gradientstops.as_ptr(),
                gradientstops.len().try_into().unwrap(),
                colorinterpolationgamma,
                extendmode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLinearGradientBrush<P2>(
        &self,
        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
        gradientstopcollection: P2,
    ) -> windows_core::Result<ID2D1LinearGradientBrush>
    where
        P2: windows_core::Param<ID2D1GradientStopCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLinearGradientBrush)(
                windows_core::Interface::as_raw(self),
                lineargradientbrushproperties,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                gradientstopcollection.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRadialGradientBrush<P2>(
        &self,
        radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
        brushproperties: Option<*const D2D1_BRUSH_PROPERTIES>,
        gradientstopcollection: P2,
    ) -> windows_core::Result<ID2D1RadialGradientBrush>
    where
        P2: windows_core::Param<ID2D1GradientStopCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRadialGradientBrush)(
                windows_core::Interface::as_raw(self),
                radialgradientbrushproperties,
                brushproperties.unwrap_or(core::mem::zeroed()) as _,
                gradientstopcollection.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCompatibleRenderTarget(
        &self,
        desiredsize: Option<*const D2D_SIZE_F>,
        desiredpixelsize: Option<*const D2D_SIZE_U>,
        desiredformat: Option<*const D2D1_PIXEL_FORMAT>,
        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
    ) -> windows_core::Result<ID2D1BitmapRenderTarget> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCompatibleRenderTarget)(
                windows_core::Interface::as_raw(self),
                desiredsize.unwrap_or(core::mem::zeroed()) as _,
                desiredpixelsize.unwrap_or(core::mem::zeroed()) as _,
                desiredformat.unwrap_or(core::mem::zeroed()) as _,
                options,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateLayer(
        &self,
        size: Option<*const D2D_SIZE_F>,
    ) -> windows_core::Result<ID2D1Layer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateLayer)(
                windows_core::Interface::as_raw(self),
                size.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMesh(&self) -> windows_core::Result<ID2D1Mesh> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMesh)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn DrawLine<P2, P4>(
        &self,
        point0: D2D_POINT_2F,
        point1: D2D_POINT_2F,
        brush: P2,
        strokewidth: f32,
        strokestyle: P4,
    ) where
        P2: windows_core::Param<ID2D1Brush>,
        P4: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawLine)(
                windows_core::Interface::as_raw(self),
                point0,
                point1,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn DrawRectangle<P1, P3>(
        &self,
        rect: *const D2D_RECT_F,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawRectangle)(
                windows_core::Interface::as_raw(self),
                rect,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillRectangle<P1>(&self, rect: *const D2D_RECT_F, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRectangle)(
                windows_core::Interface::as_raw(self),
                rect,
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn DrawRoundedRectangle<P1, P3>(
        &self,
        roundedrect: *const D2D1_ROUNDED_RECT,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawRoundedRectangle)(
                windows_core::Interface::as_raw(self),
                roundedrect,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillRoundedRectangle<P1>(&self, roundedrect: *const D2D1_ROUNDED_RECT, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillRoundedRectangle)(
                windows_core::Interface::as_raw(self),
                roundedrect,
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn DrawEllipse<P1, P3>(
        &self,
        ellipse: *const D2D1_ELLIPSE,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawEllipse)(
                windows_core::Interface::as_raw(self),
                ellipse,
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillEllipse<P1>(&self, ellipse: *const D2D1_ELLIPSE, brush: P1)
    where
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillEllipse)(
                windows_core::Interface::as_raw(self),
                ellipse,
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn DrawGeometry<P0, P1, P3>(
        &self,
        geometry: P0,
        brush: P1,
        strokewidth: f32,
        strokestyle: P3,
    ) where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P3: windows_core::Param<ID2D1StrokeStyle>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGeometry)(
                windows_core::Interface::as_raw(self),
                geometry.param().abi(),
                brush.param().abi(),
                strokewidth,
                strokestyle.param().abi(),
            );
        }
    }
    pub unsafe fn FillGeometry<P0, P1, P2>(&self, geometry: P0, brush: P1, opacitybrush: P2)
    where
        P0: windows_core::Param<ID2D1Geometry>,
        P1: windows_core::Param<ID2D1Brush>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillGeometry)(
                windows_core::Interface::as_raw(self),
                geometry.param().abi(),
                brush.param().abi(),
                opacitybrush.param().abi(),
            );
        }
    }
    pub unsafe fn FillMesh<P0, P1>(&self, mesh: P0, brush: P1)
    where
        P0: windows_core::Param<ID2D1Mesh>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillMesh)(
                windows_core::Interface::as_raw(self),
                mesh.param().abi(),
                brush.param().abi(),
            );
        }
    }
    pub unsafe fn FillOpacityMask<P0, P1>(
        &self,
        opacitymask: P0,
        brush: P1,
        content: D2D1_OPACITY_MASK_CONTENT,
        destinationrectangle: Option<*const D2D_RECT_F>,
        sourcerectangle: Option<*const D2D_RECT_F>,
    ) where
        P0: windows_core::Param<ID2D1Bitmap>,
        P1: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).FillOpacityMask)(
                windows_core::Interface::as_raw(self),
                opacitymask.param().abi(),
                brush.param().abi(),
                content,
                destinationrectangle.unwrap_or(core::mem::zeroed()) as _,
                sourcerectangle.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DrawBitmap<P0>(
        &self,
        bitmap: P0,
        destinationrectangle: Option<*const D2D_RECT_F>,
        opacity: f32,
        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
        sourcerectangle: Option<*const D2D_RECT_F>,
    ) where
        P0: windows_core::Param<ID2D1Bitmap>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawBitmap)(
                windows_core::Interface::as_raw(self),
                bitmap.param().abi(),
                destinationrectangle.unwrap_or(core::mem::zeroed()) as _,
                opacity,
                interpolationmode,
                sourcerectangle.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DrawText<P2, P4>(
        &self,
        string: &[u16],
        textformat: P2,
        layoutrect: *const D2D_RECT_F,
        defaultfillbrush: P4,
        options: D2D1_DRAW_TEXT_OPTIONS,
        measuringmode: DWRITE_MEASURING_MODE,
    ) where
        P2: windows_core::Param<IDWriteTextFormat>,
        P4: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawText)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(string.as_ptr()),
                string.len().try_into().unwrap(),
                textformat.param().abi(),
                layoutrect,
                defaultfillbrush.param().abi(),
                options,
                measuringmode,
            );
        }
    }
    pub unsafe fn DrawTextLayout<P1, P2>(
        &self,
        origin: D2D_POINT_2F,
        textlayout: P1,
        defaultfillbrush: P2,
        options: D2D1_DRAW_TEXT_OPTIONS,
    ) where
        P1: windows_core::Param<IDWriteTextLayout>,
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawTextLayout)(
                windows_core::Interface::as_raw(self),
                origin,
                textlayout.param().abi(),
                defaultfillbrush.param().abi(),
                options,
            );
        }
    }
    pub unsafe fn DrawGlyphRun<P2>(
        &self,
        baselineorigin: D2D_POINT_2F,
        glyphrun: *const DWRITE_GLYPH_RUN,
        foregroundbrush: P2,
        measuringmode: DWRITE_MEASURING_MODE,
    ) where
        P2: windows_core::Param<ID2D1Brush>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawGlyphRun)(
                windows_core::Interface::as_raw(self),
                baselineorigin,
                glyphrun,
                foregroundbrush.param().abi(),
                measuringmode,
            );
        }
    }
    pub unsafe fn SetTransform(&self, transform: *const D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTransform)(
                windows_core::Interface::as_raw(self),
                transform,
            );
        }
    }
    pub unsafe fn GetTransform(&self, transform: *mut D2D_MATRIX_3X2_F) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTransform)(
                windows_core::Interface::as_raw(self),
                transform as _,
            );
        }
    }
    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetAntialiasMode)(
                windows_core::Interface::as_raw(self),
                antialiasmode,
            );
        }
    }
    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
        unsafe {
            (windows_core::Interface::vtable(self).GetAntialiasMode)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn SetTextAntialiasMode(&self, textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextAntialiasMode)(
                windows_core::Interface::as_raw(self),
                textantialiasmode,
            );
        }
    }
    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
        unsafe {
            (windows_core::Interface::vtable(self).GetTextAntialiasMode)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn SetTextRenderingParams<P0>(&self, textrenderingparams: P0)
    where
        P0: windows_core::Param<IDWriteRenderingParams>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextRenderingParams)(
                windows_core::Interface::as_raw(self),
                textrenderingparams.param().abi(),
            );
        }
    }
    pub unsafe fn GetTextRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextRenderingParams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
        unsafe {
            (windows_core::Interface::vtable(self).SetTags)(
                windows_core::Interface::as_raw(self),
                tag1,
                tag2,
            );
        }
    }
    pub unsafe fn GetTags(&self, tag1: Option<*mut u64>, tag2: Option<*mut u64>) {
        unsafe {
            (windows_core::Interface::vtable(self).GetTags)(
                windows_core::Interface::as_raw(self),
                tag1.unwrap_or(core::mem::zeroed()) as _,
                tag2.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn PushLayer<P1>(&self, layerparameters: *const D2D1_LAYER_PARAMETERS, layer: P1)
    where
        P1: windows_core::Param<ID2D1Layer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PushLayer)(
                windows_core::Interface::as_raw(self),
                layerparameters,
                layer.param().abi(),
            );
        }
    }
    pub unsafe fn PopLayer(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopLayer)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Flush(
        &self,
        tag1: Option<*mut u64>,
        tag2: Option<*mut u64>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(
                windows_core::Interface::as_raw(self),
                tag1.unwrap_or(core::mem::zeroed()) as _,
                tag2.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SaveDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: windows_core::Param<ID2D1DrawingStateBlock>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SaveDrawingState)(
                windows_core::Interface::as_raw(self),
                drawingstateblock.param().abi(),
            );
        }
    }
    pub unsafe fn RestoreDrawingState<P0>(&self, drawingstateblock: P0)
    where
        P0: windows_core::Param<ID2D1DrawingStateBlock>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RestoreDrawingState)(
                windows_core::Interface::as_raw(self),
                drawingstateblock.param().abi(),
            );
        }
    }
    pub unsafe fn PushAxisAlignedClip(
        &self,
        cliprect: *const D2D_RECT_F,
        antialiasmode: D2D1_ANTIALIAS_MODE,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PushAxisAlignedClip)(
                windows_core::Interface::as_raw(self),
                cliprect,
                antialiasmode,
            );
        }
    }
    pub unsafe fn PopAxisAlignedClip(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PopAxisAlignedClip)(
                windows_core::Interface::as_raw(self),
            );
        }
    }
    pub unsafe fn Clear(&self, clearcolor: Option<*const D2D1_COLOR_F>) {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(
                windows_core::Interface::as_raw(self),
                clearcolor.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn BeginDraw(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginDraw)(windows_core::Interface::as_raw(
                self,
            ));
        }
    }
    pub unsafe fn EndDraw(
        &self,
        tag1: Option<*mut u64>,
        tag2: Option<*mut u64>,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).EndDraw)(
                windows_core::Interface::as_raw(self),
                tag1.unwrap_or(core::mem::zeroed()) as _,
                tag2.unwrap_or(core::mem::zeroed()) as _,
            )
        }
    }
    pub unsafe fn GetPixelFormat(&self) -> D2D1_PIXEL_FORMAT {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDpi)(
                windows_core::Interface::as_raw(self),
                dpix,
                dpiy,
            );
        }
    }
    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
        unsafe {
            (windows_core::Interface::vtable(self).GetDpi)(
                windows_core::Interface::as_raw(self),
                dpix as _,
                dpiy as _,
            );
        }
    }
    pub unsafe fn GetSize(&self) -> D2D_SIZE_F {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn GetPixelSize(&self) -> D2D_SIZE_U {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelSize)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaximumBitmapSize)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn IsSupported(
        &self,
        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsSupported)(
                windows_core::Interface::as_raw(self),
                rendertargetproperties,
            )
        }
    }
}
#[repr(C)]
pub struct ID2D1RenderTarget_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    pub CreateBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_SIZE_U,
        *const core::ffi::c_void,
        u32,
        *const D2D1_BITMAP_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBitmapFromWicBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSharedBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBitmapBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D1_BITMAP_BRUSH_PROPERTIES,
        *const D2D1_BRUSH_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSolidColorBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_COLOR_F,
        *const D2D1_BRUSH_PROPERTIES,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGradientStopCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_GRADIENT_STOP,
        u32,
        D2D1_GAMMA,
        D2D1_EXTEND_MODE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
        *const D2D1_BRUSH_PROPERTIES,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
        *const D2D1_BRUSH_PROPERTIES,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCompatibleRenderTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_SIZE_F,
        *const D2D_SIZE_U,
        *const D2D1_PIXEL_FORMAT,
        D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateLayer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_SIZE_F,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateMesh: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub DrawLine: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_POINT_2F,
        D2D_POINT_2F,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub DrawRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut core::ffi::c_void,
    ),
    pub DrawRoundedRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ROUNDED_RECT,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillRoundedRectangle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ROUNDED_RECT,
        *mut core::ffi::c_void,
    ),
    pub DrawEllipse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ELLIPSE,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillEllipse: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_ELLIPSE,
        *mut core::ffi::c_void,
    ),
    pub DrawGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        f32,
        *mut core::ffi::c_void,
    ),
    pub FillGeometry: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub FillMesh: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub FillOpacityMask: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D2D1_OPACITY_MASK_CONTENT,
        *const D2D_RECT_F,
        *const D2D_RECT_F,
    ),
    pub DrawBitmap: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        f32,
        D2D1_BITMAP_INTERPOLATION_MODE,
        *const D2D_RECT_F,
    ),
    pub DrawText: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
        *mut core::ffi::c_void,
        *const D2D_RECT_F,
        *mut core::ffi::c_void,
        D2D1_DRAW_TEXT_OPTIONS,
        DWRITE_MEASURING_MODE,
    ),
    pub DrawTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_POINT_2F,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        D2D1_DRAW_TEXT_OPTIONS,
    ),
    pub DrawGlyphRun: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D2D_POINT_2F,
        *const DWRITE_GLYPH_RUN,
        *mut core::ffi::c_void,
        DWRITE_MEASURING_MODE,
    ),
    pub SetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_MATRIX_3X2_F),
    pub GetTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_MATRIX_3X2_F),
    pub SetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_ANTIALIAS_MODE),
    pub GetAntialiasMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_ANTIALIAS_MODE,
    pub SetTextAntialiasMode:
        unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_TEXT_ANTIALIAS_MODE),
    pub GetTextAntialiasMode:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> D2D1_TEXT_ANTIALIAS_MODE,
    pub SetTextRenderingParams:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetTextRenderingParams:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetTags: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64),
    pub GetTags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64),
    pub PushLayer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_LAYER_PARAMETERS,
        *mut core::ffi::c_void,
    ),
    pub PopLayer: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u64,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub SaveDrawingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RestoreDrawingState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub PushAxisAlignedClip:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_RECT_F, D2D1_ANTIALIAS_MODE),
    pub PopAxisAlignedClip: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_COLOR_F),
    pub BeginDraw: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub EndDraw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u64,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D1_PIXEL_FORMAT),
    pub SetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32),
    pub GetDpi: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32),
    pub GetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_SIZE_F),
    pub GetPixelSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut D2D_SIZE_U),
    pub GetMaximumBitmapSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub IsSupported: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D2D1_RENDER_TARGET_PROPERTIES,
    ) -> windows_core::BOOL,
}
unsafe impl Send for ID2D1RenderTarget {}
unsafe impl Sync for ID2D1RenderTarget {}
impl windows_core::RuntimeName for ID2D1RenderTarget {}
windows_core::imp::define_interface!(
    ID2D1Resource,
    ID2D1Resource_Vtbl,
    0x2cd90691_12e2_11dc_9fed_001143a055f9
);
windows_core::imp::interface_hierarchy!(ID2D1Resource, windows_core::IUnknown);
#[repr(C)]
pub struct ID2D1Resource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetFactory: usize,
}
unsafe impl Send for ID2D1Resource {}
unsafe impl Sync for ID2D1Resource {}
impl windows_core::RuntimeName for ID2D1Resource {}
windows_core::imp::define_interface!(
    ID2D1RoundedRectangleGeometry,
    ID2D1RoundedRectangleGeometry_Vtbl,
    0x2cd906a3_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1RoundedRectangleGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1RoundedRectangleGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1RoundedRectangleGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetRoundedRect: usize,
}
unsafe impl Send for ID2D1RoundedRectangleGeometry {}
unsafe impl Sync for ID2D1RoundedRectangleGeometry {}
impl windows_core::RuntimeName for ID2D1RoundedRectangleGeometry {}
windows_core::imp::define_interface!(
    ID2D1SimplifiedGeometrySink,
    ID2D1SimplifiedGeometrySink_Vtbl,
    0x2cd9069e_12e2_11dc_9fed_001143a055f9
);
windows_core::imp::interface_hierarchy!(ID2D1SimplifiedGeometrySink, windows_core::IUnknown);
impl ID2D1SimplifiedGeometrySink {
    pub unsafe fn SetFillMode(&self, fillmode: D2D1_FILL_MODE) {
        unsafe {
            (windows_core::Interface::vtable(self).SetFillMode)(
                windows_core::Interface::as_raw(self),
                fillmode,
            );
        }
    }
    pub unsafe fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT) {
        unsafe {
            (windows_core::Interface::vtable(self).SetSegmentFlags)(
                windows_core::Interface::as_raw(self),
                vertexflags,
            );
        }
    }
    pub unsafe fn BeginFigure(&self, startpoint: D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN) {
        unsafe {
            (windows_core::Interface::vtable(self).BeginFigure)(
                windows_core::Interface::as_raw(self),
                startpoint,
                figurebegin,
            );
        }
    }
    pub unsafe fn AddLines(&self, points: &[D2D_POINT_2F]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddLines)(
                windows_core::Interface::as_raw(self),
                points.as_ptr(),
                points.len().try_into().unwrap(),
            );
        }
    }
    pub unsafe fn AddBeziers(&self, beziers: &[D2D1_BEZIER_SEGMENT]) {
        unsafe {
            (windows_core::Interface::vtable(self).AddBeziers)(
                windows_core::Interface::as_raw(self),
                beziers.as_ptr(),
                beziers.len().try_into().unwrap(),
            );
        }
    }
    pub unsafe fn EndFigure(&self, figureend: D2D1_FIGURE_END) {
        unsafe {
            (windows_core::Interface::vtable(self).EndFigure)(
                windows_core::Interface::as_raw(self),
                figureend,
            );
        }
    }
    pub unsafe fn Close(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct ID2D1SimplifiedGeometrySink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFillMode: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FILL_MODE),
    pub SetSegmentFlags: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_PATH_SEGMENT),
    pub BeginFigure:
        unsafe extern "system" fn(*mut core::ffi::c_void, D2D_POINT_2F, D2D1_FIGURE_BEGIN),
    pub AddLines: unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D_POINT_2F, u32),
    pub AddBeziers:
        unsafe extern "system" fn(*mut core::ffi::c_void, *const D2D1_BEZIER_SEGMENT, u32),
    pub EndFigure: unsafe extern "system" fn(*mut core::ffi::c_void, D2D1_FIGURE_END),
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID2D1SimplifiedGeometrySink {}
unsafe impl Sync for ID2D1SimplifiedGeometrySink {}
pub trait ID2D1SimplifiedGeometrySink_Impl: windows_core::IUnknownImpl {
    fn SetFillMode(&self, fillmode: D2D1_FILL_MODE);
    fn SetSegmentFlags(&self, vertexflags: D2D1_PATH_SEGMENT);
    fn BeginFigure(&self, startpoint: &D2D_POINT_2F, figurebegin: D2D1_FIGURE_BEGIN);
    fn AddLines(&self, points: *const D2D_POINT_2F, pointscount: u32);
    fn AddBeziers(&self, beziers: *const D2D1_BEZIER_SEGMENT, bezierscount: u32);
    fn EndFigure(&self, figureend: D2D1_FIGURE_END);
    fn Close(&self) -> windows_core::Result<()>;
}
impl ID2D1SimplifiedGeometrySink_Vtbl {
    pub const fn new<Identity: ID2D1SimplifiedGeometrySink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFillMode<
            Identity: ID2D1SimplifiedGeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fillmode: D2D1_FILL_MODE,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::SetFillMode(
                    this,
                    core::mem::transmute_copy(&fillmode),
                );
            }
        }
        unsafe extern "system" fn SetSegmentFlags<
            Identity: ID2D1SimplifiedGeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            vertexflags: D2D1_PATH_SEGMENT,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::SetSegmentFlags(
                    this,
                    core::mem::transmute_copy(&vertexflags),
                );
            }
        }
        unsafe extern "system" fn BeginFigure<
            Identity: ID2D1SimplifiedGeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            startpoint: D2D_POINT_2F,
            figurebegin: D2D1_FIGURE_BEGIN,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::BeginFigure(
                    this,
                    core::mem::transmute(&startpoint),
                    core::mem::transmute_copy(&figurebegin),
                );
            }
        }
        unsafe extern "system" fn AddLines<
            Identity: ID2D1SimplifiedGeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            points: *const D2D_POINT_2F,
            pointscount: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::AddLines(
                    this,
                    core::mem::transmute_copy(&points),
                    core::mem::transmute_copy(&pointscount),
                );
            }
        }
        unsafe extern "system" fn AddBeziers<
            Identity: ID2D1SimplifiedGeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            beziers: *const D2D1_BEZIER_SEGMENT,
            bezierscount: u32,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::AddBeziers(
                    this,
                    core::mem::transmute_copy(&beziers),
                    core::mem::transmute_copy(&bezierscount),
                );
            }
        }
        unsafe extern "system" fn EndFigure<
            Identity: ID2D1SimplifiedGeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            figureend: D2D1_FIGURE_END,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::EndFigure(
                    this,
                    core::mem::transmute_copy(&figureend),
                );
            }
        }
        unsafe extern "system" fn Close<
            Identity: ID2D1SimplifiedGeometrySink_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID2D1SimplifiedGeometrySink_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFillMode: SetFillMode::<Identity, OFFSET>,
            SetSegmentFlags: SetSegmentFlags::<Identity, OFFSET>,
            BeginFigure: BeginFigure::<Identity, OFFSET>,
            AddLines: AddLines::<Identity, OFFSET>,
            AddBeziers: AddBeziers::<Identity, OFFSET>,
            EndFigure: EndFigure::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID2D1SimplifiedGeometrySink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID2D1SimplifiedGeometrySink {}
windows_core::imp::define_interface!(
    ID2D1SolidColorBrush,
    ID2D1SolidColorBrush_Vtbl,
    0x2cd906a9_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1SolidColorBrush {
    type Target = ID2D1Brush;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1SolidColorBrush,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Brush
);
#[repr(C)]
pub struct ID2D1SolidColorBrush_Vtbl {
    pub base__: ID2D1Brush_Vtbl,
    SetColor: usize,
    GetColor: usize,
}
unsafe impl Send for ID2D1SolidColorBrush {}
unsafe impl Sync for ID2D1SolidColorBrush {}
impl windows_core::RuntimeName for ID2D1SolidColorBrush {}
windows_core::imp::define_interface!(
    ID2D1StrokeStyle,
    ID2D1StrokeStyle_Vtbl,
    0x2cd9069d_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1StrokeStyle {
    type Target = ID2D1Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID2D1StrokeStyle, windows_core::IUnknown, ID2D1Resource);
#[repr(C)]
pub struct ID2D1StrokeStyle_Vtbl {
    pub base__: ID2D1Resource_Vtbl,
    GetStartCap: usize,
    GetEndCap: usize,
    GetDashCap: usize,
    GetMiterLimit: usize,
    GetLineJoin: usize,
    GetDashOffset: usize,
    GetDashStyle: usize,
    GetDashesCount: usize,
    GetDashes: usize,
}
unsafe impl Send for ID2D1StrokeStyle {}
unsafe impl Sync for ID2D1StrokeStyle {}
impl windows_core::RuntimeName for ID2D1StrokeStyle {}
windows_core::imp::define_interface!(
    ID2D1TessellationSink,
    ID2D1TessellationSink_Vtbl,
    0x2cd906c1_12e2_11dc_9fed_001143a055f9
);
windows_core::imp::interface_hierarchy!(ID2D1TessellationSink, windows_core::IUnknown);
#[repr(C)]
pub struct ID2D1TessellationSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AddTriangles: usize,
    Close: usize,
}
unsafe impl Send for ID2D1TessellationSink {}
unsafe impl Sync for ID2D1TessellationSink {}
impl windows_core::RuntimeName for ID2D1TessellationSink {}
windows_core::imp::define_interface!(
    ID2D1TransformedGeometry,
    ID2D1TransformedGeometry_Vtbl,
    0x2cd906bb_12e2_11dc_9fed_001143a055f9
);
impl core::ops::Deref for ID2D1TransformedGeometry {
    type Target = ID2D1Geometry;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID2D1TransformedGeometry,
    windows_core::IUnknown,
    ID2D1Resource,
    ID2D1Geometry
);
#[repr(C)]
pub struct ID2D1TransformedGeometry_Vtbl {
    pub base__: ID2D1Geometry_Vtbl,
    GetSourceGeometry: usize,
    GetTransform: usize,
}
unsafe impl Send for ID2D1TransformedGeometry {}
unsafe impl Sync for ID2D1TransformedGeometry {}
impl windows_core::RuntimeName for ID2D1TransformedGeometry {}
windows_core::imp::define_interface!(
    ID3D11Asynchronous,
    ID3D11Asynchronous_Vtbl,
    0x4b35d0cd_1e15_4258_9c98_1b1333f6dd3b
);
impl core::ops::Deref for ID3D11Asynchronous {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Asynchronous,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11Asynchronous_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDataSize: usize,
}
unsafe impl Send for ID3D11Asynchronous {}
unsafe impl Sync for ID3D11Asynchronous {}
impl windows_core::RuntimeName for ID3D11Asynchronous {}
windows_core::imp::define_interface!(
    ID3D11BlendState,
    ID3D11BlendState_Vtbl,
    0x75b68faa_347d_4159_8f45_a0640f01cd9a
);
impl core::ops::Deref for ID3D11BlendState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11BlendState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11BlendState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11BlendState {}
unsafe impl Sync for ID3D11BlendState {}
impl windows_core::RuntimeName for ID3D11BlendState {}
windows_core::imp::define_interface!(
    ID3D11Buffer,
    ID3D11Buffer_Vtbl,
    0x48570b85_d1ee_4fcd_a250_eb350722b037
);
impl core::ops::Deref for ID3D11Buffer {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Buffer,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
#[repr(C)]
pub struct ID3D11Buffer_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11Buffer {}
unsafe impl Sync for ID3D11Buffer {}
impl windows_core::RuntimeName for ID3D11Buffer {}
windows_core::imp::define_interface!(
    ID3D11ClassInstance,
    ID3D11ClassInstance_Vtbl,
    0xa6cd7faa_b0b7_4a2f_9436_8662a65797cb
);
impl core::ops::Deref for ID3D11ClassInstance {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ClassInstance,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11ClassInstance_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetClassLinkage: usize,
    GetDesc: usize,
    GetInstanceName: usize,
    GetTypeName: usize,
}
unsafe impl Send for ID3D11ClassInstance {}
unsafe impl Sync for ID3D11ClassInstance {}
impl windows_core::RuntimeName for ID3D11ClassInstance {}
windows_core::imp::define_interface!(
    ID3D11ClassLinkage,
    ID3D11ClassLinkage_Vtbl,
    0xddf57cba_9543_46e4_a12b_f207a0fe7fed
);
impl core::ops::Deref for ID3D11ClassLinkage {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ClassLinkage,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11ClassLinkage_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetClassInstance: usize,
    CreateClassInstance: usize,
}
unsafe impl Send for ID3D11ClassLinkage {}
unsafe impl Sync for ID3D11ClassLinkage {}
impl windows_core::RuntimeName for ID3D11ClassLinkage {}
windows_core::imp::define_interface!(
    ID3D11CommandList,
    ID3D11CommandList_Vtbl,
    0xa24bc4d1_769e_43f7_8013_98ff566c18e2
);
impl core::ops::Deref for ID3D11CommandList {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11CommandList,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11CommandList_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetContextFlags: usize,
}
unsafe impl Send for ID3D11CommandList {}
unsafe impl Sync for ID3D11CommandList {}
impl windows_core::RuntimeName for ID3D11CommandList {}
windows_core::imp::define_interface!(
    ID3D11ComputeShader,
    ID3D11ComputeShader_Vtbl,
    0x4f5b196e_c2bd_495e_bd01_1fded38e4969
);
impl core::ops::Deref for ID3D11ComputeShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ComputeShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11ComputeShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
unsafe impl Send for ID3D11ComputeShader {}
unsafe impl Sync for ID3D11ComputeShader {}
impl windows_core::RuntimeName for ID3D11ComputeShader {}
windows_core::imp::define_interface!(
    ID3D11Counter,
    ID3D11Counter_Vtbl,
    0x6e8c49fb_a371_4770_b440_29086022b741
);
impl core::ops::Deref for ID3D11Counter {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Counter,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Asynchronous
);
#[repr(C)]
pub struct ID3D11Counter_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11Counter {}
unsafe impl Sync for ID3D11Counter {}
impl windows_core::RuntimeName for ID3D11Counter {}
windows_core::imp::define_interface!(
    ID3D11DepthStencilState,
    ID3D11DepthStencilState_Vtbl,
    0x03823efb_8d8f_4e1c_9aa2_f64bb2cbfdf1
);
impl core::ops::Deref for ID3D11DepthStencilState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DepthStencilState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11DepthStencilState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11DepthStencilState {}
unsafe impl Sync for ID3D11DepthStencilState {}
impl windows_core::RuntimeName for ID3D11DepthStencilState {}
windows_core::imp::define_interface!(
    ID3D11DepthStencilView,
    ID3D11DepthStencilView_Vtbl,
    0x9fdac92a_1876_48c3_afad_25b94f84a9b6
);
impl core::ops::Deref for ID3D11DepthStencilView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DepthStencilView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11DepthStencilView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11DepthStencilView {}
unsafe impl Sync for ID3D11DepthStencilView {}
impl windows_core::RuntimeName for ID3D11DepthStencilView {}
windows_core::imp::define_interface!(
    ID3D11Device,
    ID3D11Device_Vtbl,
    0xdb6f6ddb_ac77_4e88_8253_819df9bbf140
);
windows_core::imp::interface_hierarchy!(ID3D11Device, windows_core::IUnknown);
impl ID3D11Device {
    pub unsafe fn CreateBuffer(
        &self,
        pdesc: *const D3D11_BUFFER_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        ppbuffer: Option<*mut Option<ID3D11Buffer>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateBuffer)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                ppbuffer.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateTexture1D(
        &self,
        pdesc: *const D3D11_TEXTURE1D_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        pptexture1d: Option<*mut Option<ID3D11Texture1D>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateTexture1D)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                pptexture1d.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateTexture2D(
        &self,
        pdesc: *const D3D11_TEXTURE2D_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        pptexture2d: Option<*mut Option<ID3D11Texture2D>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateTexture2D)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                pptexture2d.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateTexture3D(
        &self,
        pdesc: *const D3D11_TEXTURE3D_DESC,
        pinitialdata: Option<*const D3D11_SUBRESOURCE_DATA>,
        pptexture3d: Option<*mut Option<ID3D11Texture3D>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateTexture3D)(
                windows_core::Interface::as_raw(self),
                pdesc,
                pinitialdata.unwrap_or(core::mem::zeroed()) as _,
                pptexture3d.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateShaderResourceView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_SHADER_RESOURCE_VIEW_DESC>,
        ppsrview: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateShaderResourceView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                ppsrview.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateUnorderedAccessView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_UNORDERED_ACCESS_VIEW_DESC>,
        ppuaview: Option<*mut Option<ID3D11UnorderedAccessView>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateUnorderedAccessView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                ppuaview.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateRenderTargetView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_RENDER_TARGET_VIEW_DESC>,
        pprtview: Option<*mut Option<ID3D11RenderTargetView>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateRenderTargetView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                pprtview.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateDepthStencilView<P0>(
        &self,
        presource: P0,
        pdesc: Option<*const D3D11_DEPTH_STENCIL_VIEW_DESC>,
        ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDepthStencilView)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                pdesc.unwrap_or(core::mem::zeroed()) as _,
                ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateInputLayout(
        &self,
        pinputelementdescs: &[D3D11_INPUT_ELEMENT_DESC],
        pshaderbytecodewithinputsignature: &[u8],
        ppinputlayout: Option<*mut Option<ID3D11InputLayout>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateInputLayout)(
                windows_core::Interface::as_raw(self),
                pinputelementdescs.as_ptr(),
                pinputelementdescs.len().try_into().unwrap(),
                core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()),
                pshaderbytecodewithinputsignature.len().try_into().unwrap(),
                ppinputlayout.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateVertexShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppvertexshader: Option<*mut Option<ID3D11VertexShader>>,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateVertexShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppvertexshader.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateGeometryShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateGeometryShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppgeometryshader.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput<P7>(
        &self,
        pshaderbytecode: &[u8],
        psodeclaration: Option<&[D3D11_SO_DECLARATION_ENTRY]>,
        pbufferstrides: Option<&[u32]>,
        rasterizedstream: u32,
        pclasslinkage: P7,
        ppgeometryshader: Option<*mut Option<ID3D11GeometryShader>>,
    ) -> windows_core::Result<()>
    where
        P7: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateGeometryShaderWithStreamOutput)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                psodeclaration.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                psodeclaration.map_or(0, |slice| slice.len().try_into().unwrap()),
                pbufferstrides.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                pbufferstrides.map_or(0, |slice| slice.len().try_into().unwrap()),
                rasterizedstream,
                pclasslinkage.param().abi(),
                ppgeometryshader.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreatePixelShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        pppixelshader: Option<*mut Option<ID3D11PixelShader>>,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreatePixelShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                pppixelshader.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateHullShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        pphullshader: Option<*mut Option<ID3D11HullShader>>,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateHullShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                pphullshader.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateDomainShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppdomainshader: Option<*mut Option<ID3D11DomainShader>>,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDomainShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppdomainshader.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateComputeShader<P2>(
        &self,
        pshaderbytecode: &[u8],
        pclasslinkage: P2,
        ppcomputeshader: Option<*mut Option<ID3D11ComputeShader>>,
    ) -> windows_core::Result<()>
    where
        P2: windows_core::Param<ID3D11ClassLinkage>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CreateComputeShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pshaderbytecode.as_ptr()),
                pshaderbytecode.len().try_into().unwrap(),
                pclasslinkage.param().abi(),
                ppcomputeshader.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateClassLinkage)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateBlendState(
        &self,
        pblendstatedesc: *const D3D11_BLEND_DESC,
        ppblendstate: Option<*mut Option<ID3D11BlendState>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateBlendState)(
                windows_core::Interface::as_raw(self),
                pblendstatedesc,
                ppblendstate.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateDepthStencilState(
        &self,
        pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC,
        ppdepthstencilstate: Option<*mut Option<ID3D11DepthStencilState>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDepthStencilState)(
                windows_core::Interface::as_raw(self),
                pdepthstencildesc,
                ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateRasterizerState(
        &self,
        prasterizerdesc: *const D3D11_RASTERIZER_DESC,
        pprasterizerstate: Option<*mut Option<ID3D11RasterizerState>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateRasterizerState)(
                windows_core::Interface::as_raw(self),
                prasterizerdesc,
                pprasterizerstate.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateSamplerState(
        &self,
        psamplerdesc: *const D3D11_SAMPLER_DESC,
        ppsamplerstate: Option<*mut Option<ID3D11SamplerState>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateSamplerState)(
                windows_core::Interface::as_raw(self),
                psamplerdesc,
                ppsamplerstate.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateQuery(
        &self,
        pquerydesc: *const D3D11_QUERY_DESC,
        ppquery: Option<*mut Option<ID3D11Query>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateQuery)(
                windows_core::Interface::as_raw(self),
                pquerydesc,
                ppquery.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreatePredicate(
        &self,
        ppredicatedesc: *const D3D11_QUERY_DESC,
        pppredicate: Option<*mut Option<ID3D11Predicate>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreatePredicate)(
                windows_core::Interface::as_raw(self),
                ppredicatedesc,
                pppredicate.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateCounter(
        &self,
        pcounterdesc: *const D3D11_COUNTER_DESC,
        ppcounter: Option<*mut Option<ID3D11Counter>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateCounter)(
                windows_core::Interface::as_raw(self),
                pcounterdesc,
                ppcounter.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CreateDeferredContext(
        &self,
        contextflags: u32,
        ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateDeferredContext)(
                windows_core::Interface::as_raw(self),
                contextflags,
                ppdeferredcontext.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn OpenSharedResource<T>(
        &self,
        hresource: HANDLE,
        result__: *mut Option<T>,
    ) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OpenSharedResource)(
                windows_core::Interface::as_raw(self),
                hresource,
                &T::IID,
                result__ as *mut _ as *mut _,
            )
            .ok()
        }
    }
    pub unsafe fn CheckFormatSupport(&self, format: DXGI_FORMAT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckFormatSupport)(
                windows_core::Interface::as_raw(self),
                format,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CheckMultisampleQualityLevels(
        &self,
        format: DXGI_FORMAT,
        samplecount: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels)(
                windows_core::Interface::as_raw(self),
                format,
                samplecount,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D11_COUNTER_INFO {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckCounterInfo)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn CheckCounter(
        &self,
        pdesc: *const D3D11_COUNTER_DESC,
        ptype: *mut D3D11_COUNTER_TYPE,
        pactivecounters: *mut u32,
        szname: Option<windows_core::PSTR>,
        pnamelength: Option<*mut u32>,
        szunits: Option<windows_core::PSTR>,
        punitslength: Option<*mut u32>,
        szdescription: Option<windows_core::PSTR>,
        pdescriptionlength: Option<*mut u32>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CheckCounter)(
                windows_core::Interface::as_raw(self),
                pdesc,
                ptype as _,
                pactivecounters as _,
                szname.unwrap_or(core::mem::zeroed()) as _,
                pnamelength.unwrap_or(core::mem::zeroed()) as _,
                szunits.unwrap_or(core::mem::zeroed()) as _,
                punitslength.unwrap_or(core::mem::zeroed()) as _,
                szdescription.unwrap_or(core::mem::zeroed()) as _,
                pdescriptionlength.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CheckFeatureSupport(
        &self,
        feature: D3D11_FEATURE,
        pfeaturesupportdata: *mut core::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CheckFeatureSupport)(
                windows_core::Interface::as_raw(self),
                feature,
                pfeaturesupportdata as _,
                featuresupportdatasize,
            )
            .ok()
        }
    }
    pub unsafe fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: Option<*mut core::ffi::c_void>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                pdatasize as _,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: Option<*const core::ffi::c_void>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateData)(
                windows_core::Interface::as_raw(self),
                guid,
                datasize,
                pdata.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(
        &self,
        guid: *const windows_core::GUID,
        pdata: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateDataInterface)(
                windows_core::Interface::as_raw(self),
                guid,
                pdata.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetFeatureLevel(&self) -> D3D_FEATURE_LEVEL {
        unsafe {
            (windows_core::Interface::vtable(self).GetFeatureLevel)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetCreationFlags)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub unsafe fn GetImmediateContext(&self) -> windows_core::Result<ID3D11DeviceContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmediateContext)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetExceptionMode)(
                windows_core::Interface::as_raw(self),
                raiseflags,
            )
            .ok()
        }
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetExceptionMode)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
}
#[repr(C)]
pub struct ID3D11Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_BUFFER_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTexture1D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_TEXTURE1D_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTexture2D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_TEXTURE2D_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTexture3D: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_TEXTURE3D_DESC,
        *const D3D11_SUBRESOURCE_DATA,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateShaderResourceView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateUnorderedAccessView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRenderTargetView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_RENDER_TARGET_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDepthStencilView: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateInputLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_INPUT_ELEMENT_DESC,
        u32,
        *const core::ffi::c_void,
        usize,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateVertexShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGeometryShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGeometryShaderWithStreamOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *const D3D11_SO_DECLARATION_ENTRY,
        u32,
        *const u32,
        u32,
        u32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    )
        -> windows_core::HRESULT,
    pub CreatePixelShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateHullShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDomainShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateComputeShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        usize,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateClassLinkage: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateBlendState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_BLEND_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDepthStencilState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_DEPTH_STENCIL_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRasterizerState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_RASTERIZER_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSamplerState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_SAMPLER_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateQuery: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_QUERY_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreatePredicate: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_QUERY_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCounter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_COUNTER_DESC,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateDeferredContext: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub OpenSharedResource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CheckFormatSupport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_FORMAT,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_FORMAT,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub CheckCounterInfo:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D11_COUNTER_INFO),
    pub CheckCounter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const D3D11_COUNTER_DESC,
        *mut D3D11_COUNTER_TYPE,
        *mut u32,
        windows_core::PSTR,
        *mut u32,
        windows_core::PSTR,
        *mut u32,
        windows_core::PSTR,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub CheckFeatureSupport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        D3D11_FEATURE,
        *mut core::ffi::c_void,
        u32,
    ) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D_FEATURE_LEVEL,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDeviceRemovedReason:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImmediateContext:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetExceptionMode:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
unsafe impl Send for ID3D11Device {}
unsafe impl Sync for ID3D11Device {}
pub trait ID3D11Device_Impl: windows_core::IUnknownImpl {
    fn CreateBuffer(
        &self,
        pdesc: *const D3D11_BUFFER_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        ppbuffer: windows_core::OutRef<ID3D11Buffer>,
    ) -> windows_core::Result<()>;
    fn CreateTexture1D(
        &self,
        pdesc: *const D3D11_TEXTURE1D_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        pptexture1d: windows_core::OutRef<ID3D11Texture1D>,
    ) -> windows_core::Result<()>;
    fn CreateTexture2D(
        &self,
        pdesc: *const D3D11_TEXTURE2D_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        pptexture2d: windows_core::OutRef<ID3D11Texture2D>,
    ) -> windows_core::Result<()>;
    fn CreateTexture3D(
        &self,
        pdesc: *const D3D11_TEXTURE3D_DESC,
        pinitialdata: *const D3D11_SUBRESOURCE_DATA,
        pptexture3d: windows_core::OutRef<ID3D11Texture3D>,
    ) -> windows_core::Result<()>;
    fn CreateShaderResourceView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
        ppsrview: windows_core::OutRef<ID3D11ShaderResourceView>,
    ) -> windows_core::Result<()>;
    fn CreateUnorderedAccessView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
        ppuaview: windows_core::OutRef<ID3D11UnorderedAccessView>,
    ) -> windows_core::Result<()>;
    fn CreateRenderTargetView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC,
        pprtview: windows_core::OutRef<ID3D11RenderTargetView>,
    ) -> windows_core::Result<()>;
    fn CreateDepthStencilView(
        &self,
        presource: windows_core::Ref<ID3D11Resource>,
        pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
        ppdepthstencilview: windows_core::OutRef<ID3D11DepthStencilView>,
    ) -> windows_core::Result<()>;
    fn CreateInputLayout(
        &self,
        pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC,
        numelements: u32,
        pshaderbytecodewithinputsignature: *const core::ffi::c_void,
        bytecodelength: usize,
        ppinputlayout: windows_core::OutRef<ID3D11InputLayout>,
    ) -> windows_core::Result<()>;
    fn CreateVertexShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppvertexshader: windows_core::OutRef<ID3D11VertexShader>,
    ) -> windows_core::Result<()>;
    fn CreateGeometryShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>,
    ) -> windows_core::Result<()>;
    fn CreateGeometryShaderWithStreamOutput(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        psodeclaration: *const D3D11_SO_DECLARATION_ENTRY,
        numentries: u32,
        pbufferstrides: *const u32,
        numstrides: u32,
        rasterizedstream: u32,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppgeometryshader: windows_core::OutRef<ID3D11GeometryShader>,
    ) -> windows_core::Result<()>;
    fn CreatePixelShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        pppixelshader: windows_core::OutRef<ID3D11PixelShader>,
    ) -> windows_core::Result<()>;
    fn CreateHullShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        pphullshader: windows_core::OutRef<ID3D11HullShader>,
    ) -> windows_core::Result<()>;
    fn CreateDomainShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppdomainshader: windows_core::OutRef<ID3D11DomainShader>,
    ) -> windows_core::Result<()>;
    fn CreateComputeShader(
        &self,
        pshaderbytecode: *const core::ffi::c_void,
        bytecodelength: usize,
        pclasslinkage: windows_core::Ref<ID3D11ClassLinkage>,
        ppcomputeshader: windows_core::OutRef<ID3D11ComputeShader>,
    ) -> windows_core::Result<()>;
    fn CreateClassLinkage(&self) -> windows_core::Result<ID3D11ClassLinkage>;
    fn CreateBlendState(
        &self,
        pblendstatedesc: *const D3D11_BLEND_DESC,
        ppblendstate: windows_core::OutRef<ID3D11BlendState>,
    ) -> windows_core::Result<()>;
    fn CreateDepthStencilState(
        &self,
        pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC,
        ppdepthstencilstate: windows_core::OutRef<ID3D11DepthStencilState>,
    ) -> windows_core::Result<()>;
    fn CreateRasterizerState(
        &self,
        prasterizerdesc: *const D3D11_RASTERIZER_DESC,
        pprasterizerstate: windows_core::OutRef<ID3D11RasterizerState>,
    ) -> windows_core::Result<()>;
    fn CreateSamplerState(
        &self,
        psamplerdesc: *const D3D11_SAMPLER_DESC,
        ppsamplerstate: windows_core::OutRef<ID3D11SamplerState>,
    ) -> windows_core::Result<()>;
    fn CreateQuery(
        &self,
        pquerydesc: *const D3D11_QUERY_DESC,
        ppquery: windows_core::OutRef<ID3D11Query>,
    ) -> windows_core::Result<()>;
    fn CreatePredicate(
        &self,
        ppredicatedesc: *const D3D11_QUERY_DESC,
        pppredicate: windows_core::OutRef<ID3D11Predicate>,
    ) -> windows_core::Result<()>;
    fn CreateCounter(
        &self,
        pcounterdesc: *const D3D11_COUNTER_DESC,
        ppcounter: windows_core::OutRef<ID3D11Counter>,
    ) -> windows_core::Result<()>;
    fn CreateDeferredContext(
        &self,
        contextflags: u32,
        ppdeferredcontext: windows_core::OutRef<ID3D11DeviceContext>,
    ) -> windows_core::Result<()>;
    fn OpenSharedResource(
        &self,
        hresource: HANDLE,
        returnedinterface: *const windows_core::GUID,
        ppresource: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn CheckFormatSupport(&self, format: DXGI_FORMAT) -> windows_core::Result<u32>;
    fn CheckMultisampleQualityLevels(
        &self,
        format: DXGI_FORMAT,
        samplecount: u32,
    ) -> windows_core::Result<u32>;
    fn CheckCounterInfo(&self, pcounterinfo: *mut D3D11_COUNTER_INFO);
    fn CheckCounter(
        &self,
        pdesc: *const D3D11_COUNTER_DESC,
        ptype: *mut D3D11_COUNTER_TYPE,
        pactivecounters: *mut u32,
        szname: windows_core::PSTR,
        pnamelength: *mut u32,
        szunits: windows_core::PSTR,
        punitslength: *mut u32,
        szdescription: windows_core::PSTR,
        pdescriptionlength: *mut u32,
    ) -> windows_core::Result<()>;
    fn CheckFeatureSupport(
        &self,
        feature: D3D11_FEATURE,
        pfeaturesupportdata: *mut core::ffi::c_void,
        featuresupportdatasize: u32,
    ) -> windows_core::Result<()>;
    fn GetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateData(
        &self,
        guid: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(
        &self,
        guid: *const windows_core::GUID,
        pdata: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn GetFeatureLevel(&self) -> D3D_FEATURE_LEVEL;
    fn GetCreationFlags(&self) -> u32;
    fn GetDeviceRemovedReason(&self) -> windows_core::Result<()>;
    fn GetImmediateContext(&self, ppimmediatecontext: windows_core::OutRef<ID3D11DeviceContext>);
    fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()>;
    fn GetExceptionMode(&self) -> u32;
}
impl ID3D11Device_Vtbl {
    pub const fn new<Identity: ID3D11Device_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBuffer<Identity: ID3D11Device_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_BUFFER_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            ppbuffer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateBuffer(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&ppbuffer),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTexture1D<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_TEXTURE1D_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            pptexture1d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture1D(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&pptexture1d),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTexture2D<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_TEXTURE2D_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            pptexture2d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture2D(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&pptexture2d),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTexture3D<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_TEXTURE3D_DESC,
            pinitialdata: *const D3D11_SUBRESOURCE_DATA,
            pptexture3d: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateTexture3D(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pinitialdata),
                    core::mem::transmute_copy(&pptexture3d),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateShaderResourceView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_SHADER_RESOURCE_VIEW_DESC,
            ppsrview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateShaderResourceView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppsrview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateUnorderedAccessView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_UNORDERED_ACCESS_VIEW_DESC,
            ppuaview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateUnorderedAccessView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppuaview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateRenderTargetView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_RENDER_TARGET_VIEW_DESC,
            pprtview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateRenderTargetView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&pprtview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilView<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            presource: *mut core::ffi::c_void,
            pdesc: *const D3D11_DEPTH_STENCIL_VIEW_DESC,
            ppdepthstencilview: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDepthStencilView(
                    this,
                    core::mem::transmute_copy(&presource),
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ppdepthstencilview),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateInputLayout<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pinputelementdescs: *const D3D11_INPUT_ELEMENT_DESC,
            numelements: u32,
            pshaderbytecodewithinputsignature: *const core::ffi::c_void,
            bytecodelength: usize,
            ppinputlayout: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateInputLayout(
                    this,
                    core::mem::transmute_copy(&pinputelementdescs),
                    core::mem::transmute_copy(&numelements),
                    core::mem::transmute_copy(&pshaderbytecodewithinputsignature),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&ppinputlayout),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateVertexShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppvertexshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateVertexShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppvertexshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateGeometryShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppgeometryshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateGeometryShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppgeometryshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateGeometryShaderWithStreamOutput<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            psodeclaration: *const D3D11_SO_DECLARATION_ENTRY,
            numentries: u32,
            pbufferstrides: *const u32,
            numstrides: u32,
            rasterizedstream: u32,
            pclasslinkage: *mut core::ffi::c_void,
            ppgeometryshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateGeometryShaderWithStreamOutput(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&psodeclaration),
                    core::mem::transmute_copy(&numentries),
                    core::mem::transmute_copy(&pbufferstrides),
                    core::mem::transmute_copy(&numstrides),
                    core::mem::transmute_copy(&rasterizedstream),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppgeometryshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreatePixelShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            pppixelshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreatePixelShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&pppixelshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateHullShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            pphullshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateHullShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&pphullshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDomainShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppdomainshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDomainShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppdomainshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateComputeShader<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pshaderbytecode: *const core::ffi::c_void,
            bytecodelength: usize,
            pclasslinkage: *mut core::ffi::c_void,
            ppcomputeshader: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateComputeShader(
                    this,
                    core::mem::transmute_copy(&pshaderbytecode),
                    core::mem::transmute_copy(&bytecodelength),
                    core::mem::transmute_copy(&pclasslinkage),
                    core::mem::transmute_copy(&ppcomputeshader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateClassLinkage<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pplinkage: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CreateClassLinkage(this) {
                    Ok(ok__) => {
                        pplinkage.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateBlendState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pblendstatedesc: *const D3D11_BLEND_DESC,
            ppblendstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateBlendState(
                    this,
                    core::mem::transmute_copy(&pblendstatedesc),
                    core::mem::transmute_copy(&ppblendstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDepthStencilState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdepthstencildesc: *const D3D11_DEPTH_STENCIL_DESC,
            ppdepthstencilstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDepthStencilState(
                    this,
                    core::mem::transmute_copy(&pdepthstencildesc),
                    core::mem::transmute_copy(&ppdepthstencilstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateRasterizerState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            prasterizerdesc: *const D3D11_RASTERIZER_DESC,
            pprasterizerstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateRasterizerState(
                    this,
                    core::mem::transmute_copy(&prasterizerdesc),
                    core::mem::transmute_copy(&pprasterizerstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateSamplerState<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psamplerdesc: *const D3D11_SAMPLER_DESC,
            ppsamplerstate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateSamplerState(
                    this,
                    core::mem::transmute_copy(&psamplerdesc),
                    core::mem::transmute_copy(&ppsamplerstate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateQuery<Identity: ID3D11Device_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pquerydesc: *const D3D11_QUERY_DESC,
            ppquery: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateQuery(
                    this,
                    core::mem::transmute_copy(&pquerydesc),
                    core::mem::transmute_copy(&ppquery),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreatePredicate<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppredicatedesc: *const D3D11_QUERY_DESC,
            pppredicate: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreatePredicate(
                    this,
                    core::mem::transmute_copy(&ppredicatedesc),
                    core::mem::transmute_copy(&pppredicate),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateCounter<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcounterdesc: *const D3D11_COUNTER_DESC,
            ppcounter: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateCounter(
                    this,
                    core::mem::transmute_copy(&pcounterdesc),
                    core::mem::transmute_copy(&ppcounter),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateDeferredContext<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            contextflags: u32,
            ppdeferredcontext: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CreateDeferredContext(
                    this,
                    core::mem::transmute_copy(&contextflags),
                    core::mem::transmute_copy(&ppdeferredcontext),
                )
                .into()
            }
        }
        unsafe extern "system" fn OpenSharedResource<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hresource: HANDLE,
            returnedinterface: *const windows_core::GUID,
            ppresource: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::OpenSharedResource(
                    this,
                    core::mem::transmute_copy(&hresource),
                    core::mem::transmute_copy(&returnedinterface),
                    core::mem::transmute_copy(&ppresource),
                )
                .into()
            }
        }
        unsafe extern "system" fn CheckFormatSupport<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            format: DXGI_FORMAT,
            pformatsupport: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CheckFormatSupport(
                    this,
                    core::mem::transmute_copy(&format),
                ) {
                    Ok(ok__) => {
                        pformatsupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckMultisampleQualityLevels<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            format: DXGI_FORMAT,
            samplecount: u32,
            pnumqualitylevels: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ID3D11Device_Impl::CheckMultisampleQualityLevels(
                    this,
                    core::mem::transmute_copy(&format),
                    core::mem::transmute_copy(&samplecount),
                ) {
                    Ok(ok__) => {
                        pnumqualitylevels.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckCounterInfo<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcounterinfo: *mut D3D11_COUNTER_INFO,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckCounterInfo(this, core::mem::transmute_copy(&pcounterinfo));
            }
        }
        unsafe extern "system" fn CheckCounter<Identity: ID3D11Device_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *const D3D11_COUNTER_DESC,
            ptype: *mut D3D11_COUNTER_TYPE,
            pactivecounters: *mut u32,
            szname: windows_core::PSTR,
            pnamelength: *mut u32,
            szunits: windows_core::PSTR,
            punitslength: *mut u32,
            szdescription: windows_core::PSTR,
            pdescriptionlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckCounter(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&ptype),
                    core::mem::transmute_copy(&pactivecounters),
                    core::mem::transmute_copy(&szname),
                    core::mem::transmute_copy(&pnamelength),
                    core::mem::transmute_copy(&szunits),
                    core::mem::transmute_copy(&punitslength),
                    core::mem::transmute_copy(&szdescription),
                    core::mem::transmute_copy(&pdescriptionlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn CheckFeatureSupport<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            feature: D3D11_FEATURE,
            pfeaturesupportdata: *mut core::ffi::c_void,
            featuresupportdatasize: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::CheckFeatureSupport(
                    this,
                    core::mem::transmute_copy(&feature),
                    core::mem::transmute_copy(&pfeaturesupportdata),
                    core::mem::transmute_copy(&featuresupportdatasize),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPrivateData<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdatasize: *mut u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdatasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateData<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetPrivateData(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetPrivateDataInterface(
                    this,
                    core::mem::transmute_copy(&guid),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetFeatureLevel<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> D3D_FEATURE_LEVEL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetFeatureLevel(this)
            }
        }
        unsafe extern "system" fn GetCreationFlags<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetCreationFlags(this)
            }
        }
        unsafe extern "system" fn GetDeviceRemovedReason<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetDeviceRemovedReason(this).into()
            }
        }
        unsafe extern "system" fn GetImmediateContext<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppimmediatecontext: *mut *mut core::ffi::c_void,
        ) {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetImmediateContext(
                    this,
                    core::mem::transmute_copy(&ppimmediatecontext),
                );
            }
        }
        unsafe extern "system" fn SetExceptionMode<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            raiseflags: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::SetExceptionMode(this, core::mem::transmute_copy(&raiseflags))
                    .into()
            }
        }
        unsafe extern "system" fn GetExceptionMode<
            Identity: ID3D11Device_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u32 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11Device_Impl::GetExceptionMode(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateBuffer: CreateBuffer::<Identity, OFFSET>,
            CreateTexture1D: CreateTexture1D::<Identity, OFFSET>,
            CreateTexture2D: CreateTexture2D::<Identity, OFFSET>,
            CreateTexture3D: CreateTexture3D::<Identity, OFFSET>,
            CreateShaderResourceView: CreateShaderResourceView::<Identity, OFFSET>,
            CreateUnorderedAccessView: CreateUnorderedAccessView::<Identity, OFFSET>,
            CreateRenderTargetView: CreateRenderTargetView::<Identity, OFFSET>,
            CreateDepthStencilView: CreateDepthStencilView::<Identity, OFFSET>,
            CreateInputLayout: CreateInputLayout::<Identity, OFFSET>,
            CreateVertexShader: CreateVertexShader::<Identity, OFFSET>,
            CreateGeometryShader: CreateGeometryShader::<Identity, OFFSET>,
            CreateGeometryShaderWithStreamOutput: CreateGeometryShaderWithStreamOutput::<
                Identity,
                OFFSET,
            >,
            CreatePixelShader: CreatePixelShader::<Identity, OFFSET>,
            CreateHullShader: CreateHullShader::<Identity, OFFSET>,
            CreateDomainShader: CreateDomainShader::<Identity, OFFSET>,
            CreateComputeShader: CreateComputeShader::<Identity, OFFSET>,
            CreateClassLinkage: CreateClassLinkage::<Identity, OFFSET>,
            CreateBlendState: CreateBlendState::<Identity, OFFSET>,
            CreateDepthStencilState: CreateDepthStencilState::<Identity, OFFSET>,
            CreateRasterizerState: CreateRasterizerState::<Identity, OFFSET>,
            CreateSamplerState: CreateSamplerState::<Identity, OFFSET>,
            CreateQuery: CreateQuery::<Identity, OFFSET>,
            CreatePredicate: CreatePredicate::<Identity, OFFSET>,
            CreateCounter: CreateCounter::<Identity, OFFSET>,
            CreateDeferredContext: CreateDeferredContext::<Identity, OFFSET>,
            OpenSharedResource: OpenSharedResource::<Identity, OFFSET>,
            CheckFormatSupport: CheckFormatSupport::<Identity, OFFSET>,
            CheckMultisampleQualityLevels: CheckMultisampleQualityLevels::<Identity, OFFSET>,
            CheckCounterInfo: CheckCounterInfo::<Identity, OFFSET>,
            CheckCounter: CheckCounter::<Identity, OFFSET>,
            CheckFeatureSupport: CheckFeatureSupport::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetFeatureLevel: GetFeatureLevel::<Identity, OFFSET>,
            GetCreationFlags: GetCreationFlags::<Identity, OFFSET>,
            GetDeviceRemovedReason: GetDeviceRemovedReason::<Identity, OFFSET>,
            GetImmediateContext: GetImmediateContext::<Identity, OFFSET>,
            SetExceptionMode: SetExceptionMode::<Identity, OFFSET>,
            GetExceptionMode: GetExceptionMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11Device as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11Device {}
windows_core::imp::define_interface!(
    ID3D11DeviceChild,
    ID3D11DeviceChild_Vtbl,
    0x1841e5c8_16b0_489b_bcc8_44cfb0d5deae
);
windows_core::imp::interface_hierarchy!(ID3D11DeviceChild, windows_core::IUnknown);
#[repr(C)]
pub struct ID3D11DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetDevice: usize,
    GetPrivateData: usize,
    SetPrivateData: usize,
    SetPrivateDataInterface: usize,
}
unsafe impl Send for ID3D11DeviceChild {}
unsafe impl Sync for ID3D11DeviceChild {}
impl windows_core::RuntimeName for ID3D11DeviceChild {}
windows_core::imp::define_interface!(
    ID3D11DeviceContext,
    ID3D11DeviceContext_Vtbl,
    0xc0bfa96c_e089_44fb_8eaf_26f8796190da
);
impl core::ops::Deref for ID3D11DeviceContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DeviceContext,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
impl ID3D11DeviceContext {
    pub unsafe fn VSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn PSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn PSSetShader<P0>(
        &self,
        ppixelshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11PixelShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetShader)(
                windows_core::Interface::as_raw(self),
                ppixelshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub unsafe fn PSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub unsafe fn VSSetShader<P0>(
        &self,
        pvertexshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11VertexShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShader)(
                windows_core::Interface::as_raw(self),
                pvertexshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub unsafe fn DrawIndexed(
        &self,
        indexcount: u32,
        startindexlocation: u32,
        basevertexlocation: i32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexed)(
                windows_core::Interface::as_raw(self),
                indexcount,
                startindexlocation,
                basevertexlocation,
            );
        }
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(
                windows_core::Interface::as_raw(self),
                vertexcount,
                startvertexlocation,
            );
        }
    }
    pub unsafe fn Map<P0>(
        &self,
        presource: P0,
        subresource: u32,
        maptype: D3D11_MAP,
        mapflags: u32,
        pmappedresource: Option<*mut D3D11_MAPPED_SUBRESOURCE>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Map)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                subresource,
                maptype,
                mapflags,
                pmappedresource.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn Unmap<P0>(&self, presource: P0, subresource: u32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                subresource,
            );
        }
    }
    pub unsafe fn PSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: windows_core::Param<ID3D11InputLayout>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetInputLayout)(
                windows_core::Interface::as_raw(self),
                pinputlayout.param().abi(),
            );
        }
    }
    pub unsafe fn IASetVertexBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppvertexbuffers: Option<*const Option<ID3D11Buffer>>,
        pstrides: Option<*const u32>,
        poffsets: Option<*const u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetVertexBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _,
                pstrides.unwrap_or(core::mem::zeroed()) as _,
                poffsets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: DXGI_FORMAT, offset: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).IASetIndexBuffer)(
                windows_core::Interface::as_raw(self),
                pindexbuffer.param().abi(),
                format,
                offset,
            );
        }
    }
    pub unsafe fn DrawIndexedInstanced(
        &self,
        indexcountperinstance: u32,
        instancecount: u32,
        startindexlocation: u32,
        basevertexlocation: i32,
        startinstancelocation: u32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstanced)(
                windows_core::Interface::as_raw(self),
                indexcountperinstance,
                instancecount,
                startindexlocation,
                basevertexlocation,
                startinstancelocation,
            );
        }
    }
    pub unsafe fn DrawInstanced(
        &self,
        vertexcountperinstance: u32,
        instancecount: u32,
        startvertexlocation: u32,
        startinstancelocation: u32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstanced)(
                windows_core::Interface::as_raw(self),
                vertexcountperinstance,
                instancecount,
                startvertexlocation,
                startinstancelocation,
            );
        }
    }
    pub unsafe fn GSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn GSSetShader<P0>(
        &self,
        pshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11GeometryShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShader)(
                windows_core::Interface::as_raw(self),
                pshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub unsafe fn IASetPrimitiveTopology(&self, topology: D3D_PRIMITIVE_TOPOLOGY) {
        unsafe {
            (windows_core::Interface::vtable(self).IASetPrimitiveTopology)(
                windows_core::Interface::as_raw(self),
                topology,
            );
        }
    }
    pub unsafe fn VSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn VSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub unsafe fn Begin<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Begin)(
                windows_core::Interface::as_raw(self),
                pasync.param().abi(),
            );
        }
    }
    pub unsafe fn End<P0>(&self, pasync: P0)
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).End)(
                windows_core::Interface::as_raw(self),
                pasync.param().abi(),
            );
        }
    }
    pub unsafe fn GetData<P0>(
        &self,
        pasync: P0,
        pdata: Option<*mut core::ffi::c_void>,
        datasize: u32,
        getdataflags: u32,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<ID3D11Asynchronous>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetData)(
                windows_core::Interface::as_raw(self),
                pasync.param().abi(),
                pdata.unwrap_or(core::mem::zeroed()) as _,
                datasize,
                getdataflags,
            )
            .ok()
        }
    }
    pub unsafe fn SetPredication<P0>(&self, ppredicate: P0, predicatevalue: bool)
    where
        P0: windows_core::Param<ID3D11Predicate>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPredication)(
                windows_core::Interface::as_raw(self),
                ppredicate.param().abi(),
                predicatevalue.into(),
            );
        }
    }
    pub unsafe fn GSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn GSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub unsafe fn OMSetRenderTargets<P2>(
        &self,
        pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>,
        pdepthstencilview: P2,
    ) where
        P2: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetRenderTargets)(
                windows_core::Interface::as_raw(self),
                pprendertargetviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    pprendertargetviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                pdepthstencilview.param().abi(),
            );
        }
    }
    pub unsafe fn OMSetRenderTargetsAndUnorderedAccessViews<P2>(
        &self,
        pprendertargetviews: Option<&[Option<ID3D11RenderTargetView>]>,
        pdepthstencilview: P2,
        uavstartslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>,
        puavinitialcounts: Option<*const u32>,
    ) where
        P2: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetRenderTargetsAndUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                pprendertargetviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    pprendertargetviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                pdepthstencilview.param().abi(),
                uavstartslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
                puavinitialcounts.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn OMSetBlendState<P0>(
        &self,
        pblendstate: P0,
        blendfactor: Option<&[f32; 4]>,
        samplemask: u32,
    ) where
        P0: windows_core::Param<ID3D11BlendState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetBlendState)(
                windows_core::Interface::as_raw(self),
                pblendstate.param().abi(),
                blendfactor.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                samplemask,
            );
        }
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: windows_core::Param<ID3D11DepthStencilState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OMSetDepthStencilState)(
                windows_core::Interface::as_raw(self),
                pdepthstencilstate.param().abi(),
                stencilref,
            );
        }
    }
    pub unsafe fn SOSetTargets(
        &self,
        numbuffers: u32,
        ppsotargets: Option<*const Option<ID3D11Buffer>>,
        poffsets: Option<*const u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).SOSetTargets)(
                windows_core::Interface::as_raw(self),
                numbuffers,
                ppsotargets.unwrap_or(core::mem::zeroed()) as _,
                poffsets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DrawAuto(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn DrawIndexedInstancedIndirect<P0>(
        &self,
        pbufferforargs: P0,
        alignedbyteoffsetforargs: u32,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstancedIndirect)(
                windows_core::Interface::as_raw(self),
                pbufferforargs.param().abi(),
                alignedbyteoffsetforargs,
            );
        }
    }
    pub unsafe fn DrawInstancedIndirect<P0>(
        &self,
        pbufferforargs: P0,
        alignedbyteoffsetforargs: u32,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstancedIndirect)(
                windows_core::Interface::as_raw(self),
                pbufferforargs.param().abi(),
                alignedbyteoffsetforargs,
            );
        }
    }
    pub unsafe fn Dispatch(
        &self,
        threadgroupcountx: u32,
        threadgroupcounty: u32,
        threadgroupcountz: u32,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).Dispatch)(
                windows_core::Interface::as_raw(self),
                threadgroupcountx,
                threadgroupcounty,
                threadgroupcountz,
            );
        }
    }
    pub unsafe fn DispatchIndirect<P0>(&self, pbufferforargs: P0, alignedbyteoffsetforargs: u32)
    where
        P0: windows_core::Param<ID3D11Buffer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DispatchIndirect)(
                windows_core::Interface::as_raw(self),
                pbufferforargs.param().abi(),
                alignedbyteoffsetforargs,
            );
        }
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: windows_core::Param<ID3D11RasterizerState>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetState)(
                windows_core::Interface::as_raw(self),
                prasterizerstate.param().abi(),
            );
        }
    }
    pub unsafe fn RSSetViewports(&self, pviewports: Option<&[D3D11_VIEWPORT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetViewports)(
                windows_core::Interface::as_raw(self),
                pviewports.map_or(0, |slice| slice.len().try_into().unwrap()),
                pviewports.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            );
        }
    }
    pub unsafe fn RSSetScissorRects(&self, prects: Option<&[RECT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetScissorRects)(
                windows_core::Interface::as_raw(self),
                prects.map_or(0, |slice| slice.len().try_into().unwrap()),
                prects.map_or(core::ptr::null(), |slice| slice.as_ptr()),
            );
        }
    }
    pub unsafe fn CopySubresourceRegion<P0, P5>(
        &self,
        pdstresource: P0,
        dstsubresource: u32,
        dstx: u32,
        dsty: u32,
        dstz: u32,
        psrcresource: P5,
        srcsubresource: u32,
        psrcbox: Option<*const D3D11_BOX>,
    ) where
        P0: windows_core::Param<ID3D11Resource>,
        P5: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopySubresourceRegion)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                dstsubresource,
                dstx,
                dsty,
                dstz,
                psrcresource.param().abi(),
                srcsubresource,
                psrcbox.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: windows_core::Param<ID3D11Resource>,
        P1: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyResource)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                psrcresource.param().abi(),
            );
        }
    }
    pub unsafe fn UpdateSubresource<P0>(
        &self,
        pdstresource: P0,
        dstsubresource: u32,
        pdstbox: Option<*const D3D11_BOX>,
        psrcdata: *const core::ffi::c_void,
        srcrowpitch: u32,
        srcdepthpitch: u32,
    ) where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateSubresource)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                dstsubresource,
                pdstbox.unwrap_or(core::mem::zeroed()) as _,
                psrcdata,
                srcrowpitch,
                srcdepthpitch,
            );
        }
    }
    pub unsafe fn CopyStructureCount<P0, P2>(
        &self,
        pdstbuffer: P0,
        dstalignedbyteoffset: u32,
        psrcview: P2,
    ) where
        P0: windows_core::Param<ID3D11Buffer>,
        P2: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyStructureCount)(
                windows_core::Interface::as_raw(self),
                pdstbuffer.param().abi(),
                dstalignedbyteoffset,
                psrcview.param().abi(),
            );
        }
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: &[f32; 4])
    where
        P0: windows_core::Param<ID3D11RenderTargetView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearRenderTargetView)(
                windows_core::Interface::as_raw(self),
                prendertargetview.param().abi(),
                colorrgba.as_ptr(),
            );
        }
    }
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(
        &self,
        punorderedaccessview: P0,
        values: &[u32; 4],
    ) where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearUnorderedAccessViewUint)(
                windows_core::Interface::as_raw(self),
                punorderedaccessview.param().abi(),
                values.as_ptr(),
            );
        }
    }
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(
        &self,
        punorderedaccessview: P0,
        values: &[f32; 4],
    ) where
        P0: windows_core::Param<ID3D11UnorderedAccessView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearUnorderedAccessViewFloat)(
                windows_core::Interface::as_raw(self),
                punorderedaccessview.param().abi(),
                values.as_ptr(),
            );
        }
    }
    pub unsafe fn ClearDepthStencilView<P0>(
        &self,
        pdepthstencilview: P0,
        clearflags: u32,
        depth: f32,
        stencil: u8,
    ) where
        P0: windows_core::Param<ID3D11DepthStencilView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ClearDepthStencilView)(
                windows_core::Interface::as_raw(self),
                pdepthstencilview.param().abi(),
                clearflags,
                depth,
                stencil,
            );
        }
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: windows_core::Param<ID3D11ShaderResourceView>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GenerateMips)(
                windows_core::Interface::as_raw(self),
                pshaderresourceview.param().abi(),
            );
        }
    }
    pub unsafe fn SetResourceMinLOD<P0>(&self, presource: P0, minlod: f32)
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetResourceMinLOD)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
                minlod,
            );
        }
    }
    pub unsafe fn GetResourceMinLOD<P0>(&self, presource: P0) -> f32
    where
        P0: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).GetResourceMinLOD)(
                windows_core::Interface::as_raw(self),
                presource.param().abi(),
            )
        }
    }
    pub unsafe fn ResolveSubresource<P0, P2>(
        &self,
        pdstresource: P0,
        dstsubresource: u32,
        psrcresource: P2,
        srcsubresource: u32,
        format: DXGI_FORMAT,
    ) where
        P0: windows_core::Param<ID3D11Resource>,
        P2: windows_core::Param<ID3D11Resource>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ResolveSubresource)(
                windows_core::Interface::as_raw(self),
                pdstresource.param().abi(),
                dstsubresource,
                psrcresource.param().abi(),
                srcsubresource,
                format,
            );
        }
    }
    pub unsafe fn ExecuteCommandList<P0>(&self, pcommandlist: P0, restorecontextstate: bool)
    where
        P0: windows_core::Param<ID3D11CommandList>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).ExecuteCommandList)(
                windows_core::Interface::as_raw(self),
                pcommandlist.param().abi(),
                restorecontextstate.into(),
            );
        }
    }
    pub unsafe fn HSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn HSSetShader<P0>(
        &self,
        phullshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11HullShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetShader)(
                windows_core::Interface::as_raw(self),
                phullshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub unsafe fn HSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub unsafe fn HSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn DSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn DSSetShader<P0>(
        &self,
        pdomainshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11DomainShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetShader)(
                windows_core::Interface::as_raw(self),
                pdomainshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub unsafe fn DSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub unsafe fn DSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn CSSetShaderResources(
        &self,
        startslot: u32,
        ppshaderresourceviews: Option<&[Option<ID3D11ShaderResourceView>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppshaderresourceviews.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppshaderresourceviews.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn CSSetUnorderedAccessViews(
        &self,
        startslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*const Option<ID3D11UnorderedAccessView>>,
        puavinitialcounts: Option<*const u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                startslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
                puavinitialcounts.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn CSSetShader<P0>(
        &self,
        pcomputeshader: P0,
        ppclassinstances: Option<&[Option<ID3D11ClassInstance>]>,
    ) where
        P0: windows_core::Param<ID3D11ComputeShader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetShader)(
                windows_core::Interface::as_raw(self),
                pcomputeshader.param().abi(),
                core::mem::transmute(
                    ppclassinstances.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
                ppclassinstances.map_or(0, |slice| slice.len().try_into().unwrap()),
            );
        }
    }
    pub unsafe fn CSSetSamplers(
        &self,
        startslot: u32,
        ppsamplers: Option<&[Option<ID3D11SamplerState>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppsamplers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(ppsamplers.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            );
        }
    }
    pub unsafe fn CSSetConstantBuffers(
        &self,
        startslot: u32,
        ppconstantbuffers: Option<&[Option<ID3D11Buffer>]>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSSetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                ppconstantbuffers.map_or(0, |slice| slice.len().try_into().unwrap()),
                core::mem::transmute(
                    ppconstantbuffers.map_or(core::ptr::null(), |slice| slice.as_ptr()),
                ),
            );
        }
    }
    pub unsafe fn VSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn PSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn PSGetShader(
        &self,
        pppixelshader: *mut Option<ID3D11PixelShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pppixelshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn PSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn VSGetShader(
        &self,
        ppvertexshader: *mut Option<ID3D11VertexShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppvertexshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn PSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).PSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn IAGetInputLayout(&self) -> windows_core::Result<ID3D11InputLayout> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetInputLayout)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn IAGetVertexBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppvertexbuffers: Option<*mut Option<ID3D11Buffer>>,
        pstrides: Option<*mut u32>,
        poffsets: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetVertexBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppvertexbuffers.unwrap_or(core::mem::zeroed()) as _,
                pstrides.unwrap_or(core::mem::zeroed()) as _,
                poffsets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn IAGetIndexBuffer(
        &self,
        pindexbuffer: Option<*mut Option<ID3D11Buffer>>,
        format: Option<*mut DXGI_FORMAT>,
        offset: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).IAGetIndexBuffer)(
                windows_core::Interface::as_raw(self),
                pindexbuffer.unwrap_or(core::mem::zeroed()) as _,
                format.unwrap_or(core::mem::zeroed()) as _,
                offset.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn GSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn GSGetShader(
        &self,
        ppgeometryshader: *mut Option<ID3D11GeometryShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppgeometryshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn IAGetPrimitiveTopology(&self) -> D3D_PRIMITIVE_TOPOLOGY {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IAGetPrimitiveTopology)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            result__
        }
    }
    pub unsafe fn VSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn VSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).VSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn GetPredication(
        &self,
        pppredicate: Option<*mut Option<ID3D11Predicate>>,
        ppredicatevalue: Option<*mut windows_core::BOOL>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GetPredication)(
                windows_core::Interface::as_raw(self),
                pppredicate.unwrap_or(core::mem::zeroed()) as _,
                ppredicatevalue.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn GSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn GSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).GSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn OMGetRenderTargets(
        &self,
        numviews: u32,
        pprendertargetviews: Option<*mut Option<ID3D11RenderTargetView>>,
        ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetRenderTargets)(
                windows_core::Interface::as_raw(self),
                numviews,
                pprendertargetviews.unwrap_or(core::mem::zeroed()) as _,
                ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn OMGetRenderTargetsAndUnorderedAccessViews(
        &self,
        numrtvs: u32,
        pprendertargetviews: Option<*mut Option<ID3D11RenderTargetView>>,
        ppdepthstencilview: Option<*mut Option<ID3D11DepthStencilView>>,
        uavstartslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*mut Option<ID3D11UnorderedAccessView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetRenderTargetsAndUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                numrtvs,
                pprendertargetviews.unwrap_or(core::mem::zeroed()) as _,
                ppdepthstencilview.unwrap_or(core::mem::zeroed()) as _,
                uavstartslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn OMGetBlendState(
        &self,
        ppblendstate: Option<*mut Option<ID3D11BlendState>>,
        blendfactor: Option<*mut f32>,
        psamplemask: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetBlendState)(
                windows_core::Interface::as_raw(self),
                ppblendstate.unwrap_or(core::mem::zeroed()) as _,
                blendfactor.unwrap_or(core::mem::zeroed()) as _,
                psamplemask.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn OMGetDepthStencilState(
        &self,
        ppdepthstencilstate: Option<*mut Option<ID3D11DepthStencilState>>,
        pstencilref: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).OMGetDepthStencilState)(
                windows_core::Interface::as_raw(self),
                ppdepthstencilstate.unwrap_or(core::mem::zeroed()) as _,
                pstencilref.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn SOGetTargets(
        &self,
        numbuffers: u32,
        ppsotargets: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).SOGetTargets)(
                windows_core::Interface::as_raw(self),
                numbuffers,
                ppsotargets.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn RSGetState(&self) -> windows_core::Result<ID3D11RasterizerState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RSGetState)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            );
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn RSGetViewports(
        &self,
        pnumviewports: *mut u32,
        pviewports: Option<*mut D3D11_VIEWPORT>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetViewports)(
                windows_core::Interface::as_raw(self),
                pnumviewports as _,
                pviewports.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: Option<*mut RECT>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetScissorRects)(
                windows_core::Interface::as_raw(self),
                pnumrects as _,
                prects.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn HSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn HSGetShader(
        &self,
        pphullshader: *mut Option<ID3D11HullShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(pphullshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn HSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn HSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).HSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DSGetShader(
        &self,
        ppdomainshader: *mut Option<ID3D11DomainShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppdomainshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn DSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).DSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn CSGetShaderResources(
        &self,
        startslot: u32,
        numviews: u32,
        ppshaderresourceviews: Option<*mut Option<ID3D11ShaderResourceView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetShaderResources)(
                windows_core::Interface::as_raw(self),
                startslot,
                numviews,
                ppshaderresourceviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn CSGetUnorderedAccessViews(
        &self,
        startslot: u32,
        numuavs: u32,
        ppunorderedaccessviews: Option<*mut Option<ID3D11UnorderedAccessView>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetUnorderedAccessViews)(
                windows_core::Interface::as_raw(self),
                startslot,
                numuavs,
                ppunorderedaccessviews.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn CSGetShader(
        &self,
        ppcomputeshader: *mut Option<ID3D11ComputeShader>,
        ppclassinstances: Option<*mut Option<ID3D11ClassInstance>>,
        pnumclassinstances: Option<*mut u32>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetShader)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppcomputeshader),
                ppclassinstances.unwrap_or(core::mem::zeroed()) as _,
                pnumclassinstances.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn CSGetSamplers(
        &self,
        startslot: u32,
        numsamplers: u32,
        ppsamplers: Option<*mut Option<ID3D11SamplerState>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetSamplers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numsamplers,
                ppsamplers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn CSGetConstantBuffers(
        &self,
        startslot: u32,
        numbuffers: u32,
        ppconstantbuffers: Option<*mut Option<ID3D11Buffer>>,
    ) {
        unsafe {
            (windows_core::Interface::vtable(self).CSGetConstantBuffers)(
                windows_core::Interface::as_raw(self),
                startslot,
                numbuffers,
                ppconstantbuffers.unwrap_or(core::mem::zeroed()) as _,
            );
        }
    }
    pub unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(
                self,
            ));
        }
    }
    pub unsafe fn Flush(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetType(&self) -> D3D11_DEVICE_CONTEXT_TYPE {
        unsafe {
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetContextFlags)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn FinishCommandList(
        &self,
        restoredeferredcontextstate: bool,
        ppcommandlist: Option<*mut Option<ID3D11CommandList>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).FinishCommandList)(
                windows_core::Interface::as_raw(self),
                restoredeferredcontextstate.into(),
                ppcommandlist.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct ID3D11DeviceContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    pub VSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub PSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub PSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    pub Map: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        D3D11_MAP,
        u32,
        *mut D3D11_MAPPED_SUBRESOURCE,
    ) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub PSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub IASetInputLayout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub IASetVertexBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
        *const u32,
    ),
    pub IASetIndexBuffer:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DXGI_FORMAT, u32),
    pub DrawIndexedInstanced:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
    pub GSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub IASetPrimitiveTopology:
        unsafe extern "system" fn(*mut core::ffi::c_void, D3D_PRIMITIVE_TOPOLOGY),
    pub VSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub GetData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
    ) -> windows_core::HRESULT,
    pub SetPredication: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ),
    pub GSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub GSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub OMSetRenderTargets: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub OMSetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
    ),
    pub OMSetBlendState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32, u32),
    pub OMSetDepthStencilState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub SOSetTargets: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
    ),
    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub DrawIndexedInstancedIndirect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub DrawInstancedIndirect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    pub DispatchIndirect:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32),
    pub RSSetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub RSSetViewports:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const D3D11_VIEWPORT),
    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const RECT),
    pub CopySubresourceRegion: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        u32,
        *mut core::ffi::c_void,
        u32,
        *const D3D11_BOX,
    ),
    pub CopyResource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ),
    pub UpdateSubresource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *const D3D11_BOX,
        *const core::ffi::c_void,
        u32,
        u32,
    ),
    pub CopyStructureCount: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
    ),
    pub ClearRenderTargetView:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearUnorderedAccessViewUint:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u32),
    pub ClearUnorderedAccessViewFloat:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const f32),
    pub ClearDepthStencilView:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, f32, u8),
    pub GenerateMips: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub SetResourceMinLOD:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, f32),
    pub GetResourceMinLOD:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> f32,
    pub ResolveSubresource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        u32,
        DXGI_FORMAT,
    ),
    pub ExecuteCommandList: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ),
    pub HSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub HSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub HSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub DSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub DSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetUnorderedAccessViews: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *const *mut core::ffi::c_void,
        *const u32,
    ),
    pub CSSetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        u32,
    ),
    pub CSSetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub CSSetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const *mut core::ffi::c_void),
    pub VSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub PSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub PSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub PSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub IAGetInputLayout:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub IAGetVertexBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
    ),
    pub IAGetIndexBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut DXGI_FORMAT,
        *mut u32,
    ),
    pub GSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub IAGetPrimitiveTopology:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut D3D_PRIMITIVE_TOPOLOGY),
    pub VSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub VSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GetPredication: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ),
    pub GSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub GSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub OMGetRenderTargets: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ),
    pub OMGetRenderTargetsAndUnorderedAccessViews: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        u32,
        u32,
        *mut *mut core::ffi::c_void,
    ),
    pub OMGetBlendState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut f32,
        *mut u32,
    ),
    pub OMGetDepthStencilState:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32),
    pub SOGetTargets:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void),
    pub RSGetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub RSGetViewports:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut D3D11_VIEWPORT),
    pub RSGetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut RECT),
    pub HSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub HSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub HSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub DSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub DSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShaderResources:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetUnorderedAccessViews:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetShader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        *mut u32,
    ),
    pub CSGetSamplers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub CSGetConstantBuffers:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void),
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> D3D11_DEVICE_CONTEXT_TYPE,
    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub FinishCommandList: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D11DeviceContext {}
unsafe impl Sync for ID3D11DeviceContext {}
impl windows_core::RuntimeName for ID3D11DeviceContext {}
windows_core::imp::define_interface!(
    ID3D11DomainShader,
    ID3D11DomainShader_Vtbl,
    0xf582c508_0f36_490c_9977_31eece268cfa
);
impl core::ops::Deref for ID3D11DomainShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11DomainShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11DomainShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
unsafe impl Send for ID3D11DomainShader {}
unsafe impl Sync for ID3D11DomainShader {}
impl windows_core::RuntimeName for ID3D11DomainShader {}
windows_core::imp::define_interface!(
    ID3D11GeometryShader,
    ID3D11GeometryShader_Vtbl,
    0x38325b96_effb_4022_ba02_2e795b70275c
);
impl core::ops::Deref for ID3D11GeometryShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11GeometryShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11GeometryShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
unsafe impl Send for ID3D11GeometryShader {}
unsafe impl Sync for ID3D11GeometryShader {}
impl windows_core::RuntimeName for ID3D11GeometryShader {}
windows_core::imp::define_interface!(
    ID3D11HullShader,
    ID3D11HullShader_Vtbl,
    0x8e5c6061_628a_4c8e_8264_bbe45cb3d5dd
);
impl core::ops::Deref for ID3D11HullShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11HullShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11HullShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
unsafe impl Send for ID3D11HullShader {}
unsafe impl Sync for ID3D11HullShader {}
impl windows_core::RuntimeName for ID3D11HullShader {}
windows_core::imp::define_interface!(
    ID3D11InputLayout,
    ID3D11InputLayout_Vtbl,
    0xe4819ddc_4cf0_4025_bd26_5de82a3e07b7
);
impl core::ops::Deref for ID3D11InputLayout {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11InputLayout,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11InputLayout_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
unsafe impl Send for ID3D11InputLayout {}
unsafe impl Sync for ID3D11InputLayout {}
impl windows_core::RuntimeName for ID3D11InputLayout {}
windows_core::imp::define_interface!(
    ID3D11PixelShader,
    ID3D11PixelShader_Vtbl,
    0xea82e40d_51dc_4f33_93d4_db7c9125ae8c
);
impl core::ops::Deref for ID3D11PixelShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11PixelShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11PixelShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
unsafe impl Send for ID3D11PixelShader {}
unsafe impl Sync for ID3D11PixelShader {}
impl windows_core::RuntimeName for ID3D11PixelShader {}
windows_core::imp::define_interface!(
    ID3D11Predicate,
    ID3D11Predicate_Vtbl,
    0x9eb576dd_9f77_4d86_81aa_8bab5fe490e2
);
impl core::ops::Deref for ID3D11Predicate {
    type Target = ID3D11Query;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Predicate,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Asynchronous,
    ID3D11Query
);
#[repr(C)]
pub struct ID3D11Predicate_Vtbl {
    pub base__: ID3D11Query_Vtbl,
}
unsafe impl Send for ID3D11Predicate {}
unsafe impl Sync for ID3D11Predicate {}
impl windows_core::RuntimeName for ID3D11Predicate {}
windows_core::imp::define_interface!(
    ID3D11Query,
    ID3D11Query_Vtbl,
    0xd6c00747_87b7_425e_b84d_44d108560afd
);
impl core::ops::Deref for ID3D11Query {
    type Target = ID3D11Asynchronous;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Query,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Asynchronous
);
#[repr(C)]
pub struct ID3D11Query_Vtbl {
    pub base__: ID3D11Asynchronous_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11Query {}
unsafe impl Sync for ID3D11Query {}
impl windows_core::RuntimeName for ID3D11Query {}
windows_core::imp::define_interface!(
    ID3D11RasterizerState,
    ID3D11RasterizerState_Vtbl,
    0x9bb4ab81_ab1a_4d8f_b506_fc04200b6ee7
);
impl core::ops::Deref for ID3D11RasterizerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11RasterizerState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11RasterizerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11RasterizerState {}
unsafe impl Sync for ID3D11RasterizerState {}
impl windows_core::RuntimeName for ID3D11RasterizerState {}
windows_core::imp::define_interface!(
    ID3D11RenderTargetView,
    ID3D11RenderTargetView_Vtbl,
    0xdfdba067_0b8d_4865_875b_d7b4516cc164
);
impl core::ops::Deref for ID3D11RenderTargetView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11RenderTargetView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11RenderTargetView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11RenderTargetView {}
unsafe impl Sync for ID3D11RenderTargetView {}
impl windows_core::RuntimeName for ID3D11RenderTargetView {}
windows_core::imp::define_interface!(
    ID3D11Resource,
    ID3D11Resource_Vtbl,
    0xdc8e63f3_d12b_4952_b47b_5e45026a862d
);
impl core::ops::Deref for ID3D11Resource {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11Resource, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
pub struct ID3D11Resource_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetType: usize,
    SetEvictionPriority: usize,
    GetEvictionPriority: usize,
}
unsafe impl Send for ID3D11Resource {}
unsafe impl Sync for ID3D11Resource {}
impl windows_core::RuntimeName for ID3D11Resource {}
windows_core::imp::define_interface!(
    ID3D11SamplerState,
    ID3D11SamplerState_Vtbl,
    0xda6fea51_564c_4487_9810_f0d0f9b4e3a5
);
impl core::ops::Deref for ID3D11SamplerState {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11SamplerState,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11SamplerState_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11SamplerState {}
unsafe impl Sync for ID3D11SamplerState {}
impl windows_core::RuntimeName for ID3D11SamplerState {}
windows_core::imp::define_interface!(
    ID3D11ShaderResourceView,
    ID3D11ShaderResourceView_Vtbl,
    0xb0e06fe0_8192_4e1a_b1ca_36d7414710b2
);
impl core::ops::Deref for ID3D11ShaderResourceView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11ShaderResourceView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11ShaderResourceView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11ShaderResourceView {}
unsafe impl Sync for ID3D11ShaderResourceView {}
impl windows_core::RuntimeName for ID3D11ShaderResourceView {}
windows_core::imp::define_interface!(
    ID3D11Texture1D,
    ID3D11Texture1D_Vtbl,
    0xf8fb5c27_c6b3_4f75_a4c8_439af2ef564c
);
impl core::ops::Deref for ID3D11Texture1D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Texture1D,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
#[repr(C)]
pub struct ID3D11Texture1D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11Texture1D {}
unsafe impl Sync for ID3D11Texture1D {}
impl windows_core::RuntimeName for ID3D11Texture1D {}
windows_core::imp::define_interface!(
    ID3D11Texture2D,
    ID3D11Texture2D_Vtbl,
    0x6f15aaf2_d208_4e89_9ab4_489535d34f9c
);
impl core::ops::Deref for ID3D11Texture2D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Texture2D,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
#[repr(C)]
pub struct ID3D11Texture2D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11Texture2D {}
unsafe impl Sync for ID3D11Texture2D {}
impl windows_core::RuntimeName for ID3D11Texture2D {}
windows_core::imp::define_interface!(
    ID3D11Texture3D,
    ID3D11Texture3D_Vtbl,
    0x037e866e_f56d_4357_a8af_9dabbe6e250e
);
impl core::ops::Deref for ID3D11Texture3D {
    type Target = ID3D11Resource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11Texture3D,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11Resource
);
#[repr(C)]
pub struct ID3D11Texture3D_Vtbl {
    pub base__: ID3D11Resource_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11Texture3D {}
unsafe impl Sync for ID3D11Texture3D {}
impl windows_core::RuntimeName for ID3D11Texture3D {}
windows_core::imp::define_interface!(
    ID3D11UnorderedAccessView,
    ID3D11UnorderedAccessView_Vtbl,
    0x28acf509_7f5c_48f6_8611_f316010a6380
);
impl core::ops::Deref for ID3D11UnorderedAccessView {
    type Target = ID3D11View;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11UnorderedAccessView,
    windows_core::IUnknown,
    ID3D11DeviceChild,
    ID3D11View
);
#[repr(C)]
pub struct ID3D11UnorderedAccessView_Vtbl {
    pub base__: ID3D11View_Vtbl,
    GetDesc: usize,
}
unsafe impl Send for ID3D11UnorderedAccessView {}
unsafe impl Sync for ID3D11UnorderedAccessView {}
impl windows_core::RuntimeName for ID3D11UnorderedAccessView {}
windows_core::imp::define_interface!(
    ID3D11VertexShader,
    ID3D11VertexShader_Vtbl,
    0x3b301d64_d678_4289_8897_22f8928b72f3
);
impl core::ops::Deref for ID3D11VertexShader {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    ID3D11VertexShader,
    windows_core::IUnknown,
    ID3D11DeviceChild
);
#[repr(C)]
pub struct ID3D11VertexShader_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
}
unsafe impl Send for ID3D11VertexShader {}
unsafe impl Sync for ID3D11VertexShader {}
impl windows_core::RuntimeName for ID3D11VertexShader {}
windows_core::imp::define_interface!(
    ID3D11View,
    ID3D11View_Vtbl,
    0x839d1216_bb2e_412b_b7f4_a9dbebe08ed1
);
impl core::ops::Deref for ID3D11View {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11View, windows_core::IUnknown, ID3D11DeviceChild);
#[repr(C)]
pub struct ID3D11View_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    GetResource: usize,
}
unsafe impl Send for ID3D11View {}
unsafe impl Sync for ID3D11View {}
impl windows_core::RuntimeName for ID3D11View {}
windows_core::imp::define_interface!(
    IDWriteFactory,
    IDWriteFactory_Vtbl,
    0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48
);
windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
impl IDWriteFactory {
    pub unsafe fn GetSystemFontCollection(
        &self,
        fontcollection: *mut Option<IDWriteFontCollection>,
        checkforupdates: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetSystemFontCollection)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(fontcollection),
                checkforupdates.into(),
            )
            .ok()
        }
    }
    pub unsafe fn CreateCustomFontCollection<P0>(
        &self,
        collectionloader: P0,
        collectionkey: *const core::ffi::c_void,
        collectionkeysize: u32,
    ) -> windows_core::Result<IDWriteFontCollection>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomFontCollection)(
                windows_core::Interface::as_raw(self),
                collectionloader.param().abi(),
                collectionkey,
                collectionkeysize,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterFontCollectionLoader<P0>(
        &self,
        fontcollectionloader: P0,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RegisterFontCollectionLoader)(
                windows_core::Interface::as_raw(self),
                fontcollectionloader.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn UnregisterFontCollectionLoader<P0>(
        &self,
        fontcollectionloader: P0,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontCollectionLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterFontCollectionLoader)(
                windows_core::Interface::as_raw(self),
                fontcollectionloader.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn CreateFontFileReference<P0>(
        &self,
        filepath: P0,
        lastwritetime: Option<*const FILETIME>,
    ) -> windows_core::Result<IDWriteFontFile>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFileReference)(
                windows_core::Interface::as_raw(self),
                filepath.param().abi(),
                lastwritetime.unwrap_or(core::mem::zeroed()) as _,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomFontFileReference<P2>(
        &self,
        fontfilereferencekey: *const core::ffi::c_void,
        fontfilereferencekeysize: u32,
        fontfileloader: P2,
    ) -> windows_core::Result<IDWriteFontFile>
    where
        P2: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomFontFileReference)(
                windows_core::Interface::as_raw(self),
                fontfilereferencekey,
                fontfilereferencekeysize,
                fontfileloader.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFontFace(
        &self,
        fontfacetype: DWRITE_FONT_FACE_TYPE,
        fontfiles: &[Option<IDWriteFontFile>],
        faceindex: u32,
        fontfacesimulationflags: DWRITE_FONT_SIMULATIONS,
    ) -> windows_core::Result<IDWriteFontFace> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFontFace)(
                windows_core::Interface::as_raw(self),
                fontfacetype,
                fontfiles.len().try_into().unwrap(),
                core::mem::transmute(fontfiles.as_ptr()),
                faceindex,
                fontfacesimulationflags,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRenderingParams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMonitorRenderingParams(
        &self,
        monitor: HMONITOR,
    ) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMonitorRenderingParams)(
                windows_core::Interface::as_raw(self),
                monitor,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCustomRenderingParams(
        &self,
        gamma: f32,
        enhancedcontrast: f32,
        cleartypelevel: f32,
        pixelgeometry: DWRITE_PIXEL_GEOMETRY,
        renderingmode: DWRITE_RENDERING_MODE,
    ) -> windows_core::Result<IDWriteRenderingParams> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCustomRenderingParams)(
                windows_core::Interface::as_raw(self),
                gamma,
                enhancedcontrast,
                cleartypelevel,
                pixelgeometry,
                renderingmode,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterFontFileLoader<P0>(&self, fontfileloader: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).RegisterFontFileLoader)(
                windows_core::Interface::as_raw(self),
                fontfileloader.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn UnregisterFontFileLoader<P0>(
        &self,
        fontfileloader: P0,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontFileLoader>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterFontFileLoader)(
                windows_core::Interface::as_raw(self),
                fontfileloader.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn CreateTextFormat<P0, P1, P6>(
        &self,
        fontfamilyname: P0,
        fontcollection: P1,
        fontweight: DWRITE_FONT_WEIGHT,
        fontstyle: DWRITE_FONT_STYLE,
        fontstretch: DWRITE_FONT_STRETCH,
        fontsize: f32,
        localename: P6,
    ) -> windows_core::Result<IDWriteTextFormat>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextFormat)(
                windows_core::Interface::as_raw(self),
                fontfamilyname.param().abi(),
                fontcollection.param().abi(),
                fontweight,
                fontstyle,
                fontstretch,
                fontsize,
                localename.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTypography)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGdiInterop)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextLayout<P2>(
        &self,
        string: &[u16],
        textformat: P2,
        maxwidth: f32,
        maxheight: f32,
    ) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextLayout)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(string.as_ptr()),
                string.len().try_into().unwrap(),
                textformat.param().abi(),
                maxwidth,
                maxheight,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGdiCompatibleTextLayout<P2>(
        &self,
        string: &[u16],
        textformat: P2,
        layoutwidth: f32,
        layoutheight: f32,
        pixelsperdip: f32,
        transform: Option<*const DWRITE_MATRIX>,
        usegdinatural: bool,
    ) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGdiCompatibleTextLayout)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(string.as_ptr()),
                string.len().try_into().unwrap(),
                textformat.param().abi(),
                layoutwidth,
                layoutheight,
                pixelsperdip,
                transform.unwrap_or(core::mem::zeroed()) as _,
                usegdinatural.into(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateEllipsisTrimmingSign<P0>(
        &self,
        textformat: P0,
    ) -> windows_core::Result<IDWriteInlineObject>
    where
        P0: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEllipsisTrimmingSign)(
                windows_core::Interface::as_raw(self),
                textformat.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextAnalyzer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateNumberSubstitution<P1>(
        &self,
        substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
        localename: P1,
        ignoreuseroverride: bool,
    ) -> windows_core::Result<IDWriteNumberSubstitution>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNumberSubstitution)(
                windows_core::Interface::as_raw(self),
                substitutionmethod,
                localename.param().abi(),
                ignoreuseroverride.into(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateGlyphRunAnalysis(
        &self,
        glyphrun: *const DWRITE_GLYPH_RUN,
        pixelsperdip: f32,
        transform: Option<*const DWRITE_MATRIX>,
        renderingmode: DWRITE_RENDERING_MODE,
        measuringmode: DWRITE_MEASURING_MODE,
        baselineoriginx: f32,
        baselineoriginy: f32,
    ) -> windows_core::Result<IDWriteGlyphRunAnalysis> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateGlyphRunAnalysis)(
                windows_core::Interface::as_raw(self),
                glyphrun,
                pixelsperdip,
                transform.unwrap_or(core::mem::zeroed()) as _,
                renderingmode,
                measuringmode,
                baselineoriginx,
                baselineoriginy,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSystemFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub CreateCustomFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RegisterFontCollectionLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UnregisterFontCollectionLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFontFileReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *const FILETIME,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCustomFontFileReference: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateFontFace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_FACE_TYPE,
        u32,
        *const *mut core::ffi::c_void,
        u32,
        DWRITE_FONT_SIMULATIONS,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateRenderingParams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateMonitorRenderingParams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HMONITOR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateCustomRenderingParams: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        f32,
        f32,
        DWRITE_PIXEL_GEOMETRY,
        DWRITE_RENDERING_MODE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub RegisterFontFileLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub UnregisterFontFileLoader: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTextFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_FONT_STYLE,
        DWRITE_FONT_STRETCH,
        f32,
        windows_core::PCWSTR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetGdiInterop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
        *mut core::ffi::c_void,
        f32,
        f32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGdiCompatibleTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        u32,
        *mut core::ffi::c_void,
        f32,
        f32,
        f32,
        *const DWRITE_MATRIX,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateEllipsisTrimmingSign: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTextAnalyzer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateNumberSubstitution: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_NUMBER_SUBSTITUTION_METHOD,
        windows_core::PCWSTR,
        windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateGlyphRunAnalysis: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DWRITE_GLYPH_RUN,
        f32,
        *const DWRITE_MATRIX,
        DWRITE_RENDERING_MODE,
        DWRITE_MEASURING_MODE,
        f32,
        f32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for IDWriteFactory {}
unsafe impl Sync for IDWriteFactory {}
pub trait IDWriteFactory_Impl: windows_core::IUnknownImpl {
    fn GetSystemFontCollection(
        &self,
        fontcollection: windows_core::OutRef<IDWriteFontCollection>,
        checkforupdates: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn CreateCustomFontCollection(
        &self,
        collectionloader: windows_core::Ref<IDWriteFontCollectionLoader>,
        collectionkey: *const core::ffi::c_void,
        collectionkeysize: u32,
    ) -> windows_core::Result<IDWriteFontCollection>;
    fn RegisterFontCollectionLoader(
        &self,
        fontcollectionloader: windows_core::Ref<IDWriteFontCollectionLoader>,
    ) -> windows_core::Result<()>;
    fn UnregisterFontCollectionLoader(
        &self,
        fontcollectionloader: windows_core::Ref<IDWriteFontCollectionLoader>,
    ) -> windows_core::Result<()>;
    fn CreateFontFileReference(
        &self,
        filepath: &windows_core::PCWSTR,
        lastwritetime: *const FILETIME,
    ) -> windows_core::Result<IDWriteFontFile>;
    fn CreateCustomFontFileReference(
        &self,
        fontfilereferencekey: *const core::ffi::c_void,
        fontfilereferencekeysize: u32,
        fontfileloader: windows_core::Ref<IDWriteFontFileLoader>,
    ) -> windows_core::Result<IDWriteFontFile>;
    fn CreateFontFace(
        &self,
        fontfacetype: DWRITE_FONT_FACE_TYPE,
        numberoffiles: u32,
        fontfiles: *const Option<IDWriteFontFile>,
        faceindex: u32,
        fontfacesimulationflags: DWRITE_FONT_SIMULATIONS,
    ) -> windows_core::Result<IDWriteFontFace>;
    fn CreateRenderingParams(&self) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateMonitorRenderingParams(
        &self,
        monitor: HMONITOR,
    ) -> windows_core::Result<IDWriteRenderingParams>;
    fn CreateCustomRenderingParams(
        &self,
        gamma: f32,
        enhancedcontrast: f32,
        cleartypelevel: f32,
        pixelgeometry: DWRITE_PIXEL_GEOMETRY,
        renderingmode: DWRITE_RENDERING_MODE,
    ) -> windows_core::Result<IDWriteRenderingParams>;
    fn RegisterFontFileLoader(
        &self,
        fontfileloader: windows_core::Ref<IDWriteFontFileLoader>,
    ) -> windows_core::Result<()>;
    fn UnregisterFontFileLoader(
        &self,
        fontfileloader: windows_core::Ref<IDWriteFontFileLoader>,
    ) -> windows_core::Result<()>;
    fn CreateTextFormat(
        &self,
        fontfamilyname: &windows_core::PCWSTR,
        fontcollection: windows_core::Ref<IDWriteFontCollection>,
        fontweight: DWRITE_FONT_WEIGHT,
        fontstyle: DWRITE_FONT_STYLE,
        fontstretch: DWRITE_FONT_STRETCH,
        fontsize: f32,
        localename: &windows_core::PCWSTR,
    ) -> windows_core::Result<IDWriteTextFormat>;
    fn CreateTypography(&self) -> windows_core::Result<IDWriteTypography>;
    fn GetGdiInterop(&self) -> windows_core::Result<IDWriteGdiInterop>;
    fn CreateTextLayout(
        &self,
        string: &windows_core::PCWSTR,
        stringlength: u32,
        textformat: windows_core::Ref<IDWriteTextFormat>,
        maxwidth: f32,
        maxheight: f32,
    ) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateGdiCompatibleTextLayout(
        &self,
        string: &windows_core::PCWSTR,
        stringlength: u32,
        textformat: windows_core::Ref<IDWriteTextFormat>,
        layoutwidth: f32,
        layoutheight: f32,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        usegdinatural: windows_core::BOOL,
    ) -> windows_core::Result<IDWriteTextLayout>;
    fn CreateEllipsisTrimmingSign(
        &self,
        textformat: windows_core::Ref<IDWriteTextFormat>,
    ) -> windows_core::Result<IDWriteInlineObject>;
    fn CreateTextAnalyzer(&self) -> windows_core::Result<IDWriteTextAnalyzer>;
    fn CreateNumberSubstitution(
        &self,
        substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
        localename: &windows_core::PCWSTR,
        ignoreuseroverride: windows_core::BOOL,
    ) -> windows_core::Result<IDWriteNumberSubstitution>;
    fn CreateGlyphRunAnalysis(
        &self,
        glyphrun: *const DWRITE_GLYPH_RUN,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        renderingmode: DWRITE_RENDERING_MODE,
        measuringmode: DWRITE_MEASURING_MODE,
        baselineoriginx: f32,
        baselineoriginy: f32,
    ) -> windows_core::Result<IDWriteGlyphRunAnalysis>;
}
impl IDWriteFactory_Vtbl {
    pub const fn new<Identity: IDWriteFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSystemFontCollection<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollection: *mut *mut core::ffi::c_void,
            checkforupdates: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::GetSystemFontCollection(
                    this,
                    core::mem::transmute_copy(&fontcollection),
                    core::mem::transmute_copy(&checkforupdates),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateCustomFontCollection<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            collectionloader: *mut core::ffi::c_void,
            collectionkey: *const core::ffi::c_void,
            collectionkeysize: u32,
            fontcollection: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomFontCollection(
                    this,
                    core::mem::transmute_copy(&collectionloader),
                    core::mem::transmute_copy(&collectionkey),
                    core::mem::transmute_copy(&collectionkeysize),
                ) {
                    Ok(ok__) => {
                        fontcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollectionloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::RegisterFontCollectionLoader(
                    this,
                    core::mem::transmute_copy(&fontcollectionloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontcollectionloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::UnregisterFontCollectionLoader(
                    this,
                    core::mem::transmute_copy(&fontcollectionloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateFontFileReference<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            filepath: windows_core::PCWSTR,
            lastwritetime: *const FILETIME,
            fontfile: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateFontFileReference(
                    this,
                    core::mem::transmute(&filepath),
                    core::mem::transmute_copy(&lastwritetime),
                ) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomFontFileReference<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfilereferencekey: *const core::ffi::c_void,
            fontfilereferencekeysize: u32,
            fontfileloader: *mut core::ffi::c_void,
            fontfile: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomFontFileReference(
                    this,
                    core::mem::transmute_copy(&fontfilereferencekey),
                    core::mem::transmute_copy(&fontfilereferencekeysize),
                    core::mem::transmute_copy(&fontfileloader),
                ) {
                    Ok(ok__) => {
                        fontfile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFontFace<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfacetype: DWRITE_FONT_FACE_TYPE,
            numberoffiles: u32,
            fontfiles: *const *mut core::ffi::c_void,
            faceindex: u32,
            fontfacesimulationflags: DWRITE_FONT_SIMULATIONS,
            fontface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateFontFace(
                    this,
                    core::mem::transmute_copy(&fontfacetype),
                    core::mem::transmute_copy(&numberoffiles),
                    core::mem::transmute_copy(&fontfiles),
                    core::mem::transmute_copy(&faceindex),
                    core::mem::transmute_copy(&fontfacesimulationflags),
                ) {
                    Ok(ok__) => {
                        fontface.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateRenderingParams<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            renderingparams: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateRenderingParams(this) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            monitor: HMONITOR,
            renderingparams: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateMonitorRenderingParams(
                    this,
                    core::mem::transmute_copy(&monitor),
                ) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCustomRenderingParams<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            gamma: f32,
            enhancedcontrast: f32,
            cleartypelevel: f32,
            pixelgeometry: DWRITE_PIXEL_GEOMETRY,
            renderingmode: DWRITE_RENDERING_MODE,
            renderingparams: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateCustomRenderingParams(
                    this,
                    core::mem::transmute_copy(&gamma),
                    core::mem::transmute_copy(&enhancedcontrast),
                    core::mem::transmute_copy(&cleartypelevel),
                    core::mem::transmute_copy(&pixelgeometry),
                    core::mem::transmute_copy(&renderingmode),
                ) {
                    Ok(ok__) => {
                        renderingparams.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterFontFileLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfileloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::RegisterFontFileLoader(
                    this,
                    core::mem::transmute_copy(&fontfileloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn UnregisterFontFileLoader<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfileloader: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDWriteFactory_Impl::UnregisterFontFileLoader(
                    this,
                    core::mem::transmute_copy(&fontfileloader),
                )
                .into()
            }
        }
        unsafe extern "system" fn CreateTextFormat<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fontfamilyname: windows_core::PCWSTR,
            fontcollection: *mut core::ffi::c_void,
            fontweight: DWRITE_FONT_WEIGHT,
            fontstyle: DWRITE_FONT_STYLE,
            fontstretch: DWRITE_FONT_STRETCH,
            fontsize: f32,
            localename: windows_core::PCWSTR,
            textformat: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextFormat(
                    this,
                    core::mem::transmute(&fontfamilyname),
                    core::mem::transmute_copy(&fontcollection),
                    core::mem::transmute_copy(&fontweight),
                    core::mem::transmute_copy(&fontstyle),
                    core::mem::transmute_copy(&fontstretch),
                    core::mem::transmute_copy(&fontsize),
                    core::mem::transmute(&localename),
                ) {
                    Ok(ok__) => {
                        textformat.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTypography<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            typography: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTypography(this) {
                    Ok(ok__) => {
                        typography.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGdiInterop<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            gdiinterop: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::GetGdiInterop(this) {
                    Ok(ok__) => {
                        gdiinterop.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTextLayout<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            string: windows_core::PCWSTR,
            stringlength: u32,
            textformat: *mut core::ffi::c_void,
            maxwidth: f32,
            maxheight: f32,
            textlayout: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextLayout(
                    this,
                    core::mem::transmute(&string),
                    core::mem::transmute_copy(&stringlength),
                    core::mem::transmute_copy(&textformat),
                    core::mem::transmute_copy(&maxwidth),
                    core::mem::transmute_copy(&maxheight),
                ) {
                    Ok(ok__) => {
                        textlayout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            string: windows_core::PCWSTR,
            stringlength: u32,
            textformat: *mut core::ffi::c_void,
            layoutwidth: f32,
            layoutheight: f32,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            usegdinatural: windows_core::BOOL,
            textlayout: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateGdiCompatibleTextLayout(
                    this,
                    core::mem::transmute(&string),
                    core::mem::transmute_copy(&stringlength),
                    core::mem::transmute_copy(&textformat),
                    core::mem::transmute_copy(&layoutwidth),
                    core::mem::transmute_copy(&layoutheight),
                    core::mem::transmute_copy(&pixelsperdip),
                    core::mem::transmute_copy(&transform),
                    core::mem::transmute_copy(&usegdinatural),
                ) {
                    Ok(ok__) => {
                        textlayout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textformat: *mut core::ffi::c_void,
            trimmingsign: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateEllipsisTrimmingSign(
                    this,
                    core::mem::transmute_copy(&textformat),
                ) {
                    Ok(ok__) => {
                        trimmingsign.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTextAnalyzer<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            textanalyzer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateTextAnalyzer(this) {
                    Ok(ok__) => {
                        textanalyzer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateNumberSubstitution<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD,
            localename: windows_core::PCWSTR,
            ignoreuseroverride: windows_core::BOOL,
            numbersubstitution: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateNumberSubstitution(
                    this,
                    core::mem::transmute_copy(&substitutionmethod),
                    core::mem::transmute(&localename),
                    core::mem::transmute_copy(&ignoreuseroverride),
                ) {
                    Ok(ok__) => {
                        numbersubstitution.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<
            Identity: IDWriteFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            glyphrun: *const DWRITE_GLYPH_RUN,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            renderingmode: DWRITE_RENDERING_MODE,
            measuringmode: DWRITE_MEASURING_MODE,
            baselineoriginx: f32,
            baselineoriginy: f32,
            glyphrunanalysis: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDWriteFactory_Impl::CreateGlyphRunAnalysis(
                    this,
                    core::mem::transmute_copy(&glyphrun),
                    core::mem::transmute_copy(&pixelsperdip),
                    core::mem::transmute_copy(&transform),
                    core::mem::transmute_copy(&renderingmode),
                    core::mem::transmute_copy(&measuringmode),
                    core::mem::transmute_copy(&baselineoriginx),
                    core::mem::transmute_copy(&baselineoriginy),
                ) {
                    Ok(ok__) => {
                        glyphrunanalysis.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSystemFontCollection: GetSystemFontCollection::<Identity, OFFSET>,
            CreateCustomFontCollection: CreateCustomFontCollection::<Identity, OFFSET>,
            RegisterFontCollectionLoader: RegisterFontCollectionLoader::<Identity, OFFSET>,
            UnregisterFontCollectionLoader: UnregisterFontCollectionLoader::<Identity, OFFSET>,
            CreateFontFileReference: CreateFontFileReference::<Identity, OFFSET>,
            CreateCustomFontFileReference: CreateCustomFontFileReference::<Identity, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, OFFSET>,
            CreateRenderingParams: CreateRenderingParams::<Identity, OFFSET>,
            CreateMonitorRenderingParams: CreateMonitorRenderingParams::<Identity, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, OFFSET>,
            RegisterFontFileLoader: RegisterFontFileLoader::<Identity, OFFSET>,
            UnregisterFontFileLoader: UnregisterFontFileLoader::<Identity, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, OFFSET>,
            CreateTypography: CreateTypography::<Identity, OFFSET>,
            GetGdiInterop: GetGdiInterop::<Identity, OFFSET>,
            CreateTextLayout: CreateTextLayout::<Identity, OFFSET>,
            CreateGdiCompatibleTextLayout: CreateGdiCompatibleTextLayout::<Identity, OFFSET>,
            CreateEllipsisTrimmingSign: CreateEllipsisTrimmingSign::<Identity, OFFSET>,
            CreateTextAnalyzer: CreateTextAnalyzer::<Identity, OFFSET>,
            CreateNumberSubstitution: CreateNumberSubstitution::<Identity, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteFactory {}
windows_core::imp::define_interface!(
    IDWriteFontCollection,
    IDWriteFontCollection_Vtbl,
    0xa84cee02_3eea_4eee_a827_87c1a02a0fcc
);
windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetFontFamilyCount: usize,
    GetFontFamily: usize,
    FindFamilyName: usize,
    GetFontFromFontFace: usize,
}
unsafe impl Send for IDWriteFontCollection {}
unsafe impl Sync for IDWriteFontCollection {}
impl windows_core::RuntimeName for IDWriteFontCollection {}
windows_core::imp::define_interface!(
    IDWriteFontCollectionLoader,
    IDWriteFontCollectionLoader_Vtbl,
    0xcca920e4_52f0_492b_bfa8_29c72ee0a468
);
windows_core::imp::interface_hierarchy!(IDWriteFontCollectionLoader, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontCollectionLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateEnumeratorFromKey: usize,
}
unsafe impl Send for IDWriteFontCollectionLoader {}
unsafe impl Sync for IDWriteFontCollectionLoader {}
impl windows_core::RuntimeName for IDWriteFontCollectionLoader {}
windows_core::imp::define_interface!(
    IDWriteFontFace,
    IDWriteFontFace_Vtbl,
    0x5f49804d_7024_4d43_bfa9_d25984f53849
);
windows_core::imp::interface_hierarchy!(IDWriteFontFace, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetType: usize,
    GetFiles: usize,
    GetIndex: usize,
    GetSimulations: usize,
    IsSymbolFont: usize,
    GetMetrics: usize,
    GetGlyphCount: usize,
    GetDesignGlyphMetrics: usize,
    GetGlyphIndices: usize,
    TryGetFontTable: usize,
    ReleaseFontTable: usize,
    GetGlyphRunOutline: usize,
    GetRecommendedRenderingMode: usize,
    GetGdiCompatibleMetrics: usize,
    GetGdiCompatibleGlyphMetrics: usize,
}
unsafe impl Send for IDWriteFontFace {}
unsafe impl Sync for IDWriteFontFace {}
impl windows_core::RuntimeName for IDWriteFontFace {}
windows_core::imp::define_interface!(
    IDWriteFontFile,
    IDWriteFontFile_Vtbl,
    0x739d886a_cef5_47dc_8769_1a8b41bebbb0
);
windows_core::imp::interface_hierarchy!(IDWriteFontFile, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetReferenceKey: usize,
    GetLoader: usize,
    Analyze: usize,
}
unsafe impl Send for IDWriteFontFile {}
unsafe impl Sync for IDWriteFontFile {}
impl windows_core::RuntimeName for IDWriteFontFile {}
windows_core::imp::define_interface!(
    IDWriteFontFileLoader,
    IDWriteFontFileLoader_Vtbl,
    0x727cad4e_d6af_4c9e_8a08_d695b11caa49
);
windows_core::imp::interface_hierarchy!(IDWriteFontFileLoader, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontFileLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateStreamFromKey: usize,
}
unsafe impl Send for IDWriteFontFileLoader {}
unsafe impl Sync for IDWriteFontFileLoader {}
impl windows_core::RuntimeName for IDWriteFontFileLoader {}
windows_core::imp::define_interface!(
    IDWriteGdiInterop,
    IDWriteGdiInterop_Vtbl,
    0x1edd9491_9853_4299_898f_6432983b6f3a
);
windows_core::imp::interface_hierarchy!(IDWriteGdiInterop, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteGdiInterop_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateFontFromLOGFONT: usize,
    ConvertFontToLOGFONT: usize,
    ConvertFontFaceToLOGFONT: usize,
    CreateFontFaceFromHdc: usize,
    CreateBitmapRenderTarget: usize,
}
unsafe impl Send for IDWriteGdiInterop {}
unsafe impl Sync for IDWriteGdiInterop {}
impl windows_core::RuntimeName for IDWriteGdiInterop {}
windows_core::imp::define_interface!(
    IDWriteGlyphRunAnalysis,
    IDWriteGlyphRunAnalysis_Vtbl,
    0x7d97dbf7_e085_42d4_81e3_6a883bded118
);
windows_core::imp::interface_hierarchy!(IDWriteGlyphRunAnalysis, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteGlyphRunAnalysis_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetAlphaTextureBounds: usize,
    CreateAlphaTexture: usize,
    GetAlphaBlendParams: usize,
}
unsafe impl Send for IDWriteGlyphRunAnalysis {}
unsafe impl Sync for IDWriteGlyphRunAnalysis {}
impl windows_core::RuntimeName for IDWriteGlyphRunAnalysis {}
windows_core::imp::define_interface!(
    IDWriteInlineObject,
    IDWriteInlineObject_Vtbl,
    0x8339fde3_106f_47ab_8373_1c6295eb10b3
);
windows_core::imp::interface_hierarchy!(IDWriteInlineObject, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteInlineObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Draw: usize,
    GetMetrics: usize,
    GetOverhangMetrics: usize,
    GetBreakConditions: usize,
}
unsafe impl Send for IDWriteInlineObject {}
unsafe impl Sync for IDWriteInlineObject {}
impl windows_core::RuntimeName for IDWriteInlineObject {}
windows_core::imp::define_interface!(
    IDWriteNumberSubstitution,
    IDWriteNumberSubstitution_Vtbl,
    0x14885cc9_bab0_4f90_b6ed_5c366a2cd03d
);
windows_core::imp::interface_hierarchy!(IDWriteNumberSubstitution, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteNumberSubstitution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
unsafe impl Send for IDWriteNumberSubstitution {}
unsafe impl Sync for IDWriteNumberSubstitution {}
pub trait IDWriteNumberSubstitution_Impl: windows_core::IUnknownImpl {}
impl IDWriteNumberSubstitution_Vtbl {
    pub const fn new<Identity: IDWriteNumberSubstitution_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDWriteNumberSubstitution as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDWriteNumberSubstitution {}
windows_core::imp::define_interface!(
    IDWritePixelSnapping,
    IDWritePixelSnapping_Vtbl,
    0xeaf3a2da_ecf4_4d24_b644_b34f6842024b
);
windows_core::imp::interface_hierarchy!(IDWritePixelSnapping, windows_core::IUnknown);
#[repr(C)]
pub struct IDWritePixelSnapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    IsPixelSnappingDisabled: usize,
    GetCurrentTransform: usize,
    GetPixelsPerDip: usize,
}
unsafe impl Send for IDWritePixelSnapping {}
unsafe impl Sync for IDWritePixelSnapping {}
impl windows_core::RuntimeName for IDWritePixelSnapping {}
windows_core::imp::define_interface!(
    IDWriteRenderingParams,
    IDWriteRenderingParams_Vtbl,
    0x2f0da53a_2add_47cd_82ee_d9ec34688e75
);
windows_core::imp::interface_hierarchy!(IDWriteRenderingParams, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteRenderingParams_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetGamma: usize,
    GetEnhancedContrast: usize,
    GetClearTypeLevel: usize,
    GetPixelGeometry: usize,
    GetRenderingMode: usize,
}
unsafe impl Send for IDWriteRenderingParams {}
unsafe impl Sync for IDWriteRenderingParams {}
impl windows_core::RuntimeName for IDWriteRenderingParams {}
windows_core::imp::define_interface!(
    IDWriteTextAnalyzer,
    IDWriteTextAnalyzer_Vtbl,
    0xb7e6163e_7f46_43b4_84b3_e4e6249c365d
);
windows_core::imp::interface_hierarchy!(IDWriteTextAnalyzer, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteTextAnalyzer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AnalyzeScript: usize,
    AnalyzeBidi: usize,
    AnalyzeNumberSubstitution: usize,
    AnalyzeLineBreakpoints: usize,
    GetGlyphs: usize,
    GetGlyphPlacements: usize,
    GetGdiCompatibleGlyphPlacements: usize,
}
unsafe impl Send for IDWriteTextAnalyzer {}
unsafe impl Sync for IDWriteTextAnalyzer {}
impl windows_core::RuntimeName for IDWriteTextAnalyzer {}
windows_core::imp::define_interface!(
    IDWriteTextFormat,
    IDWriteTextFormat_Vtbl,
    0x9c906818_31d7_4fd3_a151_7c5e225db55a
);
windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    SetTextAlignment: usize,
    SetParagraphAlignment: usize,
    SetWordWrapping: usize,
    SetReadingDirection: usize,
    SetFlowDirection: usize,
    SetIncrementalTabStop: usize,
    SetTrimming: usize,
    SetLineSpacing: usize,
    GetTextAlignment: usize,
    GetParagraphAlignment: usize,
    GetWordWrapping: usize,
    GetReadingDirection: usize,
    GetFlowDirection: usize,
    GetIncrementalTabStop: usize,
    GetTrimming: usize,
    GetLineSpacing: usize,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
}
unsafe impl Send for IDWriteTextFormat {}
unsafe impl Sync for IDWriteTextFormat {}
impl windows_core::RuntimeName for IDWriteTextFormat {}
windows_core::imp::define_interface!(
    IDWriteTextLayout,
    IDWriteTextLayout_Vtbl,
    0x53737037_6d14_410b_9bfe_0b182bb70961
);
impl core::ops::Deref for IDWriteTextLayout {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextLayout,
    windows_core::IUnknown,
    IDWriteTextFormat
);
impl IDWriteTextLayout {
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxWidth)(
                windows_core::Interface::as_raw(self),
                maxwidth,
            )
            .ok()
        }
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxHeight)(
                windows_core::Interface::as_raw(self),
                maxheight,
            )
            .ok()
        }
    }
    pub unsafe fn SetFontCollection<P0>(
        &self,
        fontcollection: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteFontCollection>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontCollection)(
                windows_core::Interface::as_raw(self),
                fontcollection.param().abi(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetFontFamilyName<P0>(
        &self,
        fontfamilyname: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontFamilyName)(
                windows_core::Interface::as_raw(self),
                fontfamilyname.param().abi(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetFontWeight(
        &self,
        fontweight: DWRITE_FONT_WEIGHT,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontWeight)(
                windows_core::Interface::as_raw(self),
                fontweight,
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetFontStyle(
        &self,
        fontstyle: DWRITE_FONT_STYLE,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontStyle)(
                windows_core::Interface::as_raw(self),
                fontstyle,
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetFontStretch(
        &self,
        fontstretch: DWRITE_FONT_STRETCH,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontStretch)(
                windows_core::Interface::as_raw(self),
                fontstretch,
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetFontSize(
        &self,
        fontsize: f32,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFontSize)(
                windows_core::Interface::as_raw(self),
                fontsize,
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetUnderline(
        &self,
        hasunderline: bool,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnderline)(
                windows_core::Interface::as_raw(self),
                hasunderline.into(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetStrikethrough(
        &self,
        hasstrikethrough: bool,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStrikethrough)(
                windows_core::Interface::as_raw(self),
                hasstrikethrough.into(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetDrawingEffect<P0>(
        &self,
        drawingeffect: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDrawingEffect)(
                windows_core::Interface::as_raw(self),
                drawingeffect.param().abi(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetInlineObject<P0>(
        &self,
        inlineobject: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteInlineObject>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetInlineObject)(
                windows_core::Interface::as_raw(self),
                inlineobject.param().abi(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetTypography<P0>(
        &self,
        typography: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IDWriteTypography>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTypography)(
                windows_core::Interface::as_raw(self),
                typography.param().abi(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn SetLocaleName<P0>(
        &self,
        localename: P0,
        textrange: DWRITE_TEXT_RANGE,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetLocaleName)(
                windows_core::Interface::as_raw(self),
                localename.param().abi(),
                textrange,
            )
            .ok()
        }
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaxWidth)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaxHeight)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetFontCollection(
        &self,
        currentposition: u32,
        fontcollection: *mut Option<IDWriteFontCollection>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontCollection)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(fontcollection),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetFontFamilyNameLength(
        &self,
        currentposition: u32,
        namelength: *mut u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(
                windows_core::Interface::as_raw(self),
                currentposition,
                namelength as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetFontFamilyName(
        &self,
        currentposition: u32,
        fontfamilyname: windows_core::PWSTR,
        namesize: u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontFamilyName)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontfamilyname,
                namesize,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetFontWeight(
        &self,
        currentposition: u32,
        fontweight: *mut DWRITE_FONT_WEIGHT,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontWeight)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontweight as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetFontStyle(
        &self,
        currentposition: u32,
        fontstyle: *mut DWRITE_FONT_STYLE,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontStyle)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontstyle as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetFontStretch(
        &self,
        currentposition: u32,
        fontstretch: *mut DWRITE_FONT_STRETCH,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontStretch)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontstretch as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetFontSize(
        &self,
        currentposition: u32,
        fontsize: *mut f32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFontSize)(
                windows_core::Interface::as_raw(self),
                currentposition,
                fontsize as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetUnderline(
        &self,
        currentposition: u32,
        hasunderline: *mut windows_core::BOOL,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetUnderline)(
                windows_core::Interface::as_raw(self),
                currentposition,
                hasunderline as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetStrikethrough(
        &self,
        currentposition: u32,
        hasstrikethrough: *mut windows_core::BOOL,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetStrikethrough)(
                windows_core::Interface::as_raw(self),
                currentposition,
                hasstrikethrough as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetDrawingEffect(
        &self,
        currentposition: u32,
        drawingeffect: *mut Option<windows_core::IUnknown>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetDrawingEffect)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(drawingeffect),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetInlineObject(
        &self,
        currentposition: u32,
        inlineobject: *mut Option<IDWriteInlineObject>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetInlineObject)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(inlineobject),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetTypography(
        &self,
        currentposition: u32,
        typography: *mut Option<IDWriteTypography>,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetTypography)(
                windows_core::Interface::as_raw(self),
                currentposition,
                core::mem::transmute(typography),
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetLocaleNameLength(
        &self,
        currentposition: u32,
        namelength: *mut u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetLocaleNameLength)(
                windows_core::Interface::as_raw(self),
                currentposition,
                namelength as _,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetLocaleName(
        &self,
        currentposition: u32,
        localename: windows_core::PWSTR,
        namesize: u32,
        textrange: Option<*mut DWRITE_TEXT_RANGE>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetLocaleName)(
                windows_core::Interface::as_raw(self),
                currentposition,
                localename,
                namesize,
                textrange.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn Draw<P1>(
        &self,
        clientdrawingcontext: Option<*const core::ffi::c_void>,
        renderer: P1,
        originx: f32,
        originy: f32,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDWriteTextRenderer>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(
                windows_core::Interface::as_raw(self),
                clientdrawingcontext.unwrap_or(core::mem::zeroed()) as _,
                renderer.param().abi(),
                originx,
                originy,
            )
            .ok()
        }
    }
    pub unsafe fn GetLineMetrics(
        &self,
        linemetrics: Option<*mut DWRITE_LINE_METRICS>,
        maxlinecount: u32,
        actuallinecount: *mut u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetLineMetrics)(
                windows_core::Interface::as_raw(self),
                linemetrics.unwrap_or(core::mem::zeroed()) as _,
                maxlinecount,
                actuallinecount as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetMetrics(
        &self,
        textmetrics: *mut DWRITE_TEXT_METRICS,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(
                windows_core::Interface::as_raw(self),
                textmetrics as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetOverhangMetrics(&self) -> windows_core::Result<DWRITE_OVERHANG_METRICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverhangMetrics)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetClusterMetrics(
        &self,
        clustermetrics: Option<*mut DWRITE_CLUSTER_METRICS>,
        maxclustercount: u32,
        actualclustercount: *mut u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetClusterMetrics)(
                windows_core::Interface::as_raw(self),
                clustermetrics.unwrap_or(core::mem::zeroed()) as _,
                maxclustercount,
                actualclustercount as _,
            )
            .ok()
        }
    }
    pub unsafe fn DetermineMinWidth(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetermineMinWidth)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn HitTestPoint(
        &self,
        pointx: f32,
        pointy: f32,
        istrailinghit: *mut windows_core::BOOL,
        isinside: *mut windows_core::BOOL,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestPoint)(
                windows_core::Interface::as_raw(self),
                pointx,
                pointy,
                istrailinghit as _,
                isinside as _,
                hittestmetrics as _,
            )
            .ok()
        }
    }
    pub unsafe fn HitTestTextPosition(
        &self,
        textposition: u32,
        istrailinghit: bool,
        pointx: *mut f32,
        pointy: *mut f32,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestTextPosition)(
                windows_core::Interface::as_raw(self),
                textposition,
                istrailinghit.into(),
                pointx as _,
                pointy as _,
                hittestmetrics as _,
            )
            .ok()
        }
    }
    pub unsafe fn HitTestTextRange(
        &self,
        textposition: u32,
        textlength: u32,
        originx: f32,
        originy: f32,
        hittestmetrics: Option<*mut DWRITE_HIT_TEST_METRICS>,
        maxhittestmetricscount: u32,
        actualhittestmetricscount: *mut u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestTextRange)(
                windows_core::Interface::as_raw(self),
                textposition,
                textlength,
                originx,
                originy,
                hittestmetrics.unwrap_or(core::mem::zeroed()) as _,
                maxhittestmetricscount,
                actualhittestmetricscount as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetMaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontFamilyName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontWeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_STYLE,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontStretch: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_FONT_STRETCH,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetFontSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetUnderline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetStrikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetDrawingEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetInlineObject: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub SetLocaleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
        DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetFontCollection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontFamilyName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::PWSTR,
        u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut DWRITE_FONT_WEIGHT,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontStyle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut DWRITE_FONT_STYLE,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontStretch: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut DWRITE_FONT_STRETCH,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetFontSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut f32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetUnderline: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetStrikethrough: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetDrawingEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetInlineObject: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetTypography: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetLocaleNameLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub GetLocaleName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::PWSTR,
        u32,
        *mut DWRITE_TEXT_RANGE,
    ) -> windows_core::HRESULT,
    pub Draw: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        *mut core::ffi::c_void,
        f32,
        f32,
    ) -> windows_core::HRESULT,
    pub GetLineMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_LINE_METRICS,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_TEXT_METRICS,
    ) -> windows_core::HRESULT,
    pub GetOverhangMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_OVERHANG_METRICS,
    ) -> windows_core::HRESULT,
    pub GetClusterMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_CLUSTER_METRICS,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub DetermineMinWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub HitTestPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f32,
        f32,
        *mut windows_core::BOOL,
        *mut windows_core::BOOL,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    pub HitTestTextPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
        *mut f32,
        *mut f32,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    pub HitTestTextRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        f32,
        f32,
        *mut DWRITE_HIT_TEST_METRICS,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for IDWriteTextLayout {}
unsafe impl Sync for IDWriteTextLayout {}
impl windows_core::RuntimeName for IDWriteTextLayout {}
windows_core::imp::define_interface!(
    IDWriteTextRenderer,
    IDWriteTextRenderer_Vtbl,
    0xef8a8135_5cc6_45fe_8825_c5a0724eb819
);
impl core::ops::Deref for IDWriteTextRenderer {
    type Target = IDWritePixelSnapping;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextRenderer,
    windows_core::IUnknown,
    IDWritePixelSnapping
);
#[repr(C)]
pub struct IDWriteTextRenderer_Vtbl {
    pub base__: IDWritePixelSnapping_Vtbl,
    DrawGlyphRun: usize,
    DrawUnderline: usize,
    DrawStrikethrough: usize,
    DrawInlineObject: usize,
}
unsafe impl Send for IDWriteTextRenderer {}
unsafe impl Sync for IDWriteTextRenderer {}
impl windows_core::RuntimeName for IDWriteTextRenderer {}
windows_core::imp::define_interface!(
    IDWriteTypography,
    IDWriteTypography_Vtbl,
    0x55f1112b_1dc2_4b3c_9541_f46894ed85b6
);
windows_core::imp::interface_hierarchy!(IDWriteTypography, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteTypography_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    AddFontFeature: usize,
    GetFontFeatureCount: usize,
    GetFontFeature: usize,
}
unsafe impl Send for IDWriteTypography {}
unsafe impl Sync for IDWriteTypography {}
impl windows_core::RuntimeName for IDWriteTypography {}
windows_core::imp::define_interface!(
    IDXGIAdapter,
    IDXGIAdapter_Vtbl,
    0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0
);
impl core::ops::Deref for IDXGIAdapter {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter, windows_core::IUnknown, IDXGIObject);
impl IDXGIAdapter {
    pub unsafe fn EnumOutputs(&self, output: u32) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumOutputs)(
                windows_core::Interface::as_raw(self),
                output,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDesc(&self) -> windows_core::Result<DXGI_ADAPTER_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CheckInterfaceSupport(
        &self,
        interfacename: *const windows_core::GUID,
    ) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckInterfaceSupport)(
                windows_core::Interface::as_raw(self),
                interfacename,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumOutputs: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_ADAPTER_DESC,
    ) -> windows_core::HRESULT,
    pub CheckInterfaceSupport: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut i64,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIAdapter {}
unsafe impl Sync for IDXGIAdapter {}
pub trait IDXGIAdapter_Impl: IDXGIObject_Impl {
    fn EnumOutputs(&self, output: u32) -> windows_core::Result<IDXGIOutput>;
    fn GetDesc(&self) -> windows_core::Result<DXGI_ADAPTER_DESC>;
    fn CheckInterfaceSupport(
        &self,
        interfacename: *const windows_core::GUID,
    ) -> windows_core::Result<i64>;
}
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: IDXGIAdapter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnumOutputs<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            output: u32,
            ppoutput: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::EnumOutputs(this, core::mem::transmute_copy(&output)) {
                    Ok(ok__) => {
                        ppoutput.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDesc<Identity: IDXGIAdapter_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *mut DXGI_ADAPTER_DESC,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::GetDesc(this) {
                    Ok(ok__) => {
                        pdesc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<
            Identity: IDXGIAdapter_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            interfacename: *const windows_core::GUID,
            pumdversion: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIAdapter_Impl::CheckInterfaceSupport(
                    this,
                    core::mem::transmute_copy(&interfacename),
                ) {
                    Ok(ok__) => {
                        pumdversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, OFFSET>,
            GetDesc: GetDesc::<Identity, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIAdapter as windows_core::Interface>::IID
            || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIAdapter {}
windows_core::imp::define_interface!(
    IDXGIDevice,
    IDXGIDevice_Vtbl,
    0x54ec77fa_1377_44e6_8c32_88fd5f44c84c
);
impl core::ops::Deref for IDXGIDevice {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice, windows_core::IUnknown, IDXGIObject);
impl IDXGIDevice {
    pub unsafe fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapter)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: DXGI_USAGE,
        psharedresource: Option<*const DXGI_SHARED_RESOURCE>,
        ppsurface: *mut Option<IDXGISurface>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CreateSurface)(
                windows_core::Interface::as_raw(self),
                pdesc,
                numsurfaces,
                usage,
                psharedresource.unwrap_or(core::mem::zeroed()) as _,
                core::mem::transmute(ppsurface),
            )
            .ok()
        }
    }
    pub unsafe fn QueryResourceResidency(
        &self,
        ppresources: *const Option<windows_core::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).QueryResourceResidency)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute(ppresources),
                presidencystatus as _,
                numresources,
            )
            .ok()
        }
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetGPUThreadPriority)(
                windows_core::Interface::as_raw(self),
                priority,
            )
            .ok()
        }
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPUThreadPriority)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSurface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_SURFACE_DESC,
        u32,
        DXGI_USAGE,
        *const DXGI_SHARED_RESOURCE,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub QueryResourceResidency: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const *mut core::ffi::c_void,
        *mut DXGI_RESIDENCY,
        u32,
    ) -> windows_core::HRESULT,
    pub SetGPUThreadPriority:
        unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetGPUThreadPriority:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice {}
unsafe impl Sync for IDXGIDevice {}
pub trait IDXGIDevice_Impl: IDXGIObject_Impl {
    fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter>;
    fn CreateSurface(
        &self,
        pdesc: *const DXGI_SURFACE_DESC,
        numsurfaces: u32,
        usage: DXGI_USAGE,
        psharedresource: *const DXGI_SHARED_RESOURCE,
        ppsurface: *mut Option<IDXGISurface>,
    ) -> windows_core::Result<()>;
    fn QueryResourceResidency(
        &self,
        ppresources: *const Option<windows_core::IUnknown>,
        presidencystatus: *mut DXGI_RESIDENCY,
        numresources: u32,
    ) -> windows_core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()>;
    fn GetGPUThreadPriority(&self) -> windows_core::Result<i32>;
}
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: IDXGIDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdapter<Identity: IDXGIDevice_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            padapter: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetAdapter(this) {
                    Ok(ok__) => {
                        padapter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: IDXGIDevice_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdesc: *const DXGI_SURFACE_DESC,
            numsurfaces: u32,
            usage: DXGI_USAGE,
            psharedresource: *const DXGI_SHARED_RESOURCE,
            ppsurface: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::CreateSurface(
                    this,
                    core::mem::transmute_copy(&pdesc),
                    core::mem::transmute_copy(&numsurfaces),
                    core::mem::transmute_copy(&usage),
                    core::mem::transmute_copy(&psharedresource),
                    core::mem::transmute_copy(&ppsurface),
                )
                .into()
            }
        }
        unsafe extern "system" fn QueryResourceResidency<
            Identity: IDXGIDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppresources: *const *mut core::ffi::c_void,
            presidencystatus: *mut DXGI_RESIDENCY,
            numresources: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::QueryResourceResidency(
                    this,
                    core::mem::transmute_copy(&ppresources),
                    core::mem::transmute_copy(&presidencystatus),
                    core::mem::transmute_copy(&numresources),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<
            Identity: IDXGIDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            priority: i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice_Impl::SetGPUThreadPriority(this, core::mem::transmute_copy(&priority))
                    .into()
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<
            Identity: IDXGIDevice_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppriority: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice_Impl::GetGPUThreadPriority(this) {
                    Ok(ok__) => {
                        ppriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, OFFSET>,
            CreateSurface: CreateSurface::<Identity, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice as windows_core::Interface>::IID
            || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDevice {}
windows_core::imp::define_interface!(
    IDXGIDevice1,
    IDXGIDevice1_Vtbl,
    0x77db970f_6276_48ba_ba28_070143b4392c
);
impl core::ops::Deref for IDXGIDevice1 {
    type Target = IDXGIDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGIDevice1,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDevice
);
impl IDXGIDevice1 {
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(
                windows_core::Interface::as_raw(self),
                maxlatency,
            )
            .ok()
        }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGIDevice1_Vtbl {
    pub base__: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice1 {}
unsafe impl Sync for IDXGIDevice1 {}
pub trait IDXGIDevice1_Impl: IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32>;
}
impl IDXGIDevice1_Vtbl {
    pub const fn new<Identity: IDXGIDevice1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMaximumFrameLatency<
            Identity: IDXGIDevice1_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            maxlatency: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDevice1_Impl::SetMaximumFrameLatency(
                    this,
                    core::mem::transmute_copy(&maxlatency),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<
            Identity: IDXGIDevice1_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pmaxlatency: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDXGIDevice1_Impl::GetMaximumFrameLatency(this) {
                    Ok(ok__) => {
                        pmaxlatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDXGIDevice_Vtbl::new::<Identity, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDevice1 as windows_core::Interface>::IID
            || iid == &<IDXGIObject as windows_core::Interface>::IID
            || iid == &<IDXGIDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDevice1 {}
windows_core::imp::define_interface!(
    IDXGIDeviceSubObject,
    IDXGIDeviceSubObject_Vtbl,
    0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6
);
impl core::ops::Deref for IDXGIDeviceSubObject {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, windows_core::IUnknown, IDXGIObject);
#[repr(C)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    GetDevice: usize,
}
unsafe impl Send for IDXGIDeviceSubObject {}
unsafe impl Sync for IDXGIDeviceSubObject {}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
windows_core::imp::define_interface!(
    IDXGIFactory,
    IDXGIFactory_Vtbl,
    0x7b7166ec_21c7_44ae_b21a_c9ae321ae369
);
impl core::ops::Deref for IDXGIFactory {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory, windows_core::IUnknown, IDXGIObject);
#[repr(C)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    EnumAdapters: usize,
    MakeWindowAssociation: usize,
    GetWindowAssociation: usize,
    CreateSwapChain: usize,
    CreateSoftwareAdapter: usize,
}
unsafe impl Send for IDXGIFactory {}
unsafe impl Sync for IDXGIFactory {}
impl windows_core::RuntimeName for IDXGIFactory {}
windows_core::imp::define_interface!(
    IDXGIFactory1,
    IDXGIFactory1_Vtbl,
    0x770aae78_f26f_4dba_a829_253c83d1b387
);
impl core::ops::Deref for IDXGIFactory1 {
    type Target = IDXGIFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGIFactory1,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIFactory
);
#[repr(C)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    EnumAdapters1: usize,
    IsCurrent: usize,
}
unsafe impl Send for IDXGIFactory1 {}
unsafe impl Sync for IDXGIFactory1 {}
impl windows_core::RuntimeName for IDXGIFactory1 {}
windows_core::imp::define_interface!(
    IDXGIFactory2,
    IDXGIFactory2_Vtbl,
    0x50c83a1c_e072_4c48_87b0_3630fa36a6d0
);
impl core::ops::Deref for IDXGIFactory2 {
    type Target = IDXGIFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGIFactory2,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIFactory,
    IDXGIFactory1
);
impl IDXGIFactory2 {
    pub unsafe fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsWindowedStereoEnabled)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn CreateSwapChainForHwnd<P0, P4>(
        &self,
        pdevice: P0,
        hwnd: HWND,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        pfullscreendesc: Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>,
        prestricttooutput: P4,
    ) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForHwnd)(
                windows_core::Interface::as_raw(self),
                pdevice.param().abi(),
                hwnd,
                pdesc,
                pfullscreendesc.unwrap_or(core::mem::zeroed()) as _,
                prestricttooutput.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P3>(
        &self,
        pdevice: P0,
        pwindow: P1,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: P3,
    ) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForCoreWindow)(
                windows_core::Interface::as_raw(self),
                pdevice.param().abi(),
                pwindow.param().abi(),
                pdesc,
                prestricttooutput.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSharedResourceAdapterLuid(
        &self,
        hresource: HANDLE,
    ) -> windows_core::Result<LUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSharedResourceAdapterLuid)(
                windows_core::Interface::as_raw(self),
                hresource,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn RegisterStereoStatusWindow(
        &self,
        windowhandle: HWND,
        wmsg: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusWindow)(
                windows_core::Interface::as_raw(self),
                windowhandle,
                wmsg,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn RegisterStereoStatusEvent(&self, hevent: HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusEvent)(
                windows_core::Interface::as_raw(self),
                hevent,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterStereoStatus)(
                windows_core::Interface::as_raw(self),
                dwcookie,
            );
        }
    }
    pub unsafe fn RegisterOcclusionStatusWindow(
        &self,
        windowhandle: HWND,
        wmsg: u32,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusWindow)(
                windows_core::Interface::as_raw(self),
                windowhandle,
                wmsg,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn RegisterOcclusionStatusEvent(&self, hevent: HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusEvent)(
                windows_core::Interface::as_raw(self),
                hevent,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterOcclusionStatus)(
                windows_core::Interface::as_raw(self),
                dwcookie,
            );
        }
    }
    pub unsafe fn CreateSwapChainForComposition<P0, P2>(
        &self,
        pdevice: P0,
        pdesc: *const DXGI_SWAP_CHAIN_DESC1,
        prestricttooutput: P2,
    ) -> windows_core::Result<IDXGISwapChain1>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSwapChainForComposition)(
                windows_core::Interface::as_raw(self),
                pdevice.param().abi(),
                pdesc,
                prestricttooutput.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: IDXGIFactory1_Vtbl,
    pub IsWindowedStereoEnabled:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub CreateSwapChainForHwnd: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        HWND,
        *const DXGI_SWAP_CHAIN_DESC1,
        *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateSwapChainForCoreWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const DXGI_SWAP_CHAIN_DESC1,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSharedResourceAdapterLuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *mut LUID,
    ) -> windows_core::HRESULT,
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub UnregisterStereoStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HWND,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        HANDLE,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub CreateSwapChainForComposition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const DXGI_SWAP_CHAIN_DESC1,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactory2 {}
unsafe impl Sync for IDXGIFactory2 {}
impl windows_core::RuntimeName for IDXGIFactory2 {}
windows_core::imp::define_interface!(
    IDXGIObject,
    IDXGIObject_Vtbl,
    0xaec22fb8_76f3_4639_9be0_28eb43a67a2e
);
windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(
        &self,
        name: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateData)(
                windows_core::Interface::as_raw(self),
                name,
                datasize,
                pdata,
            )
            .ok()
        }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(
        &self,
        name: *const windows_core::GUID,
        punknown: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetPrivateDataInterface)(
                windows_core::Interface::as_raw(self),
                name,
                punknown.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetPrivateData(
        &self,
        name: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetPrivateData)(
                windows_core::Interface::as_raw(self),
                name,
                pdatasize as _,
                pdata as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetParent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetParent)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDXGIObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
        *const core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIObject {}
unsafe impl Sync for IDXGIObject {}
pub trait IDXGIObject_Impl: windows_core::IUnknownImpl {
    fn SetPrivateData(
        &self,
        name: *const windows_core::GUID,
        datasize: u32,
        pdata: *const core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(
        &self,
        name: *const windows_core::GUID,
        punknown: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn GetPrivateData(
        &self,
        name: *const windows_core::GUID,
        pdatasize: *mut u32,
        pdata: *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetParent(
        &self,
        riid: *const windows_core::GUID,
        ppparent: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: IDXGIObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrivateData<
            Identity: IDXGIObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: *const windows_core::GUID,
            datasize: u32,
            pdata: *const core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateData(
                    this,
                    core::mem::transmute_copy(&name),
                    core::mem::transmute_copy(&datasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<
            Identity: IDXGIObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: *const windows_core::GUID,
            punknown: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateDataInterface(
                    this,
                    core::mem::transmute_copy(&name),
                    core::mem::transmute_copy(&punknown),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPrivateData<
            Identity: IDXGIObject_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            name: *const windows_core::GUID,
            pdatasize: *mut u32,
            pdata: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetPrivateData(
                    this,
                    core::mem::transmute_copy(&name),
                    core::mem::transmute_copy(&pdatasize),
                    core::mem::transmute_copy(&pdata),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDXGIObject_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            ppparent: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetParent(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppparent),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIObject {}
windows_core::imp::define_interface!(
    IDXGIOutput,
    IDXGIOutput_Vtbl,
    0xae02eedb_c735_4690_8d52_5a8dc20213aa
);
impl core::ops::Deref for IDXGIOutput {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIOutput, windows_core::IUnknown, IDXGIObject);
#[repr(C)]
pub struct IDXGIOutput_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    GetDesc: usize,
    GetDisplayModeList: usize,
    FindClosestMatchingMode: usize,
    WaitForVBlank: usize,
    TakeOwnership: usize,
    ReleaseOwnership: usize,
    GetGammaControlCapabilities: usize,
    SetGammaControl: usize,
    GetGammaControl: usize,
    SetDisplaySurface: usize,
    GetDisplaySurfaceData: usize,
    GetFrameStatistics: usize,
}
unsafe impl Send for IDXGIOutput {}
unsafe impl Sync for IDXGIOutput {}
impl windows_core::RuntimeName for IDXGIOutput {}
windows_core::imp::define_interface!(
    IDXGISurface,
    IDXGISurface_Vtbl,
    0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec
);
impl core::ops::Deref for IDXGISurface {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISurface,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject
);
impl IDXGISurface {
    pub unsafe fn GetDesc(&self) -> windows_core::Result<DXGI_SURFACE_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Map(
        &self,
        plockedrect: *mut DXGI_MAPPED_RECT,
        mapflags: DXGI_MAP_FLAGS,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Map)(
                windows_core::Interface::as_raw(self),
                plockedrect as _,
                mapflags,
            )
            .ok()
        }
    }
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
}
#[repr(C)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_SURFACE_DESC,
    ) -> windows_core::HRESULT,
    pub Map: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_MAPPED_RECT,
        DXGI_MAP_FLAGS,
    ) -> windows_core::HRESULT,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISurface {}
unsafe impl Sync for IDXGISurface {}
impl windows_core::RuntimeName for IDXGISurface {}
windows_core::imp::define_interface!(
    IDXGISwapChain,
    IDXGISwapChain_Vtbl,
    0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a
);
impl core::ops::Deref for IDXGISwapChain {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISwapChain,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject
);
impl IDXGISwapChain {
    pub unsafe fn Present(&self, syncinterval: u32, flags: DXGI_PRESENT) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Present)(
                windows_core::Interface::as_raw(self),
                syncinterval,
                flags,
            )
        }
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetBuffer)(
                windows_core::Interface::as_raw(self),
                buffer,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFullscreenState<P1>(
        &self,
        fullscreen: bool,
        ptarget: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<IDXGIOutput>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFullscreenState)(
                windows_core::Interface::as_raw(self),
                fullscreen.into(),
                ptarget.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetFullscreenState(
        &self,
        pfullscreen: Option<*mut windows_core::BOOL>,
        pptarget: Option<*mut Option<IDXGIOutput>>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFullscreenState)(
                windows_core::Interface::as_raw(self),
                pfullscreen.unwrap_or(core::mem::zeroed()) as _,
                pptarget.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetDesc(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn ResizeBuffers(
        &self,
        buffercount: u32,
        width: u32,
        height: u32,
        newformat: DXGI_FORMAT,
        swapchainflags: DXGI_SWAP_CHAIN_FLAG,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ResizeBuffers)(
                windows_core::Interface::as_raw(self),
                buffercount,
                width,
                height,
                newformat,
                swapchainflags,
            )
            .ok()
        }
    }
    pub unsafe fn ResizeTarget(
        &self,
        pnewtargetparameters: *const DXGI_MODE_DESC,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ResizeTarget)(
                windows_core::Interface::as_raw(self),
                pnewtargetparameters,
            )
            .ok()
        }
    }
    pub unsafe fn GetContainingOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainingOutput)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFrameStatistics(
        &self,
        pstats: *mut DXGI_FRAME_STATISTICS,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetFrameStatistics)(
                windows_core::Interface::as_raw(self),
                pstats as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetLastPresentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPresentCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    pub Present: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        DXGI_PRESENT,
    ) -> windows_core::HRESULT,
    pub GetBuffer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFullscreenState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFullscreenState: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_SWAP_CHAIN_DESC,
    ) -> windows_core::HRESULT,
    pub ResizeBuffers: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        u32,
        u32,
        DXGI_FORMAT,
        DXGI_SWAP_CHAIN_FLAG,
    ) -> windows_core::HRESULT,
    pub ResizeTarget: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_MODE_DESC,
    ) -> windows_core::HRESULT,
    pub GetContainingOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFrameStatistics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_FRAME_STATISTICS,
    ) -> windows_core::HRESULT,
    pub GetLastPresentCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain {}
unsafe impl Sync for IDXGISwapChain {}
impl windows_core::RuntimeName for IDXGISwapChain {}
windows_core::imp::define_interface!(
    IDXGISwapChain1,
    IDXGISwapChain1_Vtbl,
    0x790a45f7_0d42_4876_983a_0a55cfe6f4aa
);
impl core::ops::Deref for IDXGISwapChain1 {
    type Target = IDXGISwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISwapChain1,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject,
    IDXGISwapChain
);
impl IDXGISwapChain1 {
    pub unsafe fn GetDesc1(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc1)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetFullscreenDesc(
        &self,
    ) -> windows_core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullscreenDesc)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetHwnd(&self) -> windows_core::Result<HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwnd)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetCoreWindow)(
                windows_core::Interface::as_raw(self),
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Present1(
        &self,
        syncinterval: u32,
        presentflags: DXGI_PRESENT,
        ppresentparameters: *const DXGI_PRESENT_PARAMETERS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Present1)(
                windows_core::Interface::as_raw(self),
                syncinterval,
                presentflags,
                ppresentparameters,
            )
        }
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsTemporaryMonoSupported)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetRestrictToOutput(&self) -> windows_core::Result<IDXGIOutput> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestrictToOutput)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBackgroundColor)(
                windows_core::Interface::as_raw(self),
                pcolor,
            )
            .ok()
        }
    }
    pub unsafe fn GetBackgroundColor(&self) -> windows_core::Result<DXGI_RGBA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackgroundColor)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetRotation(&self, rotation: DXGI_MODE_ROTATION) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRotation)(
                windows_core::Interface::as_raw(self),
                rotation,
            )
            .ok()
        }
    }
    pub unsafe fn GetRotation(&self) -> windows_core::Result<DXGI_MODE_ROTATION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRotation)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: IDXGISwapChain_Vtbl,
    pub GetDesc1: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_SWAP_CHAIN_DESC1,
    ) -> windows_core::HRESULT,
    pub GetFullscreenDesc: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
    ) -> windows_core::HRESULT,
    pub GetHwnd:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut HWND) -> windows_core::HRESULT,
    pub GetCoreWindow: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Present1: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        DXGI_PRESENT,
        *const DXGI_PRESENT_PARAMETERS,
    ) -> windows_core::HRESULT,
    pub IsTemporaryMonoSupported:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetRestrictToOutput: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_RGBA,
    ) -> windows_core::HRESULT,
    pub GetBackgroundColor:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_RGBA) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DXGI_MODE_ROTATION,
    ) -> windows_core::HRESULT,
    pub GetRotation: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_MODE_ROTATION,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain1 {}
unsafe impl Sync for IDXGISwapChain1 {}
impl windows_core::RuntimeName for IDXGISwapChain1 {}
windows_core::imp::define_interface!(
    IDXGISwapChain2,
    IDXGISwapChain2_Vtbl,
    0xa8be2ac4_199f_4946_b331_79599fb98de7
);
impl core::ops::Deref for IDXGISwapChain2 {
    type Target = IDXGISwapChain1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDXGISwapChain2,
    windows_core::IUnknown,
    IDXGIObject,
    IDXGIDeviceSubObject,
    IDXGISwapChain,
    IDXGISwapChain1
);
impl IDXGISwapChain2 {
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSourceSize)(
                windows_core::Interface::as_raw(self),
                width,
                height,
            )
            .ok()
        }
    }
    pub unsafe fn GetSourceSize(
        &self,
        pwidth: *mut u32,
        pheight: *mut u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetSourceSize)(
                windows_core::Interface::as_raw(self),
                pwidth as _,
                pheight as _,
            )
            .ok()
        }
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(
                windows_core::Interface::as_raw(self),
                maxlatency,
            )
            .ok()
        }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> HANDLE {
        unsafe {
            (windows_core::Interface::vtable(self).GetFrameLatencyWaitableObject)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn SetMatrixTransform(
        &self,
        pmatrix: *const DXGI_MATRIX_3X2_F,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMatrixTransform)(
                windows_core::Interface::as_raw(self),
                pmatrix,
            )
            .ok()
        }
    }
    pub unsafe fn GetMatrixTransform(
        &self,
        pmatrix: *mut DXGI_MATRIX_3X2_F,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetMatrixTransform)(
                windows_core::Interface::as_raw(self),
                pmatrix as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IDXGISwapChain2_Vtbl {
    pub base__: IDXGISwapChain1_Vtbl,
    pub SetSourceSize:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub SetMaximumFrameLatency:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> HANDLE,
    pub SetMatrixTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const DXGI_MATRIX_3X2_F,
    ) -> windows_core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DXGI_MATRIX_3X2_F,
    ) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain2 {}
unsafe impl Sync for IDXGISwapChain2 {}
impl windows_core::RuntimeName for IDXGISwapChain2 {}
windows_core::imp::define_interface!(
    IDispatch,
    IDispatch_Vtbl,
    0x00020400_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
#[repr(C)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetTypeInfoCount: usize,
    GetTypeInfo: usize,
    GetIDsOfNames: usize,
    Invoke: usize,
}
impl windows_core::RuntimeName for IDispatch {}
windows_core::imp::define_interface!(
    IEnumShellItems,
    IEnumShellItems_Vtbl,
    0x70629033_e363_4a28_a567_0db78006e6d7
);
windows_core::imp::interface_hierarchy!(IEnumShellItems, windows_core::IUnknown);
#[repr(C)]
pub struct IEnumShellItems_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Next: usize,
    Skip: usize,
    Reset: usize,
    Clone: usize,
}
impl windows_core::RuntimeName for IEnumShellItems {}
windows_core::imp::define_interface!(
    IFileDialog,
    IFileDialog_Vtbl,
    0x42f85136_db7e_439c_85f1_e4075d135fc8
);
impl core::ops::Deref for IFileDialog {
    type Target = IModalWindow;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFileDialog, windows_core::IUnknown, IModalWindow);
impl IFileDialog {
    pub unsafe fn SetFileTypes(
        &self,
        rgfilterspec: &[COMDLG_FILTERSPEC],
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileTypes)(
                windows_core::Interface::as_raw(self),
                rgfilterspec.len().try_into().unwrap(),
                rgfilterspec.as_ptr(),
            )
            .ok()
        }
    }
    pub unsafe fn SetFileTypeIndex(&self, ifiletype: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileTypeIndex)(
                windows_core::Interface::as_raw(self),
                ifiletype,
            )
            .ok()
        }
    }
    pub unsafe fn GetFileTypeIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileTypeIndex)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Advise<P0>(&self, pfde: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IFileDialogEvents>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(
                windows_core::Interface::as_raw(self),
                pfde.param().abi(),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Unadvise)(
                windows_core::Interface::as_raw(self),
                dwcookie,
            )
            .ok()
        }
    }
    pub unsafe fn SetOptions(&self, fos: FILEOPENDIALOGOPTIONS) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetOptions)(
                windows_core::Interface::as_raw(self),
                fos,
            )
            .ok()
        }
    }
    pub unsafe fn GetOptions(&self) -> windows_core::Result<FILEOPENDIALOGOPTIONS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOptions)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetDefaultFolder<P0>(&self, psi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IShellItem>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultFolder)(
                windows_core::Interface::as_raw(self),
                psi.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn SetFolder<P0>(&self, psi: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IShellItem>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFolder)(
                windows_core::Interface::as_raw(self),
                psi.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetFolder(&self) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFolder)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentSelection(&self) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSelection)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetFileName<P0>(&self, pszname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileName)(
                windows_core::Interface::as_raw(self),
                pszname.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetTitle<P0>(&self, psztitle: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetTitle)(
                windows_core::Interface::as_raw(self),
                psztitle.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn SetOkButtonLabel<P0>(&self, psztext: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetOkButtonLabel)(
                windows_core::Interface::as_raw(self),
                psztext.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn SetFileNameLabel<P0>(&self, pszlabel: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFileNameLabel)(
                windows_core::Interface::as_raw(self),
                pszlabel.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn GetResult(&self) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResult)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddPlace<P0>(&self, psi: P0, fdap: FDAP) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IShellItem>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).AddPlace)(
                windows_core::Interface::as_raw(self),
                psi.param().abi(),
                fdap,
            )
            .ok()
        }
    }
    pub unsafe fn SetDefaultExtension<P0>(
        &self,
        pszdefaultextension: P0,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultExtension)(
                windows_core::Interface::as_raw(self),
                pszdefaultextension.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn Close(&self, hr: windows_core::HRESULT) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self), hr)
                .ok()
        }
    }
    pub unsafe fn SetClientGuid(
        &self,
        guid: *const windows_core::GUID,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetClientGuid)(
                windows_core::Interface::as_raw(self),
                guid,
            )
            .ok()
        }
    }
    pub unsafe fn ClearClientData(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ClearClientData)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub unsafe fn SetFilter<P0>(&self, pfilter: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IShellItemFilter>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetFilter)(
                windows_core::Interface::as_raw(self),
                pfilter.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IFileDialog_Vtbl {
    pub base__: IModalWindow_Vtbl,
    pub SetFileTypes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const COMDLG_FILTERSPEC,
    ) -> windows_core::HRESULT,
    pub SetFileTypeIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetFileTypeIndex:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Advise: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FILEOPENDIALOGOPTIONS,
    ) -> windows_core::HRESULT,
    pub GetOptions: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut FILEOPENDIALOGOPTIONS,
    ) -> windows_core::HRESULT,
    pub SetDefaultFolder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFolder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetFolder: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCurrentSelection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetFileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetOkButtonLabel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetFileNameLabel: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub GetResult: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub AddPlace: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        FDAP,
    ) -> windows_core::HRESULT,
    pub SetDefaultExtension: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::HRESULT,
    ) -> windows_core::HRESULT,
    pub SetClientGuid: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub ClearClientData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFilter: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IFileDialog_Impl: IModalWindow_Impl {
    fn SetFileTypes(
        &self,
        cfiletypes: u32,
        rgfilterspec: *const COMDLG_FILTERSPEC,
    ) -> windows_core::Result<()>;
    fn SetFileTypeIndex(&self, ifiletype: u32) -> windows_core::Result<()>;
    fn GetFileTypeIndex(&self) -> windows_core::Result<u32>;
    fn Advise(&self, pfde: windows_core::Ref<IFileDialogEvents>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> windows_core::Result<()>;
    fn SetOptions(&self, fos: FILEOPENDIALOGOPTIONS) -> windows_core::Result<()>;
    fn GetOptions(&self) -> windows_core::Result<FILEOPENDIALOGOPTIONS>;
    fn SetDefaultFolder(&self, psi: windows_core::Ref<IShellItem>) -> windows_core::Result<()>;
    fn SetFolder(&self, psi: windows_core::Ref<IShellItem>) -> windows_core::Result<()>;
    fn GetFolder(&self) -> windows_core::Result<IShellItem>;
    fn GetCurrentSelection(&self) -> windows_core::Result<IShellItem>;
    fn SetFileName(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetFileName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTitle(&self, psztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetOkButtonLabel(&self, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetFileNameLabel(&self, pszlabel: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetResult(&self) -> windows_core::Result<IShellItem>;
    fn AddPlace(&self, psi: windows_core::Ref<IShellItem>, fdap: FDAP) -> windows_core::Result<()>;
    fn SetDefaultExtension(
        &self,
        pszdefaultextension: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn Close(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn SetClientGuid(&self, guid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn ClearClientData(&self) -> windows_core::Result<()>;
    fn SetFilter(&self, pfilter: windows_core::Ref<IShellItemFilter>) -> windows_core::Result<()>;
}
impl IFileDialog_Vtbl {
    pub const fn new<Identity: IFileDialog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFileTypes<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            cfiletypes: u32,
            rgfilterspec: *const COMDLG_FILTERSPEC,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetFileTypes(
                    this,
                    core::mem::transmute_copy(&cfiletypes),
                    core::mem::transmute_copy(&rgfilterspec),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetFileTypeIndex<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ifiletype: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetFileTypeIndex(this, core::mem::transmute_copy(&ifiletype))
                    .into()
            }
        }
        unsafe extern "system" fn GetFileTypeIndex<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pifiletype: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileDialog_Impl::GetFileTypeIndex(this) {
                    Ok(ok__) => {
                        pifiletype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Advise<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pfde: *mut core::ffi::c_void,
            pdwcookie: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileDialog_Impl::Advise(this, core::mem::transmute_copy(&pfde)) {
                    Ok(ok__) => {
                        pdwcookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwcookie: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::Unadvise(this, core::mem::transmute_copy(&dwcookie)).into()
            }
        }
        unsafe extern "system" fn SetOptions<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            fos: FILEOPENDIALOGOPTIONS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetOptions(this, core::mem::transmute_copy(&fos)).into()
            }
        }
        unsafe extern "system" fn GetOptions<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pfos: *mut FILEOPENDIALOGOPTIONS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileDialog_Impl::GetOptions(this) {
                    Ok(ok__) => {
                        pfos.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultFolder<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psi: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetDefaultFolder(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn SetFolder<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            psi: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetFolder(this, core::mem::transmute_copy(&psi)).into()
            }
        }
        unsafe extern "system" fn GetFolder<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppsi: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileDialog_Impl::GetFolder(this) {
                    Ok(ok__) => {
                        ppsi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentSelection<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppsi: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileDialog_Impl::GetCurrentSelection(this) {
                    Ok(ok__) => {
                        ppsi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFileName<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pszname: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetFileName(this, core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetFileName<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pszname: *mut windows_core::PWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileDialog_Impl::GetFileName(this) {
                    Ok(ok__) => {
                        pszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTitle<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            psztitle: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetTitle(this, core::mem::transmute(&psztitle)).into()
            }
        }
        unsafe extern "system" fn SetOkButtonLabel<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psztext: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetOkButtonLabel(this, core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn SetFileNameLabel<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pszlabel: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetFileNameLabel(this, core::mem::transmute(&pszlabel)).into()
            }
        }
        unsafe extern "system" fn GetResult<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppsi: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileDialog_Impl::GetResult(this) {
                    Ok(ok__) => {
                        ppsi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPlace<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            psi: *mut core::ffi::c_void,
            fdap: FDAP,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::AddPlace(
                    this,
                    core::mem::transmute_copy(&psi),
                    core::mem::transmute_copy(&fdap),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetDefaultExtension<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pszdefaultextension: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetDefaultExtension(
                    this,
                    core::mem::transmute(&pszdefaultextension),
                )
                .into()
            }
        }
        unsafe extern "system" fn Close<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            hr: windows_core::HRESULT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::Close(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn SetClientGuid<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guid: *const windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetClientGuid(this, core::mem::transmute_copy(&guid)).into()
            }
        }
        unsafe extern "system" fn ClearClientData<
            Identity: IFileDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::ClearClientData(this).into()
            }
        }
        unsafe extern "system" fn SetFilter<Identity: IFileDialog_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pfilter: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFileDialog_Impl::SetFilter(this, core::mem::transmute_copy(&pfilter)).into()
            }
        }
        Self {
            base__: IModalWindow_Vtbl::new::<Identity, OFFSET>(),
            SetFileTypes: SetFileTypes::<Identity, OFFSET>,
            SetFileTypeIndex: SetFileTypeIndex::<Identity, OFFSET>,
            GetFileTypeIndex: GetFileTypeIndex::<Identity, OFFSET>,
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            SetOptions: SetOptions::<Identity, OFFSET>,
            GetOptions: GetOptions::<Identity, OFFSET>,
            SetDefaultFolder: SetDefaultFolder::<Identity, OFFSET>,
            SetFolder: SetFolder::<Identity, OFFSET>,
            GetFolder: GetFolder::<Identity, OFFSET>,
            GetCurrentSelection: GetCurrentSelection::<Identity, OFFSET>,
            SetFileName: SetFileName::<Identity, OFFSET>,
            GetFileName: GetFileName::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            SetOkButtonLabel: SetOkButtonLabel::<Identity, OFFSET>,
            SetFileNameLabel: SetFileNameLabel::<Identity, OFFSET>,
            GetResult: GetResult::<Identity, OFFSET>,
            AddPlace: AddPlace::<Identity, OFFSET>,
            SetDefaultExtension: SetDefaultExtension::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            SetClientGuid: SetClientGuid::<Identity, OFFSET>,
            ClearClientData: ClearClientData::<Identity, OFFSET>,
            SetFilter: SetFilter::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileDialog as windows_core::Interface>::IID
            || iid == &<IModalWindow as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFileDialog {}
windows_core::imp::define_interface!(
    IFileDialogEvents,
    IFileDialogEvents_Vtbl,
    0x973510db_7d7f_452b_8975_74a85828d354
);
windows_core::imp::interface_hierarchy!(IFileDialogEvents, windows_core::IUnknown);
#[repr(C)]
pub struct IFileDialogEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    OnFileOk: usize,
    OnFolderChanging: usize,
    OnFolderChange: usize,
    OnSelectionChange: usize,
    OnShareViolation: usize,
    OnTypeChange: usize,
    OnOverwrite: usize,
}
impl windows_core::RuntimeName for IFileDialogEvents {}
windows_core::imp::define_interface!(
    IFileOpenDialog,
    IFileOpenDialog_Vtbl,
    0xd57c7288_d4ad_4768_be02_9d969532d960
);
impl core::ops::Deref for IFileOpenDialog {
    type Target = IFileDialog;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IFileOpenDialog,
    windows_core::IUnknown,
    IModalWindow,
    IFileDialog
);
impl IFileOpenDialog {
    pub unsafe fn GetResults(&self) -> windows_core::Result<IShellItemArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResults)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSelectedItems(&self) -> windows_core::Result<IShellItemArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectedItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IFileOpenDialog_Vtbl {
    pub base__: IFileDialog_Vtbl,
    pub GetResults: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSelectedItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IFileOpenDialog_Impl: IFileDialog_Impl {
    fn GetResults(&self) -> windows_core::Result<IShellItemArray>;
    fn GetSelectedItems(&self) -> windows_core::Result<IShellItemArray>;
}
impl IFileOpenDialog_Vtbl {
    pub const fn new<Identity: IFileOpenDialog_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetResults<
            Identity: IFileOpenDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppenum: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileOpenDialog_Impl::GetResults(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelectedItems<
            Identity: IFileOpenDialog_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppsai: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFileOpenDialog_Impl::GetSelectedItems(this) {
                    Ok(ok__) => {
                        ppsai.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFileDialog_Vtbl::new::<Identity, OFFSET>(),
            GetResults: GetResults::<Identity, OFFSET>,
            GetSelectedItems: GetSelectedItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFileOpenDialog as windows_core::Interface>::IID
            || iid == &<IModalWindow as windows_core::Interface>::IID
            || iid == &<IFileDialog as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFileOpenDialog {}
windows_core::imp::define_interface!(
    IMFAttributes,
    IMFAttributes_Vtbl,
    0x2cd2d921_c447_44a7_a13c_4adabfc247e3
);
windows_core::imp::interface_hierarchy!(IMFAttributes, windows_core::IUnknown);
impl IMFAttributes {
    pub unsafe fn GetItem(
        &self,
        guidkey: *const windows_core::GUID,
        pvalue: Option<*mut PROPVARIANT>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pvalue.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetItemType(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<MF_ATTRIBUTE_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemType)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CompareItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
                value,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Compare<P0>(
        &self,
        ptheirs: P0,
        matchtype: MF_ATTRIBUTES_MATCH_TYPE,
    ) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(
                windows_core::Interface::as_raw(self),
                ptheirs.param().abi(),
                matchtype,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetUINT32(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUINT32)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetUINT64(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUINT64)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetDouble(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDouble)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetGUID(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetStringLength(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringLength)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetString(
        &self,
        guidkey: *const windows_core::GUID,
        pwszvalue: windows_core::PWSTR,
        cchbufsize: u32,
        pcchlength: Option<*mut u32>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetString)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pwszvalue,
                cchbufsize,
                pcchlength.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetAllocatedString(
        &self,
        guidkey: *const windows_core::GUID,
        ppwszvalue: *mut windows_core::PWSTR,
        pcchlength: *mut u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetAllocatedString)(
                windows_core::Interface::as_raw(self),
                guidkey,
                ppwszvalue as _,
                pcchlength as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetBlobSize(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobSize)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: *mut u8,
        cbbufsize: u32,
        pcbblobsize: Option<*mut u32>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetBlob)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pbuf as _,
                cbbufsize,
                pcbblobsize.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetAllocatedBlob(
        &self,
        guidkey: *const windows_core::GUID,
        ppbuf: *mut *mut u8,
        pcbsize: *mut u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetAllocatedBlob)(
                windows_core::Interface::as_raw(self),
                guidkey,
                ppbuf as _,
                pcbsize as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetUnknown<T>(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetUnknown)(
                windows_core::Interface::as_raw(self),
                guidkey,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
                value,
            )
            .ok()
        }
    }
    pub unsafe fn DeleteItem(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteItem)(
                windows_core::Interface::as_raw(self),
                guidkey,
            )
            .ok()
        }
    }
    pub unsafe fn DeleteAllItems(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).DeleteAllItems)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub unsafe fn SetUINT32(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUINT32)(
                windows_core::Interface::as_raw(self),
                guidkey,
                unvalue,
            )
            .ok()
        }
    }
    pub unsafe fn SetUINT64(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u64,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetUINT64)(
                windows_core::Interface::as_raw(self),
                guidkey,
                unvalue,
            )
            .ok()
        }
    }
    pub unsafe fn SetDouble(
        &self,
        guidkey: *const windows_core::GUID,
        fvalue: f64,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDouble)(
                windows_core::Interface::as_raw(self),
                guidkey,
                fvalue,
            )
            .ok()
        }
    }
    pub unsafe fn SetGUID(
        &self,
        guidkey: *const windows_core::GUID,
        guidvalue: *const windows_core::GUID,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetGUID)(
                windows_core::Interface::as_raw(self),
                guidkey,
                guidvalue,
            )
            .ok()
        }
    }
    pub unsafe fn SetString<P1>(
        &self,
        guidkey: *const windows_core::GUID,
        wszvalue: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetString)(
                windows_core::Interface::as_raw(self),
                guidkey,
                wszvalue.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn SetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: &[u8],
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBlob)(
                windows_core::Interface::as_raw(self),
                guidkey,
                pbuf.as_ptr(),
                pbuf.len().try_into().unwrap(),
            )
            .ok()
        }
    }
    pub unsafe fn SetUnknown<P1>(
        &self,
        guidkey: *const windows_core::GUID,
        punknown: P1,
    ) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetUnknown)(
                windows_core::Interface::as_raw(self),
                guidkey,
                punknown.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn LockStore(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).LockStore)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn UnlockStore(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).UnlockStore)(windows_core::Interface::as_raw(
                self,
            ))
            .ok()
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetItemByIndex(
        &self,
        unindex: u32,
        pguidkey: *mut windows_core::GUID,
        pvalue: Option<*mut PROPVARIANT>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetItemByIndex)(
                windows_core::Interface::as_raw(self),
                unindex,
                pguidkey as _,
                pvalue.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).CopyAllItems)(
                windows_core::Interface::as_raw(self),
                pdest.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMFAttributes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub GetItemType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut MF_ATTRIBUTE_TYPE,
    ) -> windows_core::HRESULT,
    pub CompareItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const PROPVARIANT,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        MF_ATTRIBUTES_MATCH_TYPE,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetUINT32: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetUINT64: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u64,
    ) -> windows_core::HRESULT,
    pub GetDouble: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut f64,
    ) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        windows_core::PWSTR,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetAllocatedString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut windows_core::PWSTR,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetBlobSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetBlob: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut u8,
        u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetAllocatedBlob: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut *mut u8,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetUnknown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub DeleteItem: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub DeleteAllItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUINT32: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u32,
    ) -> windows_core::HRESULT,
    pub SetUINT64: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        u64,
    ) -> windows_core::HRESULT,
    pub SetDouble: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        f64,
    ) -> windows_core::HRESULT,
    pub SetGUID: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
    ) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        windows_core::PCWSTR,
    ) -> windows_core::HRESULT,
    pub SetBlob: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const u8,
        u32,
    ) -> windows_core::HRESULT,
    pub SetUnknown: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub LockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemByIndex: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::GUID,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub CopyAllItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IMFAttributes_Impl: windows_core::IUnknownImpl {
    fn GetItem(
        &self,
        guidkey: *const windows_core::GUID,
        pvalue: *mut PROPVARIANT,
    ) -> windows_core::Result<()>;
    fn GetItemType(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<MF_ATTRIBUTE_TYPE>;
    fn CompareItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn Compare(
        &self,
        ptheirs: windows_core::Ref<IMFAttributes>,
        matchtype: MF_ATTRIBUTES_MATCH_TYPE,
    ) -> windows_core::Result<windows_core::BOOL>;
    fn GetUINT32(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetUINT64(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u64>;
    fn GetDouble(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<f64>;
    fn GetGUID(
        &self,
        guidkey: *const windows_core::GUID,
    ) -> windows_core::Result<windows_core::GUID>;
    fn GetStringLength(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetString(
        &self,
        guidkey: *const windows_core::GUID,
        pwszvalue: windows_core::PWSTR,
        cchbufsize: u32,
        pcchlength: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetAllocatedString(
        &self,
        guidkey: *const windows_core::GUID,
        ppwszvalue: *mut windows_core::PWSTR,
        pcchlength: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetBlobSize(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32>;
    fn GetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: *mut u8,
        cbbufsize: u32,
        pcbblobsize: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetAllocatedBlob(
        &self,
        guidkey: *const windows_core::GUID,
        ppbuf: *mut *mut u8,
        pcbsize: *mut u32,
    ) -> windows_core::Result<()>;
    fn GetUnknown(
        &self,
        guidkey: *const windows_core::GUID,
        riid: *const windows_core::GUID,
        ppv: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn SetItem(
        &self,
        guidkey: *const windows_core::GUID,
        value: *const PROPVARIANT,
    ) -> windows_core::Result<()>;
    fn DeleteItem(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<()>;
    fn DeleteAllItems(&self) -> windows_core::Result<()>;
    fn SetUINT32(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u32,
    ) -> windows_core::Result<()>;
    fn SetUINT64(
        &self,
        guidkey: *const windows_core::GUID,
        unvalue: u64,
    ) -> windows_core::Result<()>;
    fn SetDouble(
        &self,
        guidkey: *const windows_core::GUID,
        fvalue: f64,
    ) -> windows_core::Result<()>;
    fn SetGUID(
        &self,
        guidkey: *const windows_core::GUID,
        guidvalue: *const windows_core::GUID,
    ) -> windows_core::Result<()>;
    fn SetString(
        &self,
        guidkey: *const windows_core::GUID,
        wszvalue: &windows_core::PCWSTR,
    ) -> windows_core::Result<()>;
    fn SetBlob(
        &self,
        guidkey: *const windows_core::GUID,
        pbuf: *const u8,
        cbbufsize: u32,
    ) -> windows_core::Result<()>;
    fn SetUnknown(
        &self,
        guidkey: *const windows_core::GUID,
        punknown: windows_core::Ref<windows_core::IUnknown>,
    ) -> windows_core::Result<()>;
    fn LockStore(&self) -> windows_core::Result<()>;
    fn UnlockStore(&self) -> windows_core::Result<()>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetItemByIndex(
        &self,
        unindex: u32,
        pguidkey: *mut windows_core::GUID,
        pvalue: *mut PROPVARIANT,
    ) -> windows_core::Result<()>;
    fn CopyAllItems(&self, pdest: windows_core::Ref<IMFAttributes>) -> windows_core::Result<()>;
}
impl IMFAttributes_Vtbl {
    pub const fn new<Identity: IMFAttributes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pvalue: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetItem(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetItemType<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            ptype: *mut MF_ATTRIBUTE_TYPE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetItemType(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            value: *const PROPVARIANT,
            pbresult: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::CompareItem(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&value),
                ) {
                    Ok(ok__) => {
                        pbresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Compare<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ptheirs: *mut core::ffi::c_void,
            matchtype: MF_ATTRIBUTES_MATCH_TYPE,
            pbresult: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::Compare(
                    this,
                    core::mem::transmute_copy(&ptheirs),
                    core::mem::transmute_copy(&matchtype),
                ) {
                    Ok(ok__) => {
                        pbresult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUINT32<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            punvalue: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetUINT32(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        punvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUINT64<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            punvalue: *mut u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetUINT64(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        punvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDouble<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pfvalue: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetDouble(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        pfvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGUID<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pguidvalue: *mut windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetGUID(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        pguidvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStringLength<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pcchlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetStringLength(this, core::mem::transmute_copy(&guidkey))
                {
                    Ok(ok__) => {
                        pcchlength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetString<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pwszvalue: windows_core::PWSTR,
            cchbufsize: u32,
            pcchlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetString(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pwszvalue),
                    core::mem::transmute_copy(&cchbufsize),
                    core::mem::transmute_copy(&pcchlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetAllocatedString<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            ppwszvalue: *mut windows_core::PWSTR,
            pcchlength: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetAllocatedString(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&ppwszvalue),
                    core::mem::transmute_copy(&pcchlength),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetBlobSize<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pcbblobsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetBlobSize(this, core::mem::transmute_copy(&guidkey)) {
                    Ok(ok__) => {
                        pcbblobsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBlob<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pbuf: *mut u8,
            cbbufsize: u32,
            pcbblobsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetBlob(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pbuf),
                    core::mem::transmute_copy(&cbbufsize),
                    core::mem::transmute_copy(&pcbblobsize),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetAllocatedBlob<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            ppbuf: *mut *mut u8,
            pcbsize: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetAllocatedBlob(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&ppbuf),
                    core::mem::transmute_copy(&pcbsize),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetUnknown<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            riid: *const windows_core::GUID,
            ppv: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetUnknown(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppv),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            value: *const PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetItem(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&value),
                )
                .into()
            }
        }
        unsafe extern "system" fn DeleteItem<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::DeleteItem(this, core::mem::transmute_copy(&guidkey)).into()
            }
        }
        unsafe extern "system" fn DeleteAllItems<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::DeleteAllItems(this).into()
            }
        }
        unsafe extern "system" fn SetUINT32<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            unvalue: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUINT32(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&unvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetUINT64<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            unvalue: u64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUINT64(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&unvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetDouble<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            fvalue: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetDouble(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&fvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetGUID<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            guidvalue: *const windows_core::GUID,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetGUID(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&guidvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetString<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            wszvalue: windows_core::PCWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetString(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute(&wszvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetBlob<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            pbuf: *const u8,
            cbbufsize: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetBlob(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&pbuf),
                    core::mem::transmute_copy(&cbbufsize),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetUnknown<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            guidkey: *const windows_core::GUID,
            punknown: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::SetUnknown(
                    this,
                    core::mem::transmute_copy(&guidkey),
                    core::mem::transmute_copy(&punknown),
                )
                .into()
            }
        }
        unsafe extern "system" fn LockStore<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::LockStore(this).into()
            }
        }
        unsafe extern "system" fn UnlockStore<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::UnlockStore(this).into()
            }
        }
        unsafe extern "system" fn GetCount<Identity: IMFAttributes_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pcitems: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAttributes_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcitems.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemByIndex<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            unindex: u32,
            pguidkey: *mut windows_core::GUID,
            pvalue: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::GetItemByIndex(
                    this,
                    core::mem::transmute_copy(&unindex),
                    core::mem::transmute_copy(&pguidkey),
                    core::mem::transmute_copy(&pvalue),
                )
                .into()
            }
        }
        unsafe extern "system" fn CopyAllItems<
            Identity: IMFAttributes_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdest: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAttributes_Impl::CopyAllItems(this, core::mem::transmute_copy(&pdest)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, OFFSET>,
            GetItemType: GetItemType::<Identity, OFFSET>,
            CompareItem: CompareItem::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            GetUINT32: GetUINT32::<Identity, OFFSET>,
            GetUINT64: GetUINT64::<Identity, OFFSET>,
            GetDouble: GetDouble::<Identity, OFFSET>,
            GetGUID: GetGUID::<Identity, OFFSET>,
            GetStringLength: GetStringLength::<Identity, OFFSET>,
            GetString: GetString::<Identity, OFFSET>,
            GetAllocatedString: GetAllocatedString::<Identity, OFFSET>,
            GetBlobSize: GetBlobSize::<Identity, OFFSET>,
            GetBlob: GetBlob::<Identity, OFFSET>,
            GetAllocatedBlob: GetAllocatedBlob::<Identity, OFFSET>,
            GetUnknown: GetUnknown::<Identity, OFFSET>,
            SetItem: SetItem::<Identity, OFFSET>,
            DeleteItem: DeleteItem::<Identity, OFFSET>,
            DeleteAllItems: DeleteAllItems::<Identity, OFFSET>,
            SetUINT32: SetUINT32::<Identity, OFFSET>,
            SetUINT64: SetUINT64::<Identity, OFFSET>,
            SetDouble: SetDouble::<Identity, OFFSET>,
            SetGUID: SetGUID::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            SetBlob: SetBlob::<Identity, OFFSET>,
            SetUnknown: SetUnknown::<Identity, OFFSET>,
            LockStore: LockStore::<Identity, OFFSET>,
            UnlockStore: UnlockStore::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetItemByIndex: GetItemByIndex::<Identity, OFFSET>,
            CopyAllItems: CopyAllItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAttributes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFAttributes {}
windows_core::imp::define_interface!(
    IMFByteStream,
    IMFByteStream_Vtbl,
    0xad4c1b00_4bf7_422f_9175_756693d9130d
);
windows_core::imp::interface_hierarchy!(IMFByteStream, windows_core::IUnknown);
#[repr(C)]
pub struct IMFByteStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetCapabilities: usize,
    GetLength: usize,
    SetLength: usize,
    GetCurrentPosition: usize,
    SetCurrentPosition: usize,
    IsEndOfStream: usize,
    Read: usize,
    BeginRead: usize,
    EndRead: usize,
    Write: usize,
    BeginWrite: usize,
    EndWrite: usize,
    Seek: usize,
    Flush: usize,
    Close: usize,
}
impl windows_core::RuntimeName for IMFByteStream {}
windows_core::imp::define_interface!(
    IMFMediaEngine,
    IMFMediaEngine_Vtbl,
    0x98a1b0bb_03eb_4935_ae7c_93c1fa0e1c93
);
windows_core::imp::interface_hierarchy!(IMFMediaEngine, windows_core::IUnknown);
impl IMFMediaEngine {
    pub unsafe fn GetError(&self) -> windows_core::Result<IMFMediaError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetError)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetErrorCode)(
                windows_core::Interface::as_raw(self),
                error,
            )
            .ok()
        }
    }
    pub unsafe fn SetSourceElements<P0>(&self, psrcelements: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMFMediaEngineSrcElements>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSourceElements)(
                windows_core::Interface::as_raw(self),
                psrcelements.param().abi(),
            )
            .ok()
        }
    }
    pub unsafe fn SetSource(&self, purl: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetSource)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(purl),
            )
            .ok()
        }
    }
    pub unsafe fn GetCurrentSource(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSource)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNetworkState(&self) -> u16 {
        unsafe {
            (windows_core::Interface::vtable(self).GetNetworkState)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn GetPreload(&self) -> MF_MEDIA_ENGINE_PRELOAD {
        unsafe {
            (windows_core::Interface::vtable(self).GetPreload)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn SetPreload(&self, preload: MF_MEDIA_ENGINE_PRELOAD) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPreload)(
                windows_core::Interface::as_raw(self),
                preload,
            )
            .ok()
        }
    }
    pub unsafe fn GetBuffered(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffered)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Load(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub unsafe fn CanPlayType(
        &self,
        r#type: &windows_core::BSTR,
    ) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPlayType)(
                windows_core::Interface::as_raw(self),
                core::mem::transmute_copy(r#type),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetReadyState(&self) -> u16 {
        unsafe {
            (windows_core::Interface::vtable(self).GetReadyState)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn IsSeeking(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsSeeking)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn GetCurrentTime(&self) -> f64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetCurrentTime)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn SetCurrentTime(&self, seektime: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCurrentTime)(
                windows_core::Interface::as_raw(self),
                seektime,
            )
            .ok()
        }
    }
    pub unsafe fn GetStartTime(&self) -> f64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetStartTime)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetDuration(&self) -> f64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn IsPaused(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsPaused)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn GetDefaultPlaybackRate(&self) -> f64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetDefaultPlaybackRate)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn SetDefaultPlaybackRate(&self, rate: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetDefaultPlaybackRate)(
                windows_core::Interface::as_raw(self),
                rate,
            )
            .ok()
        }
    }
    pub unsafe fn GetPlaybackRate(&self) -> f64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetPlaybackRate)(
                windows_core::Interface::as_raw(self),
            )
        }
    }
    pub unsafe fn SetPlaybackRate(&self, rate: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetPlaybackRate)(
                windows_core::Interface::as_raw(self),
                rate,
            )
            .ok()
        }
    }
    pub unsafe fn GetPlayed(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlayed)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSeekable(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSeekable)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsEnded(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsEnded)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn GetAutoPlay(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).GetAutoPlay)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn SetAutoPlay(&self, autoplay: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAutoPlay)(
                windows_core::Interface::as_raw(self),
                autoplay.into(),
            )
            .ok()
        }
    }
    pub unsafe fn GetLoop(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).GetLoop)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn SetLoop(&self, r#loop: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetLoop)(
                windows_core::Interface::as_raw(self),
                r#loop.into(),
            )
            .ok()
        }
    }
    pub unsafe fn Play(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok()
        }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn GetMuted(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).GetMuted)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn SetMuted(&self, muted: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetMuted)(
                windows_core::Interface::as_raw(self),
                muted.into(),
            )
            .ok()
        }
    }
    pub unsafe fn GetVolume(&self) -> f64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn SetVolume(&self, volume: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetVolume)(
                windows_core::Interface::as_raw(self),
                volume,
            )
            .ok()
        }
    }
    pub unsafe fn HasVideo(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).HasVideo)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn HasAudio(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).HasAudio)(windows_core::Interface::as_raw(self))
        }
    }
    pub unsafe fn GetNativeVideoSize(
        &self,
        cx: Option<*mut u32>,
        cy: Option<*mut u32>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetNativeVideoSize)(
                windows_core::Interface::as_raw(self),
                cx.unwrap_or(core::mem::zeroed()) as _,
                cy.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetVideoAspectRatio(
        &self,
        cx: Option<*mut u32>,
        cy: Option<*mut u32>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetVideoAspectRatio)(
                windows_core::Interface::as_raw(self),
                cx.unwrap_or(core::mem::zeroed()) as _,
                cy.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self))
                .ok()
        }
    }
    pub unsafe fn TransferVideoFrame<P0>(
        &self,
        pdstsurf: P0,
        psrc: Option<*const MFVideoNormalizedRect>,
        pdst: *const RECT,
        pborderclr: Option<*const MFARGB>,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).TransferVideoFrame)(
                windows_core::Interface::as_raw(self),
                pdstsurf.param().abi(),
                psrc.unwrap_or(core::mem::zeroed()) as _,
                pdst,
                pborderclr.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn OnVideoStreamTick(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnVideoStreamTick)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IMFMediaEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetError: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetErrorCode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MF_MEDIA_ENGINE_ERR,
    ) -> windows_core::HRESULT,
    pub SetSourceElements: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetCurrentSource: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetNetworkState: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub GetPreload: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_MEDIA_ENGINE_PRELOAD,
    pub SetPreload: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MF_MEDIA_ENGINE_PRELOAD,
    ) -> windows_core::HRESULT,
    pub GetBuffered: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanPlayType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut MF_MEDIA_ENGINE_CANPLAY,
    ) -> windows_core::HRESULT,
    pub GetReadyState: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub IsSeeking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetCurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetCurrentTime:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub IsPaused: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDefaultPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetDefaultPlaybackRate:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetPlaybackRate:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetPlayed: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetSeekable: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub IsEnded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetAutoPlay: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetLoop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetLoop: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMuted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetMuted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub HasVideo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub HasAudio: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetNativeVideoSize: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub GetVideoAspectRatio: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut u32,
        *mut u32,
    ) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TransferVideoFrame: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const MFVideoNormalizedRect,
        *const RECT,
        *const MFARGB,
    ) -> windows_core::HRESULT,
    pub OnVideoStreamTick:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
pub trait IMFMediaEngine_Impl: windows_core::IUnknownImpl {
    fn GetError(&self) -> windows_core::Result<IMFMediaError>;
    fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> windows_core::Result<()>;
    fn SetSourceElements(
        &self,
        psrcelements: windows_core::Ref<IMFMediaEngineSrcElements>,
    ) -> windows_core::Result<()>;
    fn SetSource(&self, purl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCurrentSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetNetworkState(&self) -> u16;
    fn GetPreload(&self) -> MF_MEDIA_ENGINE_PRELOAD;
    fn SetPreload(&self, preload: MF_MEDIA_ENGINE_PRELOAD) -> windows_core::Result<()>;
    fn GetBuffered(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn Load(&self) -> windows_core::Result<()>;
    fn CanPlayType(
        &self,
        r#type: &windows_core::BSTR,
    ) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY>;
    fn GetReadyState(&self) -> u16;
    fn IsSeeking(&self) -> windows_core::BOOL;
    fn GetCurrentTime(&self) -> f64;
    fn SetCurrentTime(&self, seektime: f64) -> windows_core::Result<()>;
    fn GetStartTime(&self) -> f64;
    fn GetDuration(&self) -> f64;
    fn IsPaused(&self) -> windows_core::BOOL;
    fn GetDefaultPlaybackRate(&self) -> f64;
    fn SetDefaultPlaybackRate(&self, rate: f64) -> windows_core::Result<()>;
    fn GetPlaybackRate(&self) -> f64;
    fn SetPlaybackRate(&self, rate: f64) -> windows_core::Result<()>;
    fn GetPlayed(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn GetSeekable(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn IsEnded(&self) -> windows_core::BOOL;
    fn GetAutoPlay(&self) -> windows_core::BOOL;
    fn SetAutoPlay(&self, autoplay: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLoop(&self) -> windows_core::BOOL;
    fn SetLoop(&self, r#loop: windows_core::BOOL) -> windows_core::Result<()>;
    fn Play(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn GetMuted(&self) -> windows_core::BOOL;
    fn SetMuted(&self, muted: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetVolume(&self) -> f64;
    fn SetVolume(&self, volume: f64) -> windows_core::Result<()>;
    fn HasVideo(&self) -> windows_core::BOOL;
    fn HasAudio(&self) -> windows_core::BOOL;
    fn GetNativeVideoSize(&self, cx: *mut u32, cy: *mut u32) -> windows_core::Result<()>;
    fn GetVideoAspectRatio(&self, cx: *mut u32, cy: *mut u32) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn TransferVideoFrame(
        &self,
        pdstsurf: windows_core::Ref<windows_core::IUnknown>,
        psrc: *const MFVideoNormalizedRect,
        pdst: *const RECT,
        pborderclr: *const MFARGB,
    ) -> windows_core::Result<()>;
    fn OnVideoStreamTick(&self) -> windows_core::Result<i64>;
}
impl IMFMediaEngine_Vtbl {
    pub const fn new<Identity: IMFMediaEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetError<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pperror: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetError(this) {
                    Ok(ok__) => {
                        pperror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorCode<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            error: MF_MEDIA_ENGINE_ERR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetErrorCode(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn SetSourceElements<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psrcelements: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetSourceElements(
                    this,
                    core::mem::transmute_copy(&psrcelements),
                )
                .into()
            }
        }
        unsafe extern "system" fn SetSource<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            purl: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetSource(this, core::mem::transmute(&purl)).into()
            }
        }
        unsafe extern "system" fn GetCurrentSource<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppurl: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetCurrentSource(this) {
                    Ok(ok__) => {
                        ppurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNetworkState<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u16 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetNetworkState(this)
            }
        }
        unsafe extern "system" fn GetPreload<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> MF_MEDIA_ENGINE_PRELOAD {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetPreload(this)
            }
        }
        unsafe extern "system" fn SetPreload<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            preload: MF_MEDIA_ENGINE_PRELOAD,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetPreload(this, core::mem::transmute_copy(&preload)).into()
            }
        }
        unsafe extern "system" fn GetBuffered<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppbuffered: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetBuffered(this) {
                    Ok(ok__) => {
                        ppbuffered.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Load<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Load(this).into()
            }
        }
        unsafe extern "system" fn CanPlayType<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            r#type: *mut core::ffi::c_void,
            panswer: *mut MF_MEDIA_ENGINE_CANPLAY,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::CanPlayType(this, core::mem::transmute(&r#type)) {
                    Ok(ok__) => {
                        panswer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReadyState<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> u16 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetReadyState(this)
            }
        }
        unsafe extern "system" fn IsSeeking<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::IsSeeking(this)
            }
        }
        unsafe extern "system" fn GetCurrentTime<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetCurrentTime(this)
            }
        }
        unsafe extern "system" fn SetCurrentTime<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            seektime: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetCurrentTime(this, core::mem::transmute_copy(&seektime))
                    .into()
            }
        }
        unsafe extern "system" fn GetStartTime<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetStartTime(this)
            }
        }
        unsafe extern "system" fn GetDuration<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetDuration(this)
            }
        }
        unsafe extern "system" fn IsPaused<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::IsPaused(this)
            }
        }
        unsafe extern "system" fn GetDefaultPlaybackRate<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetDefaultPlaybackRate(this)
            }
        }
        unsafe extern "system" fn SetDefaultPlaybackRate<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rate: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetDefaultPlaybackRate(this, core::mem::transmute_copy(&rate))
                    .into()
            }
        }
        unsafe extern "system" fn GetPlaybackRate<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetPlaybackRate(this)
            }
        }
        unsafe extern "system" fn SetPlaybackRate<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rate: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetPlaybackRate(this, core::mem::transmute_copy(&rate)).into()
            }
        }
        unsafe extern "system" fn GetPlayed<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppplayed: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetPlayed(this) {
                    Ok(ok__) => {
                        ppplayed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSeekable<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppseekable: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetSeekable(this) {
                    Ok(ok__) => {
                        ppseekable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEnded<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::IsEnded(this)
            }
        }
        unsafe extern "system" fn GetAutoPlay<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetAutoPlay(this)
            }
        }
        unsafe extern "system" fn SetAutoPlay<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            autoplay: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetAutoPlay(this, core::mem::transmute_copy(&autoplay)).into()
            }
        }
        unsafe extern "system" fn GetLoop<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetLoop(this)
            }
        }
        unsafe extern "system" fn SetLoop<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            r#loop: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetLoop(this, core::mem::transmute_copy(&r#loop)).into()
            }
        }
        unsafe extern "system" fn Play<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Play(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn GetMuted<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetMuted(this)
            }
        }
        unsafe extern "system" fn SetMuted<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            muted: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetMuted(this, core::mem::transmute_copy(&muted)).into()
            }
        }
        unsafe extern "system" fn GetVolume<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> f64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetVolume(this)
            }
        }
        unsafe extern "system" fn SetVolume<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            volume: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetVolume(this, core::mem::transmute_copy(&volume)).into()
            }
        }
        unsafe extern "system" fn HasVideo<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::HasVideo(this)
            }
        }
        unsafe extern "system" fn HasAudio<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::HasAudio(this)
            }
        }
        unsafe extern "system" fn GetNativeVideoSize<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cx: *mut u32,
            cy: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetNativeVideoSize(
                    this,
                    core::mem::transmute_copy(&cx),
                    core::mem::transmute_copy(&cy),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetVideoAspectRatio<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            cx: *mut u32,
            cy: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetVideoAspectRatio(
                    this,
                    core::mem::transmute_copy(&cx),
                    core::mem::transmute_copy(&cy),
                )
                .into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Shutdown(this).into()
            }
        }
        unsafe extern "system" fn TransferVideoFrame<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdstsurf: *mut core::ffi::c_void,
            psrc: *const MFVideoNormalizedRect,
            pdst: *const RECT,
            pborderclr: *const MFARGB,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::TransferVideoFrame(
                    this,
                    core::mem::transmute_copy(&pdstsurf),
                    core::mem::transmute_copy(&psrc),
                    core::mem::transmute_copy(&pdst),
                    core::mem::transmute_copy(&pborderclr),
                )
                .into()
            }
        }
        unsafe extern "system" fn OnVideoStreamTick<
            Identity: IMFMediaEngine_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ppts: *mut i64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::OnVideoStreamTick(this) {
                    Ok(ok__) => {
                        ppts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, OFFSET>,
            SetErrorCode: SetErrorCode::<Identity, OFFSET>,
            SetSourceElements: SetSourceElements::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
            GetCurrentSource: GetCurrentSource::<Identity, OFFSET>,
            GetNetworkState: GetNetworkState::<Identity, OFFSET>,
            GetPreload: GetPreload::<Identity, OFFSET>,
            SetPreload: SetPreload::<Identity, OFFSET>,
            GetBuffered: GetBuffered::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            CanPlayType: CanPlayType::<Identity, OFFSET>,
            GetReadyState: GetReadyState::<Identity, OFFSET>,
            IsSeeking: IsSeeking::<Identity, OFFSET>,
            GetCurrentTime: GetCurrentTime::<Identity, OFFSET>,
            SetCurrentTime: SetCurrentTime::<Identity, OFFSET>,
            GetStartTime: GetStartTime::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            IsPaused: IsPaused::<Identity, OFFSET>,
            GetDefaultPlaybackRate: GetDefaultPlaybackRate::<Identity, OFFSET>,
            SetDefaultPlaybackRate: SetDefaultPlaybackRate::<Identity, OFFSET>,
            GetPlaybackRate: GetPlaybackRate::<Identity, OFFSET>,
            SetPlaybackRate: SetPlaybackRate::<Identity, OFFSET>,
            GetPlayed: GetPlayed::<Identity, OFFSET>,
            GetSeekable: GetSeekable::<Identity, OFFSET>,
            IsEnded: IsEnded::<Identity, OFFSET>,
            GetAutoPlay: GetAutoPlay::<Identity, OFFSET>,
            SetAutoPlay: SetAutoPlay::<Identity, OFFSET>,
            GetLoop: GetLoop::<Identity, OFFSET>,
            SetLoop: SetLoop::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            GetMuted: GetMuted::<Identity, OFFSET>,
            SetMuted: SetMuted::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            HasVideo: HasVideo::<Identity, OFFSET>,
            HasAudio: HasAudio::<Identity, OFFSET>,
            GetNativeVideoSize: GetNativeVideoSize::<Identity, OFFSET>,
            GetVideoAspectRatio: GetVideoAspectRatio::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            TransferVideoFrame: TransferVideoFrame::<Identity, OFFSET>,
            OnVideoStreamTick: OnVideoStreamTick::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngine as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngine {}
windows_core::imp::define_interface!(
    IMFMediaEngineClassFactory,
    IMFMediaEngineClassFactory_Vtbl,
    0x4d645ace_26aa_4688_9be1_df3516990b93
);
windows_core::imp::interface_hierarchy!(IMFMediaEngineClassFactory, windows_core::IUnknown);
impl IMFMediaEngineClassFactory {
    pub unsafe fn CreateInstance<P1>(
        &self,
        dwflags: u32,
        pattr: P1,
    ) -> windows_core::Result<IMFMediaEngine>
    where
        P1: windows_core::Param<IMFAttributes>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(
                windows_core::Interface::as_raw(self),
                dwflags,
                pattr.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTimeRange(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTimeRange)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateError(&self) -> windows_core::Result<IMFMediaError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateError)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IMFMediaEngineClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateTimeRange: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub CreateError: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(
        &self,
        dwflags: u32,
        pattr: windows_core::Ref<IMFAttributes>,
    ) -> windows_core::Result<IMFMediaEngine>;
    fn CreateTimeRange(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn CreateError(&self) -> windows_core::Result<IMFMediaError>;
}
impl IMFMediaEngineClassFactory_Vtbl {
    pub const fn new<Identity: IMFMediaEngineClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<
            Identity: IMFMediaEngineClassFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwflags: u32,
            pattr: *mut core::ffi::c_void,
            ppplayer: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactory_Impl::CreateInstance(
                    this,
                    core::mem::transmute_copy(&dwflags),
                    core::mem::transmute_copy(&pattr),
                ) {
                    Ok(ok__) => {
                        ppplayer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTimeRange<
            Identity: IMFMediaEngineClassFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pptimerange: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactory_Impl::CreateTimeRange(this) {
                    Ok(ok__) => {
                        pptimerange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateError<
            Identity: IMFMediaEngineClassFactory_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pperror: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactory_Impl::CreateError(this) {
                    Ok(ok__) => {
                        pperror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            CreateTimeRange: CreateTimeRange::<Identity, OFFSET>,
            CreateError: CreateError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineClassFactory {}
windows_core::imp::define_interface!(
    IMFMediaEngineEx,
    IMFMediaEngineEx_Vtbl,
    0x83015ead_b1e6_40d0_a98a_37145ffe1ad1
);
impl core::ops::Deref for IMFMediaEngineEx {
    type Target = IMFMediaEngine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaEngineEx, windows_core::IUnknown, IMFMediaEngine);
impl IMFMediaEngineEx {
    pub unsafe fn SetSourceFromByteStream<P0>(
        &self,
        pbytestream: P0,
        purl: &windows_core::BSTR,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMFByteStream>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSourceFromByteStream)(
                windows_core::Interface::as_raw(self),
                pbytestream.param().abi(),
                core::mem::transmute_copy(purl),
            )
            .ok()
        }
    }
    pub unsafe fn GetStatistics(
        &self,
        statisticid: MF_MEDIA_ENGINE_STATISTIC,
    ) -> windows_core::Result<PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatistics)(
                windows_core::Interface::as_raw(self),
                statisticid,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn UpdateVideoStream(
        &self,
        psrc: Option<*const MFVideoNormalizedRect>,
        pdst: Option<*const RECT>,
        pborderclr: Option<*const MFARGB>,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).UpdateVideoStream)(
                windows_core::Interface::as_raw(self),
                psrc.unwrap_or(core::mem::zeroed()) as _,
                pdst.unwrap_or(core::mem::zeroed()) as _,
                pborderclr.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
    pub unsafe fn GetBalance(&self) -> f64 {
        unsafe {
            (windows_core::Interface::vtable(self).GetBalance)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn SetBalance(&self, balance: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetBalance)(
                windows_core::Interface::as_raw(self),
                balance,
            )
            .ok()
        }
    }
    pub unsafe fn IsPlaybackRateSupported(&self, rate: f64) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsPlaybackRateSupported)(
                windows_core::Interface::as_raw(self),
                rate,
            )
        }
    }
    pub unsafe fn FrameStep(&self, forward: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).FrameStep)(
                windows_core::Interface::as_raw(self),
                forward.into(),
            )
            .ok()
        }
    }
    pub unsafe fn GetResourceCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceCharacteristics)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetPresentationAttribute(
        &self,
        guidmfattribute: *const windows_core::GUID,
    ) -> windows_core::Result<PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentationAttribute)(
                windows_core::Interface::as_raw(self),
                guidmfattribute,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNumberOfStreams(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfStreams)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetStreamAttribute(
        &self,
        dwstreamindex: u32,
        guidmfattribute: *const windows_core::GUID,
    ) -> windows_core::Result<PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamAttribute)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                guidmfattribute,
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetStreamSelection(
        &self,
        dwstreamindex: u32,
    ) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelection)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetStreamSelection(
        &self,
        dwstreamindex: u32,
        enabled: bool,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStreamSelection)(
                windows_core::Interface::as_raw(self),
                dwstreamindex,
                enabled.into(),
            )
            .ok()
        }
    }
    pub unsafe fn ApplyStreamSelections(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).ApplyStreamSelections)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub unsafe fn IsProtected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsProtected)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn InsertVideoEffect<P0>(
        &self,
        peffect: P0,
        foptional: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertVideoEffect)(
                windows_core::Interface::as_raw(self),
                peffect.param().abi(),
                foptional.into(),
            )
            .ok()
        }
    }
    pub unsafe fn InsertAudioEffect<P0>(
        &self,
        peffect: P0,
        foptional: bool,
    ) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).InsertAudioEffect)(
                windows_core::Interface::as_raw(self),
                peffect.param().abi(),
                foptional.into(),
            )
            .ok()
        }
    }
    pub unsafe fn RemoveAllEffects(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveAllEffects)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub unsafe fn SetTimelineMarkerTimer(&self, timetofire: f64) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetTimelineMarkerTimer)(
                windows_core::Interface::as_raw(self),
                timetofire,
            )
            .ok()
        }
    }
    pub unsafe fn GetTimelineMarkerTimer(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimelineMarkerTimer)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn CancelTimelineMarkerTimer(&self) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).CancelTimelineMarkerTimer)(
                windows_core::Interface::as_raw(self),
            )
            .ok()
        }
    }
    pub unsafe fn IsStereo3D(&self) -> windows_core::BOOL {
        unsafe {
            (windows_core::Interface::vtable(self).IsStereo3D)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub unsafe fn GetStereo3DFramePackingMode(
        &self,
    ) -> windows_core::Result<MF_MEDIA_ENGINE_S3D_PACKING_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStereo3DFramePackingMode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetStereo3DFramePackingMode(
        &self,
        packmode: MF_MEDIA_ENGINE_S3D_PACKING_MODE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStereo3DFramePackingMode)(
                windows_core::Interface::as_raw(self),
                packmode,
            )
            .ok()
        }
    }
    pub unsafe fn GetStereo3DRenderMode(&self) -> windows_core::Result<MF3DVideoOutputType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStereo3DRenderMode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetStereo3DRenderMode(
        &self,
        outputtype: MF3DVideoOutputType,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetStereo3DRenderMode)(
                windows_core::Interface::as_raw(self),
                outputtype,
            )
            .ok()
        }
    }
    pub unsafe fn EnableWindowlessSwapchainMode(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).EnableWindowlessSwapchainMode)(
                windows_core::Interface::as_raw(self),
                fenable.into(),
            )
            .ok()
        }
    }
    pub unsafe fn GetVideoSwapchainHandle(&self) -> windows_core::Result<HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoSwapchainHandle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn EnableHorizontalMirrorMode(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).EnableHorizontalMirrorMode)(
                windows_core::Interface::as_raw(self),
                fenable.into(),
            )
            .ok()
        }
    }
    pub unsafe fn GetAudioStreamCategory(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioStreamCategory)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetAudioStreamCategory(&self, category: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAudioStreamCategory)(
                windows_core::Interface::as_raw(self),
                category,
            )
            .ok()
        }
    }
    pub unsafe fn GetAudioEndpointRole(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioEndpointRole)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetAudioEndpointRole(&self, role: u32) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetAudioEndpointRole)(
                windows_core::Interface::as_raw(self),
                role,
            )
            .ok()
        }
    }
    pub unsafe fn GetRealTimeMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRealTimeMode)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn SetRealTimeMode(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetRealTimeMode)(
                windows_core::Interface::as_raw(self),
                fenable.into(),
            )
            .ok()
        }
    }
    pub unsafe fn SetCurrentTimeEx(
        &self,
        seektime: f64,
        seekmode: MF_MEDIA_ENGINE_SEEK_MODE,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).SetCurrentTimeEx)(
                windows_core::Interface::as_raw(self),
                seektime,
                seekmode,
            )
            .ok()
        }
    }
    pub unsafe fn EnableTimeUpdateTimer(&self, fenabletimer: bool) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).EnableTimeUpdateTimer)(
                windows_core::Interface::as_raw(self),
                fenabletimer.into(),
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMFMediaEngineEx_Vtbl {
    pub base__: IMFMediaEngine_Vtbl,
    pub SetSourceFromByteStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MF_MEDIA_ENGINE_STATISTIC,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub UpdateVideoStream: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const MFVideoNormalizedRect,
        *const RECT,
        *const MFARGB,
    ) -> windows_core::HRESULT,
    pub GetBalance: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetBalance: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsPlaybackRateSupported:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::BOOL,
    pub FrameStep: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetResourceCharacteristics:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPresentationAttribute: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub GetNumberOfStreams:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStreamAttribute: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const windows_core::GUID,
        *mut PROPVARIANT,
    ) -> windows_core::HRESULT,
    pub GetStreamSelection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetStreamSelection: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub ApplyStreamSelections:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub InsertVideoEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub InsertAudioEffect: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub RemoveAllEffects:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimelineMarkerTimer:
        unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetTimelineMarkerTimer:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CancelTimelineMarkerTimer:
        unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsStereo3D: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetStereo3DFramePackingMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MF_MEDIA_ENGINE_S3D_PACKING_MODE,
    ) -> windows_core::HRESULT,
    pub SetStereo3DFramePackingMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MF_MEDIA_ENGINE_S3D_PACKING_MODE,
    ) -> windows_core::HRESULT,
    pub GetStereo3DRenderMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut MF3DVideoOutputType,
    ) -> windows_core::HRESULT,
    pub SetStereo3DRenderMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        MF3DVideoOutputType,
    ) -> windows_core::HRESULT,
    pub EnableWindowlessSwapchainMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetVideoSwapchainHandle:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut HANDLE) -> windows_core::HRESULT,
    pub EnableHorizontalMirrorMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub GetAudioStreamCategory:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAudioStreamCategory:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAudioEndpointRole:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAudioEndpointRole:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetRealTimeMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetRealTimeMode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
    pub SetCurrentTimeEx: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        f64,
        MF_MEDIA_ENGINE_SEEK_MODE,
    ) -> windows_core::HRESULT,
    pub EnableTimeUpdateTimer: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_core::BOOL,
    ) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineEx_Impl: IMFMediaEngine_Impl {
    fn SetSourceFromByteStream(
        &self,
        pbytestream: windows_core::Ref<IMFByteStream>,
        purl: &windows_core::BSTR,
    ) -> windows_core::Result<()>;
    fn GetStatistics(
        &self,
        statisticid: MF_MEDIA_ENGINE_STATISTIC,
    ) -> windows_core::Result<PROPVARIANT>;
    fn UpdateVideoStream(
        &self,
        psrc: *const MFVideoNormalizedRect,
        pdst: *const RECT,
        pborderclr: *const MFARGB,
    ) -> windows_core::Result<()>;
    fn GetBalance(&self) -> f64;
    fn SetBalance(&self, balance: f64) -> windows_core::Result<()>;
    fn IsPlaybackRateSupported(&self, rate: f64) -> windows_core::BOOL;
    fn FrameStep(&self, forward: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetResourceCharacteristics(&self) -> windows_core::Result<u32>;
    fn GetPresentationAttribute(
        &self,
        guidmfattribute: *const windows_core::GUID,
    ) -> windows_core::Result<PROPVARIANT>;
    fn GetNumberOfStreams(&self) -> windows_core::Result<u32>;
    fn GetStreamAttribute(
        &self,
        dwstreamindex: u32,
        guidmfattribute: *const windows_core::GUID,
    ) -> windows_core::Result<PROPVARIANT>;
    fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetStreamSelection(
        &self,
        dwstreamindex: u32,
        enabled: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn ApplyStreamSelections(&self) -> windows_core::Result<()>;
    fn IsProtected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn InsertVideoEffect(
        &self,
        peffect: windows_core::Ref<windows_core::IUnknown>,
        foptional: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn InsertAudioEffect(
        &self,
        peffect: windows_core::Ref<windows_core::IUnknown>,
        foptional: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn RemoveAllEffects(&self) -> windows_core::Result<()>;
    fn SetTimelineMarkerTimer(&self, timetofire: f64) -> windows_core::Result<()>;
    fn GetTimelineMarkerTimer(&self) -> windows_core::Result<f64>;
    fn CancelTimelineMarkerTimer(&self) -> windows_core::Result<()>;
    fn IsStereo3D(&self) -> windows_core::BOOL;
    fn GetStereo3DFramePackingMode(&self)
    -> windows_core::Result<MF_MEDIA_ENGINE_S3D_PACKING_MODE>;
    fn SetStereo3DFramePackingMode(
        &self,
        packmode: MF_MEDIA_ENGINE_S3D_PACKING_MODE,
    ) -> windows_core::Result<()>;
    fn GetStereo3DRenderMode(&self) -> windows_core::Result<MF3DVideoOutputType>;
    fn SetStereo3DRenderMode(&self, outputtype: MF3DVideoOutputType) -> windows_core::Result<()>;
    fn EnableWindowlessSwapchainMode(
        &self,
        fenable: windows_core::BOOL,
    ) -> windows_core::Result<()>;
    fn GetVideoSwapchainHandle(&self) -> windows_core::Result<HANDLE>;
    fn EnableHorizontalMirrorMode(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAudioStreamCategory(&self) -> windows_core::Result<u32>;
    fn SetAudioStreamCategory(&self, category: u32) -> windows_core::Result<()>;
    fn GetAudioEndpointRole(&self) -> windows_core::Result<u32>;
    fn SetAudioEndpointRole(&self, role: u32) -> windows_core::Result<()>;
    fn GetRealTimeMode(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetRealTimeMode(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetCurrentTimeEx(
        &self,
        seektime: f64,
        seekmode: MF_MEDIA_ENGINE_SEEK_MODE,
    ) -> windows_core::Result<()>;
    fn EnableTimeUpdateTimer(&self, fenabletimer: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IMFMediaEngineEx_Vtbl {
    pub const fn new<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSourceFromByteStream<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbytestream: *mut core::ffi::c_void,
            purl: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetSourceFromByteStream(
                    this,
                    core::mem::transmute_copy(&pbytestream),
                    core::mem::transmute(&purl),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetStatistics<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            statisticid: MF_MEDIA_ENGINE_STATISTIC,
            pstatistic: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStatistics(
                    this,
                    core::mem::transmute_copy(&statisticid),
                ) {
                    Ok(ok__) => {
                        pstatistic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateVideoStream<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            psrc: *const MFVideoNormalizedRect,
            pdst: *const RECT,
            pborderclr: *const MFARGB,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::UpdateVideoStream(
                    this,
                    core::mem::transmute_copy(&psrc),
                    core::mem::transmute_copy(&pdst),
                    core::mem::transmute_copy(&pborderclr),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetBalance<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> f64 {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::GetBalance(this)
            }
        }
        unsafe extern "system" fn SetBalance<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            balance: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetBalance(this, core::mem::transmute_copy(&balance)).into()
            }
        }
        unsafe extern "system" fn IsPlaybackRateSupported<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            rate: f64,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::IsPlaybackRateSupported(
                    this,
                    core::mem::transmute_copy(&rate),
                )
            }
        }
        unsafe extern "system" fn FrameStep<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            forward: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::FrameStep(this, core::mem::transmute_copy(&forward)).into()
            }
        }
        unsafe extern "system" fn GetResourceCharacteristics<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcharacteristics: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetResourceCharacteristics(this) {
                    Ok(ok__) => {
                        pcharacteristics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentationAttribute<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            guidmfattribute: *const windows_core::GUID,
            pvvalue: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetPresentationAttribute(
                    this,
                    core::mem::transmute_copy(&guidmfattribute),
                ) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfStreams<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pdwstreamcount: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetNumberOfStreams(this) {
                    Ok(ok__) => {
                        pdwstreamcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamAttribute<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            guidmfattribute: *const windows_core::GUID,
            pvvalue: *mut PROPVARIANT,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStreamAttribute(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&guidmfattribute),
                ) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamSelection<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            penabled: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStreamSelection(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                ) {
                    Ok(ok__) => {
                        penabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamSelection<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            dwstreamindex: u32,
            enabled: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetStreamSelection(
                    this,
                    core::mem::transmute_copy(&dwstreamindex),
                    core::mem::transmute_copy(&enabled),
                )
                .into()
            }
        }
        unsafe extern "system" fn ApplyStreamSelections<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::ApplyStreamSelections(this).into()
            }
        }
        unsafe extern "system" fn IsProtected<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pprotected: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::IsProtected(this) {
                    Ok(ok__) => {
                        pprotected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertVideoEffect<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            peffect: *mut core::ffi::c_void,
            foptional: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::InsertVideoEffect(
                    this,
                    core::mem::transmute_copy(&peffect),
                    core::mem::transmute_copy(&foptional),
                )
                .into()
            }
        }
        unsafe extern "system" fn InsertAudioEffect<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            peffect: *mut core::ffi::c_void,
            foptional: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::InsertAudioEffect(
                    this,
                    core::mem::transmute_copy(&peffect),
                    core::mem::transmute_copy(&foptional),
                )
                .into()
            }
        }
        unsafe extern "system" fn RemoveAllEffects<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::RemoveAllEffects(this).into()
            }
        }
        unsafe extern "system" fn SetTimelineMarkerTimer<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            timetofire: f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetTimelineMarkerTimer(
                    this,
                    core::mem::transmute_copy(&timetofire),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetTimelineMarkerTimer<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            ptimetofire: *mut f64,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetTimelineMarkerTimer(this) {
                    Ok(ok__) => {
                        ptimetofire.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelTimelineMarkerTimer<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::CancelTimelineMarkerTimer(this).into()
            }
        }
        unsafe extern "system" fn IsStereo3D<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::BOOL {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::IsStereo3D(this)
            }
        }
        unsafe extern "system" fn GetStereo3DFramePackingMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            packmode: *mut MF_MEDIA_ENGINE_S3D_PACKING_MODE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStereo3DFramePackingMode(this) {
                    Ok(ok__) => {
                        packmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStereo3DFramePackingMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            packmode: MF_MEDIA_ENGINE_S3D_PACKING_MODE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetStereo3DFramePackingMode(
                    this,
                    core::mem::transmute_copy(&packmode),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetStereo3DRenderMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            outputtype: *mut MF3DVideoOutputType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStereo3DRenderMode(this) {
                    Ok(ok__) => {
                        outputtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStereo3DRenderMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            outputtype: MF3DVideoOutputType,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetStereo3DRenderMode(
                    this,
                    core::mem::transmute_copy(&outputtype),
                )
                .into()
            }
        }
        unsafe extern "system" fn EnableWindowlessSwapchainMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fenable: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::EnableWindowlessSwapchainMode(
                    this,
                    core::mem::transmute_copy(&fenable),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetVideoSwapchainHandle<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            phswapchain: *mut HANDLE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetVideoSwapchainHandle(this) {
                    Ok(ok__) => {
                        phswapchain.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableHorizontalMirrorMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fenable: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::EnableHorizontalMirrorMode(
                    this,
                    core::mem::transmute_copy(&fenable),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetAudioStreamCategory<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pcategory: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetAudioStreamCategory(this) {
                    Ok(ok__) => {
                        pcategory.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAudioStreamCategory<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            category: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetAudioStreamCategory(
                    this,
                    core::mem::transmute_copy(&category),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetAudioEndpointRole<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            prole: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetAudioEndpointRole(this) {
                    Ok(ok__) => {
                        prole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAudioEndpointRole<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            role: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetAudioEndpointRole(this, core::mem::transmute_copy(&role))
                    .into()
            }
        }
        unsafe extern "system" fn GetRealTimeMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pfenabled: *mut windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetRealTimeMode(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRealTimeMode<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fenable: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetRealTimeMode(this, core::mem::transmute_copy(&fenable))
                    .into()
            }
        }
        unsafe extern "system" fn SetCurrentTimeEx<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            seektime: f64,
            seekmode: MF_MEDIA_ENGINE_SEEK_MODE,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetCurrentTimeEx(
                    this,
                    core::mem::transmute_copy(&seektime),
                    core::mem::transmute_copy(&seekmode),
                )
                .into()
            }
        }
        unsafe extern "system" fn EnableTimeUpdateTimer<
            Identity: IMFMediaEngineEx_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            fenabletimer: windows_core::BOOL,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::EnableTimeUpdateTimer(
                    this,
                    core::mem::transmute_copy(&fenabletimer),
                )
                .into()
            }
        }
        Self {
            base__: IMFMediaEngine_Vtbl::new::<Identity, OFFSET>(),
            SetSourceFromByteStream: SetSourceFromByteStream::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            UpdateVideoStream: UpdateVideoStream::<Identity, OFFSET>,
            GetBalance: GetBalance::<Identity, OFFSET>,
            SetBalance: SetBalance::<Identity, OFFSET>,
            IsPlaybackRateSupported: IsPlaybackRateSupported::<Identity, OFFSET>,
            FrameStep: FrameStep::<Identity, OFFSET>,
            GetResourceCharacteristics: GetResourceCharacteristics::<Identity, OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Identity, OFFSET>,
            GetNumberOfStreams: GetNumberOfStreams::<Identity, OFFSET>,
            GetStreamAttribute: GetStreamAttribute::<Identity, OFFSET>,
            GetStreamSelection: GetStreamSelection::<Identity, OFFSET>,
            SetStreamSelection: SetStreamSelection::<Identity, OFFSET>,
            ApplyStreamSelections: ApplyStreamSelections::<Identity, OFFSET>,
            IsProtected: IsProtected::<Identity, OFFSET>,
            InsertVideoEffect: InsertVideoEffect::<Identity, OFFSET>,
            InsertAudioEffect: InsertAudioEffect::<Identity, OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Identity, OFFSET>,
            SetTimelineMarkerTimer: SetTimelineMarkerTimer::<Identity, OFFSET>,
            GetTimelineMarkerTimer: GetTimelineMarkerTimer::<Identity, OFFSET>,
            CancelTimelineMarkerTimer: CancelTimelineMarkerTimer::<Identity, OFFSET>,
            IsStereo3D: IsStereo3D::<Identity, OFFSET>,
            GetStereo3DFramePackingMode: GetStereo3DFramePackingMode::<Identity, OFFSET>,
            SetStereo3DFramePackingMode: SetStereo3DFramePackingMode::<Identity, OFFSET>,
            GetStereo3DRenderMode: GetStereo3DRenderMode::<Identity, OFFSET>,
            SetStereo3DRenderMode: SetStereo3DRenderMode::<Identity, OFFSET>,
            EnableWindowlessSwapchainMode: EnableWindowlessSwapchainMode::<Identity, OFFSET>,
            GetVideoSwapchainHandle: GetVideoSwapchainHandle::<Identity, OFFSET>,
            EnableHorizontalMirrorMode: EnableHorizontalMirrorMode::<Identity, OFFSET>,
            GetAudioStreamCategory: GetAudioStreamCategory::<Identity, OFFSET>,
            SetAudioStreamCategory: SetAudioStreamCategory::<Identity, OFFSET>,
            GetAudioEndpointRole: GetAudioEndpointRole::<Identity, OFFSET>,
            SetAudioEndpointRole: SetAudioEndpointRole::<Identity, OFFSET>,
            GetRealTimeMode: GetRealTimeMode::<Identity, OFFSET>,
            SetRealTimeMode: SetRealTimeMode::<Identity, OFFSET>,
            SetCurrentTimeEx: SetCurrentTimeEx::<Identity, OFFSET>,
            EnableTimeUpdateTimer: EnableTimeUpdateTimer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineEx as windows_core::Interface>::IID
            || iid == &<IMFMediaEngine as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineEx {}
windows_core::imp::define_interface!(
    IMFMediaEngineNotify,
    IMFMediaEngineNotify_Vtbl,
    0xfee7c112_e776_42b5_9bbf_0048524e2bd5
);
windows_core::imp::interface_hierarchy!(IMFMediaEngineNotify, windows_core::IUnknown);
impl IMFMediaEngineNotify {
    pub unsafe fn EventNotify(
        &self,
        event: u32,
        param1: usize,
        param2: u32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).EventNotify)(
                windows_core::Interface::as_raw(self),
                event,
                param1,
                param2,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IMFMediaEngineNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EventNotify:
        unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, u32) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineNotify_Impl: windows_core::IUnknownImpl {
    fn EventNotify(&self, event: u32, param1: usize, param2: u32) -> windows_core::Result<()>;
}
impl IMFMediaEngineNotify_Vtbl {
    pub const fn new<Identity: IMFMediaEngineNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventNotify<
            Identity: IMFMediaEngineNotify_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            event: u32,
            param1: usize,
            param2: u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineNotify_Impl::EventNotify(
                    this,
                    core::mem::transmute_copy(&event),
                    core::mem::transmute_copy(&param1),
                    core::mem::transmute_copy(&param2),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EventNotify: EventNotify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineNotify {}
windows_core::imp::define_interface!(
    IMFMediaEngineSrcElements,
    IMFMediaEngineSrcElements_Vtbl,
    0x7a5e5354_b114_4c72_b991_3131d75032ea
);
windows_core::imp::interface_hierarchy!(IMFMediaEngineSrcElements, windows_core::IUnknown);
#[repr(C)]
pub struct IMFMediaEngineSrcElements_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetLength: usize,
    GetURL: usize,
    GetType: usize,
    GetMedia: usize,
    AddElement: usize,
    RemoveAllElements: usize,
}
impl windows_core::RuntimeName for IMFMediaEngineSrcElements {}
windows_core::imp::define_interface!(
    IMFMediaError,
    IMFMediaError_Vtbl,
    0xfc0e10d2_ab2a_4501_a951_06bb1075184c
);
windows_core::imp::interface_hierarchy!(IMFMediaError, windows_core::IUnknown);
#[repr(C)]
pub struct IMFMediaError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetErrorCode: usize,
    GetExtendedErrorCode: usize,
    SetErrorCode: usize,
    SetExtendedErrorCode: usize,
}
impl windows_core::RuntimeName for IMFMediaError {}
windows_core::imp::define_interface!(
    IMFMediaTimeRange,
    IMFMediaTimeRange_Vtbl,
    0xdb71a2fc_078a_414e_9df9_8c2531b0aa6c
);
windows_core::imp::interface_hierarchy!(IMFMediaTimeRange, windows_core::IUnknown);
#[repr(C)]
pub struct IMFMediaTimeRange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetLength: usize,
    GetStart: usize,
    GetEnd: usize,
    ContainsTime: usize,
    AddRange: usize,
    Clear: usize,
}
impl windows_core::RuntimeName for IMFMediaTimeRange {}
windows_core::imp::define_interface!(
    IModalWindow,
    IModalWindow_Vtbl,
    0xb4db1657_70d7_485e_8e3e_6fcb5a5c1802
);
windows_core::imp::interface_hierarchy!(IModalWindow, windows_core::IUnknown);
impl IModalWindow {
    pub unsafe fn Show(&self, hwndowner: Option<HWND>) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).Show)(
                windows_core::Interface::as_raw(self),
                hwndowner.unwrap_or(core::mem::zeroed()) as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
pub struct IModalWindow_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Show: unsafe extern "system" fn(*mut core::ffi::c_void, HWND) -> windows_core::HRESULT,
}
pub trait IModalWindow_Impl: windows_core::IUnknownImpl {
    fn Show(&self, hwndowner: HWND) -> windows_core::Result<()>;
}
impl IModalWindow_Vtbl {
    pub const fn new<Identity: IModalWindow_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Show<Identity: IModalWindow_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            hwndowner: HWND,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IModalWindow_Impl::Show(this, core::mem::transmute_copy(&hwndowner)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IModalWindow as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IModalWindow {}
windows_core::imp::define_interface!(
    IPrintDocumentPackageTarget,
    IPrintDocumentPackageTarget_Vtbl,
    0x1b8efec4_3019_4c27_964e_367202156906
);
windows_core::imp::interface_hierarchy!(IPrintDocumentPackageTarget, windows_core::IUnknown);
#[repr(C)]
pub struct IPrintDocumentPackageTarget_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetPackageTargetTypes: usize,
    GetPackageTarget: usize,
    Cancel: usize,
}
impl windows_core::RuntimeName for IPrintDocumentPackageTarget {}
windows_core::imp::define_interface!(
    ISequentialStream,
    ISequentialStream_Vtbl,
    0x0c733a30_2a1c_11ce_ade5_00aa0044773d
);
windows_core::imp::interface_hierarchy!(ISequentialStream, windows_core::IUnknown);
#[repr(C)]
pub struct ISequentialStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    Read: usize,
    Write: usize,
}
impl windows_core::RuntimeName for ISequentialStream {}
windows_core::imp::define_interface!(
    IShellItem,
    IShellItem_Vtbl,
    0x43826d1e_e718_42ee_bc55_a1e261c37bfe
);
windows_core::imp::interface_hierarchy!(IShellItem, windows_core::IUnknown);
impl IShellItem {
    pub unsafe fn BindToHandler<P0, T>(
        &self,
        pbc: P0,
        bhid: *const windows_core::GUID,
    ) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IBindCtx>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).BindToHandler)(
                windows_core::Interface::as_raw(self),
                pbc.param().abi(),
                bhid,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParent)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDisplayName(
        &self,
        sigdnname: SIGDN,
    ) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(
                windows_core::Interface::as_raw(self),
                sigdnname,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetAttributes(
        &self,
        sfgaomask: SFGAO_FLAGS,
    ) -> windows_core::Result<SFGAO_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributes)(
                windows_core::Interface::as_raw(self),
                sfgaomask,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn Compare<P0>(&self, psi: P0, hint: u32) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(
                windows_core::Interface::as_raw(self),
                psi.param().abi(),
                hint,
                &mut result__,
            )
            .map(|| result__)
        }
    }
}
#[repr(C)]
pub struct IShellItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BindToHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SIGDN,
        *mut windows_core::PWSTR,
    ) -> windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SFGAO_FLAGS,
        *mut SFGAO_FLAGS,
    ) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut i32,
    ) -> windows_core::HRESULT,
}
pub trait IShellItem_Impl: windows_core::IUnknownImpl {
    fn BindToHandler(
        &self,
        pbc: windows_core::Ref<IBindCtx>,
        bhid: *const windows_core::GUID,
        riid: *const windows_core::GUID,
        ppv: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetParent(&self) -> windows_core::Result<IShellItem>;
    fn GetDisplayName(&self, sigdnname: SIGDN) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAttributes(&self, sfgaomask: SFGAO_FLAGS) -> windows_core::Result<SFGAO_FLAGS>;
    fn Compare(&self, psi: windows_core::Ref<IShellItem>, hint: u32) -> windows_core::Result<i32>;
}
impl IShellItem_Vtbl {
    pub const fn new<Identity: IShellItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindToHandler<Identity: IShellItem_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pbc: *mut core::ffi::c_void,
            bhid: *const windows_core::GUID,
            riid: *const windows_core::GUID,
            ppv: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellItem_Impl::BindToHandler(
                    this,
                    core::mem::transmute_copy(&pbc),
                    core::mem::transmute_copy(&bhid),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppv),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IShellItem_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppsi: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItem_Impl::GetParent(this) {
                    Ok(ok__) => {
                        ppsi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IShellItem_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            sigdnname: SIGDN,
            ppszname: *mut windows_core::PWSTR,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItem_Impl::GetDisplayName(this, core::mem::transmute_copy(&sigdnname)) {
                    Ok(ok__) => {
                        ppszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributes<Identity: IShellItem_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            sfgaomask: SFGAO_FLAGS,
            psfgaoattribs: *mut SFGAO_FLAGS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItem_Impl::GetAttributes(this, core::mem::transmute_copy(&sfgaomask)) {
                    Ok(ok__) => {
                        psfgaoattribs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Compare<Identity: IShellItem_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            psi: *mut core::ffi::c_void,
            hint: u32,
            piorder: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItem_Impl::Compare(
                    this,
                    core::mem::transmute_copy(&psi),
                    core::mem::transmute_copy(&hint),
                ) {
                    Ok(ok__) => {
                        piorder.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindToHandler: BindToHandler::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellItem as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IShellItem {}
windows_core::imp::define_interface!(
    IShellItemArray,
    IShellItemArray_Vtbl,
    0xb63ea76d_1f85_456f_a19c_48159efa858b
);
windows_core::imp::interface_hierarchy!(IShellItemArray, windows_core::IUnknown);
impl IShellItemArray {
    pub unsafe fn BindToHandler<P0, T>(
        &self,
        pbc: P0,
        bhid: *const windows_core::GUID,
    ) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IBindCtx>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).BindToHandler)(
                windows_core::Interface::as_raw(self),
                pbc.param().abi(),
                bhid,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyStore<T>(
        &self,
        flags: GETPROPERTYSTOREFLAGS,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetPropertyStore)(
                windows_core::Interface::as_raw(self),
                flags,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPropertyDescriptionList<T>(
        &self,
        keytype: *const PROPERTYKEY,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetPropertyDescriptionList)(
                windows_core::Interface::as_raw(self),
                keytype,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetAttributes(
        &self,
        attribflags: SIATTRIBFLAGS,
        sfgaomask: SFGAO_FLAGS,
    ) -> windows_core::Result<SFGAO_FLAGS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributes)(
                windows_core::Interface::as_raw(self),
                attribflags,
                sfgaomask,
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetItemAt(&self, dwindex: u32) -> windows_core::Result<IShellItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemAt)(
                windows_core::Interface::as_raw(self),
                dwindex,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnumItems(&self) -> windows_core::Result<IEnumShellItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumItems)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IShellItemArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub BindToHandler: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPropertyStore: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        GETPROPERTYSTOREFLAGS,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetPropertyDescriptionList: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const PROPERTYKEY,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub GetAttributes: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        SIATTRIBFLAGS,
        SFGAO_FLAGS,
        *mut SFGAO_FLAGS,
    ) -> windows_core::HRESULT,
    pub GetCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetItemAt: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    pub EnumItems: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IShellItemArray_Impl: windows_core::IUnknownImpl {
    fn BindToHandler(
        &self,
        pbc: windows_core::Ref<IBindCtx>,
        bhid: *const windows_core::GUID,
        riid: *const windows_core::GUID,
        ppvout: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetPropertyStore(
        &self,
        flags: GETPROPERTYSTOREFLAGS,
        riid: *const windows_core::GUID,
        ppv: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetPropertyDescriptionList(
        &self,
        keytype: *const PROPERTYKEY,
        riid: *const windows_core::GUID,
        ppv: *mut *mut core::ffi::c_void,
    ) -> windows_core::Result<()>;
    fn GetAttributes(
        &self,
        attribflags: SIATTRIBFLAGS,
        sfgaomask: SFGAO_FLAGS,
    ) -> windows_core::Result<SFGAO_FLAGS>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetItemAt(&self, dwindex: u32) -> windows_core::Result<IShellItem>;
    fn EnumItems(&self) -> windows_core::Result<IEnumShellItems>;
}
impl IShellItemArray_Vtbl {
    pub const fn new<Identity: IShellItemArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindToHandler<
            Identity: IShellItemArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pbc: *mut core::ffi::c_void,
            bhid: *const windows_core::GUID,
            riid: *const windows_core::GUID,
            ppvout: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellItemArray_Impl::BindToHandler(
                    this,
                    core::mem::transmute_copy(&pbc),
                    core::mem::transmute_copy(&bhid),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppvout),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPropertyStore<
            Identity: IShellItemArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            flags: GETPROPERTYSTOREFLAGS,
            riid: *const windows_core::GUID,
            ppv: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellItemArray_Impl::GetPropertyStore(
                    this,
                    core::mem::transmute_copy(&flags),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppv),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetPropertyDescriptionList<
            Identity: IShellItemArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            keytype: *const PROPERTYKEY,
            riid: *const windows_core::GUID,
            ppv: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IShellItemArray_Impl::GetPropertyDescriptionList(
                    this,
                    core::mem::transmute_copy(&keytype),
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&ppv),
                )
                .into()
            }
        }
        unsafe extern "system" fn GetAttributes<
            Identity: IShellItemArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            attribflags: SIATTRIBFLAGS,
            sfgaomask: SFGAO_FLAGS,
            psfgaoattribs: *mut SFGAO_FLAGS,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItemArray_Impl::GetAttributes(
                    this,
                    core::mem::transmute_copy(&attribflags),
                    core::mem::transmute_copy(&sfgaomask),
                ) {
                    Ok(ok__) => {
                        psfgaoattribs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: IShellItemArray_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            pdwnumitems: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItemArray_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pdwnumitems.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetItemAt<Identity: IShellItemArray_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            dwindex: u32,
            ppsi: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItemArray_Impl::GetItemAt(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppsi.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumItems<Identity: IShellItemArray_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            ppenumshellitems: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShellItemArray_Impl::EnumItems(this) {
                    Ok(ok__) => {
                        ppenumshellitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BindToHandler: BindToHandler::<Identity, OFFSET>,
            GetPropertyStore: GetPropertyStore::<Identity, OFFSET>,
            GetPropertyDescriptionList: GetPropertyDescriptionList::<Identity, OFFSET>,
            GetAttributes: GetAttributes::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            GetItemAt: GetItemAt::<Identity, OFFSET>,
            EnumItems: EnumItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShellItemArray as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IShellItemArray {}
windows_core::imp::define_interface!(
    IShellItemFilter,
    IShellItemFilter_Vtbl,
    0x2659b475_eeb8_48b7_8f07_b378810f48cf
);
windows_core::imp::interface_hierarchy!(IShellItemFilter, windows_core::IUnknown);
#[repr(C)]
pub struct IShellItemFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    IncludeItem: usize,
    GetEnumFlagsForItem: usize,
}
impl windows_core::RuntimeName for IShellItemFilter {}
windows_core::imp::define_interface!(
    IStorage,
    IStorage_Vtbl,
    0x0000000b_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IStorage, windows_core::IUnknown);
#[repr(C)]
pub struct IStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateStream: usize,
    OpenStream: usize,
    CreateStorage: usize,
    OpenStorage: usize,
    CopyTo: usize,
    MoveElementTo: usize,
    Commit: usize,
    Revert: usize,
    EnumElements: usize,
    DestroyElement: usize,
    RenameElement: usize,
    SetElementTimes: usize,
    SetClass: usize,
    SetStateBits: usize,
    Stat: usize,
}
impl windows_core::RuntimeName for IStorage {}
windows_core::imp::define_interface!(
    IStream,
    IStream_Vtbl,
    0x0000000c_0000_0000_c000_000000000046
);
impl core::ops::Deref for IStream {
    type Target = ISequentialStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IStream, windows_core::IUnknown, ISequentialStream);
#[repr(C)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    Seek: usize,
    SetSize: usize,
    CopyTo: usize,
    Commit: usize,
    Revert: usize,
    LockRegion: usize,
    UnlockRegion: usize,
    Stat: usize,
    Clone: usize,
}
impl windows_core::RuntimeName for IStream {}
windows_core::imp::define_interface!(
    IWICBitmap,
    IWICBitmap_Vtbl,
    0x00000121_a8f2_4877_ba0a_fd2b6645fb94
);
impl core::ops::Deref for IWICBitmap {
    type Target = IWICBitmapSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWICBitmap, windows_core::IUnknown, IWICBitmapSource);
#[repr(C)]
pub struct IWICBitmap_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    Lock: usize,
    SetPalette: usize,
    SetResolution: usize,
}
impl windows_core::RuntimeName for IWICBitmap {}
windows_core::imp::define_interface!(
    IWICBitmapSource,
    IWICBitmapSource_Vtbl,
    0x00000120_a8f2_4877_ba0a_fd2b6645fb94
);
windows_core::imp::interface_hierarchy!(IWICBitmapSource, windows_core::IUnknown);
#[repr(C)]
pub struct IWICBitmapSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetSize: usize,
    GetPixelFormat: usize,
    GetResolution: usize,
    CopyPalette: usize,
    CopyPixels: usize,
}
impl windows_core::RuntimeName for IWICBitmapSource {}
windows_core::imp::define_interface!(
    IWICColorContext,
    IWICColorContext_Vtbl,
    0x3c613a02_34b2_44ea_9a7c_45aea9c6fd6d
);
windows_core::imp::interface_hierarchy!(IWICColorContext, windows_core::IUnknown);
#[repr(C)]
pub struct IWICColorContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    InitializeFromFilename: usize,
    InitializeFromMemory: usize,
    InitializeFromExifColorSpace: usize,
    GetType: usize,
    GetProfileBytes: usize,
    GetExifColorSpace: usize,
}
impl windows_core::RuntimeName for IWICColorContext {}
windows_core::imp::define_interface!(
    IWICImagingFactory,
    IWICImagingFactory_Vtbl,
    0xec5ec8a9_c395_4314_9c77_54d7a935ff70
);
windows_core::imp::interface_hierarchy!(IWICImagingFactory, windows_core::IUnknown);
#[repr(C)]
pub struct IWICImagingFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateDecoderFromFilename: usize,
    CreateDecoderFromStream: usize,
    CreateDecoderFromFileHandle: usize,
    CreateComponentInfo: usize,
    CreateDecoder: usize,
    CreateEncoder: usize,
    CreatePalette: usize,
    CreateFormatConverter: usize,
    CreateBitmapScaler: usize,
    CreateBitmapClipper: usize,
    CreateBitmapFlipRotator: usize,
    CreateStream: usize,
    CreateColorContext: usize,
    CreateColorTransformer: usize,
    CreateBitmap: usize,
    CreateBitmapFromSource: usize,
    CreateBitmapFromSourceRect: usize,
    CreateBitmapFromMemory: usize,
    CreateBitmapFromHBITMAP: usize,
    CreateBitmapFromHICON: usize,
    CreateComponentEnumerator: usize,
    CreateFastMetadataEncoderFromDecoder: usize,
    CreateFastMetadataEncoderFromFrameDecode: usize,
    CreateQueryWriter: usize,
    CreateQueryWriterFromReader: usize,
}
impl windows_core::RuntimeName for IWICImagingFactory {}
pub const LOAD_IGNORE_CODE_AUTHZ_LEVEL: LOAD_LIBRARY_FLAGS = 16;
pub const LOAD_LIBRARY_AS_DATAFILE: LOAD_LIBRARY_FLAGS = 2;
pub const LOAD_LIBRARY_AS_DATAFILE_EXCLUSIVE: LOAD_LIBRARY_FLAGS = 64;
pub const LOAD_LIBRARY_AS_IMAGE_RESOURCE: LOAD_LIBRARY_FLAGS = 32;
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_REQUIRE_SIGNED_TARGET: LOAD_LIBRARY_FLAGS = 128;
pub const LOAD_LIBRARY_SAFE_CURRENT_DIRS: LOAD_LIBRARY_FLAGS = 8192;
pub const LOAD_LIBRARY_SEARCH_APPLICATION_DIR: LOAD_LIBRARY_FLAGS = 512;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096;
pub const LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR: LOAD_LIBRARY_FLAGS = 256;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32: LOAD_LIBRARY_FLAGS = 2048;
pub const LOAD_LIBRARY_SEARCH_SYSTEM32_NO_FORWARDER: LOAD_LIBRARY_FLAGS = 16384;
pub const LOAD_LIBRARY_SEARCH_USER_DIRS: LOAD_LIBRARY_FLAGS = 1024;
pub const LOAD_WITH_ALTERED_SEARCH_PATH: LOAD_LIBRARY_FLAGS = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
pub type MF3DVideoOutputType = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFARGB {
    pub rgbBlue: u8,
    pub rgbGreen: u8,
    pub rgbRed: u8,
    pub rgbAlpha: u8,
}
pub const MFSTARTUP_FULL: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFVideoNormalizedRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type MF_ATTRIBUTES_MATCH_TYPE = i32;
pub type MF_ATTRIBUTE_TYPE = i32;
pub const MF_MEDIA_ENGINE_CALLBACK: windows_core::GUID =
    windows_core::GUID::from_u128(0xc60381b8_83a4_41f8_a3d0_de05076849a9);
pub type MF_MEDIA_ENGINE_CANPLAY = i32;
pub type MF_MEDIA_ENGINE_ERR = i32;
pub type MF_MEDIA_ENGINE_EVENT = i32;
pub const MF_MEDIA_ENGINE_EVENT_ABORT: MF_MEDIA_ENGINE_EVENT = 4;
pub const MF_MEDIA_ENGINE_EVENT_AUDIOENDPOINTCHANGE: MF_MEDIA_ENGINE_EVENT = 1016;
pub const MF_MEDIA_ENGINE_EVENT_BALANCECHANGE: MF_MEDIA_ENGINE_EVENT = 1003;
pub const MF_MEDIA_ENGINE_EVENT_BUFFERINGENDED: MF_MEDIA_ENGINE_EVENT = 1006;
pub const MF_MEDIA_ENGINE_EVENT_BUFFERINGSTARTED: MF_MEDIA_ENGINE_EVENT = 1005;
pub const MF_MEDIA_ENGINE_EVENT_CANPLAY: MF_MEDIA_ENGINE_EVENT = 14;
pub const MF_MEDIA_ENGINE_EVENT_CANPLAYTHROUGH: MF_MEDIA_ENGINE_EVENT = 15;
pub const MF_MEDIA_ENGINE_EVENT_DELAYLOADEVENT_CHANGED: MF_MEDIA_ENGINE_EVENT = 1013;
pub const MF_MEDIA_ENGINE_EVENT_DOWNLOADCOMPLETE: MF_MEDIA_ENGINE_EVENT = 1004;
pub const MF_MEDIA_ENGINE_EVENT_DURATIONCHANGE: MF_MEDIA_ENGINE_EVENT = 21;
pub const MF_MEDIA_ENGINE_EVENT_EMPTIED: MF_MEDIA_ENGINE_EVENT = 6;
pub const MF_MEDIA_ENGINE_EVENT_ENDED: MF_MEDIA_ENGINE_EVENT = 19;
pub const MF_MEDIA_ENGINE_EVENT_ERROR: MF_MEDIA_ENGINE_EVENT = 5;
pub const MF_MEDIA_ENGINE_EVENT_FIRSTFRAMEREADY: MF_MEDIA_ENGINE_EVENT = 1009;
pub const MF_MEDIA_ENGINE_EVENT_FORMATCHANGE: MF_MEDIA_ENGINE_EVENT = 1000;
pub const MF_MEDIA_ENGINE_EVENT_FRAMESTEPCOMPLETED: MF_MEDIA_ENGINE_EVENT = 1007;
pub const MF_MEDIA_ENGINE_EVENT_LOADEDDATA: MF_MEDIA_ENGINE_EVENT = 11;
pub const MF_MEDIA_ENGINE_EVENT_LOADEDMETADATA: MF_MEDIA_ENGINE_EVENT = 10;
pub const MF_MEDIA_ENGINE_EVENT_LOADSTART: MF_MEDIA_ENGINE_EVENT = 1;
pub const MF_MEDIA_ENGINE_EVENT_NOTIFYSTABLESTATE: MF_MEDIA_ENGINE_EVENT = 1008;
pub const MF_MEDIA_ENGINE_EVENT_OPMINFO: MF_MEDIA_ENGINE_EVENT = 1011;
pub const MF_MEDIA_ENGINE_EVENT_PAUSE: MF_MEDIA_ENGINE_EVENT = 9;
pub const MF_MEDIA_ENGINE_EVENT_PLAY: MF_MEDIA_ENGINE_EVENT = 8;
pub const MF_MEDIA_ENGINE_EVENT_PLAYING: MF_MEDIA_ENGINE_EVENT = 13;
pub const MF_MEDIA_ENGINE_EVENT_PROGRESS: MF_MEDIA_ENGINE_EVENT = 2;
pub const MF_MEDIA_ENGINE_EVENT_PURGEQUEUEDEVENTS: MF_MEDIA_ENGINE_EVENT = 1001;
pub const MF_MEDIA_ENGINE_EVENT_RATECHANGE: MF_MEDIA_ENGINE_EVENT = 20;
pub const MF_MEDIA_ENGINE_EVENT_RESOURCELOST: MF_MEDIA_ENGINE_EVENT = 1012;
pub const MF_MEDIA_ENGINE_EVENT_SEEKED: MF_MEDIA_ENGINE_EVENT = 17;
pub const MF_MEDIA_ENGINE_EVENT_SEEKING: MF_MEDIA_ENGINE_EVENT = 16;
pub const MF_MEDIA_ENGINE_EVENT_STALLED: MF_MEDIA_ENGINE_EVENT = 7;
pub const MF_MEDIA_ENGINE_EVENT_STREAMRENDERINGERROR: MF_MEDIA_ENGINE_EVENT = 1014;
pub const MF_MEDIA_ENGINE_EVENT_SUPPORTEDRATES_CHANGED: MF_MEDIA_ENGINE_EVENT = 1015;
pub const MF_MEDIA_ENGINE_EVENT_SUSPEND: MF_MEDIA_ENGINE_EVENT = 3;
pub const MF_MEDIA_ENGINE_EVENT_TIMELINE_MARKER: MF_MEDIA_ENGINE_EVENT = 1002;
pub const MF_MEDIA_ENGINE_EVENT_TIMEUPDATE: MF_MEDIA_ENGINE_EVENT = 18;
pub const MF_MEDIA_ENGINE_EVENT_TRACKSCHANGE: MF_MEDIA_ENGINE_EVENT = 1010;
pub const MF_MEDIA_ENGINE_EVENT_VOLUMECHANGE: MF_MEDIA_ENGINE_EVENT = 22;
pub const MF_MEDIA_ENGINE_EVENT_WAITING: MF_MEDIA_ENGINE_EVENT = 12;
pub const MF_MEDIA_ENGINE_PLAYBACK_HWND: windows_core::GUID =
    windows_core::GUID::from_u128(0xd988879b_67c9_4d92_baa7_6eadd446039d);
pub type MF_MEDIA_ENGINE_PRELOAD = i32;
pub type MF_MEDIA_ENGINE_S3D_PACKING_MODE = i32;
pub type MF_MEDIA_ENGINE_SEEK_MODE = i32;
pub type MF_MEDIA_ENGINE_STATISTIC = i32;
pub const MF_VERSION: u32 = 131184;
pub const PATHCCH_NONE: PATHCCH_OPTIONS = 0;
pub type PATHCCH_OPTIONS = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROPERTYKEY {
    pub fmtid: windows_core::GUID,
    pub pid: u32,
}
#[repr(C)]
pub struct PROPVARIANT {
    pub Anonymous: PROPVARIANT_0,
}
impl Default for PROPVARIANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROPVARIANT_0 {
    pub Anonymous: core::mem::ManuallyDrop<PROPVARIANT_0_0>,
    pub decVal: DECIMAL,
}
impl Clone for PROPVARIANT_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PROPVARIANT_0_0 {
    pub vt: VARENUM,
    pub wReserved1: u16,
    pub wReserved2: u16,
    pub wReserved3: u16,
    pub Anonymous: PROPVARIANT_0_0_0,
}
impl Clone for PROPVARIANT_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
pub union PROPVARIANT_0_0_0 {
    pub cVal: i8,
    pub bVal: u8,
    pub iVal: i16,
    pub uiVal: u16,
    pub lVal: i32,
    pub ulVal: u32,
    pub intVal: i32,
    pub uintVal: u32,
    pub hVal: i64,
    pub uhVal: u64,
    pub fltVal: f32,
    pub dblVal: f64,
    pub boolVal: VARIANT_BOOL,
    pub __OBSOLETE__VARIANT_BOOL: VARIANT_BOOL,
    pub scode: i32,
    pub cyVal: CY,
    pub date: f64,
    pub filetime: FILETIME,
    pub puuid: *mut windows_core::GUID,
    pub pclipdata: *mut CLIPDATA,
    pub bstrVal: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub bstrblobVal: BSTRBLOB,
    pub blob: BLOB,
    pub pszVal: windows_core::PSTR,
    pub pwszVal: windows_core::PWSTR,
    pub punkVal: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub pdispVal: core::mem::ManuallyDrop<Option<IDispatch>>,
    pub pStream: core::mem::ManuallyDrop<Option<IStream>>,
    pub pStorage: core::mem::ManuallyDrop<Option<IStorage>>,
    pub pVersionedStream: *mut VERSIONEDSTREAM,
    pub parray: *mut SAFEARRAY,
    pub cac: CAC,
    pub caub: CAUB,
    pub cai: CAI,
    pub caui: CAUI,
    pub cal: CAL,
    pub caul: CAUL,
    pub cah: CAH,
    pub cauh: CAUH,
    pub caflt: CAFLT,
    pub cadbl: CADBL,
    pub cabool: CABOOL,
    pub cascode: CASCODE,
    pub cacy: CACY,
    pub cadate: CADATE,
    pub cafiletime: CAFILETIME,
    pub cauuid: CACLSID,
    pub caclipdata: CACLIPDATA,
    pub cabstr: CABSTR,
    pub cabstrblob: CABSTRBLOB,
    pub calpstr: CALPSTR,
    pub calpwstr: CALPWSTR,
    pub capropvar: CAPROPVARIANT,
    pub pcVal: windows_core::PSTR,
    pub pbVal: *mut u8,
    pub piVal: *mut i16,
    pub puiVal: *mut u16,
    pub plVal: *mut i32,
    pub pulVal: *mut u32,
    pub pintVal: *mut i32,
    pub puintVal: *mut u32,
    pub pfltVal: *mut f32,
    pub pdblVal: *mut f64,
    pub pboolVal: *mut VARIANT_BOOL,
    pub pdecVal: *mut DECIMAL,
    pub pscode: *mut i32,
    pub pcyVal: *mut CY,
    pub pdate: *mut f64,
    pub pbstrVal: *mut windows_core::BSTR,
    pub ppunkVal: *mut Option<windows_core::IUnknown>,
    pub ppdispVal: *mut Option<IDispatch>,
    pub pparray: *mut *mut SAFEARRAY,
    pub pvarVal: *mut PROPVARIANT,
}
impl Clone for PROPVARIANT_0_0_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
impl Default for PROPVARIANT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SAFEARRAY {
    pub cDims: u16,
    pub fFeatures: ADVANCED_FEATURE_FLAGS,
    pub cbElements: u32,
    pub cLocks: u32,
    pub pvData: *mut core::ffi::c_void,
    pub rgsabound: [SAFEARRAYBOUND; 1],
}
impl Default for SAFEARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SAFEARRAYBOUND {
    pub cElements: u32,
    pub lLbound: i32,
}
pub type SFGAO_FLAGS = u32;
pub type SIATTRIBFLAGS = i32;
pub type SIGDN = i32;
pub const SIGDN_DESKTOPABSOLUTEEDITING: SIGDN = -2147172352;
pub const SIGDN_DESKTOPABSOLUTEPARSING: SIGDN = -2147319808;
pub const SIGDN_FILESYSPATH: SIGDN = -2147123200;
pub const SIGDN_NORMALDISPLAY: SIGDN = 0;
pub const SIGDN_PARENTRELATIVE: SIGDN = -2146959359;
pub const SIGDN_PARENTRELATIVEEDITING: SIGDN = -2147282943;
pub const SIGDN_PARENTRELATIVEFORADDRESSBAR: SIGDN = -2146975743;
pub const SIGDN_PARENTRELATIVEFORUI: SIGDN = -2146877439;
pub const SIGDN_PARENTRELATIVEPARSING: SIGDN = -2147385343;
pub const SIGDN_URL: SIGDN = -2147057664;
pub type VARENUM = u16;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct VARIANT_BOOL(pub i16);
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VERSIONEDSTREAM {
    pub guidVersion: windows_core::GUID,
    pub pStream: core::mem::ManuallyDrop<Option<IStream>>,
}
pub type WIN32_ERROR = u32;
