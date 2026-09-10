windows_core::imp::define_interface!(IMediaPlaybackSource, IMediaPlaybackSource_Vtbl, 0xef9dc2bc_9317_4696_b051_2bad643177b5);
impl windows_core::RuntimeType for IMediaPlaybackSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlaybackSource");
}
windows_core::imp::interface_hierarchy!(IMediaPlaybackSource, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IMediaPlaybackSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlaybackSource";
}
pub trait IMediaPlaybackSource_Impl: windows_core::IUnknownImpl {}
impl IMediaPlaybackSource_Vtbl {
    pub const fn new<Identity: IMediaPlaybackSource_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaPlaybackSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaPlaybackSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlaybackSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaPlayer, IMediaPlayer_Vtbl, 0x381a83cb_6fff_499b_8d64_2885dfc1249e);
impl windows_core::RuntimeType for IMediaPlayer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer");
}
impl windows_core::RuntimeName for IMediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub NaturalDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::TimeSpan) -> windows_core::HRESULT,
    pub BufferingProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    CurrentState: usize,
    pub CanSeek: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanPause: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLoopingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsLoopingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMuted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsMuted: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub PlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Volume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    PlaybackMediaMarkers: usize,
    pub MediaOpened: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMediaOpened: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub MediaEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMediaEnded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub MediaFailed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMediaFailed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CurrentStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCurrentStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PlaybackMediaMarkerReached: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePlaybackMediaMarkerReached: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub MediaPlayerRateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMediaPlayerRateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub VolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVolumeChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SeekCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSeekCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub BufferingStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveBufferingStarted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub BufferingEnded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveBufferingEnded: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUriSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayer2, IMediaPlayer2_Vtbl, 0x3c841218_2123_4fc5_9082_2f883f77bdf5);
impl windows_core::RuntimeType for IMediaPlayer2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer2");
}
impl windows_core::RuntimeName for IMediaPlayer2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaPlayer3, IMediaPlayer3_Vtbl, 0xee0660da_031b_4feb_bd9b_92e0a0a8d299);
impl windows_core::RuntimeType for IMediaPlayer3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer3");
}
impl windows_core::RuntimeName for IMediaPlayer3 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer3";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsMutedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveIsMutedChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SourceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSourceChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub AudioBalance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetAudioBalance: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub RealTimePlayback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetRealTimePlayback: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    StereoscopicVideoRenderMode: usize,
    SetStereoscopicVideoRenderMode: usize,
    BreakManager: usize,
    CommandManager: usize,
    AudioDevice: usize,
    SetAudioDevice: usize,
    TimelineController: usize,
    SetTimelineController: usize,
    pub TimelineControllerPositionOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
    pub SetTimelineControllerPositionOffset: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::TimeSpan) -> windows_core::HRESULT,
    PlaybackSession: usize,
    pub StepForwardOneFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub StepBackwardOneFrame: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayer4, IMediaPlayer4_Vtbl, 0x80035db0_7448_4770_afcf_2a57450914c5);
impl windows_core::RuntimeType for IMediaPlayer4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer4");
}
impl windows_core::RuntimeName for IMediaPlayer4 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer4";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetSurfaceSize: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::Size) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayer5, IMediaPlayer5_Vtbl, 0xcfe537fd_f86a_4446_bf4d_c8e792b7b4b3);
impl windows_core::RuntimeType for IMediaPlayer5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer5");
}
impl windows_core::RuntimeName for IMediaPlayer5 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer5";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub VideoFrameAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVideoFrameAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub IsVideoFrameServerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsVideoFrameServerEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayer6, IMediaPlayer6_Vtbl, 0xe0caa086_ae65_414c_b010_8bc55f00e692);
impl windows_core::RuntimeType for IMediaPlayer6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer6");
}
impl windows_core::RuntimeName for IMediaPlayer6 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer6";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SubtitleFrameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSubtitleFrameChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayer7, IMediaPlayer7_Vtbl, 0x5d1dc478_4500_4531_b3f4_777a71491f7f);
impl windows_core::RuntimeType for IMediaPlayer7 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayer7");
}
impl windows_core::RuntimeName for IMediaPlayer7 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayer7";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayer7_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaPlayerEffects, IMediaPlayerEffects_Vtbl, 0x85a1deda_cab6_4cc0_8be3_6035f4de2591);
impl windows_core::RuntimeType for IMediaPlayerEffects {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayerEffects");
}
impl windows_core::RuntimeName for IMediaPlayerEffects {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerEffects";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    AddAudioEffect: usize,
    pub RemoveAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayerEffects2, IMediaPlayerEffects2_Vtbl, 0xfa419a79_1bbe_46c5_ae1f_8ee69fb3c2c7);
