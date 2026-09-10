#[cfg(feature = "UI_Composition_SystemBackdrops")]
pub mod SystemBackdrops;
windows_core::imp::define_interface!(IAnimationObject, IAnimationObject_Vtbl, 0x8f56119d_b96d_58d0_9916_d1c5e390f890);
impl windows_core::RuntimeType for IAnimationObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.IAnimationObject");
}
windows_core::imp::interface_hierarchy!(IAnimationObject, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Microsoft.UI.Composition.IAnimationObject";
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionSupportsSystemBackdrop, ICompositionSupportsSystemBackdrop_Vtbl, 0x397dafe4_b6c2_5bb9_951d_f5707de8b7bc);
impl windows_core::RuntimeType for ICompositionSupportsSystemBackdrop {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.ICompositionSupportsSystemBackdrop");
}
windows_core::imp::interface_hierarchy!(ICompositionSupportsSystemBackdrop, windows_core::IUnknown, windows_core::IInspectable);
impl ICompositionSupportsSystemBackdrop {
    pub fn SystemBackdrop(&self) -> windows_core::Result<windows::UI::Composition::CompositionBrush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SystemBackdrop)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSystemBackdrop<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::UI::Composition::CompositionBrush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSystemBackdrop)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
}
impl windows_core::RuntimeName for ICompositionSupportsSystemBackdrop {
    const NAME: &'static str = "Microsoft.UI.Composition.ICompositionSupportsSystemBackdrop";
}
pub trait ICompositionSupportsSystemBackdrop_Impl: windows_core::IUnknownImpl {
    fn SystemBackdrop(&self) -> windows_core::Result<windows::UI::Composition::CompositionBrush>;
    fn SetSystemBackdrop(&self, value: windows_core::Ref<windows::UI::Composition::CompositionBrush>) -> windows_core::Result<()>;
}
impl ICompositionSupportsSystemBackdrop_Vtbl {
    pub const fn new<Identity: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SystemBackdrop<Identity: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICompositionSupportsSystemBackdrop_Impl::SystemBackdrop(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSystemBackdrop<Identity: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICompositionSupportsSystemBackdrop_Impl::SetSystemBackdrop(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionSupportsSystemBackdrop, OFFSET>(),
            SystemBackdrop: SystemBackdrop::<Identity, OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionSupportsSystemBackdrop as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionSupportsSystemBackdrop_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SystemBackdrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSystemBackdrop: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IVisualElement, IVisualElement_Vtbl, 0x2180f1f5_b5d8_4bf6_920a_12006e63efef);
impl windows_core::RuntimeType for IVisualElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.IVisualElement");
}
windows_core::imp::interface_hierarchy!(IVisualElement, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IVisualElement {
    const NAME: &'static str = "Microsoft.UI.Composition.IVisualElement";
}
pub trait IVisualElement_Impl: windows_core::IUnknownImpl {}
impl IVisualElement_Vtbl {
    pub const fn new<Identity: IVisualElement_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IVisualElement, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualElement as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IVisualElement2, IVisualElement2_Vtbl, 0xbc950c8d_1db0_53aa_9dee_34271cd18ce6);
impl windows_core::RuntimeType for IVisualElement2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Composition.IVisualElement2");
}
windows_core::imp::interface_hierarchy!(IVisualElement2, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IVisualElement2 {
    const NAME: &'static str = "Microsoft.UI.Composition.IVisualElement2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualElement2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
