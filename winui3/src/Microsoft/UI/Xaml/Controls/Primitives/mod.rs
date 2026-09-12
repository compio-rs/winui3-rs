#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ButtonBase(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(ButtonBase, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(ButtonBase, super::super::super::Composition::IAnimationObject, super::super::super::Composition::IVisualElement, super::super::super::Composition::IVisualElement2, super::ContentControl, super::Control, super::super::FrameworkElement, super::super::UIElement, super::super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl ButtonBase {
    pub fn IsPointerOver(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPointerOver)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPressed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Command(&self) -> windows_core::Result<super::super::Input::ICommand> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Command)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ICommand>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCommand)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn CommandParameter(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommandParameter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetCommandParameter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCommandParameter)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Click)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveClick))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IButtonBaseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IButtonBaseFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<super::super::UIElement> {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(&self, oldcontent: P0, newcontent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IContentControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnContentChanged)(windows_core::Interface::as_raw(this), oldcontent.param().abi(), newcontent.param().abi()).ok() }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngaged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> windows_core::Result<super::super::Media::FontFamily> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontFamily)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FontWeight(&self) -> windows_core::Result<windows::UI::Text::FontWeight> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> windows_core::Result<windows::UI::Text::FontStyle> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: windows::UI::Text::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStretch(&self) -> windows_core::Result<windows::UI::Text::FontStretch> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: windows::UI::Text::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForeground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalContentAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalContentAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<super::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(&self, value: super::BackgroundSizing) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundSizing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FocusEngaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusEngagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusEngagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusEngaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusEngaged))
        }
    }
    pub fn FocusDisengaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusDisengagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusDisengagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusDisengaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusDisengaged))
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEngagement)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerEntered)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerPressed)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerMoved)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerReleased)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerExited)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCaptureLost)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCanceled)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerWheelChanged)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDoubleTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnHolding)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnRightTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationDelta)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationCompleted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnGotFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnLostFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnCharacterReceived)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleKey)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetTemplateChild(&self, childname: &windows_core::HSTRING) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(childname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::super::Style> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
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
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::super::Visibility> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::super::FocusState> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn IButtonBaseFactory<R, F: FnOnce(&IButtonBaseFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ButtonBase, IButtonBaseFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IButtonBaseStatics<R, F: FnOnce(&IButtonBaseStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ButtonBase, IButtonBaseStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for ButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IButtonBase>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for ButtonBase {
    type Vtable = <IButtonBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IButtonBase as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ButtonBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ButtonBase";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for ButtonBase {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for ButtonBase {}
windows_core::imp::define_interface!(IButtonBase, IButtonBase_Vtbl, 0x65714269_2473_5327_a652_0ea6bce7f403);
impl windows_core::RuntimeType for IButtonBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IButtonBase");
}
#[cfg(feature = "UI_Xaml_Input")]
impl windows_core::RuntimeName for IButtonBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IButtonBase";
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    ClickMode: usize,
    SetClickMode: usize,
    pub IsPointerOver: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "UI_Xaml_Input")]
    pub Command: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    Command: usize,
    #[cfg(feature = "UI_Xaml_Input")]
    pub SetCommand: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Input"))]
    SetCommand: usize,
    pub CommandParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCommandParameter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Click: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveClick: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IButtonBaseFactory, IButtonBaseFactory_Vtbl, 0x21251aa9_6fd1_5e51_ab3b_e6fcaf3395ed);
impl windows_core::RuntimeType for IButtonBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IButtonBaseFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IButtonBaseFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IButtonBaseFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait IButtonBaseFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<ButtonBase>;
}
#[cfg(feature = "UI_Composition")]
impl IButtonBaseFactory_Vtbl {
    pub const fn new<Identity: IButtonBaseFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IButtonBaseFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IButtonBaseFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IButtonBaseFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IButtonBaseFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IButtonBaseStatics, IButtonBaseStatics_Vtbl, 0xdbe812f6_adf8_51d3_8137_a8fbf6445b3c);
impl windows_core::RuntimeType for IButtonBaseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IButtonBaseStatics");
}
impl windows_core::RuntimeName for IButtonBaseStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IButtonBaseStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IButtonBaseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IRangeBase, IRangeBase_Vtbl, 0x540d6d61_8fac_5d5c_b5b0_e172a7dde103);
impl windows_core::RuntimeType for IRangeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IRangeBase");
}
impl windows_core::RuntimeName for IRangeBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IRangeBase";
}
pub trait IRangeBase_Impl: windows_core::IUnknownImpl {
    fn Minimum(&self) -> windows_core::Result<f64>;
    fn SetMinimum(&self, value: f64) -> windows_core::Result<()>;
    fn Maximum(&self) -> windows_core::Result<f64>;
    fn SetMaximum(&self, value: f64) -> windows_core::Result<()>;
    fn SmallChange(&self) -> windows_core::Result<f64>;
    fn SetSmallChange(&self, value: f64) -> windows_core::Result<()>;
    fn LargeChange(&self) -> windows_core::Result<f64>;
    fn SetLargeChange(&self, value: f64) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<f64>;
    fn SetValue(&self, value: f64) -> windows_core::Result<()>;
    fn ValueChanged(&self, handler: windows_core::Ref<RangeBaseValueChangedEventHandler>) -> windows_core::Result<i64>;
    fn RemoveValueChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IRangeBase_Vtbl {
    pub const fn new<Identity: IRangeBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Minimum<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBase_Impl::Minimum(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMinimum<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBase_Impl::SetMinimum(this, value).into()
            }
        }
        unsafe extern "system" fn Maximum<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBase_Impl::Maximum(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMaximum<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBase_Impl::SetMaximum(this, value).into()
            }
        }
        unsafe extern "system" fn SmallChange<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBase_Impl::SmallChange(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSmallChange<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBase_Impl::SetSmallChange(this, value).into()
            }
        }
        unsafe extern "system" fn LargeChange<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBase_Impl::LargeChange(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLargeChange<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBase_Impl::SetLargeChange(this, value).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBase_Impl::Value(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetValue<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBase_Impl::SetValue(this, value).into()
            }
        }
        unsafe extern "system" fn ValueChanged<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBase_Impl::ValueChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveValueChanged<Identity: IRangeBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBase_Impl::RemoveValueChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRangeBase, OFFSET>(),
            Minimum: Minimum::<Identity, OFFSET>,
            SetMinimum: SetMinimum::<Identity, OFFSET>,
            Maximum: Maximum::<Identity, OFFSET>,
            SetMaximum: SetMaximum::<Identity, OFFSET>,
            SmallChange: SmallChange::<Identity, OFFSET>,
            SetSmallChange: SetSmallChange::<Identity, OFFSET>,
            LargeChange: LargeChange::<Identity, OFFSET>,
            SetLargeChange: SetLargeChange::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            ValueChanged: ValueChanged::<Identity, OFFSET>,
            RemoveValueChanged: RemoveValueChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeBase as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBase_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Minimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Maximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetSmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetLargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub ValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRangeBaseFactory, IRangeBaseFactory_Vtbl, 0x41c205e2_4422_5dca_9b49_e31210ea396c);
impl windows_core::RuntimeType for IRangeBaseFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IRangeBaseFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait IRangeBaseFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<RangeBase>;
}
#[cfg(feature = "UI_Composition")]
impl IRangeBaseFactory_Vtbl {
    pub const fn new<Identity: IRangeBaseFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IRangeBaseFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBaseFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IRangeBaseFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeBaseFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IRangeBaseOverrides, IRangeBaseOverrides_Vtbl, 0xb3deb76f_68a6_5c14_a824_ab58e8774745);
