windows_core::imp::define_interface!(IMediaCue, IMediaCue_Vtbl, 0xc7d15e5d_59dc_431f_a0ee_27744323b36d);
impl windows_core::RuntimeType for IMediaCue {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaCue");
}
windows_core::imp::interface_hierarchy!(IMediaCue, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaCue {
    pub fn SetStartTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStartTime)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn StartTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDuration(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Duration(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetId)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IMediaCue {
    const NAME: &'static str = "Windows.Media.Core.IMediaCue";
}
pub trait IMediaCue_Impl: windows_core::IUnknownImpl {
    fn SetStartTime(&self, value: &windows_time::TimeSpan) -> windows_core::Result<()>;
    fn StartTime(&self) -> windows_core::Result<windows_time::TimeSpan>;
    fn SetDuration(&self, value: &windows_time::TimeSpan) -> windows_core::Result<()>;
    fn Duration(&self) -> windows_core::Result<windows_time::TimeSpan>;
    fn SetId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IMediaCue_Vtbl {
    pub const fn new<Identity: IMediaCue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStartTime<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_time::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaCue_Impl::SetStartTime(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn StartTime<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_time::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaCue_Impl::StartTime(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDuration<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_time::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaCue_Impl::SetDuration(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Duration<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_time::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaCue_Impl::Duration(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetId<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaCue_Impl::SetId(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IMediaCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaCue_Impl::Id(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaCue, OFFSET>(),
            SetStartTime: SetStartTime::<Identity, OFFSET>,
            StartTime: StartTime::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            Duration: Duration::<Identity, OFFSET>,
            SetId: SetId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaCue as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCue_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::TimeSpan) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::TimeSpan) -> windows_core::HRESULT,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
    pub SetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaCueEventArgs, IMediaCueEventArgs_Vtbl, 0xd12f47f7_5fa4_4e68_9fe5_32160dcee57e);
impl windows_core::RuntimeType for IMediaCueEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaCueEventArgs");
}
impl windows_core::RuntimeName for IMediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaCueEventArgs";
}
pub trait IMediaCueEventArgs_Impl: windows_core::IUnknownImpl {
    fn Cue(&self) -> windows_core::Result<IMediaCue>;
}
impl IMediaCueEventArgs_Vtbl {
    pub const fn new<Identity: IMediaCueEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cue<Identity: IMediaCueEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaCueEventArgs_Impl::Cue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaCueEventArgs, OFFSET>(), Cue: Cue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaCueEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaCueEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Media_Playback")]
windows_core::imp::define_interface!(IMediaSource2, IMediaSource2_Vtbl, 0x2eb61048_655f_4c37_b813_b4e45dfa0abe);
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeType for IMediaSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSource2");
}
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeName for IMediaSource2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource2";
}
#[cfg(feature = "Media_Playback")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenOperationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveOpenOperationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    CustomProperties: usize,
    pub Duration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ExternalTimedTextSources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExternalTimedMetadataTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Media_Playback")]
windows_core::imp::define_interface!(IMediaSource3, IMediaSource3_Vtbl, 0xb59f0d9b_4b6e_41ed_bbb4_7c7509a994ad);
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeType for IMediaSource3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSource3");
}
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeName for IMediaSource3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource3";
}
#[cfg(feature = "Media_Playback")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub StateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    State: usize,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Media_Playback")]
windows_core::imp::define_interface!(IMediaSource4, IMediaSource4_Vtbl, 0xbdafad57_8eff_4c63_85a6_84de0ae3e4f2);
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeType for IMediaSource4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSource4");
}
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeName for IMediaSource4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource4";
}
#[cfg(feature = "Media_Playback")]
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AdaptiveMediaSource: usize,
    MediaStreamSource: usize,
    MseStreamSource: usize,
    pub Uri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaSource5, IMediaSource5_Vtbl, 0x331a22ae_ed2e_4a22_94c8_b743a92b3022);
