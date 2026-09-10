#[inline]
pub unsafe fn MFCreateAttributes(ppmfattributes: *mut Option<IMFAttributes>, cinitialsize: u32) -> windows_core::Result<()> {
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
pub const CLSID_MFMediaEngineClassFactory: windows_core::GUID = windows_core::GUID::from_u128(0xb44392da_499b_446b_a4cb_005fead0e6d5);
windows_core::imp::define_interface!(IMFAttributes, IMFAttributes_Vtbl, 0x2cd2d921_c447_44a7_a13c_4adabfc247e3);
windows_core::imp::interface_hierarchy!(IMFAttributes, windows_core::IUnknown);
impl IMFAttributes {
    pub unsafe fn GetUINT32(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUINT32)(windows_core::Interface::as_raw(self), guidkey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUINT64(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUINT64)(windows_core::Interface::as_raw(self), guidkey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDouble(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDouble)(windows_core::Interface::as_raw(self), guidkey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetGUID(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGUID)(windows_core::Interface::as_raw(self), guidkey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStringLength(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStringLength)(windows_core::Interface::as_raw(self), guidkey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetString(&self, guidkey: *const windows_core::GUID, pwszvalue: windows_core::PWSTR, cchbufsize: u32, pcchlength: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), guidkey, pwszvalue, cchbufsize, pcchlength.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetAllocatedString(&self, guidkey: *const windows_core::GUID, ppwszvalue: *mut windows_core::PWSTR, pcchlength: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAllocatedString)(windows_core::Interface::as_raw(self), guidkey, ppwszvalue as _, pcchlength as _).ok() }
    }
    pub unsafe fn GetBlobSize(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBlobSize)(windows_core::Interface::as_raw(self), guidkey, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBlob(&self, guidkey: *const windows_core::GUID, pbuf: *mut u8, cbbufsize: u32, pcbblobsize: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetBlob)(windows_core::Interface::as_raw(self), guidkey, pbuf as _, cbbufsize, pcbblobsize.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetAllocatedBlob(&self, guidkey: *const windows_core::GUID, ppbuf: *mut *mut u8, pcbsize: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetAllocatedBlob)(windows_core::Interface::as_raw(self), guidkey, ppbuf as _, pcbsize as _).ok() }
    }
    pub unsafe fn GetUnknown<T>(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetUnknown)(windows_core::Interface::as_raw(self), guidkey, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    pub unsafe fn DeleteItem(&self, guidkey: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteItem)(windows_core::Interface::as_raw(self), guidkey).ok() }
    }
    pub unsafe fn DeleteAllItems(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).DeleteAllItems)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetUINT32(&self, guidkey: *const windows_core::GUID, unvalue: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUINT32)(windows_core::Interface::as_raw(self), guidkey, unvalue).ok() }
    }
    pub unsafe fn SetUINT64(&self, guidkey: *const windows_core::GUID, unvalue: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetUINT64)(windows_core::Interface::as_raw(self), guidkey, unvalue).ok() }
    }
    pub unsafe fn SetDouble(&self, guidkey: *const windows_core::GUID, fvalue: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDouble)(windows_core::Interface::as_raw(self), guidkey, fvalue).ok() }
    }
    pub unsafe fn SetGUID(&self, guidkey: *const windows_core::GUID, guidvalue: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGUID)(windows_core::Interface::as_raw(self), guidkey, guidvalue).ok() }
    }
    pub unsafe fn SetString<P1>(&self, guidkey: *const windows_core::GUID, wszvalue: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetString)(windows_core::Interface::as_raw(self), guidkey, wszvalue.param().abi()).ok() }
    }
    pub unsafe fn SetBlob(&self, guidkey: *const windows_core::GUID, pbuf: &[u8]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBlob)(windows_core::Interface::as_raw(self), guidkey, pbuf.as_ptr(), pbuf.len().try_into().unwrap()).ok() }
    }
    pub unsafe fn SetUnknown<P1>(&self, guidkey: *const windows_core::GUID, punknown: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUnknown)(windows_core::Interface::as_raw(self), guidkey, punknown.param().abi()).ok() }
    }
    pub unsafe fn LockStore(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).LockStore)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn UnlockStore(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).UnlockStore)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CopyAllItems<P0>(&self, pdest: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyAllItems)(windows_core::Interface::as_raw(self), pdest.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAttributes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetItem: usize,
    GetItemType: usize,
    CompareItem: usize,
    Compare: usize,
    pub GetUINT32: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetUINT64: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u64) -> windows_core::HRESULT,
    pub GetDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut f64) -> windows_core::HRESULT,
    pub GetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetAllocatedString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetBlobSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub GetBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u8, u32, *mut u32) -> windows_core::HRESULT,
    pub GetAllocatedBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    SetItem: usize,
    pub DeleteItem: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub DeleteAllItems: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUINT32: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SetUINT64: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u64) -> windows_core::HRESULT,
    pub SetDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, f64) -> windows_core::HRESULT,
    pub SetGUID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetBlob: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const u8, u32) -> windows_core::HRESULT,
    pub SetUnknown: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockStore: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetItemByIndex: usize,
    pub CopyAllItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IMFAttributes {}