impl windows_core::RuntimeType for IRangeBaseOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseOverrides");
}
impl IRangeBaseOverrides {
    pub fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMinimumChanged)(windows_core::Interface::as_raw(self), oldminimum, newminimum).ok() }
    }
    pub fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnMaximumChanged)(windows_core::Interface::as_raw(self), oldmaximum, newmaximum).ok() }
    }
    pub fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnValueChanged)(windows_core::Interface::as_raw(self), oldvalue, newvalue).ok() }
    }
}
impl windows_core::RuntimeName for IRangeBaseOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseOverrides";
}
pub trait IRangeBaseOverrides_Impl: windows_core::IUnknownImpl {
    fn OnMinimumChanged(&self, oldMinimum: f64, newMinimum: f64) -> windows_core::Result<()>;
    fn OnMaximumChanged(&self, oldMaximum: f64, newMaximum: f64) -> windows_core::Result<()>;
    fn OnValueChanged(&self, oldValue: f64, newValue: f64) -> windows_core::Result<()>;
}
impl IRangeBaseOverrides_Vtbl {
    pub const fn new<Identity: IRangeBaseOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMinimumChanged<Identity: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldminimum: f64, newminimum: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBaseOverrides_Impl::OnMinimumChanged(this, oldminimum, newminimum).into()
            }
        }
        unsafe extern "system" fn OnMaximumChanged<Identity: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldmaximum: f64, newmaximum: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBaseOverrides_Impl::OnMaximumChanged(this, oldmaximum, newmaximum).into()
            }
        }
        unsafe extern "system" fn OnValueChanged<Identity: IRangeBaseOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, oldvalue: f64, newvalue: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeBaseOverrides_Impl::OnValueChanged(this, oldvalue, newvalue).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRangeBaseOverrides, OFFSET>(),
            OnMinimumChanged: OnMinimumChanged::<Identity, OFFSET>,
            OnMaximumChanged: OnMaximumChanged::<Identity, OFFSET>,
            OnValueChanged: OnValueChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeBaseOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnMinimumChanged: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub OnMaximumChanged: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub OnValueChanged: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRangeBaseStatics, IRangeBaseStatics_Vtbl, 0x4aed5e49_64ec_56f1_874d_b8c0f83f9ac8);
impl windows_core::RuntimeType for IRangeBaseStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseStatics");
}
impl windows_core::RuntimeName for IRangeBaseStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IRangeBaseValueChangedEventArgs, IRangeBaseValueChangedEventArgs_Vtbl, 0xb0181692_9578_51c7_9d1c_adfcf8945aa9);
impl windows_core::RuntimeType for IRangeBaseValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseValueChangedEventArgs");
}
impl windows_core::RuntimeName for IRangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IRangeBaseValueChangedEventArgs";
}
pub trait IRangeBaseValueChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn OldValue(&self) -> windows_core::Result<f64>;
    fn NewValue(&self) -> windows_core::Result<f64>;
}
impl IRangeBaseValueChangedEventArgs_Vtbl {
    pub const fn new<Identity: IRangeBaseValueChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OldValue<Identity: IRangeBaseValueChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBaseValueChangedEventArgs_Impl::OldValue(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NewValue<Identity: IRangeBaseValueChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeBaseValueChangedEventArgs_Impl::NewValue(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRangeBaseValueChangedEventArgs, OFFSET>(),
            OldValue: OldValue::<Identity, OFFSET>,
            NewValue: NewValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeBaseValueChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeBaseValueChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OldValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub NewValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScrollBar, IScrollBar_Vtbl, 0x568cbf41_f741_5f05_8e08_c0a50ac17c8c);
impl windows_core::RuntimeType for IScrollBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IScrollBar");
}
impl windows_core::RuntimeName for IScrollBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollBar";
}
pub trait IScrollBar_Impl: windows_core::IUnknownImpl {
    fn Orientation(&self) -> windows_core::Result<super::Orientation>;
    fn SetOrientation(&self, value: super::Orientation) -> windows_core::Result<()>;
    fn ViewportSize(&self) -> windows_core::Result<f64>;
    fn SetViewportSize(&self, value: f64) -> windows_core::Result<()>;
    fn IndicatorMode(&self) -> windows_core::Result<ScrollingIndicatorMode>;
    fn SetIndicatorMode(&self, value: ScrollingIndicatorMode) -> windows_core::Result<()>;
    fn Scroll(&self, handler: windows_core::Ref<ScrollEventHandler>) -> windows_core::Result<i64>;
    fn RemoveScroll(&self, token: i64) -> windows_core::Result<()>;
}
impl IScrollBar_Vtbl {
    pub const fn new<Identity: IScrollBar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Orientation<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::Orientation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollBar_Impl::Orientation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOrientation<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::Orientation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollBar_Impl::SetOrientation(this, value).into()
            }
        }
        unsafe extern "system" fn ViewportSize<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollBar_Impl::ViewportSize(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetViewportSize<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollBar_Impl::SetViewportSize(this, value).into()
            }
        }
        unsafe extern "system" fn IndicatorMode<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ScrollingIndicatorMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollBar_Impl::IndicatorMode(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIndicatorMode<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ScrollingIndicatorMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollBar_Impl::SetIndicatorMode(this, value).into()
            }
        }
        unsafe extern "system" fn Scroll<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollBar_Impl::Scroll(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveScroll<Identity: IScrollBar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollBar_Impl::RemoveScroll(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IScrollBar, OFFSET>(),
            Orientation: Orientation::<Identity, OFFSET>,
            SetOrientation: SetOrientation::<Identity, OFFSET>,
            ViewportSize: ViewportSize::<Identity, OFFSET>,
            SetViewportSize: SetViewportSize::<Identity, OFFSET>,
            IndicatorMode: IndicatorMode::<Identity, OFFSET>,
            SetIndicatorMode: SetIndicatorMode::<Identity, OFFSET>,
            Scroll: Scroll::<Identity, OFFSET>,
            RemoveScroll: RemoveScroll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollBar as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBar_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::Orientation) -> windows_core::HRESULT,
    pub SetOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, super::Orientation) -> windows_core::HRESULT,
    pub ViewportSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetViewportSize: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IndicatorMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ScrollingIndicatorMode) -> windows_core::HRESULT,
    pub SetIndicatorMode: unsafe extern "system" fn(*mut core::ffi::c_void, ScrollingIndicatorMode) -> windows_core::HRESULT,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveScroll: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScrollBarStatics, IScrollBarStatics_Vtbl, 0x88b52e18_9528_579f_bd84_eba585a01c7a);