impl windows_core::RuntimeType for IMediaSource5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSource5");
}
impl windows_core::RuntimeName for IMediaSource5 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSource5";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSource5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaSourceOpenOperationCompletedEventArgs, IMediaSourceOpenOperationCompletedEventArgs_Vtbl, 0xfc682ceb_e281_477c_a8e0_1acd654114c8);
impl windows_core::RuntimeType for IMediaSourceOpenOperationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSourceOpenOperationCompletedEventArgs");
}
impl windows_core::RuntimeName for IMediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceOpenOperationCompletedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceOpenOperationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaSourceStateChangedEventArgs, IMediaSourceStateChangedEventArgs_Vtbl, 0x0a30af82_9071_4bac_bc39_ca2a93b717a9);
impl windows_core::RuntimeType for IMediaSourceStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSourceStateChangedEventArgs");
}
impl windows_core::RuntimeName for IMediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStateChangedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaSourceStatics, IMediaSourceStatics_Vtbl, 0xf77d6fa4_4652_410e_b1d8_e9a5e245a45c);
impl windows_core::RuntimeType for IMediaSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSourceStatics");
}
#[cfg(all(feature = "Media_Playback", feature = "Storage_Streams"))]
impl windows_core::RuntimeName for IMediaSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    CreateFromAdaptiveMediaSource: usize,
    CreateFromMediaStreamSource: usize,
    CreateFromMseStreamSource: usize,
    CreateFromIMediaSource: usize,
    CreateFromStorageFile: usize,
    #[cfg(all(feature = "Media_Playback", feature = "Storage_Streams"))]
    pub CreateFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Playback", feature = "Storage_Streams")))]
    CreateFromStream: usize,
    #[cfg(all(feature = "Media_Playback", feature = "Storage_Streams"))]
    pub CreateFromStreamReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Media_Playback", feature = "Storage_Streams")))]
    CreateFromStreamReference: usize,
    #[cfg(feature = "Media_Playback")]
    pub CreateFromUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Media_Playback"))]
    CreateFromUri: usize,
}
windows_core::imp::define_interface!(IMediaSourceStatics2, IMediaSourceStatics2_Vtbl, 0xeee161a4_7f13_4896_b8cb_df0de5bcb9f1);
impl windows_core::RuntimeType for IMediaSourceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSourceStatics2");
}
impl windows_core::RuntimeName for IMediaSourceStatics2 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaSourceStatics3, IMediaSourceStatics3_Vtbl, 0x453a30d6_2bea_4122_9f73_eace04526e35);
impl windows_core::RuntimeType for IMediaSourceStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSourceStatics3");
}
impl windows_core::RuntimeName for IMediaSourceStatics3 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics3";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaSourceStatics4, IMediaSourceStatics4_Vtbl, 0x281b3bfc_e50a_4428_a500_9c4ed918d3f0);
impl windows_core::RuntimeType for IMediaSourceStatics4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaSourceStatics4");
}
impl windows_core::RuntimeName for IMediaSourceStatics4 {
    const NAME: &'static str = "Windows.Media.Core.IMediaSourceStatics4";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaSourceStatics4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaTrack, IMediaTrack_Vtbl, 0x03e1fafc_c931_491a_b46b_c10ee8c256b7);
impl windows_core::RuntimeType for IMediaTrack {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.IMediaTrack");
}
windows_core::imp::interface_hierarchy!(IMediaTrack, windows_core::IUnknown, windows_core::IInspectable);
impl IMediaTrack {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Language)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Label)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IMediaTrack {
    const NAME: &'static str = "Windows.Media.Core.IMediaTrack";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaTrack_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Language: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    TrackKind: usize,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Label: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimedMetadataTrack, ITimedMetadataTrack_Vtbl, 0x9e6aed9e_f67a_49a9_b330_cf03b0e9cf07);
impl windows_core::RuntimeType for ITimedMetadataTrack {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedMetadataTrack");
}
impl windows_core::RuntimeName for ITimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrack";
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrack_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CueEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCueEntered: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CueExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCueExited: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TrackFailed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTrackFailed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Cues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActiveCues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    TimedMetadataKind: usize,
    pub DispatchType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddCue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveCue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimedMetadataTrack2, ITimedMetadataTrack2_Vtbl, 0x21b4b648_9f9d_40ba_a8f3_1a92753aef0b);
