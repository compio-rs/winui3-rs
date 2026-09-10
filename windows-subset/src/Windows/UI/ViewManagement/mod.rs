windows_core::imp::define_interface!(IUISettings, IUISettings_Vtbl, 0x85361600_1c63_4627_bcb1_3a89e0bc9c55);
impl windows_core::RuntimeType for IUISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettings");
}
impl windows_core::RuntimeName for IUISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings";
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    HandPreference: usize,
    pub CursorSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub ScrollBarSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub ScrollBarArrowSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub ScrollBarThumbBoxSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::Size) -> windows_core::HRESULT,
    pub MessageDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AnimationsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CaretBrowsingEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CaretBlinkRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CaretWidth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub DoubleClickTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MouseHoverTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings2, IUISettings2_Vtbl, 0xbad82401_2721_44f9_bb91_2bb228be442f);
impl windows_core::RuntimeType for IUISettings2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettings2");
}
impl windows_core::RuntimeName for IUISettings2 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings2";
}
pub trait IUISettings2_Impl: windows_core::IUnknownImpl {
    fn TextScaleFactor(&self) -> windows_core::Result<f64>;
    fn TextScaleFactorChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<UISettings, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveTextScaleFactorChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IUISettings2_Vtbl {
    pub const fn new<Identity: IUISettings2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TextScaleFactor<Identity: IUISettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings2_Impl::TextScaleFactor(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TextScaleFactorChanged<Identity: IUISettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings2_Impl::TextScaleFactorChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveTextScaleFactorChanged<Identity: IUISettings2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUISettings2_Impl::RemoveTextScaleFactorChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettings2, OFFSET>(),
            TextScaleFactor: TextScaleFactor::<Identity, OFFSET>,
            TextScaleFactorChanged: TextScaleFactorChanged::<Identity, OFFSET>,
            RemoveTextScaleFactorChanged: RemoveTextScaleFactorChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettings2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TextScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub TextScaleFactorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTextScaleFactorChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings3, IUISettings3_Vtbl, 0x03021be4_5254_4781_8194_5168f7d06d7b);
impl windows_core::RuntimeType for IUISettings3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettings3");
}
impl windows_core::RuntimeName for IUISettings3 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings3";
}
pub trait IUISettings3_Impl: windows_core::IUnknownImpl {
    fn GetColorValue(&self, desiredColor: UIColorType) -> windows_core::Result<super::Color>;
    fn ColorValuesChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<UISettings, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveColorValuesChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IUISettings3_Vtbl {
    pub const fn new<Identity: IUISettings3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetColorValue<Identity: IUISettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, desiredcolor: UIColorType, result__: *mut super::Color) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings3_Impl::GetColorValue(this, desiredcolor) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ColorValuesChanged<Identity: IUISettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings3_Impl::ColorValuesChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveColorValuesChanged<Identity: IUISettings3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUISettings3_Impl::RemoveColorValuesChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettings3, OFFSET>(),
            GetColorValue: GetColorValue::<Identity, OFFSET>,
            ColorValuesChanged: ColorValuesChanged::<Identity, OFFSET>,
            RemoveColorValuesChanged: RemoveColorValuesChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettings3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetColorValue: unsafe extern "system" fn(*mut core::ffi::c_void, UIColorType, *mut super::Color) -> windows_core::HRESULT,
    pub ColorValuesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveColorValuesChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings4, IUISettings4_Vtbl, 0x52bb3002_919b_4d6b_9b78_8dd66ff4b93b);