impl windows_core::RuntimeType for IScrollBarStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IScrollBarStatics");
}
impl windows_core::RuntimeName for IScrollBarStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollBarStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollBarStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IScrollEventArgs, IScrollEventArgs_Vtbl, 0xdbd27f11_f937_5ad0_9f75_b962c33254cf);
impl windows_core::RuntimeType for IScrollEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IScrollEventArgs");
}
impl windows_core::RuntimeName for IScrollEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollEventArgs";
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NewValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IScrollSnapPointsInfo, IScrollSnapPointsInfo_Vtbl, 0xd3ea6e09_ecf7_51a8_bd54_fc84b9653766);
impl windows_core::RuntimeType for IScrollSnapPointsInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo");
}
windows_core::imp::interface_hierarchy!(IScrollSnapPointsInfo, windows_core::IUnknown, windows_core::IInspectable);
impl IScrollSnapPointsInfo {
    pub fn AreHorizontalSnapPointsRegular(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreHorizontalSnapPointsRegular)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn AreVerticalSnapPointsRegular(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AreVerticalSnapPointsRegular)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn HorizontalSnapPointsChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).HorizontalSnapPointsChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveHorizontalSnapPointsChanged))
        }
    }
    pub fn VerticalSnapPointsChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).VerticalSnapPointsChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveVerticalSnapPointsChanged))
        }
    }
}
impl windows_core::RuntimeName for IScrollSnapPointsInfo {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IScrollSnapPointsInfo";
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollSnapPointsInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AreHorizontalSnapPointsRegular: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AreVerticalSnapPointsRegular: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub HorizontalSnapPointsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveHorizontalSnapPointsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub VerticalSnapPointsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveVerticalSnapPointsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelector, ISelector_Vtbl, 0x8f7e2159_e61d_576f_8476_f83fde3d689e);
impl windows_core::RuntimeType for ISelector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.ISelector");
}
impl windows_core::RuntimeName for ISelector {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ISelector";
}
pub trait ISelector_Impl: windows_core::IUnknownImpl {
    fn SelectedIndex(&self) -> windows_core::Result<i32>;
    fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()>;
    fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetSelectedItem(&self, value: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>;
    fn SelectedValue(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetSelectedValue(&self, value: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()>;
    fn SelectedValuePath(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetSelectedValuePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn IsSynchronizedWithCurrentItem(&self) -> windows_core::Result<windows_reference::IReference<bool>>;
    fn SetIsSynchronizedWithCurrentItem(&self, value: windows_core::Ref<windows_reference::IReference<bool>>) -> windows_core::Result<()>;
    fn SelectionChanged(&self, handler: windows_core::Ref<super::SelectionChangedEventHandler>) -> windows_core::Result<i64>;
    fn RemoveSelectionChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl ISelector_Vtbl {
    pub const fn new<Identity: ISelector_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SelectedIndex<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelector_Impl::SelectedIndex(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelectedIndex<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelector_Impl::SetSelectedIndex(this, value).into()
            }
        }
        unsafe extern "system" fn SelectedItem<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelector_Impl::SelectedItem(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelectedItem<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelector_Impl::SetSelectedItem(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SelectedValue<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelector_Impl::SelectedValue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelectedValue<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelector_Impl::SetSelectedValue(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SelectedValuePath<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelector_Impl::SelectedValuePath(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelectedValuePath<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelector_Impl::SetSelectedValuePath(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn IsSynchronizedWithCurrentItem<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelector_Impl::IsSynchronizedWithCurrentItem(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsSynchronizedWithCurrentItem<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelector_Impl::SetIsSynchronizedWithCurrentItem(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn SelectionChanged<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelector_Impl::SelectionChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSelectionChanged<Identity: ISelector_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelector_Impl::RemoveSelectionChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISelector, OFFSET>(),
            SelectedIndex: SelectedIndex::<Identity, OFFSET>,
            SetSelectedIndex: SetSelectedIndex::<Identity, OFFSET>,
            SelectedItem: SelectedItem::<Identity, OFFSET>,
            SetSelectedItem: SetSelectedItem::<Identity, OFFSET>,
            SelectedValue: SelectedValue::<Identity, OFFSET>,
            SetSelectedValue: SetSelectedValue::<Identity, OFFSET>,
            SelectedValuePath: SelectedValuePath::<Identity, OFFSET>,
            SetSelectedValuePath: SetSelectedValuePath::<Identity, OFFSET>,
            IsSynchronizedWithCurrentItem: IsSynchronizedWithCurrentItem::<Identity, OFFSET>,
            SetIsSynchronizedWithCurrentItem: SetIsSynchronizedWithCurrentItem::<Identity, OFFSET>,
            SelectionChanged: SelectionChanged::<Identity, OFFSET>,
            RemoveSelectionChanged: RemoveSelectionChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelector as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectedIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetSelectedIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSelectedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectedValuePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSelectedValuePath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSynchronizedWithCurrentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsSynchronizedWithCurrentItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSelectionChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelectorFactory, ISelectorFactory_Vtbl, 0x21a42024_af07_58f9_8789_848d3324d901);
impl windows_core::RuntimeType for ISelectorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.ISelectorFactory");
}
impl windows_core::RuntimeName for ISelectorFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ISelectorFactory";
}
pub trait ISelectorFactory_Impl: windows_core::IUnknownImpl {}
impl ISelectorFactory_Vtbl {
    pub const fn new<Identity: ISelectorFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISelectorFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectorFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISelectorItem, ISelectorItem_Vtbl, 0x5772c4de_60ea_5492_8c5e_b3323d5a3ca6);
impl windows_core::RuntimeType for ISelectorItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.ISelectorItem");
}
impl windows_core::RuntimeName for ISelectorItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ISelectorItem";
}
pub trait ISelectorItem_Impl: windows_core::IUnknownImpl {
    fn IsSelected(&self) -> windows_core::Result<bool>;
    fn SetIsSelected(&self, value: bool) -> windows_core::Result<()>;
}
impl ISelectorItem_Vtbl {
    pub const fn new<Identity: ISelectorItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSelected<Identity: ISelectorItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectorItem_Impl::IsSelected(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsSelected<Identity: ISelectorItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelectorItem_Impl::SetIsSelected(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ISelectorItem, OFFSET>(),
            IsSelected: IsSelected::<Identity, OFFSET>,
            SetIsSelected: SetIsSelected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectorItem as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItem_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ISelectorItemFactory, ISelectorItemFactory_Vtbl, 0x078039f5_76ed_5299_9715_fc8c58173560);
impl windows_core::RuntimeType for ISelectorItemFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.ISelectorItemFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ISelectorItemFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ISelectorItemFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait ISelectorItemFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<SelectorItem>;
}
#[cfg(feature = "UI_Composition")]
impl ISelectorItemFactory_Vtbl {
    pub const fn new<Identity: ISelectorItemFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: ISelectorItemFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectorItemFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, ISelectorItemFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectorItemFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(ISelectorItemStatics, ISelectorItemStatics_Vtbl, 0x4b201a54_a414_5e79_9b6b_3da9de442a35);
impl windows_core::RuntimeType for ISelectorItemStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.ISelectorItemStatics");
}
impl windows_core::RuntimeName for ISelectorItemStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ISelectorItemStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorItemStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(ISelectorStatics, ISelectorStatics_Vtbl, 0x569b2234_1ceb_516e_b64e_0d479452e279);
impl windows_core::RuntimeType for ISelectorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.ISelectorStatics");
}
impl windows_core::RuntimeName for ISelectorStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ISelectorStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    SelectedIndexProperty: usize,
    SelectedItemProperty: usize,
    SelectedValueProperty: usize,
    SelectedValuePathProperty: usize,
    IsSynchronizedWithCurrentItemProperty: usize,
    pub GetIsSelectionActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToggleButton, IToggleButton_Vtbl, 0x686fbaa4_c866_568b_8f75_481d8d545291);
impl windows_core::RuntimeType for IToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IToggleButton");
}
impl windows_core::RuntimeName for IToggleButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IToggleButton";
}
pub trait IToggleButton_Impl: windows_core::IUnknownImpl {
    fn IsChecked(&self) -> windows_core::Result<windows_reference::IReference<bool>>;
    fn SetIsChecked(&self, value: windows_core::Ref<windows_reference::IReference<bool>>) -> windows_core::Result<()>;
    fn IsThreeState(&self) -> windows_core::Result<bool>;
    fn SetIsThreeState(&self, value: bool) -> windows_core::Result<()>;
    fn Checked(&self, handler: windows_core::Ref<super::super::RoutedEventHandler>) -> windows_core::Result<i64>;
    fn RemoveChecked(&self, token: i64) -> windows_core::Result<()>;
    fn Unchecked(&self, handler: windows_core::Ref<super::super::RoutedEventHandler>) -> windows_core::Result<i64>;
    fn RemoveUnchecked(&self, token: i64) -> windows_core::Result<()>;
    fn Indeterminate(&self, handler: windows_core::Ref<super::super::RoutedEventHandler>) -> windows_core::Result<i64>;
    fn RemoveIndeterminate(&self, token: i64) -> windows_core::Result<()>;
}
impl IToggleButton_Vtbl {
    pub const fn new<Identity: IToggleButton_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsChecked<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleButton_Impl::IsChecked(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsChecked<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleButton_Impl::SetIsChecked(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn IsThreeState<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleButton_Impl::IsThreeState(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsThreeState<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleButton_Impl::SetIsThreeState(this, value).into()
            }
        }
        unsafe extern "system" fn Checked<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleButton_Impl::Checked(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveChecked<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleButton_Impl::RemoveChecked(this, token).into()
            }
        }
        unsafe extern "system" fn Unchecked<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleButton_Impl::Unchecked(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveUnchecked<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleButton_Impl::RemoveUnchecked(this, token).into()
            }
        }
        unsafe extern "system" fn Indeterminate<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleButton_Impl::Indeterminate(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveIndeterminate<Identity: IToggleButton_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleButton_Impl::RemoveIndeterminate(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IToggleButton, OFFSET>(),
            IsChecked: IsChecked::<Identity, OFFSET>,
            SetIsChecked: SetIsChecked::<Identity, OFFSET>,
            IsThreeState: IsThreeState::<Identity, OFFSET>,
            SetIsThreeState: SetIsThreeState::<Identity, OFFSET>,
            Checked: Checked::<Identity, OFFSET>,
            RemoveChecked: RemoveChecked::<Identity, OFFSET>,
            Unchecked: Unchecked::<Identity, OFFSET>,
            RemoveUnchecked: RemoveUnchecked::<Identity, OFFSET>,
            Indeterminate: Indeterminate::<Identity, OFFSET>,
            RemoveIndeterminate: RemoveIndeterminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToggleButton as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButton_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsChecked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsThreeState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsThreeState: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Checked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveChecked: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Unchecked: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveUnchecked: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Indeterminate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveIndeterminate: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToggleButtonFactory, IToggleButtonFactory_Vtbl, 0x519511bb_d35b_5e2d_966c_8369405a4408);
impl windows_core::RuntimeType for IToggleButtonFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IToggleButtonFactory");
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for IToggleButtonFactory {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IToggleButtonFactory";
}
#[cfg(feature = "UI_Composition")]
pub trait IToggleButtonFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, baseInterface: windows_core::Ref<windows_core::IInspectable>, innerInterface: windows_core::OutRef<windows_core::IInspectable>) -> windows_core::Result<ToggleButton>;
}
#[cfg(feature = "UI_Composition")]
impl IToggleButtonFactory_Vtbl {
    pub const fn new<Identity: IToggleButtonFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IToggleButtonFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseinterface: *mut core::ffi::c_void, innerinterface: *mut *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleButtonFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&baseinterface), core::mem::transmute_copy(&innerinterface)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IToggleButtonFactory, OFFSET>(), CreateInstance: CreateInstance::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToggleButtonFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Composition")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Composition"))]
    CreateInstance: usize,
}
windows_core::imp::define_interface!(IToggleButtonOverrides, IToggleButtonOverrides_Vtbl, 0xee55f85d_9061_5d18_b31a_90bc5625cfe9);
impl windows_core::RuntimeType for IToggleButtonOverrides {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IToggleButtonOverrides");
}
impl IToggleButtonOverrides {
    pub fn OnToggle(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnToggle)(windows_core::Interface::as_raw(self)).ok() }
    }
}
impl windows_core::RuntimeName for IToggleButtonOverrides {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IToggleButtonOverrides";
}
pub trait IToggleButtonOverrides_Impl: windows_core::IUnknownImpl {
    fn OnToggle(&self) -> windows_core::Result<()>;
}
impl IToggleButtonOverrides_Vtbl {
    pub const fn new<Identity: IToggleButtonOverrides_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnToggle<Identity: IToggleButtonOverrides_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleButtonOverrides_Impl::OnToggle(this).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IToggleButtonOverrides, OFFSET>(), OnToggle: OnToggle::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToggleButtonOverrides as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonOverrides_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OnToggle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IToggleButtonStatics, IToggleButtonStatics_Vtbl, 0x4b8397e3_76fd_59df_824f_40ae339fb00b);