impl windows_core::RuntimeType for ITimedMetadataTrack2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedMetadataTrack2");
}
impl windows_core::RuntimeName for ITimedMetadataTrack2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrack2";
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrack2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    PlaybackItem: usize,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimedMetadataTrackFactory, ITimedMetadataTrackFactory_Vtbl, 0x8dd57611_97b3_4e1f_852c_0f482c81ad26);
impl windows_core::RuntimeType for ITimedMetadataTrackFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedMetadataTrackFactory");
}
impl windows_core::RuntimeName for ITimedMetadataTrackFactory {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackFactory";
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ITimedMetadataTrackFailedEventArgs, ITimedMetadataTrackFailedEventArgs_Vtbl, 0xa57fc9d1_6789_4d4d_b07f_84b4f31acb70);
impl windows_core::RuntimeType for ITimedMetadataTrackFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedMetadataTrackFailedEventArgs");
}
impl windows_core::RuntimeName for ITimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ITimedMetadataTrackFailedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedMetadataTrackFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ITimedTextSource, ITimedTextSource_Vtbl, 0xc4ed9ba6_101f_404d_a949_82f33fcd93b7);
impl windows_core::RuntimeType for ITimedTextSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedTextSource");
}
impl windows_core::RuntimeName for ITimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSource";
}
pub trait ITimedTextSource_Impl: windows_core::IUnknownImpl {
    fn Resolved(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<TimedTextSource, TimedTextSourceResolveResultEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveResolved(&self, token: i64) -> windows_core::Result<()>;
}
impl ITimedTextSource_Vtbl {
    pub const fn new<Identity: ITimedTextSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolved<Identity: ITimedTextSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSource_Impl::Resolved(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveResolved<Identity: ITimedTextSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimedTextSource_Impl::RemoveResolved(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITimedTextSource, OFFSET>(),
            Resolved: Resolved::<Identity, OFFSET>,
            RemoveResolved: RemoveResolved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimedTextSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Resolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveResolved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimedTextSourceResolveResultEventArgs, ITimedTextSourceResolveResultEventArgs_Vtbl, 0x48907c9c_dcd8_4c33_9ad3_6cdce7b1c566);
impl windows_core::RuntimeType for ITimedTextSourceResolveResultEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedTextSourceResolveResultEventArgs");
}
impl windows_core::RuntimeName for ITimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceResolveResultEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceResolveResultEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Error: usize,
    pub Tracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimedTextSourceStatics, ITimedTextSourceStatics_Vtbl, 0x7e311853_9aba_4ac4_bb98_2fb176c3bfdd);
impl windows_core::RuntimeType for ITimedTextSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedTextSourceStatics");
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITimedTextSourceStatics {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceStatics";
}
#[cfg(feature = "Storage_Streams")]
pub trait ITimedTextSourceStatics_Impl: windows_core::IUnknownImpl {
    fn CreateFromStream(&self, stream: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<TimedTextSource>;
    fn CreateFromUri(&self, uri: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<TimedTextSource>;
    fn CreateFromStreamWithLanguage(&self, stream: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>, defaultLanguage: &windows_core::HSTRING) -> windows_core::Result<TimedTextSource>;
    fn CreateFromUriWithLanguage(&self, uri: windows_core::Ref<super::super::Foundation::Uri>, defaultLanguage: &windows_core::HSTRING) -> windows_core::Result<TimedTextSource>;
}
#[cfg(feature = "Storage_Streams")]
impl ITimedTextSourceStatics_Vtbl {
    pub const fn new<Identity: ITimedTextSourceStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromStream<Identity: ITimedTextSourceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics_Impl::CreateFromStream(this, core::mem::transmute_copy(&stream)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromUri<Identity: ITimedTextSourceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics_Impl::CreateFromUri(this, core::mem::transmute_copy(&uri)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromStreamWithLanguage<Identity: ITimedTextSourceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, defaultlanguage: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics_Impl::CreateFromStreamWithLanguage(this, core::mem::transmute_copy(&stream), core::mem::transmute(&defaultlanguage)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromUriWithLanguage<Identity: ITimedTextSourceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, defaultlanguage: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics_Impl::CreateFromUriWithLanguage(this, core::mem::transmute_copy(&uri), core::mem::transmute(&defaultlanguage)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITimedTextSourceStatics, OFFSET>(),
            CreateFromStream: CreateFromStream::<Identity, OFFSET>,
            CreateFromUri: CreateFromUri::<Identity, OFFSET>,
            CreateFromStreamWithLanguage: CreateFromStreamWithLanguage::<Identity, OFFSET>,
            CreateFromUriWithLanguage: CreateFromUriWithLanguage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimedTextSourceStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStream: usize,
    pub CreateFromUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithLanguage: usize,
    pub CreateFromUriWithLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITimedTextSourceStatics2, ITimedTextSourceStatics2_Vtbl, 0xb66b7602_923e_43fa_9633_587075812db5);
impl windows_core::RuntimeType for ITimedTextSourceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Core.ITimedTextSourceStatics2");
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for ITimedTextSourceStatics2 {
    const NAME: &'static str = "Windows.Media.Core.ITimedTextSourceStatics2";
}
#[cfg(feature = "Storage_Streams")]
pub trait ITimedTextSourceStatics2_Impl: windows_core::IUnknownImpl {
    fn CreateFromStreamWithIndex(&self, stream: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>, indexStream: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>) -> windows_core::Result<TimedTextSource>;
    fn CreateFromUriWithIndex(&self, uri: windows_core::Ref<super::super::Foundation::Uri>, indexUri: windows_core::Ref<super::super::Foundation::Uri>) -> windows_core::Result<TimedTextSource>;
    fn CreateFromStreamWithIndexAndLanguage(&self, stream: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>, indexStream: windows_core::Ref<super::super::Storage::Streams::IRandomAccessStream>, defaultLanguage: &windows_core::HSTRING) -> windows_core::Result<TimedTextSource>;
    fn CreateFromUriWithIndexAndLanguage(&self, uri: windows_core::Ref<super::super::Foundation::Uri>, indexUri: windows_core::Ref<super::super::Foundation::Uri>, defaultLanguage: &windows_core::HSTRING) -> windows_core::Result<TimedTextSource>;
}
#[cfg(feature = "Storage_Streams")]
impl ITimedTextSourceStatics2_Vtbl {
    pub const fn new<Identity: ITimedTextSourceStatics2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromStreamWithIndex<Identity: ITimedTextSourceStatics2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, indexstream: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics2_Impl::CreateFromStreamWithIndex(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&indexstream)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromUriWithIndex<Identity: ITimedTextSourceStatics2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, indexuri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics2_Impl::CreateFromUriWithIndex(this, core::mem::transmute_copy(&uri), core::mem::transmute_copy(&indexuri)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromStreamWithIndexAndLanguage<Identity: ITimedTextSourceStatics2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void, indexstream: *mut core::ffi::c_void, defaultlanguage: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics2_Impl::CreateFromStreamWithIndexAndLanguage(this, core::mem::transmute_copy(&stream), core::mem::transmute_copy(&indexstream), core::mem::transmute(&defaultlanguage)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromUriWithIndexAndLanguage<Identity: ITimedTextSourceStatics2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, indexuri: *mut core::ffi::c_void, defaultlanguage: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITimedTextSourceStatics2_Impl::CreateFromUriWithIndexAndLanguage(this, core::mem::transmute_copy(&uri), core::mem::transmute_copy(&indexuri), core::mem::transmute(&defaultlanguage)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITimedTextSourceStatics2, OFFSET>(),
            CreateFromStreamWithIndex: CreateFromStreamWithIndex::<Identity, OFFSET>,
            CreateFromUriWithIndex: CreateFromUriWithIndex::<Identity, OFFSET>,
            CreateFromStreamWithIndexAndLanguage: CreateFromStreamWithIndexAndLanguage::<Identity, OFFSET>,
            CreateFromUriWithIndexAndLanguage: CreateFromUriWithIndexAndLanguage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimedTextSourceStatics2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimedTextSourceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndex: usize,
    pub CreateFromUriWithIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateFromStreamWithIndexAndLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateFromStreamWithIndexAndLanguage: usize,
    pub CreateFromUriWithIndexAndLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaCueEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaCueEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaCueEventArgs {
    pub fn Cue(&self) -> windows_core::Result<IMediaCue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaCueEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaCueEventArgs>();
}
unsafe impl windows_core::Interface for MediaCueEventArgs {
    type Vtable = <IMediaCueEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaCueEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaCueEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaCueEventArgs";
}
unsafe impl Send for MediaCueEventArgs {}
unsafe impl Sync for MediaCueEventArgs {}
#[cfg(feature = "Media_Playback")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaSource(windows_core::IUnknown);
#[cfg(feature = "Media_Playback")]
windows_core::imp::interface_hierarchy!(MediaSource, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Media_Playback")]
windows_core::imp::required_hierarchy!(MediaSource, super::super::Foundation::IClosable, super::Playback::IMediaPlaybackSource);
#[cfg(feature = "Media_Playback")]
impl MediaSource {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OpenOperationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<MediaSourceOpenOperationCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, MediaSourceOpenOperationCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).OpenOperationCompleted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveOpenOperationCompleted))
        }
    }
    pub fn Duration(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Duration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)).and_then(|r__: windows_reference::IReference<windows_time::TimeSpan>| r__.Value())
        }
    }
    pub fn IsOpen(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOpen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ExternalTimedTextSources(&self) -> windows_core::Result<windows_collections::IObservableVector<TimedTextSource>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExternalTimedTextSources)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ExternalTimedMetadataTracks(&self) -> windows_core::Result<windows_collections::IObservableVector<TimedMetadataTrack>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExternalTimedMetadataTracks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StateChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<MediaSourceStateChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IMediaSource3>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, MediaSourceStateChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).StateChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveStateChanged))
        }
    }
    pub fn Reset(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaSource3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Reset)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Uri(&self) -> windows_core::Result<super::super::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Uri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OpenAsync(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IMediaSource4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<P0>(stream: P0, contenttype: &windows_core::HSTRING) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStream)(windows_core::Interface::as_raw(this), stream.param().abi(), core::mem::transmute_copy(contenttype), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamReference<P0>(stream: P0, contenttype: &windows_core::HSTRING) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStreamReference>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStreamReference)(windows_core::Interface::as_raw(this), stream.param().abi(), core::mem::transmute_copy(contenttype), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUri<P0>(uri: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IMediaSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUri)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IMediaSourceStatics<R, F: FnOnce(&IMediaSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IMediaSourceStatics2<R, F: FnOnce(&IMediaSourceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IMediaSourceStatics3<R, F: FnOnce(&IMediaSourceStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IMediaSourceStatics4<R, F: FnOnce(&IMediaSourceStatics4) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaSource, IMediaSourceStatics4> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeType for MediaSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaSource2>();
}
#[cfg(feature = "Media_Playback")]
unsafe impl windows_core::Interface for MediaSource {
    type Vtable = <IMediaSource2 as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaSource2 as windows_core::Interface>::IID;
}
#[cfg(feature = "Media_Playback")]
impl windows_core::RuntimeName for MediaSource {
    const NAME: &'static str = "Windows.Media.Core.MediaSource";
}
#[cfg(feature = "Media_Playback")]
unsafe impl Send for MediaSource {}
#[cfg(feature = "Media_Playback")]
unsafe impl Sync for MediaSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaSourceOpenOperationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaSourceOpenOperationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for MediaSourceOpenOperationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaSourceOpenOperationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for MediaSourceOpenOperationCompletedEventArgs {
    type Vtable = <IMediaSourceOpenOperationCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaSourceOpenOperationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaSourceOpenOperationCompletedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceOpenOperationCompletedEventArgs";
}
unsafe impl Send for MediaSourceOpenOperationCompletedEventArgs {}
unsafe impl Sync for MediaSourceOpenOperationCompletedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaSourceStateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaSourceStateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for MediaSourceStateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaSourceStateChangedEventArgs>();
}
unsafe impl windows_core::Interface for MediaSourceStateChangedEventArgs {
    type Vtable = <IMediaSourceStateChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaSourceStateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaSourceStateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.MediaSourceStateChangedEventArgs";
}
unsafe impl Send for MediaSourceStateChangedEventArgs {}
unsafe impl Sync for MediaSourceStateChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimedMetadataTrack(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TimedMetadataTrack, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(TimedMetadataTrack, IMediaTrack);
impl TimedMetadataTrack {
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLabel(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLabel)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Label(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IMediaTrack>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Label)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn CueEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<MediaCueEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, MediaCueEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CueEntered)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCueEntered))
        }
    }
    pub fn CueExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<MediaCueEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, MediaCueEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CueExited)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCueExited))
        }
    }
    pub fn TrackFailed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<TimedMetadataTrackFailedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, TimedMetadataTrackFailedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).TrackFailed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveTrackFailed))
        }
    }
    pub fn Cues(&self) -> windows_core::Result<windows_collections::IVectorView<IMediaCue>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cues)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ActiveCues(&self) -> windows_core::Result<windows_collections::IVectorView<IMediaCue>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActiveCues)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DispatchType(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatchType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn AddCue<P0>(&self, cue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMediaCue>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddCue)(windows_core::Interface::as_raw(self), cue.param().abi()).ok() }
    }
    pub fn RemoveCue<P0>(&self, cue: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMediaCue>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveCue)(windows_core::Interface::as_raw(self), cue.param().abi()).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ITimedMetadataTrack2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn ITimedMetadataTrackFactory<R, F: FnOnce(&ITimedMetadataTrackFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TimedMetadataTrack, ITimedMetadataTrackFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TimedMetadataTrack {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITimedMetadataTrack>();
}
unsafe impl windows_core::Interface for TimedMetadataTrack {
    type Vtable = <ITimedMetadataTrack as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITimedMetadataTrack as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TimedMetadataTrack {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrack";
}
unsafe impl Send for TimedMetadataTrack {}
unsafe impl Sync for TimedMetadataTrack {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimedMetadataTrackFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TimedMetadataTrackFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for TimedMetadataTrackFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITimedMetadataTrackFailedEventArgs>();
}
unsafe impl windows_core::Interface for TimedMetadataTrackFailedEventArgs {
    type Vtable = <ITimedMetadataTrackFailedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITimedMetadataTrackFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TimedMetadataTrackFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedMetadataTrackFailedEventArgs";
}
unsafe impl Send for TimedMetadataTrackFailedEventArgs {}
unsafe impl Sync for TimedMetadataTrackFailedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimedTextSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TimedTextSource, windows_core::IUnknown, windows_core::IInspectable);
impl TimedTextSource {
    pub fn Resolved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<TimedTextSourceResolveResultEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, TimedTextSourceResolveResultEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Resolved)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveResolved))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStream<P0>(stream: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStream)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUri<P0>(uri: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUri)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithLanguage<P0>(stream: P0, defaultlanguage: &windows_core::HSTRING) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStreamWithLanguage)(windows_core::Interface::as_raw(this), stream.param().abi(), core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUriWithLanguage<P0>(uri: P0, defaultlanguage: &windows_core::HSTRING) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUriWithLanguage)(windows_core::Interface::as_raw(this), uri.param().abi(), core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndex<P0, P1>(stream: P0, indexstream: P1) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStreamWithIndex)(windows_core::Interface::as_raw(this), stream.param().abi(), indexstream.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUriWithIndex<P0, P1>(uri: P0, indexuri: P1) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUriWithIndex)(windows_core::Interface::as_raw(this), uri.param().abi(), indexuri.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateFromStreamWithIndexAndLanguage<P0, P1>(stream: P0, indexstream: P1, defaultlanguage: &windows_core::HSTRING) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
        P1: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStreamWithIndexAndLanguage)(windows_core::Interface::as_raw(this), stream.param().abi(), indexstream.param().abi(), core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUriWithIndexAndLanguage<P0, P1>(uri: P0, indexuri: P1, defaultlanguage: &windows_core::HSTRING) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
        P1: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::ITimedTextSourceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUriWithIndexAndLanguage)(windows_core::Interface::as_raw(this), uri.param().abi(), indexuri.param().abi(), core::mem::transmute_copy(defaultlanguage), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn ITimedTextSourceStatics<R, F: FnOnce(&ITimedTextSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TimedTextSource, ITimedTextSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITimedTextSourceStatics2<R, F: FnOnce(&ITimedTextSourceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TimedTextSource, ITimedTextSourceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TimedTextSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITimedTextSource>();
}
unsafe impl windows_core::Interface for TimedTextSource {
    type Vtable = <ITimedTextSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITimedTextSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TimedTextSource {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSource";
}
unsafe impl Send for TimedTextSource {}
unsafe impl Sync for TimedTextSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimedTextSourceResolveResultEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TimedTextSourceResolveResultEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TimedTextSourceResolveResultEventArgs {
    pub fn Tracks(&self) -> windows_core::Result<windows_collections::IVectorView<TimedMetadataTrack>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Tracks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for TimedTextSourceResolveResultEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITimedTextSourceResolveResultEventArgs>();
}
unsafe impl windows_core::Interface for TimedTextSourceResolveResultEventArgs {
    type Vtable = <ITimedTextSourceResolveResultEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITimedTextSourceResolveResultEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TimedTextSourceResolveResultEventArgs {
    const NAME: &'static str = "Windows.Media.Core.TimedTextSourceResolveResultEventArgs";
}
unsafe impl Send for TimedTextSourceResolveResultEventArgs {}
unsafe impl Sync for TimedTextSourceResolveResultEventArgs {}