windows_core::imp::define_interface!(IMFMediaEngine, IMFMediaEngine_Vtbl, 0x98a1b0bb_03eb_4935_ae7c_93c1fa0e1c93);
windows_core::imp::interface_hierarchy!(IMFMediaEngine, windows_core::IUnknown);
impl IMFMediaEngine {
    pub unsafe fn SetSource(&self, purl: &windows_core::BSTR) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(purl)).ok() }
    }
    pub unsafe fn GetCurrentSource(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNetworkState(&self) -> u16 {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Load(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetReadyState(&self) -> u16 {
        unsafe { (windows_core::Interface::vtable(self).GetReadyState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsSeeking(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsSeeking)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCurrentTime(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentTime)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetCurrentTime(&self, seektime: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTime)(windows_core::Interface::as_raw(self), seektime).ok() }
    }
    pub unsafe fn GetStartTime(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetStartTime)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDuration(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsPaused(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsPaused)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDefaultPlaybackRate(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetDefaultPlaybackRate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetDefaultPlaybackRate(&self, rate: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultPlaybackRate)(windows_core::Interface::as_raw(self), rate).ok() }
    }
    pub unsafe fn GetPlaybackRate(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetPlaybackRate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPlaybackRate(&self, rate: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlaybackRate)(windows_core::Interface::as_raw(self), rate).ok() }
    }
    pub unsafe fn IsEnded(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsEnded)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAutoPlay(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetAutoPlay)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAutoPlay(&self, autoplay: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoPlay)(windows_core::Interface::as_raw(self), autoplay.into()).ok() }
    }
    pub unsafe fn GetLoop(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetLoop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLoop(&self, r#loop: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLoop)(windows_core::Interface::as_raw(self), r#loop.into()).ok() }
    }
    pub unsafe fn Play(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetMuted(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMuted)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMuted(&self, muted: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMuted)(windows_core::Interface::as_raw(self), muted.into()).ok() }
    }
    pub unsafe fn GetVolume(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetVolume(&self, volume: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume).ok() }
    }
    pub unsafe fn HasVideo(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasVideo)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn HasAudio(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasAudio)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNativeVideoSize(&self, cx: Option<*mut u32>, cy: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetNativeVideoSize)(windows_core::Interface::as_raw(self), cx.unwrap_or(core::mem::zeroed()) as _, cy.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn GetVideoAspectRatio(&self, cx: Option<*mut u32>, cy: Option<*mut u32>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVideoAspectRatio)(windows_core::Interface::as_raw(self), cx.unwrap_or(core::mem::zeroed()) as _, cy.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OnVideoStreamTick(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnVideoStreamTick)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetError: usize,
    SetErrorCode: usize,
    SetSourceElements: usize,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetworkState: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    GetPreload: usize,
    SetPreload: usize,
    GetBuffered: usize,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    CanPlayType: usize,
    pub GetReadyState: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub IsSeeking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetCurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetCurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub IsPaused: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDefaultPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetDefaultPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    GetPlayed: usize,
    GetSeekable: usize,
    pub IsEnded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLoop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetLoop: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMuted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetMuted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub HasVideo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub HasAudio: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetNativeVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetVideoAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    TransferVideoFrame: usize,
    pub OnVideoStreamTick: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IMFMediaEngine {}
windows_core::imp::define_interface!(IMFMediaEngineClassFactory, IMFMediaEngineClassFactory_Vtbl, 0x4d645ace_26aa_4688_9be1_df3516990b93);
windows_core::imp::interface_hierarchy!(IMFMediaEngineClassFactory, windows_core::IUnknown);
impl IMFMediaEngineClassFactory {
    pub unsafe fn CreateInstance<P1>(&self, dwflags: u32, pattr: P1) -> windows_core::Result<IMFMediaEngine>
    where
        P1: windows_core::Param<IMFAttributes>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), dwflags, pattr.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateTimeRange: usize,
    CreateError: usize,
}
impl windows_core::RuntimeName for IMFMediaEngineClassFactory {}
windows_core::imp::define_interface!(IMFMediaEngineEx, IMFMediaEngineEx_Vtbl, 0x83015ead_b1e6_40d0_a98a_37145ffe1ad1);
impl core::ops::Deref for IMFMediaEngineEx {
    type Target = IMFMediaEngine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaEngineEx, windows_core::IUnknown, IMFMediaEngine);
impl IMFMediaEngineEx {
    pub unsafe fn GetBalance(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetBalance)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetBalance(&self, balance: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetBalance)(windows_core::Interface::as_raw(self), balance).ok() }
    }
    pub unsafe fn IsPlaybackRateSupported(&self, rate: f64) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsPlaybackRateSupported)(windows_core::Interface::as_raw(self), rate) }
    }
    pub unsafe fn FrameStep(&self, forward: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).FrameStep)(windows_core::Interface::as_raw(self), forward.into()).ok() }
    }
    pub unsafe fn GetResourceCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNumberOfStreams(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfStreams)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamSelection(&self, dwstreamindex: u32, enabled: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, enabled.into()).ok() }
    }
    pub unsafe fn ApplyStreamSelections(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ApplyStreamSelections)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsProtected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsProtected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertVideoEffect<P0>(&self, peffect: P0, foptional: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertVideoEffect)(windows_core::Interface::as_raw(self), peffect.param().abi(), foptional.into()).ok() }
    }
    pub unsafe fn InsertAudioEffect<P0>(&self, peffect: P0, foptional: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAudioEffect)(windows_core::Interface::as_raw(self), peffect.param().abi(), foptional.into()).ok() }
    }
    pub unsafe fn RemoveAllEffects(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllEffects)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn SetTimelineMarkerTimer(&self, timetofire: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetTimelineMarkerTimer)(windows_core::Interface::as_raw(self), timetofire).ok() }
    }
    pub unsafe fn GetTimelineMarkerTimer(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimelineMarkerTimer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CancelTimelineMarkerTimer(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CancelTimelineMarkerTimer)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn IsStereo3D(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsStereo3D)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnableWindowlessSwapchainMode(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableWindowlessSwapchainMode)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
    pub unsafe fn GetVideoSwapchainHandle(&self) -> windows_core::Result<super::super::Foundation::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoSwapchainHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnableHorizontalMirrorMode(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableHorizontalMirrorMode)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
    pub unsafe fn GetAudioStreamCategory(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioStreamCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAudioStreamCategory(&self, category: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAudioStreamCategory)(windows_core::Interface::as_raw(self), category).ok() }
    }
    pub unsafe fn GetAudioEndpointRole(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioEndpointRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAudioEndpointRole(&self, role: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAudioEndpointRole)(windows_core::Interface::as_raw(self), role).ok() }
    }
    pub unsafe fn GetRealTimeMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRealTimeMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRealTimeMode(&self, fenable: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRealTimeMode)(windows_core::Interface::as_raw(self), fenable.into()).ok() }
    }
    pub unsafe fn EnableTimeUpdateTimer(&self, fenabletimer: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EnableTimeUpdateTimer)(windows_core::Interface::as_raw(self), fenabletimer.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineEx_Vtbl {
    pub base__: IMFMediaEngine_Vtbl,
    SetSourceFromByteStream: usize,
    GetStatistics: usize,
    UpdateVideoStream: usize,
    pub GetBalance: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetBalance: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsPlaybackRateSupported: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::BOOL,
    pub FrameStep: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetResourceCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetPresentationAttribute: usize,
    pub GetNumberOfStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetStreamAttribute: usize,
    pub GetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub ApplyStreamSelections: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub InsertVideoEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub InsertAudioEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RemoveAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimelineMarkerTimer: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetTimelineMarkerTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CancelTimelineMarkerTimer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsStereo3D: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    GetStereo3DFramePackingMode: usize,
    SetStereo3DFramePackingMode: usize,
    GetStereo3DRenderMode: usize,
    SetStereo3DRenderMode: usize,
    pub EnableWindowlessSwapchainMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetVideoSwapchainHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT,
    pub EnableHorizontalMirrorMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAudioStreamCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAudioStreamCategory: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAudioEndpointRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAudioEndpointRole: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetRealTimeMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetRealTimeMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    SetCurrentTimeEx: usize,
    pub EnableTimeUpdateTimer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
impl windows_core::RuntimeName for IMFMediaEngineEx {}
windows_core::imp::define_interface!(IMFMediaEngineNotify, IMFMediaEngineNotify_Vtbl, 0xfee7c112_e776_42b5_9bbf_0048524e2bd5);
windows_core::imp::interface_hierarchy!(IMFMediaEngineNotify, windows_core::IUnknown);
impl IMFMediaEngineNotify {
    pub unsafe fn EventNotify(&self, event: u32, param1: usize, param2: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).EventNotify)(windows_core::Interface::as_raw(self), event, param1, param2).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EventNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, u32) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineNotify_Impl: windows_core::IUnknownImpl {
    fn EventNotify(&self, event: u32, param1: usize, param2: u32) -> windows_core::Result<()>;
}
impl IMFMediaEngineNotify_Vtbl {
    pub const fn new<Identity: IMFMediaEngineNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventNotify<Identity: IMFMediaEngineNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: u32, param1: usize, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineNotify_Impl::EventNotify(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EventNotify: EventNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineNotify {}
pub const MFSTARTUP_FULL: u32 = 0;
pub const MF_MEDIA_ENGINE_CALLBACK: windows_core::GUID = windows_core::GUID::from_u128(0xc60381b8_83a4_41f8_a3d0_de05076849a9);
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
pub const MF_MEDIA_ENGINE_PLAYBACK_HWND: windows_core::GUID = windows_core::GUID::from_u128(0xd988879b_67c9_4d92_baa7_6eadd446039d);
pub const MF_VERSION: u32 = 131184;