impl windows_core::RuntimeType for IUISettings4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettings4");
}
impl windows_core::RuntimeName for IUISettings4 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings4";
}
pub trait IUISettings4_Impl: windows_core::IUnknownImpl {
    fn AdvancedEffectsEnabled(&self) -> windows_core::Result<bool>;
    fn AdvancedEffectsEnabledChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<UISettings, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveAdvancedEffectsEnabledChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IUISettings4_Vtbl {
    pub const fn new<Identity: IUISettings4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdvancedEffectsEnabled<Identity: IUISettings4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings4_Impl::AdvancedEffectsEnabled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AdvancedEffectsEnabledChanged<Identity: IUISettings4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings4_Impl::AdvancedEffectsEnabledChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveAdvancedEffectsEnabledChanged<Identity: IUISettings4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUISettings4_Impl::RemoveAdvancedEffectsEnabledChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettings4, OFFSET>(),
            AdvancedEffectsEnabled: AdvancedEffectsEnabled::<Identity, OFFSET>,
            AdvancedEffectsEnabledChanged: AdvancedEffectsEnabledChanged::<Identity, OFFSET>,
            RemoveAdvancedEffectsEnabledChanged: RemoveAdvancedEffectsEnabledChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettings4 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AdvancedEffectsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AdvancedEffectsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAdvancedEffectsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings5, IUISettings5_Vtbl, 0x5349d588_0cb5_5f05_bd34_706b3231f0bd);
impl windows_core::RuntimeType for IUISettings5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettings5");
}
impl windows_core::RuntimeName for IUISettings5 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings5";
}
pub trait IUISettings5_Impl: windows_core::IUnknownImpl {
    fn AutoHideScrollBars(&self) -> windows_core::Result<bool>;
    fn AutoHideScrollBarsChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAutoHideScrollBarsChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveAutoHideScrollBarsChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IUISettings5_Vtbl {
    pub const fn new<Identity: IUISettings5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AutoHideScrollBars<Identity: IUISettings5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings5_Impl::AutoHideScrollBars(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AutoHideScrollBarsChanged<Identity: IUISettings5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings5_Impl::AutoHideScrollBarsChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveAutoHideScrollBarsChanged<Identity: IUISettings5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUISettings5_Impl::RemoveAutoHideScrollBarsChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettings5, OFFSET>(),
            AutoHideScrollBars: AutoHideScrollBars::<Identity, OFFSET>,
            AutoHideScrollBarsChanged: AutoHideScrollBarsChanged::<Identity, OFFSET>,
            RemoveAutoHideScrollBarsChanged: RemoveAutoHideScrollBarsChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettings5 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoHideScrollBars: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AutoHideScrollBarsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAutoHideScrollBarsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettings6, IUISettings6_Vtbl, 0xaef19bd7_fe31_5a04_ada4_469aaec6dfa9);
impl windows_core::RuntimeType for IUISettings6 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettings6");
}
impl windows_core::RuntimeName for IUISettings6 {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings6";
}
pub trait IUISettings6_Impl: windows_core::IUnknownImpl {
    fn AnimationsEnabledChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<UISettings, UISettingsAnimationsEnabledChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveAnimationsEnabledChanged(&self, token: i64) -> windows_core::Result<()>;
    fn MessageDurationChanged(&self, handler: windows_core::Ref<super::super::Foundation::TypedEventHandler<UISettings, UISettingsMessageDurationChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveMessageDurationChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IUISettings6_Vtbl {
    pub const fn new<Identity: IUISettings6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AnimationsEnabledChanged<Identity: IUISettings6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings6_Impl::AnimationsEnabledChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveAnimationsEnabledChanged<Identity: IUISettings6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUISettings6_Impl::RemoveAnimationsEnabledChanged(this, token).into()
            }
        }
        unsafe extern "system" fn MessageDurationChanged<Identity: IUISettings6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUISettings6_Impl::MessageDurationChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveMessageDurationChanged<Identity: IUISettings6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUISettings6_Impl::RemoveMessageDurationChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettings6, OFFSET>(),
            AnimationsEnabledChanged: AnimationsEnabledChanged::<Identity, OFFSET>,
            RemoveAnimationsEnabledChanged: RemoveAnimationsEnabledChanged::<Identity, OFFSET>,
            MessageDurationChanged: MessageDurationChanged::<Identity, OFFSET>,
            RemoveMessageDurationChanged: RemoveMessageDurationChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettings6 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettings6_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AnimationsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveAnimationsEnabledChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub MessageDurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveMessageDurationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettingsAnimationsEnabledChangedEventArgs, IUISettingsAnimationsEnabledChangedEventArgs_Vtbl, 0x0c7b4b3d_2ea1_533e_894d_415bc5243c29);
impl windows_core::RuntimeType for IUISettingsAnimationsEnabledChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs");
}
impl windows_core::RuntimeName for IUISettingsAnimationsEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsAnimationsEnabledChangedEventArgs";
}
pub trait IUISettingsAnimationsEnabledChangedEventArgs_Impl: windows_core::IUnknownImpl {}
impl IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
    pub const fn new<Identity: IUISettingsAnimationsEnabledChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettingsAnimationsEnabledChangedEventArgs, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsAnimationsEnabledChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IUISettingsAutoHideScrollBarsChangedEventArgs, IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl, 0x87afd4b2_9146_5f02_8f6b_06d454174c0f);
impl windows_core::RuntimeType for IUISettingsAutoHideScrollBarsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs");
}
impl windows_core::RuntimeName for IUISettingsAutoHideScrollBarsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsAutoHideScrollBarsChangedEventArgs";
}
pub trait IUISettingsAutoHideScrollBarsChangedEventArgs_Impl: windows_core::IUnknownImpl {}
impl IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
    pub const fn new<Identity: IUISettingsAutoHideScrollBarsChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettingsAutoHideScrollBarsChangedEventArgs, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsAutoHideScrollBarsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IUISettingsMessageDurationChangedEventArgs, IUISettingsMessageDurationChangedEventArgs_Vtbl, 0x338aad52_4a5d_5b59_8002_d930f608fd6e);