impl windows_core::RuntimeType for IToggleButtonStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.IToggleButtonStatics");
}
impl windows_core::RuntimeName for IToggleButtonStatics {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.IToggleButtonStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleButtonStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeBase(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(RangeBase, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(RangeBase, super::super::super::Composition::IAnimationObject, super::super::super::Composition::IVisualElement, super::super::super::Composition::IVisualElement2, super::Control, super::super::FrameworkElement, super::super::UIElement, super::super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl RangeBase {
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngaged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> windows_core::Result<super::super::Media::FontFamily> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontFamily)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FontWeight(&self) -> windows_core::Result<windows::UI::Text::FontWeight> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> windows_core::Result<windows::UI::Text::FontStyle> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: windows::UI::Text::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStretch(&self) -> windows_core::Result<windows::UI::Text::FontStretch> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: windows::UI::Text::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForeground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalContentAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalContentAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<super::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(&self, value: super::BackgroundSizing) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundSizing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FocusEngaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusEngagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusEngagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusEngaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusEngaged))
        }
    }
    pub fn FocusDisengaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusDisengagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusDisengagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusDisengaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusDisengaged))
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEngagement)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerEntered)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerPressed)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerMoved)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerReleased)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerExited)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCaptureLost)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCanceled)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerWheelChanged)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDoubleTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnHolding)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnRightTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationDelta)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationCompleted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnGotFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnLostFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnCharacterReceived)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleKey)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetTemplateChild(&self, childname: &windows_core::HSTRING) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(childname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::super::Style> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
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
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Minimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Minimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinimum(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMinimum)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Maximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Maximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaximum(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximum)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn SmallChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetSmallChange(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSmallChange)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn LargeChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetLargeChange(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetLargeChange)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ValueChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RangeBaseValueChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <RangeBaseValueChangedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ValueChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveValueChanged))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IRangeBaseFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IRangeBaseFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBaseOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnMinimumChanged)(windows_core::Interface::as_raw(this), oldminimum, newminimum).ok() }
    }
    pub fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBaseOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnMaximumChanged)(windows_core::Interface::as_raw(this), oldmaximum, newmaximum).ok() }
    }
    pub fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBaseOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnValueChanged)(windows_core::Interface::as_raw(this), oldvalue, newvalue).ok() }
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::super::Visibility> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::super::FocusState> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn IRangeBaseFactory<R, F: FnOnce(&IRangeBaseFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RangeBase, IRangeBaseFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IRangeBaseStatics<R, F: FnOnce(&IRangeBaseStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RangeBase, IRangeBaseStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for RangeBase {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRangeBase>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for RangeBase {
    type Vtable = <IRangeBase as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRangeBase as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for RangeBase {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.RangeBase";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for RangeBase {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for RangeBase {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RangeBaseValueChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RangeBaseValueChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(RangeBaseValueChangedEventArgs, super::super::RoutedEventArgs);
impl RangeBaseValueChangedEventArgs {
    pub fn OldValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OldValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for RangeBaseValueChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRangeBaseValueChangedEventArgs>();
}
unsafe impl windows_core::Interface for RangeBaseValueChangedEventArgs {
    type Vtable = <IRangeBaseValueChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRangeBaseValueChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RangeBaseValueChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.RangeBaseValueChangedEventArgs";
}
unsafe impl Send for RangeBaseValueChangedEventArgs {}
unsafe impl Sync for RangeBaseValueChangedEventArgs {}
windows_core::imp::define_interface!(RangeBaseValueChangedEventHandler, RangeBaseValueChangedEventHandler_Vtbl, 0x23f0e209_9455_54cb_b8bc_0b49553c7dcc);
impl windows_core::RuntimeType for RangeBaseValueChangedEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl RangeBaseValueChangedEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RangeBaseValueChangedEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&RangeBaseValueChangedEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<RangeBaseValueChangedEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RangeBaseValueChangedEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct RangeBaseValueChangedEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RangeBaseValueChangedEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RangeBaseValueChangedEventArgs>) -> windows_core::Result<()> + Send + 'static> RangeBaseValueChangedEventHandlerBox<F> {
    const VTABLE: RangeBaseValueChangedEventHandler_Vtbl = RangeBaseValueChangedEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<RangeBaseValueChangedEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<RangeBaseValueChangedEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollBar(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(ScrollBar, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(ScrollBar, super::super::super::Composition::IAnimationObject, super::super::super::Composition::IVisualElement, super::super::super::Composition::IVisualElement2, RangeBase, super::Control, super::super::FrameworkElement, super::super::UIElement, super::super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl ScrollBar {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScrollBar, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngaged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> windows_core::Result<super::super::Media::FontFamily> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontFamily)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FontWeight(&self) -> windows_core::Result<windows::UI::Text::FontWeight> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> windows_core::Result<windows::UI::Text::FontStyle> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: windows::UI::Text::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStretch(&self) -> windows_core::Result<windows::UI::Text::FontStretch> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: windows::UI::Text::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForeground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalContentAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalContentAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<super::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(&self, value: super::BackgroundSizing) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundSizing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FocusEngaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusEngagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusEngagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusEngaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusEngaged))
        }
    }
    pub fn FocusDisengaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusDisengagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusDisengagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusDisengaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusDisengaged))
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEngagement)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerEntered)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerPressed)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerMoved)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerReleased)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerExited)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCaptureLost)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCanceled)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerWheelChanged)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDoubleTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnHolding)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnRightTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationDelta)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationCompleted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnGotFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnLostFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnCharacterReceived)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleKey)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetTemplateChild(&self, childname: &windows_core::HSTRING) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(childname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::super::Style> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
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
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Minimum(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Minimum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinimum(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinimum)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Maximum(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Maximum)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaximum(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaximum)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SmallChange(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SmallChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSmallChange(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSmallChange)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn LargeChange(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LargeChange)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLargeChange(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLargeChange)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Value(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValue(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ValueChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<RangeBaseValueChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IRangeBase>(self)?;
        let handler = <RangeBaseValueChangedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ValueChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveValueChanged))
        }
    }
    pub fn OnMinimumChanged(&self, oldminimum: f64, newminimum: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBaseOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnMinimumChanged)(windows_core::Interface::as_raw(this), oldminimum, newminimum).ok() }
    }
    pub fn OnMaximumChanged(&self, oldmaximum: f64, newmaximum: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBaseOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnMaximumChanged)(windows_core::Interface::as_raw(this), oldmaximum, newmaximum).ok() }
    }
    pub fn OnValueChanged(&self, oldvalue: f64, newvalue: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRangeBaseOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnValueChanged)(windows_core::Interface::as_raw(this), oldvalue, newvalue).ok() }
    }
    pub fn Orientation(&self) -> windows_core::Result<super::Orientation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Orientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetOrientation(&self, value: super::Orientation) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetOrientation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ViewportSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ViewportSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetViewportSize(&self, value: f64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetViewportSize)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IndicatorMode(&self) -> windows_core::Result<ScrollingIndicatorMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IndicatorMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIndicatorMode(&self, value: ScrollingIndicatorMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIndicatorMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Scroll<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ScrollEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <ScrollEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveScroll))
        }
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::super::Visibility> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::super::FocusState> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn IScrollBarStatics<R, F: FnOnce(&IScrollBarStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScrollBar, IScrollBarStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for ScrollBar {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScrollBar>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for ScrollBar {
    type Vtable = <IScrollBar as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScrollBar as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ScrollBar {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ScrollBar";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for ScrollBar {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for ScrollBar {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ScrollEventArgs, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(ScrollEventArgs, super::super::RoutedEventArgs);
impl ScrollEventArgs {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ScrollEventArgs, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn OriginalSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IRoutedEventArgs>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OriginalSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn NewValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ScrollEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IScrollEventArgs>();
}
unsafe impl windows_core::Interface for ScrollEventArgs {
    type Vtable = <IScrollEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IScrollEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ScrollEventArgs {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ScrollEventArgs";
}
unsafe impl Send for ScrollEventArgs {}
unsafe impl Sync for ScrollEventArgs {}
windows_core::imp::define_interface!(ScrollEventHandler, ScrollEventHandler_Vtbl, 0xff661ba9_8c06_5785_a23c_30d6b31631e8);
impl windows_core::RuntimeType for ScrollEventHandler {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
impl ScrollEventHandler {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ScrollEventArgs>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&ScrollEventHandlerBox::<F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, e: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<ScrollEventArgs>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), e.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ScrollEventHandler_Vtbl {
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT,
}
struct ScrollEventHandlerBox<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ScrollEventArgs>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(fn() -> F,)>);
impl<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<ScrollEventArgs>) -> windows_core::Result<()> + Send + 'static> ScrollEventHandlerBox<F> {
    const VTABLE: ScrollEventHandler_Vtbl = ScrollEventHandler_Vtbl {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<ScrollEventHandler, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<ScrollEventHandler, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<ScrollEventHandler, F>::Release,
        },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, e: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<ScrollEventHandler, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&e)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ScrollingIndicatorMode(pub i32);
impl ScrollingIndicatorMode {
    pub const None: Self = Self(0);
    pub const TouchIndicator: Self = Self(1);
    pub const MouseIndicator: Self = Self(2);
}
impl windows_core::imp::TypeKind for ScrollingIndicatorMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ScrollingIndicatorMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Controls.Primitives.ScrollingIndicatorMode;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.ScrollingIndicatorMode");
}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Selector(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(Selector, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(Selector, super::super::super::Composition::IAnimationObject, super::IItemContainerMapping, super::super::super::Composition::IVisualElement, super::super::super::Composition::IVisualElement2, super::ItemsControl, super::Control, super::super::FrameworkElement, super::super::UIElement, super::super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl Selector {
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngaged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> windows_core::Result<super::super::Media::FontFamily> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontFamily)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FontWeight(&self) -> windows_core::Result<windows::UI::Text::FontWeight> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> windows_core::Result<windows::UI::Text::FontStyle> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: windows::UI::Text::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStretch(&self) -> windows_core::Result<windows::UI::Text::FontStretch> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: windows::UI::Text::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForeground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalContentAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalContentAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<super::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(&self, value: super::BackgroundSizing) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundSizing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FocusEngaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusEngagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusEngagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusEngaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusEngaged))
        }
    }
    pub fn FocusDisengaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusDisengagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusDisengagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusDisengaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusDisengaged))
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEngagement)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerEntered)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerPressed)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerMoved)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerReleased)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerExited)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCaptureLost)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCanceled)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerWheelChanged)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDoubleTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnHolding)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnRightTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationDelta)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationCompleted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnGotFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnLostFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnCharacterReceived)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleKey)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetTemplateChild(&self, childname: &windows_core::HSTRING) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(childname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::super::Style> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
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
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ItemFromContainer<P0>(&self, container: P0) -> windows_core::Result<windows_core::IInspectable>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IItemContainerMapping>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemFromContainer)(windows_core::Interface::as_raw(this), container.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ContainerFromItem<P0>(&self, item: P0) -> windows_core::Result<super::super::DependencyObject>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IItemContainerMapping>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainerFromItem)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IndexFromContainer<P0>(&self, container: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IItemContainerMapping>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexFromContainer)(windows_core::Interface::as_raw(this), container.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn ContainerFromIndex(&self, index: i32) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IItemContainerMapping>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContainerFromIndex)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ItemsSource(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemsSource)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetItemsSource<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetItemsSource)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Items(&self) -> windows_core::Result<super::ItemCollection> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Items)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DisplayMemberPath(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayMemberPath)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetDisplayMemberPath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDisplayMemberPath)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ItemsPanelRoot(&self) -> windows_core::Result<super::Panel> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemsPanelRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ItemContainerStyle(&self) -> windows_core::Result<super::super::Style> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ItemContainerStyle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetItemContainerStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetItemContainerStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Data")]
    pub fn GroupStyle(&self) -> windows_core::Result<windows_collections::IObservableVector<super::GroupStyle>> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GroupStyle)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsGrouping(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsGrouping)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn GroupHeaderContainerFromItemContainer<P0>(&self, itemcontainer: P0) -> windows_core::Result<super::super::DependencyObject>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GroupHeaderContainerFromItemContainer)(windows_core::Interface::as_raw(this), itemcontainer.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsItemItsOwnContainerOverride<P0>(&self, item: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControlOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsItemItsOwnContainerOverride)(windows_core::Interface::as_raw(this), item.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn GetContainerForItemOverride(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IItemsControlOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetContainerForItemOverride)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ClearContainerForItemOverride<P0, P1>(&self, element: P0, item: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ClearContainerForItemOverride)(windows_core::Interface::as_raw(this), element.param().abi(), item.param().abi()).ok() }
    }
    pub fn PrepareContainerForItemOverride<P0, P1>(&self, element: P0, item: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).PrepareContainerForItemOverride)(windows_core::Interface::as_raw(this), element.param().abi(), item.param().abi()).ok() }
    }
    pub fn OnItemsChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnItemsChanged)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnItemContainerStyleChanged<P0, P1>(&self, olditemcontainerstyle: P0, newitemcontainerstyle: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
        P1: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::IItemsControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnItemContainerStyleChanged)(windows_core::Interface::as_raw(this), olditemcontainerstyle.param().abi(), newitemcontainerstyle.param().abi()).ok() }
    }
    pub fn SelectedIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetSelectedIndex(&self, value: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedIndex)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn SelectedItem(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSelectedItem<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedItem)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn SelectedValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetSelectedValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedValue)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn SelectedValuePath(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectedValuePath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetSelectedValuePath(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedValuePath)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsSynchronizedWithCurrentItem(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSynchronizedWithCurrentItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)).and_then(|r__: windows_reference::IReference<bool>| r__.Value())
        }
    }
    pub fn SetIsSynchronizedWithCurrentItem(&self, value: Option<bool>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<bool> as From<_>>::from);
        unsafe { (windows_core::Interface::vtable(self).SetIsSynchronizedWithCurrentItem)(windows_core::Interface::as_raw(self), windows_core::Param::param(value__.as_ref()).abi()).ok() }
    }
    pub fn SelectionChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::SelectionChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::SelectionChangedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).SelectionChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveSelectionChanged))
        }
    }
    pub fn GetIsSelectionActive<P0>(element: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        Self::ISelectorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetIsSelectionActive)(windows_core::Interface::as_raw(this), element.param().abi(), &mut result__).map(|| result__)
        })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::super::Visibility> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::super::FocusState> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn ISelectorStatics<R, F: FnOnce(&ISelectorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Selector, ISelectorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for Selector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISelector>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for Selector {
    type Vtable = <ISelector as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelector as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for Selector {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.Selector";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for Selector {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for Selector {}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SelectorItem(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(SelectorItem, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(SelectorItem, super::super::super::Composition::IAnimationObject, super::super::super::Composition::IVisualElement, super::super::super::Composition::IVisualElement2, super::ContentControl, super::Control, super::super::FrameworkElement, super::super::UIElement, super::super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl SelectorItem {
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<super::super::UIElement> {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(&self, oldcontent: P0, newcontent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IContentControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnContentChanged)(windows_core::Interface::as_raw(this), oldcontent.param().abi(), newcontent.param().abi()).ok() }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngaged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> windows_core::Result<super::super::Media::FontFamily> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontFamily)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FontWeight(&self) -> windows_core::Result<windows::UI::Text::FontWeight> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> windows_core::Result<windows::UI::Text::FontStyle> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: windows::UI::Text::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStretch(&self) -> windows_core::Result<windows::UI::Text::FontStretch> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: windows::UI::Text::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForeground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalContentAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalContentAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<super::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(&self, value: super::BackgroundSizing) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundSizing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FocusEngaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusEngagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusEngagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusEngaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusEngaged))
        }
    }
    pub fn FocusDisengaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusDisengagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusDisengagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusDisengaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusDisengaged))
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEngagement)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerEntered)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerPressed)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerMoved)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerReleased)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerExited)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCaptureLost)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCanceled)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerWheelChanged)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDoubleTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnHolding)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnRightTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationDelta)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationCompleted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnGotFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnLostFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnCharacterReceived)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleKey)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetTemplateChild(&self, childname: &windows_core::HSTRING) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(childname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::super::Style> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
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
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsSelected(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsSelected(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsSelected)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::ISelectorItemFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::ISelectorItemFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::super::Visibility> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::super::FocusState> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn ISelectorItemFactory<R, F: FnOnce(&ISelectorItemFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SelectorItem, ISelectorItemFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ISelectorItemStatics<R, F: FnOnce(&ISelectorItemStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<SelectorItem, ISelectorItemStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for SelectorItem {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ISelectorItem>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for SelectorItem {
    type Vtable = <ISelectorItem as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ISelectorItem as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for SelectorItem {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.SelectorItem";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for SelectorItem {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for SelectorItem {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TickPlacement(pub i32);
impl TickPlacement {
    pub const None: Self = Self(0);
    pub const TopLeft: Self = Self(1);
    pub const BottomRight: Self = Self(2);
    pub const Outside: Self = Self(3);
    pub const Inline: Self = Self(4);
}
impl windows_core::imp::TypeKind for TickPlacement {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for TickPlacement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Xaml.Controls.Primitives.TickPlacement;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Xaml.Controls.Primitives.TickPlacement");
}
#[cfg(feature = "UI_Composition")]
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToggleButton(windows_core::IUnknown);
#[cfg(feature = "UI_Composition")]
windows_core::imp::interface_hierarchy!(ToggleButton, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "UI_Composition")]
windows_core::imp::required_hierarchy!(ToggleButton, super::super::super::Composition::IAnimationObject, super::super::super::Composition::IVisualElement, super::super::super::Composition::IVisualElement2, ButtonBase, super::ContentControl, super::Control, super::super::FrameworkElement, super::super::UIElement, super::super::DependencyObject);
#[cfg(feature = "UI_Composition")]
impl ToggleButton {
    pub fn IsPointerOver(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IButtonBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPointerOver)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn IsPressed(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IButtonBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPressed)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Command(&self) -> windows_core::Result<super::super::Input::ICommand> {
        let this = &windows_core::Interface::cast::<IButtonBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Command)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetCommand<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ICommand>,
    {
        let this = &windows_core::Interface::cast::<IButtonBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCommand)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn CommandParameter(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IButtonBase>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CommandParameter)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetCommandParameter<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IButtonBase>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCommandParameter)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Click<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IButtonBase>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Click)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveClick))
        }
    }
    pub fn Content(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Content)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetContent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetContent)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ContentTemplateRoot(&self) -> windows_core::Result<super::super::UIElement> {
        let this = &windows_core::Interface::cast::<super::IContentControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentTemplateRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn OnContentChanged<P0, P1>(&self, oldcontent: P0, newcontent: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IContentControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnContentChanged)(windows_core::Interface::as_raw(this), oldcontent.param().abi(), newcontent.param().abi()).ok() }
    }
    pub fn IsFocusEngagementEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngagementEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngagementEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsFocusEngaged(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsFocusEngaged)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsFocusEngaged(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsFocusEngaged)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontSize(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontSize(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FontFamily(&self) -> windows_core::Result<super::super::Media::FontFamily> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontFamily)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFontFamily<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::FontFamily>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontFamily)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FontWeight(&self) -> windows_core::Result<windows::UI::Text::FontWeight> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontWeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontWeight(&self, value: windows::UI::Text::FontWeight) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontWeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStyle(&self) -> windows_core::Result<windows::UI::Text::FontStyle> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStyle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStyle(&self, value: windows::UI::Text::FontStyle) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStyle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FontStretch(&self) -> windows_core::Result<windows::UI::Text::FontStretch> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FontStretch)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFontStretch(&self, value: windows::UI::Text::FontStretch) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFontStretch)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CharacterSpacing(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CharacterSpacing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharacterSpacing(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCharacterSpacing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Foreground(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Foreground)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetForeground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetForeground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTextScaleFactorEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTextScaleFactorEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTextScaleFactorEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Padding(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Padding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetPadding(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPadding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalContentAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalContentAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalContentAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalContentAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalContentAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalContentAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Background(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Background)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBackground<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackground)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn BackgroundSizing(&self) -> windows_core::Result<super::BackgroundSizing> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BackgroundSizing)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBackgroundSizing(&self, value: super::BackgroundSizing) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBackgroundSizing)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BorderThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetBorderThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn BorderBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BorderBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetBorderBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetBorderBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn DefaultStyleResourceUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleResourceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleResourceUri<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleResourceUri)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn FocusEngaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusEngagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusEngagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusEngaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusEngaged))
        }
    }
    pub fn FocusDisengaged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::Control>, windows_core::Ref<super::FocusDisengagedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::Control, super::FocusDisengagedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).FocusDisengaged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveFocusDisengaged))
        }
    }
    pub fn RemoveFocusEngagement(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveFocusEngagement)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ApplyTemplate(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::IControl>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ApplyTemplate)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerEntered<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerEntered)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerPressed<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerPressed)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerMoved<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerMoved)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerReleased<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerReleased)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerExited<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerExited)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCaptureLost<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCaptureLost)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerCanceled<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerCanceled)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPointerWheelChanged<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::PointerRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPointerWheelChanged)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::TappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnDoubleTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::DoubleTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDoubleTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnHolding<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::HoldingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnHolding)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnRightTapped<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::RightTappedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnRightTapped)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationInertiaStarting<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationInertiaStarting)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationStarted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationStartedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationStarted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationDelta<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationDeltaRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationDelta)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnManipulationCompleted<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ManipulationCompletedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnManipulationCompleted)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyDown<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyDown)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnPreviewKeyUp<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnPreviewKeyUp)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnGotFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnGotFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn OnLostFocus<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::RoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnLostFocus)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnCharacterReceived<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::CharacterReceivedRoutedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::IControlOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnCharacterReceived)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    pub fn DefaultStyleKey(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DefaultStyleKey)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDefaultStyleKey<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDefaultStyleKey)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn GetTemplateChild(&self, childname: &windows_core::HSTRING) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::IControlProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetTemplateChild)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(childname), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Dispatcher(&self) -> windows_core::Result<windows::UI::Core::CoreDispatcher> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Dispatcher)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::super::super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<super::super::IDependencyObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Resources(&self) -> windows_core::Result<super::super::ResourceDictionary> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Resources)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetResources<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::ResourceDictionary>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetResources)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Tag(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Tag)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetTag<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTag)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Language(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Language)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetLanguage(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetLanguage)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn ActualWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Width(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Width)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Height(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Height)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxWidth(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxWidth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxWidth(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxWidth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MinHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MinHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMinHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMinHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxHeight(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxHeight)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxHeight(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMaxHeight)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn HorizontalAlignment(&self) -> windows_core::Result<super::super::HorizontalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HorizontalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHorizontalAlignment(&self, value: super::super::HorizontalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHorizontalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn VerticalAlignment(&self) -> windows_core::Result<super::super::VerticalAlignment> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).VerticalAlignment)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVerticalAlignment(&self, value: super::super::VerticalAlignment) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVerticalAlignment)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Margin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Margin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn BaseUri(&self) -> windows_core::Result<windows::Foundation::Uri> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).BaseUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn DataContext(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DataContext)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetDataContext<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetDataContext)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusOnInteraction(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusOnInteraction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusOnInteraction(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusOnInteraction)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualMargin(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualMargin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualMargin(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualMargin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualSecondaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualSecondaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusVisualPrimaryThickness(&self) -> windows_core::Result<super::super::Thickness> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetFocusVisualPrimaryThickness(&self, value: super::super::Thickness) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryThickness)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualSecondaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualSecondaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualSecondaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn FocusVisualPrimaryBrush(&self) -> windows_core::Result<super::super::Media::Brush> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn SetFocusVisualPrimaryBrush<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Media::Brush>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetFocusVisualPrimaryBrush)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AllowFocusWhenDisabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowFocusWhenDisabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowFocusWhenDisabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Style(&self) -> windows_core::Result<super::super::Style> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Style)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetStyle<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Style>,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetStyle)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn Parent(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Parent)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn IsLoaded(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsLoaded)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Loaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoaded))
        }
    }
    pub fn Unloaded<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Unloaded)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveUnloaded))
        }
    }
    pub fn DataContextChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::DataContextChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::DataContextChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
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
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::EventHandler<windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LayoutUpdated)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLayoutUpdated))
        }
    }
    pub fn Loading<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Loading)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLoading))
        }
    }
    pub fn ActualThemeChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ActualThemeChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveActualThemeChanged))
        }
    }
    pub fn EffectiveViewportChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::FrameworkElement>, windows_core::Ref<super::super::EffectiveViewportChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::FrameworkElement, super::super::EffectiveViewportChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EffectiveViewportChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEffectiveViewportChanged))
        }
    }
    pub fn FindName(&self, name: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn MeasureOverride(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureOverride)(windows_core::Interface::as_raw(this), availablesize, &mut result__).map(|| result__)
        }
    }
    pub fn ArrangeOverride(&self, finalsize: windows::Foundation::Size) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ArrangeOverride)(windows_core::Interface::as_raw(this), finalsize, &mut result__).map(|| result__)
        }
    }
    pub fn OnApplyTemplate(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnApplyTemplate)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn GoToElementStateCore(&self, statename: &windows_core::HSTRING, usetransitions: bool) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GoToElementStateCore)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(statename), usetransitions, &mut result__).map(|| result__)
        }
    }
    pub fn InvalidateViewport(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IFrameworkElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateViewport)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsChecked(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsChecked)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)).and_then(|r__: windows_reference::IReference<bool>| r__.Value())
        }
    }
    pub fn SetIsChecked(&self, value: Option<bool>) -> windows_core::Result<()> {
        let value__ = value.map(<windows_reference::IReference<bool> as From<_>>::from);
        unsafe { (windows_core::Interface::vtable(self).SetIsChecked)(windows_core::Interface::as_raw(self), windows_core::Param::param(value__.as_ref()).abi()).ok() }
    }
    pub fn IsThreeState(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsThreeState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsThreeState(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsThreeState)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Checked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Checked)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveChecked))
        }
    }
    pub fn Unchecked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Unchecked)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveUnchecked))
        }
    }
    pub fn Indeterminate<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Indeterminate)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveIndeterminate))
        }
    }
    pub fn new() -> windows_core::Result<Self> {
        Self::IToggleButtonFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::ptr::null_mut(), core::ptr::null_mut(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn compose<T>(compose: T) -> windows_core::Result<Self>
    where
        T: windows_core::Compose,
    {
        Self::IToggleButtonFactory(|this| unsafe {
            let (derived__, base__) = windows_core::Compose::compose(compose);
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).ok()?;
            let _ = &derived__;
            windows_core::imp::Type::from_abi(result__)
        })
    }
    pub fn OnToggle(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IToggleButtonOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnToggle)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn DesiredSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DesiredSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AllowDrop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowDrop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowDrop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAllowDrop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Opacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetOpacity)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderTransformOrigin(&self) -> windows_core::Result<windows::Foundation::Point> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderTransformOrigin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRenderTransformOrigin(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRenderTransformOrigin)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHitTestVisible(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHitTestVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHitTestVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHitTestVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Visibility(&self) -> windows_core::Result<super::super::Visibility> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Visibility)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetVisibility(&self, value: super::super::Visibility) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetVisibility)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RenderSize(&self) -> windows_core::Result<windows::Foundation::Size> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RenderSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseLayoutRounding(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseLayoutRounding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseLayoutRounding(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseLayoutRounding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsDoubleTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsDoubleTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsDoubleTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsDoubleTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CanDrag(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanDrag)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanDrag(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanDrag)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsRightTapEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsRightTapEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsRightTapEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsRightTapEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsHoldingEnabled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsHoldingEnabled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsHoldingEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsHoldingEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationMode(&self) -> windows_core::Result<super::super::Input::ManipulationModes> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ManipulationMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetManipulationMode(&self, value: super::super::Input::ManipulationModes) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetManipulationMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptures(&self) -> windows_core::Result<windows_collections::IVectorView<super::super::Input::Pointer>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PointerCaptures)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    pub fn Lights(&self) -> windows_core::Result<windows_collections::IVector<super::super::Media::XamlLight>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Lights)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CanBeScrollAnchor(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanBeScrollAnchor)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCanBeScrollAnchor(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCanBeScrollAnchor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ExitDisplayModeOnAccessKeyInvoked(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetExitDisplayModeOnAccessKeyInvoked(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetExitDisplayModeOnAccessKeyInvoked)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsAccessKeyScope(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsAccessKeyScope)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsAccessKeyScope(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsAccessKeyScope)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AccessKeyScopeOwner(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKeyScopeOwner)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetAccessKeyScopeOwner<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKeyScopeOwner)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn AccessKey(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AccessKey)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetAccessKey(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetAccessKey)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyTipPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyTipPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyTipPlacementMode(&self, value: super::super::Input::KeyTipPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipHorizontalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipHorizontalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipHorizontalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipVerticalOffset(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipVerticalOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeyTipVerticalOffset(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipVerticalOffset)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeyTipTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyTipTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyTipTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyTipTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusKeyboardNavigation(&self) -> windows_core::Result<super::super::Input::XYFocusKeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusKeyboardNavigation(&self, value: super::super::Input::XYFocusKeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusKeyboardNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusUpNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusUpNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUpNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusDownNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusDownNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDownNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusLeftNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusLeftNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeftNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn XYFocusRightNavigationStrategy(&self) -> windows_core::Result<super::super::Input::XYFocusNavigationStrategy> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetXYFocusRightNavigationStrategy(&self, value: super::super::Input::XYFocusNavigationStrategy) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRightNavigationStrategy)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAccelerators(&self) -> windows_core::Result<windows_collections::IVector<super::super::Input::KeyboardAccelerator>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAccelerators)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn KeyboardAcceleratorPlacementTarget(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetKeyboardAcceleratorPlacementTarget<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementTarget)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyboardAcceleratorPlacementMode(&self) -> windows_core::Result<super::super::Input::KeyboardAcceleratorPlacementMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetKeyboardAcceleratorPlacementMode(&self, value: super::super::Input::KeyboardAcceleratorPlacementMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetKeyboardAcceleratorPlacementMode)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TabFocusNavigation(&self) -> windows_core::Result<super::super::Input::KeyboardNavigationMode> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabFocusNavigation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn SetTabFocusNavigation(&self, value: super::super::Input::KeyboardNavigationMode) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabFocusNavigation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Translation(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Translation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTranslation(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTranslation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> windows_core::Result<f32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Rotation)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotation(&self, value: f32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotation)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Scale(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Scale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetScale(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransformMatrix(&self) -> windows_core::Result<windows_numerics::Matrix4x4> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransformMatrix)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransformMatrix(&self, value: windows_numerics::Matrix4x4) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTransformMatrix)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CenterPoint(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CenterPoint)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetCenterPoint(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetCenterPoint)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RotationAxis(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RotationAxis)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRotationAxis(&self, value: windows_numerics::Vector3) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRotationAxis)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ActualOffset(&self) -> windows_core::Result<windows_numerics::Vector3> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualOffset)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ActualSize(&self) -> windows_core::Result<windows_numerics::Vector2> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActualSize)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XamlRoot(&self) -> windows_core::Result<super::super::XamlRoot> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XamlRoot)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXamlRoot<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::XamlRoot>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXamlRoot)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn RasterizationScale(&self) -> windows_core::Result<f64> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RasterizationScale)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetRasterizationScale(&self, value: f64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetRasterizationScale)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn FocusState(&self) -> windows_core::Result<super::super::FocusState> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FocusState)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UseSystemFocusVisuals(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UseSystemFocusVisuals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUseSystemFocusVisuals(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetUseSystemFocusVisuals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn XYFocusLeft(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusLeft)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusLeft<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusLeft)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusRight(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusRight)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusRight<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusRight)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusUp(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusUp)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusUp<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusUp)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn XYFocusDown(&self) -> windows_core::Result<super::super::DependencyObject> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusDown)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetXYFocusDown<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::DependencyObject>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetXYFocusDown)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn IsTabStop(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsTabStop)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsTabStop(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetIsTabStop)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TabIndex(&self) -> windows_core::Result<i32> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TabIndex)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTabIndex(&self, value: i32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetTabIndex)(windows_core::Interface::as_raw(this), value).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GotFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::RoutedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LostFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLostFocus))
        }
    }
    pub fn DragStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DragStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DragStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DragStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDragStarting))
        }
    }
    pub fn DropCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::DropCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::DropCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DropCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDropCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::CharacterReceivedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::CharacterReceivedRoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerPressed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerPressed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerMoved)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerMoved))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerReleased)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerReleased))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerEntered)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerEntered))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerExited)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerExited))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCaptureLost)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCaptureLost))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::PointerRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::PointerEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PointerWheelChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePointerWheelChanged))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::TappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::TappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Tapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn DoubleTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::DoubleTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::DoubleTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DoubleTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDoubleTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::HoldingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::HoldingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).Holding)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveHolding))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ContextRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ContextRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ContextRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextRequested))
        }
    }
    pub fn ContextCanceled<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::RoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::RoutedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextCanceled)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextCanceled))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::RightTappedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::RightTappedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).RightTapped)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveRightTapped))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationInertiaStartingRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationInertiaStartingEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationInertiaStarting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationInertiaStarting))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationStartedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationStartedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationStarted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationStarted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationDelta<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationDeltaRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationDeltaEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationDelta)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationDelta))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::ManipulationCompletedRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::ManipulationCompletedEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ManipulationCompleted)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveManipulationCompleted))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayRequested))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyDisplayDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyDisplayDismissedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyDisplayDismissedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyDisplayDismissed)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyDisplayDismissed))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn AccessKeyInvoked<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::AccessKeyInvokedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::AccessKeyInvokedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).AccessKeyInvoked)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveAccessKeyInvoked))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ProcessKeyboardAccelerators<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::ProcessKeyboardAcceleratorEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::ProcessKeyboardAcceleratorEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveProcessKeyboardAccelerators))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn GettingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::GettingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::GettingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).GettingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveGettingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn LosingFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::LosingFocusEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::LosingFocusEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).LosingFocus)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveLosingFocus))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn NoFocusCandidateFound<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::Input::NoFocusCandidateFoundEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::Input::NoFocusCandidateFoundEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NoFocusCandidateFound)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNoFocusCandidateFound))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyDown))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn PreviewKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<super::super::Input::KeyRoutedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <super::super::Input::KeyEventHandler>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).PreviewKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemovePreviewKeyUp))
        }
    }
    pub fn BringIntoViewRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<super::super::UIElement>, windows_core::Ref<super::super::BringIntoViewRequestedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<super::super::UIElement, super::super::BringIntoViewRequestedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).BringIntoViewRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveBringIntoViewRequested))
        }
    }
    pub fn Measure(&self, availablesize: windows::Foundation::Size) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Measure)(windows_core::Interface::as_raw(this), availablesize).ok() }
    }
    pub fn Arrange(&self, finalrect: windows::Foundation::Rect) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Arrange)(windows_core::Interface::as_raw(this), finalrect).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn CapturePointer<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CapturePointer)(windows_core::Interface::as_raw(this), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn ReleasePointerCapture<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::Pointer>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCapture)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn ReleasePointerCaptures(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReleasePointerCaptures)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateMeasure(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateMeasure)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn InvalidateArrange(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InvalidateArrange)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UpdateLayout(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateLayout)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CancelDirectManipulations(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CancelDirectManipulations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn StartDragAsync<P0>(&self, pointerpoint: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows::ApplicationModel::DataTransfer::DataPackageOperation>>
    where
        P0: windows_core::Param<super::super::super::Input::PointerPoint>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartDragAsync)(windows_core::Interface::as_raw(this), pointerpoint.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn StartBringIntoView(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).StartBringIntoView)(windows_core::Interface::as_raw(this)).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn TryInvokeKeyboardAccelerator<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe { (windows_core::Interface::vtable(this).TryInvokeKeyboardAccelerator)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn Focus(&self, value: super::super::FocusState) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<super::super::IUIElement>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Focus)(windows_core::Interface::as_raw(this), value, &mut result__).map(|| result__)
        }
    }
    pub fn OnDisconnectVisualChildren(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnDisconnectVisualChildren)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn FindSubElementsForTouchTargeting(&self, point: windows::Foundation::Point, boundingrect: windows::Foundation::Rect) -> windows_core::Result<windows_collections::IIterable<windows_collections::IIterable<windows::Foundation::Point>>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FindSubElementsForTouchTargeting)(windows_core::Interface::as_raw(this), point, boundingrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetChildrenInTabFocusOrder(&self) -> windows_core::Result<windows_collections::IIterable<super::super::DependencyObject>> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetChildrenInTabFocusOrder)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnKeyboardAcceleratorInvoked<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::KeyboardAcceleratorInvokedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnKeyboardAcceleratorInvoked)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Input")]
    pub fn OnProcessKeyboardAccelerators<P0>(&self, args: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Input::ProcessKeyboardAcceleratorEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnProcessKeyboardAccelerators)(windows_core::Interface::as_raw(this), args.param().abi()).ok() }
    }
    pub fn OnBringIntoViewRequested<P0>(&self, e: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::BringIntoViewRequestedEventArgs>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementOverrides>(self)?;
        unsafe { (windows_core::Interface::vtable(this).OnBringIntoViewRequested)(windows_core::Interface::as_raw(this), e.param().abi()).ok() }
    }
    #[cfg(feature = "UI_Input")]
    pub fn ProtectedCursor(&self) -> windows_core::Result<super::super::super::Input::InputCursor> {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProtectedCursor)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "UI_Input")]
    pub fn SetProtectedCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Input::InputCursor>,
    {
        let this = &windows_core::Interface::cast::<super::super::IUIElementProtected>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetProtectedCursor)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    fn IToggleButtonFactory<R, F: FnOnce(&IToggleButtonFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToggleButton, IToggleButtonFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IToggleButtonStatics<R, F: FnOnce(&IToggleButtonStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<ToggleButton, IToggleButtonStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeType for ToggleButton {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IToggleButton>();
}
#[cfg(feature = "UI_Composition")]
unsafe impl windows_core::Interface for ToggleButton {
    type Vtable = <IToggleButton as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IToggleButton as windows_core::Interface>::IID;
}
#[cfg(feature = "UI_Composition")]
impl windows_core::RuntimeName for ToggleButton {
    const NAME: &'static str = "Microsoft.UI.Xaml.Controls.Primitives.ToggleButton";
}
#[cfg(feature = "UI_Composition")]
unsafe impl Send for ToggleButton {}
#[cfg(feature = "UI_Composition")]
unsafe impl Sync for ToggleButton {}
