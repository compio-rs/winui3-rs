windows_core::imp::define_interface!(ITextHighlighter, ITextHighlighter_Vtbl, 0xb756e861_1d2b_5f6f_81fd_c51a5bc068ff);
impl windows_core::RuntimeType for ITextHighlighter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Documents.ITextHighlighter");
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
impl windows_core::RuntimeName for ITextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.ITextHighlighter";
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
pub trait ITextHighlighter_Impl: windows_core::IUnknownImpl {
    fn Ranges(&self) -> windows_core::Result<windows_collections::IVector<TextRange>>;
    fn Foreground(&self) -> windows_core::Result<super::Media::Brush>;
    fn SetForeground(&self, value: windows_core::Ref<super::Media::Brush>) -> windows_core::Result<()>;
    fn Background(&self) -> windows_core::Result<super::Media::Brush>;
    fn SetBackground(&self, value: windows_core::Ref<super::Media::Brush>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
impl ITextHighlighter_Vtbl {
    pub const fn new<Identity: ITextHighlighter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Ranges<Identity: ITextHighlighter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHighlighter_Impl::Ranges(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Foreground<Identity: ITextHighlighter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHighlighter_Impl::Foreground(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForeground<Identity: ITextHighlighter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHighlighter_Impl::SetForeground(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Background<Identity: ITextHighlighter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHighlighter_Impl::Background(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackground<Identity: ITextHighlighter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextHighlighter_Impl::SetBackground(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextHighlighter, OFFSET>(),
            Ranges: Ranges::<Identity, OFFSET>,
            Foreground: Foreground::<Identity, OFFSET>,
            SetForeground: SetForeground::<Identity, OFFSET>,
            Background: Background::<Identity, OFFSET>,
            SetBackground: SetBackground::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHighlighter as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Ranges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub Foreground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    Foreground: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub SetForeground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    SetForeground: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub Background: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    Background: usize,
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub SetBackground: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "UI_Xaml_Media")))]
    SetBackground: usize,
}
windows_core::imp::define_interface!(ITextHighlighterFactory, ITextHighlighterFactory_Vtbl, 0x69c7311f_c019_5b93_b511_81418543bab7);
impl windows_core::RuntimeType for ITextHighlighterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Documents.ITextHighlighterFactory");
}
impl windows_core::RuntimeName for ITextHighlighterFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.ITextHighlighterFactory";
}
pub trait ITextHighlighterFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<TextHighlighter>;
}
impl ITextHighlighterFactory_Vtbl {
    pub const fn new<Identity: ITextHighlighterFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ITextHighlighterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextHighlighterFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ITextHighlighterFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextHighlighterFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITextHighlighterStatics, ITextHighlighterStatics_Vtbl, 0x4975047a_87ad_51a2_977c_e771de4f4035);
impl windows_core::RuntimeType for ITextHighlighterStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Documents.ITextHighlighterStatics");
}
impl windows_core::RuntimeName for ITextHighlighterStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.ITextHighlighterStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextHighlighterStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextHighlighter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TextHighlighter, windows_core::IUnknown, windows_core::IInspectable);
impl TextHighlighter {
    pub fn Ranges(&self) -> windows_core::Result<windows_collections::IVector<TextRange>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Ranges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub fn Foreground(&self) -> windows_core::Result<super::Media::Brush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Foreground)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetForeground)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub fn Background(&self) -> windows_core::Result<super::Media::Brush> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Background)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "UI_Composition", feature = "UI_Xaml_Media"))]
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::Media::Brush>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackground)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::ITextHighlighterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::ITextHighlighterFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    fn ITextHighlighterFactory<R, F: FnOnce(&ITextHighlighterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextHighlighter, ITextHighlighterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ITextHighlighterStatics<R, F: FnOnce(&ITextHighlighterStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<TextHighlighter, ITextHighlighterStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for TextHighlighter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITextHighlighter>();
}
unsafe impl windows_core::Interface for TextHighlighter {
    type Vtable = <ITextHighlighter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITextHighlighter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TextHighlighter {
    const NAME: &'static str = "Microsoft.UI.Xaml.Documents.TextHighlighter";
}
unsafe impl Send for TextHighlighter {}
unsafe impl Sync for TextHighlighter {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TextRange {
    pub StartIndex: i32,
    pub Length: i32,
}
impl windows_core::imp::TypeKind for TextRange {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TextRange {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Xaml.Documents.TextRange;i4;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Documents.TextRange");
}