impl windows_core::RuntimeType for IUISettingsMessageDurationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs");
}
impl windows_core::RuntimeName for IUISettingsMessageDurationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.IUISettingsMessageDurationChangedEventArgs";
}
pub trait IUISettingsMessageDurationChangedEventArgs_Impl: windows_core::IUnknownImpl {}
impl IUISettingsMessageDurationChangedEventArgs_Vtbl {
    pub const fn new<Identity: IUISettingsMessageDurationChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IUISettingsMessageDurationChangedEventArgs, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsMessageDurationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIColorType(pub i32);
impl UIColorType {
    pub const Background: Self = Self(0);
    pub const Foreground: Self = Self(1);
    pub const AccentDark3: Self = Self(2);
    pub const AccentDark2: Self = Self(3);
    pub const AccentDark1: Self = Self(4);
    pub const Accent: Self = Self(5);
    pub const AccentLight1: Self = Self(6);
    pub const AccentLight2: Self = Self(7);
    pub const AccentLight3: Self = Self(8);
    pub const Complement: Self = Self(9);
}
impl windows_core::imp::TypeKind for UIColorType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for UIColorType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.UIColorType;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.ViewManagement.UIColorType");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettings, windows_core::IUnknown, windows_core::IInspectable);
impl UISettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UISettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn CursorSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CursorSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ScrollBarSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScrollBarSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ScrollBarArrowSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScrollBarArrowSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ScrollBarThumbBoxSize(&self) -> windows_core::Result<super::super::Foundation::Size> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ScrollBarThumbBoxSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MessageDuration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MessageDuration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn AnimationsEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnimationsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CaretBrowsingEnabled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaretBrowsingEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CaretBlinkRate(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaretBlinkRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn CaretWidth(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CaretWidth)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn DoubleClickTime(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoubleClickTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MouseHoverTime(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MouseHoverTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TextScaleFactor(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IUISettings2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TextScaleFactor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TextScaleFactorChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUISettings2>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).TextScaleFactorChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTextScaleFactorChanged))
        }
    }
    pub fn GetColorValue(&self, desiredcolor: UIColorType) -> windows_core::Result<super::Color> {
        let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetColorValue)(windows_core::Interface::as_raw(this), desiredcolor, &mut result__).map(|| result__)
        }
    }
    pub fn ColorValuesChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ColorValuesChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveColorValuesChanged))
        }
    }
    pub fn AdvancedEffectsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUISettings4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AdvancedEffectsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AdvancedEffectsEnabledChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUISettings4>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AdvancedEffectsEnabledChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAdvancedEffectsEnabledChanged))
        }
    }
    pub fn AutoHideScrollBars(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IUISettings5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AutoHideScrollBars)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AutoHideScrollBarsChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<UISettingsAutoHideScrollBarsChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUISettings5>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, UISettingsAutoHideScrollBarsChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AutoHideScrollBarsChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAutoHideScrollBarsChanged))
        }
    }
    pub fn AnimationsEnabledChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<UISettingsAnimationsEnabledChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, UISettingsAnimationsEnabledChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AnimationsEnabledChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAnimationsEnabledChanged))
        }
    }
    pub fn MessageDurationChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<UISettingsMessageDurationChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IUISettings6>(self)?;
        let handler = <super::super::Foundation::TypedEventHandler<Self, UISettingsMessageDurationChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).MessageDurationChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveMessageDurationChanged))
        }
    }
}
impl windows_core::RuntimeType for UISettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettings>();
}
unsafe impl windows_core::Interface for UISettings {
    type Vtable = <IUISettings as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUISettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettings {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettings";
}
unsafe impl Send for UISettings {}
unsafe impl Sync for UISettings {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettingsAnimationsEnabledChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsAnimationsEnabledChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for UISettingsAnimationsEnabledChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsAnimationsEnabledChangedEventArgs>();
}
unsafe impl windows_core::Interface for UISettingsAnimationsEnabledChangedEventArgs {
    type Vtable = <IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUISettingsAnimationsEnabledChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsAnimationsEnabledChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsAnimationsEnabledChangedEventArgs";
}
unsafe impl Send for UISettingsAnimationsEnabledChangedEventArgs {}
unsafe impl Sync for UISettingsAnimationsEnabledChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettingsAutoHideScrollBarsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsAutoHideScrollBarsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for UISettingsAutoHideScrollBarsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsAutoHideScrollBarsChangedEventArgs>();
}
unsafe impl windows_core::Interface for UISettingsAutoHideScrollBarsChangedEventArgs {
    type Vtable = <IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUISettingsAutoHideScrollBarsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsAutoHideScrollBarsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsAutoHideScrollBarsChangedEventArgs";
}
unsafe impl Send for UISettingsAutoHideScrollBarsChangedEventArgs {}
unsafe impl Sync for UISettingsAutoHideScrollBarsChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettingsMessageDurationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsMessageDurationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for UISettingsMessageDurationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsMessageDurationChangedEventArgs>();
}
unsafe impl windows_core::Interface for UISettingsMessageDurationChangedEventArgs {
    type Vtable = <IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUISettingsMessageDurationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsMessageDurationChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.UISettingsMessageDurationChangedEventArgs";
}
unsafe impl Send for UISettingsMessageDurationChangedEventArgs {}
unsafe impl Sync for UISettingsMessageDurationChangedEventArgs {}
