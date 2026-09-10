#[inline]
pub unsafe fn DWriteCreateFactory<T>(factorytype: DWRITE_FACTORY_TYPE) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { DWriteCreateFactory(factorytype, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
}
pub type DWRITE_FACTORY_TYPE = i32;
pub const DWRITE_FACTORY_TYPE_ISOLATED: DWRITE_FACTORY_TYPE = 1;
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0;
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
windows_core::imp::define_interface!(IDWriteFactory, IDWriteFactory_Vtbl, 0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48);
windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
impl IDWriteFactory {
    pub unsafe fn GetSystemFontCollection(&self, fontcollection: *mut Option<IDWriteFontCollection>, checkforupdates: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSystemFontCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(fontcollection), checkforupdates.into()).ok() }
    }
    pub unsafe fn CreateTextFormat<P0, P1, P6>(&self, fontfamilyname: P0, fontcollection: P1, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: P6) -> windows_core::Result<IDWriteTextFormat>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IDWriteFontCollection>,
        P6: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextFormat)(windows_core::Interface::as_raw(self), fontfamilyname.param().abi(), fontcollection.param().abi(), fontweight, fontstyle, fontstretch, fontsize, localename.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTextLayout<P2>(&self, string: &[u16], textformat: P2, maxwidth: f32, maxheight: f32) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextLayout)(windows_core::Interface::as_raw(self), core::mem::transmute(string.as_ptr()), string.len().try_into().unwrap(), textformat.param().abi(), maxwidth, maxheight, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSystemFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    CreateCustomFontCollection: usize,
    RegisterFontCollectionLoader: usize,
    UnregisterFontCollectionLoader: usize,
    CreateFontFileReference: usize,
    CreateCustomFontFileReference: usize,
    CreateFontFace: usize,
    CreateRenderingParams: usize,
    CreateMonitorRenderingParams: usize,
    CreateCustomRenderingParams: usize,
    RegisterFontFileLoader: usize,
    UnregisterFontFileLoader: usize,
    pub CreateTextFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, DWRITE_FONT_WEIGHT, DWRITE_FONT_STYLE, DWRITE_FONT_STRETCH, f32, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateTypography: usize,
    GetGdiInterop: usize,
    pub CreateTextLayout: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, f32, f32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateGdiCompatibleTextLayout: usize,
    CreateEllipsisTrimmingSign: usize,
    CreateTextAnalyzer: usize,
    CreateNumberSubstitution: usize,
    CreateGlyphRunAnalysis: usize,
}
unsafe impl Send for IDWriteFactory {}
unsafe impl Sync for IDWriteFactory {}
impl windows_core::RuntimeName for IDWriteFactory {}
windows_core::imp::define_interface!(IDWriteFontCollection, IDWriteFontCollection_Vtbl, 0xa84cee02_3eea_4eee_a827_87c1a02a0fcc);
windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
impl IDWriteFontCollection {
    pub unsafe fn GetFontFamilyCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FindFamilyName<P0>(&self, familyname: P0, index: *mut u32, exists: *mut windows_core::BOOL) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindFamilyName)(windows_core::Interface::as_raw(self), familyname.param().abi(), index as _, exists as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFontFamilyCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    GetFontFamily: usize,
    pub FindFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    GetFontFromFontFace: usize,
}
unsafe impl Send for IDWriteFontCollection {}
unsafe impl Sync for IDWriteFontCollection {}
impl windows_core::RuntimeName for IDWriteFontCollection {}
windows_core::imp::define_interface!(IDWriteTextFormat, IDWriteTextFormat_Vtbl, 0x9c906818_31d7_4fd3_a151_7c5e225db55a);
windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
impl IDWriteTextFormat {
    pub unsafe fn SetIncrementalTabStop(&self, incrementaltabstop: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIncrementalTabStop)(windows_core::Interface::as_raw(self), incrementaltabstop).ok() }
    }
    pub unsafe fn GetIncrementalTabStop(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetIncrementalTabStop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontCollection(&self) -> windows_core::Result<IDWriteFontCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFontFamilyNameLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyNameLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontFamilyName(&self, fontfamilyname: windows_core::PWSTR, namesize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetFontFamilyName)(windows_core::Interface::as_raw(self), fontfamilyname, namesize).ok() }
    }
    pub unsafe fn GetFontWeight(&self) -> DWRITE_FONT_WEIGHT {
        unsafe { (windows_core::Interface::vtable(self).GetFontWeight)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontStyle(&self) -> DWRITE_FONT_STYLE {
        unsafe { (windows_core::Interface::vtable(self).GetFontStyle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontStretch(&self) -> DWRITE_FONT_STRETCH {
        unsafe { (windows_core::Interface::vtable(self).GetFontStretch)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontSize(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetFontSize)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLocaleNameLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleNameLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLocaleName(&self, localename: windows_core::PWSTR, namesize: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetLocaleName)(windows_core::Interface::as_raw(self), localename, namesize).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    SetTextAlignment: usize,
    SetParagraphAlignment: usize,
    SetWordWrapping: usize,
    SetReadingDirection: usize,
    SetFlowDirection: usize,
    pub SetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    SetTrimming: usize,
    SetLineSpacing: usize,
    GetTextAlignment: usize,
    GetParagraphAlignment: usize,
    GetWordWrapping: usize,
    GetReadingDirection: usize,
    GetFlowDirection: usize,
    pub GetIncrementalTabStop: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    GetTrimming: usize,
    GetLineSpacing: usize,
    pub GetFontCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFontFamilyNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetFontFamilyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
    pub GetFontWeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_WEIGHT,
    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STYLE,
    pub GetFontStretch: unsafe extern "system" fn(*mut core::ffi::c_void) -> DWRITE_FONT_STRETCH,
    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetLocaleNameLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLocaleName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDWriteTextFormat {}
unsafe impl Sync for IDWriteTextFormat {}
impl windows_core::RuntimeName for IDWriteTextFormat {}
windows_core::imp::define_interface!(IDWriteTextLayout, IDWriteTextLayout_Vtbl, 0x53737037_6d14_410b_9bfe_0b182bb70961);
impl core::ops::Deref for IDWriteTextLayout {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDWriteTextLayout, windows_core::IUnknown, IDWriteTextFormat);
impl IDWriteTextLayout {
    pub unsafe fn SetMaxWidth(&self, maxwidth: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxWidth)(windows_core::Interface::as_raw(self), maxwidth).ok() }
    }
    pub unsafe fn SetMaxHeight(&self, maxheight: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaxHeight)(windows_core::Interface::as_raw(self), maxheight).ok() }
    }
    pub unsafe fn GetMaxWidth(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetMaxWidth)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMaxHeight(&self) -> f32 {
        unsafe { (windows_core::Interface::vtable(self).GetMaxHeight)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMetrics(&self, textmetrics: *mut DWRITE_TEXT_METRICS) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMetrics)(windows_core::Interface::as_raw(self), textmetrics as _).ok() }
    }
    pub unsafe fn DetermineMinWidth(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DetermineMinWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub SetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    SetFontCollection: usize,
    SetFontFamilyName: usize,
    SetFontWeight: usize,
    SetFontStyle: usize,
    SetFontStretch: usize,
    SetFontSize: usize,
    SetUnderline: usize,
    SetStrikethrough: usize,
    SetDrawingEffect: usize,
    SetInlineObject: usize,
    SetTypography: usize,
    SetLocaleName: usize,
    pub GetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    pub GetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> f32,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetUnderline: usize,
    GetStrikethrough: usize,
    GetDrawingEffect: usize,
    GetInlineObject: usize,
    GetTypography: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
    Draw: usize,
    GetLineMetrics: usize,
    pub GetMetrics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DWRITE_TEXT_METRICS) -> windows_core::HRESULT,
    GetOverhangMetrics: usize,
    GetClusterMetrics: usize,
    pub DetermineMinWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    HitTestPoint: usize,
    HitTestTextPosition: usize,
    HitTestTextRange: usize,
}
unsafe impl Send for IDWriteTextLayout {}
unsafe impl Sync for IDWriteTextLayout {}
impl windows_core::RuntimeName for IDWriteTextLayout {}
