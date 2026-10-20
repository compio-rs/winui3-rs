windows_core::imp::define_interface!(IPath, IPath_Vtbl, 0x757d1cd8_0ec0_55c5_b400_66657e493e78);
impl windows_core::RuntimeType for IPath {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Shapes.IPath");
}
#[cfg(feature = "UI_Xaml_Media")]
impl windows_core::RuntimeName for IPath {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.IPath";
}
#[cfg(feature = "UI_Xaml_Media")]
pub trait IPath_Impl: windows_core::IUnknownImpl {
    fn Data(&self) -> windows_core::Result<super::Media::Geometry>;
    fn SetData(&self, value: windows_core::Ref<super::Media::Geometry>) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Media")]
impl IPath_Vtbl {
    pub const fn new<Identity: IPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Data<Identity: IPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPath_Impl::Data(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetData<Identity: IPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPath_Impl::SetData(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPath, OFFSET>(), Data: Data::<Identity, OFFSET>, SetData: SetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPath as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPath_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Data: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetData: usize,
}
windows_core::imp::define_interface!(IPathFactory, IPathFactory_Vtbl, 0x5e82e4c9_7502_5b1f_b940_c3346a71362a);
impl windows_core::RuntimeType for IPathFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Shapes.IPathFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IPathFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.IPathFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait IPathFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<Path>;
}
#[cfg(feature = "UI_Composition")]
impl IPathFactory_Vtbl {
    pub const fn new<Identity: IPathFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IPathFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPathFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPathFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPathFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IPathStatics, IPathStatics_Vtbl, 0x2146d36f_721c_5b54_af7d_60f3adc4fbca);
impl windows_core::RuntimeType for IPathStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Shapes.IPathStatics");
}
impl windows_core::RuntimeName for IPathStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.IPathStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IPathStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IShape, IShape_Vtbl, 0x9941aad3_6af2_5ba2_9085_8506d5f2485e);
impl windows_core::RuntimeType for IShape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Shapes.IShape");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
impl windows_core::RuntimeName for IShape {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.IShape";
}
#[repr(C)]
#[doc(hidden)]
pub struct IShape_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub Fill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    Fill: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub SetFill: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    SetFill: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub Stroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    Stroke: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub SetStroke: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    SetStroke: usize,
    pub StrokeMiterLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub StrokeThickness: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    StrokeStartLineCap: usize,
    SetStrokeStartLineCap: usize,
    StrokeEndLineCap: usize,
    SetStrokeEndLineCap: usize,
    StrokeLineJoin: usize,
    SetStrokeLineJoin: usize,
    pub StrokeDashOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    StrokeDashCap: usize,
    SetStrokeDashCap: usize,
    StrokeDashArray: usize,
    SetStrokeDashArray: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub Stretch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Media::Stretch) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    Stretch: usize,
    #[cfg(feature = "UI_Xaml_Media")]
    pub SetStretch: unsafe extern "system" fn(*mut core::ffi::c_void, super::Media::Stretch) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))]
    SetStretch: usize,
}
windows_core::imp::define_interface!(IShapeFactory, IShapeFactory_Vtbl, 0x4fecafaf_8265_5252_ba5c_f43639f974a5);
impl windows_core::RuntimeType for IShapeFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Shapes.IShapeFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IShapeFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.IShapeFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait IShapeFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<Shape>;
}
#[cfg(feature = "UI_Composition")]
impl IShapeFactory_Vtbl {
    pub const fn new<Identity: IShapeFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IShapeFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IShapeFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IShapeFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IShapeFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IShapeFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IShapeStatics, IShapeStatics_Vtbl, 0xea407c43_8a09_587a_958a_4dd17d217ce1);
impl windows_core::RuntimeType for IShapeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Shapes.IShapeStatics");
}
impl windows_core::RuntimeName for IShapeStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.IShapeStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IShapeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Path(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(Path, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(Path, super::super::Composition::IAnimationObject, super::super::Composition::IVisualElement, super::super::Composition::IVisualElement2, Shape, super::FrameworkElement, super::UIElement, super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl Path {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::Style> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DataContextChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDataContextChanged))
        }
    }
    pub fn LayoutUpdated<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Data(&self) -> windows_core::Result<super::Media::Geometry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetData<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Geometry>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IPathFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IPathFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> windows_core::Result<super::Media::Brush> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Fill)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFill)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stroke(&self) -> windows_core::Result<super::Media::Brush> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stroke)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStroke<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStroke)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn StrokeMiterLimit(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeMiterLimit)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrokeMiterLimit(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStrokeMiterLimit)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StrokeThickness(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrokeThickness(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStrokeThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn StrokeDashOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StrokeDashOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrokeDashOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStrokeDashOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stretch(&self) -> windows_core::Result<super::Media::Stretch> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Stretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStretch(&self, value: super::Media::Stretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IShape>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Clip(&self) -> windows_core::Result<super::Media::RectangleGeometry> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clip)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetClip<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::RectangleGeometry>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetClip)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::Visibility> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::FocusState> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn IPathFactory<R, F: FnOnce(&IPathFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Path, IPathFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IPathStatics<R, F: FnOnce(&IPathStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Path, IPathStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for Path {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPath>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for Path {
    type Vtable = <IPath as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPath as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for Path {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.Path";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for Path {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for Path {}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Shape(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(Shape, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(Shape, super::super::Composition::IAnimationObject, super::super::Composition::IVisualElement, super::super::Composition::IVisualElement2, super::FrameworkElement, super::UIElement, super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl Shape {
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::Style> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DataContextChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDataContextChanged))
        }
    }
    pub fn LayoutUpdated<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::FrameworkElement>, windows_core::Ref<super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::FrameworkElement, super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Fill(&self) -> windows_core::Result<super::Media::Brush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Fill)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFill<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFill)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stroke(&self) -> windows_core::Result<super::Media::Brush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stroke)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStroke<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStroke)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn StrokeMiterLimit(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrokeMiterLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrokeMiterLimit(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeMiterLimit)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn StrokeThickness(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrokeThickness)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrokeThickness(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeThickness)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn StrokeDashOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StrokeDashOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetStrokeDashOffset(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStrokeDashOffset)(windows_core::Interface::as_raw(self), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Stretch(&self) -> windows_core::Result<super::Media::Stretch> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Stretch)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetStretch(&self, value: super::Media::Stretch) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetStretch)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IShapeFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IShapeFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Clip(&self) -> windows_core::Result<super::Media::RectangleGeometry> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Clip)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetClip<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::RectangleGeometry>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetClip)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::Visibility> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::FocusState> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::UIElement>, windows_core::Ref<super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::UIElement, super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn IShapeFactory<R, F: FnOnce(&IShapeFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Shape, IShapeFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IShapeStatics<R, F: FnOnce(&IShapeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Shape, IShapeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for Shape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IShape>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for Shape {
    type Vtable = <IShape as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IShape as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for Shape {
    const NAME: &'static str = "Microsoft.UI.Xaml.Shapes.Shape";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for Shape {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for Shape {}
