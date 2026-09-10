windows_core::imp::define_interface!(IActivatedEventArgs, IActivatedEventArgs_Vtbl, 0xcf651713_cd08_4fd8_b697_a281b6544e2e);
impl windows_core::RuntimeType for IActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Activation.IActivatedEventArgs");
}
windows_core::imp::interface_hierarchy!(IActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IActivatedEventArgsWithUser, IActivatedEventArgsWithUser_Vtbl, 0x1cf09b9e_9962_4936_80ff_afc8e8ae5c8c);
impl windows_core::RuntimeType for IActivatedEventArgsWithUser {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser");
}
windows_core::imp::interface_hierarchy!(IActivatedEventArgsWithUser, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IActivatedEventArgsWithUser, IActivatedEventArgs);
impl windows_core::RuntimeName for IActivatedEventArgsWithUser {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IActivatedEventArgsWithUser";
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsWithUser_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IApplicationViewActivatedEventArgs, IApplicationViewActivatedEventArgs_Vtbl, 0x930cef4b_b829_40fc_88f4_8513e8a64738);
impl windows_core::RuntimeType for IApplicationViewActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs");
}
windows_core::imp::interface_hierarchy!(IApplicationViewActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IApplicationViewActivatedEventArgs, IActivatedEventArgs);
impl IApplicationViewActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IApplicationViewActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IApplicationViewActivatedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IApplicationViewActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentlyShownApplicationViewId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILaunchActivatedEventArgs, ILaunchActivatedEventArgs_Vtbl, 0xfbc93e26_a14a_4b4f_82b0_33bed920af52);
impl windows_core::RuntimeType for ILaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs");
}
windows_core::imp::interface_hierarchy!(ILaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILaunchActivatedEventArgs, IActivatedEventArgs);
impl ILaunchActivatedEventArgs {
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TileId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for ILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Arguments: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ILaunchActivatedEventArgs2, ILaunchActivatedEventArgs2_Vtbl, 0x0fd37ebc_9dc9_46b5_9ace_bd95d4565345);
impl windows_core::RuntimeType for ILaunchActivatedEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2");
}
windows_core::imp::interface_hierarchy!(ILaunchActivatedEventArgs2, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ILaunchActivatedEventArgs2, IActivatedEventArgs, ILaunchActivatedEventArgs);
impl ILaunchActivatedEventArgs2 {
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Arguments)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TileId)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for ILaunchActivatedEventArgs2 {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.ILaunchActivatedEventArgs2";
}
#[repr(C)]
#[doc(hidden)]
pub struct ILaunchActivatedEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IPrelaunchActivatedEventArgs, IPrelaunchActivatedEventArgs_Vtbl, 0x0c44717b_19f7_48d6_b046_cf22826eaa74);
impl windows_core::RuntimeType for IPrelaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs");
}
windows_core::imp::interface_hierarchy!(IPrelaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IPrelaunchActivatedEventArgs, IActivatedEventArgs);
impl IPrelaunchActivatedEventArgs {
    pub fn PrelaunchActivated(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PrelaunchActivated)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPrelaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IPrelaunchActivatedEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrelaunchActivatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrelaunchActivated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IViewSwitcherProvider, IViewSwitcherProvider_Vtbl, 0x33f288a6_5c2c_4d27_bac7_7536088f1219);
impl windows_core::RuntimeType for IViewSwitcherProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.Activation.IViewSwitcherProvider");
}
windows_core::imp::interface_hierarchy!(IViewSwitcherProvider, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IViewSwitcherProvider, IActivatedEventArgs);
impl windows_core::RuntimeName for IViewSwitcherProvider {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.IViewSwitcherProvider";
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewSwitcherProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchActivatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(LaunchActivatedEventArgs, windows_core::IUnknown, windows_core::IInspectable, ILaunchActivatedEventArgs);
windows_core::imp::required_hierarchy!(LaunchActivatedEventArgs, IActivatedEventArgs, IActivatedEventArgsWithUser, IApplicationViewActivatedEventArgs, ILaunchActivatedEventArgs2, IPrelaunchActivatedEventArgs, IViewSwitcherProvider);
impl LaunchActivatedEventArgs {
    pub fn CurrentlyShownApplicationViewId(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CurrentlyShownApplicationViewId)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Arguments(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Arguments)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn TileId(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TileId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn PrelaunchActivated(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrelaunchActivated)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for LaunchActivatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ILaunchActivatedEventArgs>();
}
unsafe impl windows_core::Interface for LaunchActivatedEventArgs {
    type Vtable = <ILaunchActivatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ILaunchActivatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for LaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.ApplicationModel.Activation.LaunchActivatedEventArgs";
}
unsafe impl Send for LaunchActivatedEventArgs {}
unsafe impl Sync for LaunchActivatedEventArgs {}
