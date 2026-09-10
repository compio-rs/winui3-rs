#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionBrush(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositionBrush, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CompositionBrush, IAnimationObject, super::super::Foundation::IClosable, CompositionObject);
impl CompositionBrush {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for CompositionBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositionBrush>();
}
unsafe impl windows_core::Interface for CompositionBrush {
    type Vtable = <ICompositionBrush as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionBrush as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionBrush {
    const NAME: &'static str = "Windows.UI.Composition.CompositionBrush";
}
unsafe impl Send for CompositionBrush {}
unsafe impl Sync for CompositionBrush {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositionObject, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CompositionObject, IAnimationObject, super::super::Foundation::IClosable);
impl CompositionObject {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::Core::CoreDispatcher> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Dispatcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).StopAnimation)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    fn ICompositionObjectStatics<R, F: FnOnce(&ICompositionObjectStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CompositionObject, ICompositionObjectStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CompositionObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositionObject>();
}
unsafe impl windows_core::Interface for CompositionObject {
    type Vtable = <ICompositionObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionObject {
    const NAME: &'static str = "Windows.UI.Composition.CompositionObject";
}
unsafe impl Send for CompositionObject {}
unsafe impl Sync for CompositionObject {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompositionTarget(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CompositionTarget, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(CompositionTarget, IAnimationObject, super::super::Foundation::IClosable, CompositionObject);
impl CompositionTarget {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> windows_core::Result<super::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<ICompositionObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StopAnimation(&self, propertyname: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompositionObject>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StopAnimation)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyname)).ok() }
    }
    pub fn Comment(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICompositionObject2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Comment)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetComment(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICompositionObject2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for CompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICompositionTarget>();
}
unsafe impl windows_core::Interface for CompositionTarget {
    type Vtable = <ICompositionTarget as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICompositionTarget as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CompositionTarget {
    const NAME: &'static str = "Windows.UI.Composition.CompositionTarget";
}
unsafe impl Send for CompositionTarget {}
unsafe impl Sync for CompositionTarget {}
windows_core::imp::define_interface!(IAnimationObject, IAnimationObject_Vtbl, 0xe7141e0a_04b8_4fc5_a4dc_195392e57807);
impl windows_core::RuntimeType for IAnimationObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.IAnimationObject");
}
windows_core::imp::interface_hierarchy!(IAnimationObject, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeName for IAnimationObject {
    const NAME: &'static str = "Windows.UI.Composition.IAnimationObject";
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnimationObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionBrush, ICompositionBrush_Vtbl, 0xab0d7608_30c0_40e9_b568_b60a6bd1fb46);
impl windows_core::RuntimeType for ICompositionBrush {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionBrush");
}
impl windows_core::RuntimeName for ICompositionBrush {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBrush";
}
pub trait ICompositionBrush_Impl: windows_core::IUnknownImpl {}
impl ICompositionBrush_Vtbl {
    pub const fn new<Identity: ICompositionBrush_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionBrush, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionBrush as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionBrush_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionBrushFactory, ICompositionBrushFactory_Vtbl, 0xda53fb4c_4650_47c4_ad76_765379607ed6);
impl windows_core::RuntimeType for ICompositionBrushFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionBrushFactory");
}
impl windows_core::RuntimeName for ICompositionBrushFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionBrushFactory";
}
pub trait ICompositionBrushFactory_Impl: windows_core::IUnknownImpl {}
impl ICompositionBrushFactory_Vtbl {
    pub const fn new<Identity: ICompositionBrushFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionBrushFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionBrushFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionBrushFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionObject, ICompositionObject_Vtbl, 0xbcb4ad45_7609_4550_934f_16002a68fded);
impl windows_core::RuntimeType for ICompositionObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionObject");
}
#[cfg(feature = "UI_Core")]
impl windows_core::RuntimeName for ICompositionObject {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Compositor: usize,
    #[cfg(feature = "UI_Core")]
    pub Dispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Core"))]
    Dispatcher: usize,
    Properties: usize,
    StartAnimation: usize,
    pub StopAnimation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionObject2, ICompositionObject2_Vtbl, 0xef874ea1_5cff_4b68_9e30_a1519d08ba03);
impl windows_core::RuntimeType for ICompositionObject2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionObject2");
}
impl windows_core::RuntimeName for ICompositionObject2 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject2";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObject2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Comment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetComment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICompositionObject3, ICompositionObject3_Vtbl, 0x4bc27925_dacd_4cf2_98b1_986b76e7ebe6);
impl windows_core::RuntimeType for ICompositionObject3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionObject3");
}
impl windows_core::RuntimeName for ICompositionObject3 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject3";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObject3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionObject4, ICompositionObject4_Vtbl, 0x0bb3784c_346b_4a7c_966b_7310966553d5);
impl windows_core::RuntimeType for ICompositionObject4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionObject4");
}
impl windows_core::RuntimeName for ICompositionObject4 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject4";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObject4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionObject5, ICompositionObject5_Vtbl, 0x1d7f391b_a130_5265_a62b_60b8e668965a);
impl windows_core::RuntimeType for ICompositionObject5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionObject5");
}
impl windows_core::RuntimeName for ICompositionObject5 {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObject5";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObject5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionObjectFactory, ICompositionObjectFactory_Vtbl, 0x51205c5e_558a_4f2a_8d39_37bfe1e20ddd);
impl windows_core::RuntimeType for ICompositionObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionObjectFactory");
}
impl windows_core::RuntimeName for ICompositionObjectFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObjectFactory";
}
pub trait ICompositionObjectFactory_Impl: windows_core::IUnknownImpl {}
impl ICompositionObjectFactory_Vtbl {
    pub const fn new<Identity: ICompositionObjectFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionObjectFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionObjectFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionObjectStatics, ICompositionObjectStatics_Vtbl, 0xc1ed052f_1ba2_44ba_a904_6a882a0a5adb);
impl windows_core::RuntimeType for ICompositionObjectStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionObjectStatics");
}
impl windows_core::RuntimeName for ICompositionObjectStatics {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionObjectStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionObjectStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionTarget, ICompositionTarget_Vtbl, 0xa1bea8ba_d726_4663_8129_6b5e7927ffa6);
impl windows_core::RuntimeType for ICompositionTarget {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionTarget");
}
impl windows_core::RuntimeName for ICompositionTarget {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTarget";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTarget_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ICompositionTargetFactory, ICompositionTargetFactory_Vtbl, 0x93cd9d2b_8516_4b14_a8ce_f49e2119ec42);
impl windows_core::RuntimeType for ICompositionTargetFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Composition.ICompositionTargetFactory");
}
impl windows_core::RuntimeName for ICompositionTargetFactory {
    const NAME: &'static str = "Windows.UI.Composition.ICompositionTargetFactory";
}
pub trait ICompositionTargetFactory_Impl: windows_core::IUnknownImpl {}
impl ICompositionTargetFactory_Vtbl {
    pub const fn new<Identity: ICompositionTargetFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ICompositionTargetFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICompositionTargetFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionTargetFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