impl windows_core::RuntimeType for IMediaPlayerEffects2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayerEffects2");
}
impl windows_core::RuntimeName for IMediaPlayerEffects2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerEffects2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerEffects2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMediaPlayerFailedEventArgs, IMediaPlayerFailedEventArgs_Vtbl, 0x2744e9b9_a7e3_4f16_bac4_7914ebc08301);
impl windows_core::RuntimeType for IMediaPlayerFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayerFailedEventArgs");
}
impl windows_core::RuntimeName for IMediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerFailedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerFailedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Error: usize,
    pub ExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub ErrorMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayerRateChangedEventArgs, IMediaPlayerRateChangedEventArgs_Vtbl, 0x40600d58_3b61_4bb2_989f_fc65608b6cab);
impl windows_core::RuntimeType for IMediaPlayerRateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayerRateChangedEventArgs");
}
impl windows_core::RuntimeName for IMediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerRateChangedEventArgs";
}
pub trait IMediaPlayerRateChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn NewRate(&self) -> windows_core::Result<f64>;
}
impl IMediaPlayerRateChangedEventArgs_Vtbl {
    pub const fn new<Identity: IMediaPlayerRateChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NewRate<Identity: IMediaPlayerRateChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPlayerRateChangedEventArgs_Impl::NewRate(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaPlayerRateChangedEventArgs, OFFSET>(), NewRate: NewRate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaPlayerRateChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerRateChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMediaPlayerSource, IMediaPlayerSource_Vtbl, 0xbd4f8897_1423_4c3e_82c5_0fb1af94f715);
impl windows_core::RuntimeType for IMediaPlayerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayerSource");
}
#[cfg(feature = "Storage_Streams")]
impl windows_core::RuntimeName for IMediaPlayerSource {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSource";
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ProtectionManager: usize,
    SetProtectionManager: usize,
    SetFileSource: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStreamSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStreamSource: usize,
}
windows_core::imp::define_interface!(IMediaPlayerSource2, IMediaPlayerSource2_Vtbl, 0x82449b9f_7322_4c0b_b03b_3e69a48260c5);
impl windows_core::RuntimeType for IMediaPlayerSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IMediaPlayerSource2");
}
impl windows_core::RuntimeName for IMediaPlayerSource2 {
    const NAME: &'static str = "Windows.Media.Playback.IMediaPlayerSource2";
}
pub trait IMediaPlayerSource2_Impl: windows_core::IUnknownImpl {
    fn Source(&self) -> windows_core::Result<IMediaPlaybackSource>;
    fn SetSource(&self, value: windows_core::Ref<IMediaPlaybackSource>) -> windows_core::Result<()>;
}
impl IMediaPlayerSource2_Vtbl {
    pub const fn new<Identity: IMediaPlayerSource2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Source<Identity: IMediaPlayerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMediaPlayerSource2_Impl::Source(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSource<Identity: IMediaPlayerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMediaPlayerSource2_Impl::SetSource(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMediaPlayerSource2, OFFSET>(),
            Source: Source::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMediaPlayerSource2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMediaPlayerSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPlaybackMediaMarkerReachedEventArgs, IPlaybackMediaMarkerReachedEventArgs_Vtbl, 0x578cd1b9_90e2_4e60_abc4_8740b01f6196);
impl windows_core::RuntimeType for IPlaybackMediaMarkerReachedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Media.Playback.IPlaybackMediaMarkerReachedEventArgs");
}
impl windows_core::RuntimeName for IPlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.IPlaybackMediaMarkerReachedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlaybackMediaMarkerReachedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPlayer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayer, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(MediaPlayer, super::super::Foundation::IClosable);
impl MediaPlayer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<MediaPlayer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn AutoPlay(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoPlay)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoPlay(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoPlay)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn NaturalDuration(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NaturalDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPosition(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn BufferingProgress(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferingProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CanSeek(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanSeek)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CanPause(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPause)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsLoopingEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLoopingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsLoopingEnabled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsLoopingEnabled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsProtected(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsProtected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsMuted(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMuted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsMuted(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsMuted)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn PlaybackRate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PlaybackRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPlaybackRate(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPlaybackRate)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Volume(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Volume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetVolume(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn MediaOpened<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).MediaOpened)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveMediaOpened))
        }
    }
    pub fn MediaEnded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).MediaEnded)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveMediaEnded))
        }
    }
    pub fn MediaFailed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<MediaPlayerFailedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, MediaPlayerFailedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).MediaFailed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveMediaFailed))
        }
    }
    pub fn CurrentStateChanged<F>(&self, value: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let value = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| value(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CurrentStateChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&value), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCurrentStateChanged))
        }
    }
    pub fn PlaybackMediaMarkerReached<F>(&self, value: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PlaybackMediaMarkerReachedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let value = <super::super::Foundation::TypedEventHandler<Self, PlaybackMediaMarkerReachedEventArgs>>::new(move |a0, a1| value(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PlaybackMediaMarkerReached)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&value), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePlaybackMediaMarkerReached))
        }
    }
    pub fn MediaPlayerRateChanged<F>(&self, value: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<MediaPlayerRateChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let value = <super::super::Foundation::TypedEventHandler<Self, MediaPlayerRateChangedEventArgs>>::new(move |a0, a1| value(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).MediaPlayerRateChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&value), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveMediaPlayerRateChanged))
        }
    }
    pub fn VolumeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).VolumeChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveVolumeChanged))
        }
    }
    pub fn SeekCompleted<F>(&self, value: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let value = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| value(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SeekCompleted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&value), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveSeekCompleted))
        }
    }
    pub fn BufferingStarted<F>(&self, value: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let value = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| value(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BufferingStarted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&value), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveBufferingStarted))
        }
    }
    pub fn BufferingEnded<F>(&self, value: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let value = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| value(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).BufferingEnded)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&value), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveBufferingEnded))
        }
    }
    pub fn Play(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn Pause(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn SetUriSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUriSource)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn IsMutedChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).IsMutedChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveIsMutedChanged))
        }
    }
    pub fn SourceChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).SourceChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveSourceChanged))
        }
    }
    pub fn AudioBalance(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AudioBalance)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAudioBalance(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAudioBalance)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RealTimePlayback(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RealTimePlayback)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRealTimePlayback(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRealTimePlayback)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TimelineControllerPositionOffset(&self) -> windows_core::Result<windows_time::TimeSpan> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TimelineControllerPositionOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTimelineControllerPositionOffset(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTimelineControllerPositionOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StepForwardOneFrame(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StepForwardOneFrame)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn StepBackwardOneFrame(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer3>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StepBackwardOneFrame)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetSurfaceSize(&self, size: super::super::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSurfaceSize)(windows_core::Interface::as_raw(this), size).ok() }
    }
    pub fn VideoFrameAvailable<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).VideoFrameAvailable)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveVideoFrameAvailable))
        }
    }
    pub fn IsVideoFrameServerEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsVideoFrameServerEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsVideoFrameServerEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayer5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsVideoFrameServerEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SubtitleFrameChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayer6>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).SubtitleFrameChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveSubtitleFrameChanged))
        }
    }
    pub fn RemoveAllEffects(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IMediaPlayerEffects>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveAllEffects)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStreamSource<P0>(&self, stream: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStreamSource)(windows_core::Interface::as_raw(this), stream.param().abi()).ok() }
    }
    pub fn Source(&self) -> windows_core::Result<IMediaPlaybackSource> {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Source)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IMediaPlaybackSource>,
    {
        let this = &windows_core::Interface::cast::<IMediaPlayerSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeType for MediaPlayer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayer>();
}
unsafe impl windows_core::Interface for MediaPlayer {
    type Vtable = <IMediaPlayer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaPlayer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayer {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayer";
}
unsafe impl Send for MediaPlayer {}
unsafe impl Sync for MediaPlayer {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPlayerFailedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayerFailedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlayerFailedEventArgs {
    pub fn ExtendedErrorCode(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedErrorCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ErrorMessage(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ErrorMessage)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeType for MediaPlayerFailedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayerFailedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlayerFailedEventArgs {
    type Vtable = <IMediaPlayerFailedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaPlayerFailedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayerFailedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerFailedEventArgs";
}
unsafe impl Send for MediaPlayerFailedEventArgs {}
unsafe impl Sync for MediaPlayerFailedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MediaPlayerRateChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MediaPlayerRateChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl MediaPlayerRateChangedEventArgs {
    pub fn NewRate(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for MediaPlayerRateChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMediaPlayerRateChangedEventArgs>();
}
unsafe impl windows_core::Interface for MediaPlayerRateChangedEventArgs {
    type Vtable = <IMediaPlayerRateChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMediaPlayerRateChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MediaPlayerRateChangedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.MediaPlayerRateChangedEventArgs";
}
unsafe impl Send for MediaPlayerRateChangedEventArgs {}
unsafe impl Sync for MediaPlayerRateChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaybackMediaMarkerReachedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PlaybackMediaMarkerReachedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for PlaybackMediaMarkerReachedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPlaybackMediaMarkerReachedEventArgs>();
}
unsafe impl windows_core::Interface for PlaybackMediaMarkerReachedEventArgs {
    type Vtable = <IPlaybackMediaMarkerReachedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPlaybackMediaMarkerReachedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PlaybackMediaMarkerReachedEventArgs {
    const NAME: &'static str = "Windows.Media.Playback.PlaybackMediaMarkerReachedEventArgs";
}
unsafe impl Send for PlaybackMediaMarkerReachedEventArgs {}
unsafe impl Sync for PlaybackMediaMarkerReachedEventArgs {}
