#[cfg(feature = "UI_Input_DragDrop")]
pub mod DragDrop;
#[cfg(feature = "UI_Input_Experimental")]
pub mod Experimental;
#[cfg(feature = "UI_Input_Interop")]
pub mod Interop;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharacterReceivedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CharacterReceivedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CharacterReceivedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn KeyCode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyCode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<PhysicalKeyStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CharacterReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICharacterReceivedEventArgs>();
}
unsafe impl windows_core::Interface for CharacterReceivedEventArgs {
    type Vtable = <ICharacterReceivedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICharacterReceivedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CharacterReceivedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CharacterReceivedEventArgs";
}
unsafe impl Send for CharacterReceivedEventArgs {}
unsafe impl Sync for CharacterReceivedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContextMenuKeyEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ContextMenuKeyEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ContextMenuKeyEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for ContextMenuKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IContextMenuKeyEventArgs>();
}
unsafe impl windows_core::Interface for ContextMenuKeyEventArgs {
    type Vtable = <IContextMenuKeyEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IContextMenuKeyEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ContextMenuKeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ContextMenuKeyEventArgs";
}
unsafe impl Send for ContextMenuKeyEventArgs {}
unsafe impl Sync for ContextMenuKeyEventArgs {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CrossSlideThresholds {
    pub SelectionStart: f32,
    pub SpeedBumpStart: f32,
    pub SpeedBumpEnd: f32,
    pub RearrangeStart: f32,
}
impl windows_core::imp::TypeKind for CrossSlideThresholds {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for CrossSlideThresholds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Input.CrossSlideThresholds;f4;f4;f4;f4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.CrossSlideThresholds");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CrossSlidingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CrossSlidingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CrossSlidingEventArgs {
    pub fn CrossSlidingState(&self) -> windows_core::Result<CrossSlidingState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlidingState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CrossSlidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICrossSlidingEventArgs>();
}
unsafe impl windows_core::Interface for CrossSlidingEventArgs {
    type Vtable = <ICrossSlidingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICrossSlidingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CrossSlidingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.CrossSlidingEventArgs";
}
unsafe impl Send for CrossSlidingEventArgs {}
unsafe impl Sync for CrossSlidingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CrossSlidingState(pub i32);
impl CrossSlidingState {
    pub const Started: Self = Self(0);
    pub const Dragging: Self = Self(1);
    pub const Selecting: Self = Self(2);
    pub const SelectSpeedBumping: Self = Self(3);
    pub const SpeedBumping: Self = Self(4);
    pub const Rearranging: Self = Self(5);
    pub const Completed: Self = Self(6);
}
impl windows_core::imp::TypeKind for CrossSlidingState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for CrossSlidingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.CrossSlidingState;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.CrossSlidingState");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraggingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DraggingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl DraggingEventArgs {
    pub fn DraggingState(&self) -> windows_core::Result<DraggingState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DraggingState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for DraggingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDraggingEventArgs>();
}
unsafe impl windows_core::Interface for DraggingEventArgs {
    type Vtable = <IDraggingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDraggingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DraggingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.DraggingEventArgs";
}
unsafe impl Send for DraggingEventArgs {}
unsafe impl Sync for DraggingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DraggingState(pub i32);
impl DraggingState {
    pub const Started: Self = Self(0);
    pub const Continuing: Self = Self(1);
    pub const Completed: Self = Self(2);
}
impl windows_core::imp::TypeKind for DraggingState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for DraggingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.DraggingState;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.DraggingState");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnteredMoveSizeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EnteredMoveSizeEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EnteredMoveSizeEventArgs {
    pub fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerScreenPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for EnteredMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEnteredMoveSizeEventArgs>();
}
unsafe impl windows_core::Interface for EnteredMoveSizeEventArgs {
    type Vtable = <IEnteredMoveSizeEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IEnteredMoveSizeEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EnteredMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.EnteredMoveSizeEventArgs";
}
unsafe impl Send for EnteredMoveSizeEventArgs {}
unsafe impl Sync for EnteredMoveSizeEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnteringMoveSizeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(EnteringMoveSizeEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl EnteringMoveSizeEventArgs {
    pub fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerScreenPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSizeWindowId(&self) -> windows_core::Result<super::WindowId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeWindowId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMoveSizeWindowId(&self, value: super::WindowId) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMoveSizeWindowId)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for EnteringMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IEnteringMoveSizeEventArgs>();
}
unsafe impl windows_core::Interface for EnteringMoveSizeEventArgs {
    type Vtable = <IEnteringMoveSizeEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IEnteringMoveSizeEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for EnteringMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.EnteringMoveSizeEventArgs";
}
unsafe impl Send for EnteringMoveSizeEventArgs {}
unsafe impl Sync for EnteringMoveSizeEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExitedMoveSizeEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ExitedMoveSizeEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ExitedMoveSizeEventArgs {
    pub fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerScreenPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ExitedMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IExitedMoveSizeEventArgs>();
}
unsafe impl windows_core::Interface for ExitedMoveSizeEventArgs {
    type Vtable = <IExitedMoveSizeEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IExitedMoveSizeEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ExitedMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ExitedMoveSizeEventArgs";
}
unsafe impl Send for ExitedMoveSizeEventArgs {}
unsafe impl Sync for ExitedMoveSizeEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FocusChangedEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for FocusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusChangedEventArgs>();
}
unsafe impl windows_core::Interface for FocusChangedEventArgs {
    type Vtable = <IFocusChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.FocusChangedEventArgs";
}
unsafe impl Send for FocusChangedEventArgs {}
unsafe impl Sync for FocusChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusNavigationReason(pub i32);
impl FocusNavigationReason {
    pub const Programmatic: Self = Self(0);
    pub const Restore: Self = Self(1);
    pub const First: Self = Self(2);
    pub const Last: Self = Self(3);
    pub const Left: Self = Self(4);
    pub const Up: Self = Self(5);
    pub const Right: Self = Self(6);
    pub const Down: Self = Self(7);
}
impl windows_core::imp::TypeKind for FocusNavigationReason {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusNavigationReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.FocusNavigationReason;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.FocusNavigationReason");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusNavigationRequest(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusNavigationRequest, windows_core::IUnknown, windows_core::IInspectable);
impl FocusNavigationRequest {
    pub fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CorrelationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn HintRect(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HintRect)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)).and_then(|r__: windows_reference::IReference<windows::Foundation::Rect>| r__.Value())
        }
    }
    pub fn Reason(&self) -> windows_core::Result<FocusNavigationReason> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Reason)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Create(reason: FocusNavigationReason) -> windows_core::Result<Self> {
        Self::IFocusNavigationRequestStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), reason, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWithHintRect(reason: FocusNavigationReason, hintrect: windows::Foundation::Rect) -> windows_core::Result<Self> {
        Self::IFocusNavigationRequestStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithHintRect)(windows_core::Interface::as_raw(this), reason, hintrect, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWithHintRectAndId(reason: FocusNavigationReason, hintrect: windows::Foundation::Rect, correlationid: windows_core::GUID) -> windows_core::Result<Self> {
        Self::IFocusNavigationRequestStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithHintRectAndId)(windows_core::Interface::as_raw(this), reason, hintrect, correlationid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IFocusNavigationRequestStatics<R, F: FnOnce(&IFocusNavigationRequestStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FocusNavigationRequest, IFocusNavigationRequestStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FocusNavigationRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusNavigationRequest>();
}
unsafe impl windows_core::Interface for FocusNavigationRequest {
    type Vtable = <IFocusNavigationRequest as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusNavigationRequest as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusNavigationRequest {
    const NAME: &'static str = "Microsoft.UI.Input.FocusNavigationRequest";
}
unsafe impl Send for FocusNavigationRequest {}
unsafe impl Sync for FocusNavigationRequest {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FocusNavigationRequestEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FocusNavigationRequestEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl FocusNavigationRequestEventArgs {
    pub fn Request(&self) -> windows_core::Result<FocusNavigationRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Request)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Result(&self) -> windows_core::Result<FocusNavigationResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Result)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetResult(&self, value: FocusNavigationResult) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetResult)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for FocusNavigationRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IFocusNavigationRequestEventArgs>();
}
unsafe impl windows_core::Interface for FocusNavigationRequestEventArgs {
    type Vtable = <IFocusNavigationRequestEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IFocusNavigationRequestEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FocusNavigationRequestEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.FocusNavigationRequestEventArgs";
}
unsafe impl Send for FocusNavigationRequestEventArgs {}
unsafe impl Sync for FocusNavigationRequestEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusNavigationResult(pub i32);
impl FocusNavigationResult {
    pub const NotMoved: Self = Self(0);
    pub const Moved: Self = Self(1);
    pub const NoFocusableElements: Self = Self(2);
}
impl windows_core::imp::TypeKind for FocusNavigationResult {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for FocusNavigationResult {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.FocusNavigationResult;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.FocusNavigationResult");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GestureRecognizer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GestureRecognizer, windows_core::IUnknown, windows_core::IInspectable);
impl GestureRecognizer {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<GestureRecognizer, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AutoProcessInertia(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoProcessInertia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAutoProcessInertia(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAutoProcessInertia)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CrossSlideExact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlideExact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCrossSlideExact(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCrossSlideExact)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CrossSlideHorizontally(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlideHorizontally)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCrossSlideHorizontally(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCrossSlideHorizontally)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CrossSlideThresholds(&self) -> windows_core::Result<CrossSlideThresholds> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CrossSlideThresholds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCrossSlideThresholds(&self, value: CrossSlideThresholds) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCrossSlideThresholds)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn GestureSettings(&self) -> windows_core::Result<GestureSettings> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GestureSettings)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetGestureSettings(&self, value: GestureSettings) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGestureSettings)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsActive(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInertial(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInertial)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PivotCenter(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PivotCenter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPivotCenter(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPivotCenter)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn PivotRadius(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PivotRadius)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPivotRadius(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPivotRadius)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn InertiaExpansionDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaExpansionDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaExpansionDeceleration(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInertiaExpansionDeceleration)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn InertiaExpansion(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaExpansion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaExpansion(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInertiaExpansion)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn InertiaRotationAngle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaRotationAngle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInertiaRotationAngle)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn InertiaRotationDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaRotationDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaRotationDeceleration(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInertiaRotationDeceleration)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn InertiaTranslationDeceleration(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaTranslationDeceleration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaTranslationDeceleration(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInertiaTranslationDeceleration)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn InertiaTranslationDisplacement(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InertiaTranslationDisplacement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetInertiaTranslationDisplacement(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetInertiaTranslationDisplacement)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ManipulationExact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ManipulationExact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetManipulationExact(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetManipulationExact)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn MouseWheelParameters(&self) -> windows_core::Result<MouseWheelParameters> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MouseWheelParameters)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ShowGestureFeedback(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowGestureFeedback)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowGestureFeedback(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowGestureFeedback)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn CanBeDoubleTap<P0>(&self, value: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanBeDoubleTap)(windows_core::Interface::as_raw(self), value.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn CompleteGesture(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CompleteGesture)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn ProcessDownEvent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessDownEvent)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ProcessMoveEvents<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_collections::IVector<PointerPoint>>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessMoveEvents)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn ProcessMouseWheelEvent<P0>(&self, value: P0, isshiftkeydown: bool, iscontrolkeydown: bool) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessMouseWheelEvent)(windows_core::Interface::as_raw(self), value.param().abi(), isshiftkeydown, iscontrolkeydown).ok() }
    }
    pub fn ProcessInertia(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ProcessInertia)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn ProcessUpEvent<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessUpEvent)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn Tapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<TappedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, TappedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Tapped)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveTapped))
        }
    }
    pub fn RightTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<RightTappedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, RightTappedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).RightTapped)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveRightTapped))
        }
    }
    pub fn Holding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<HoldingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, HoldingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Holding)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveHolding))
        }
    }
    pub fn Dragging<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<DraggingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, DraggingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Dragging)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDragging))
        }
    }
    pub fn ManipulationStarted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ManipulationStartedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, ManipulationStartedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationStarted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationStarted))
        }
    }
    pub fn ManipulationUpdated<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ManipulationUpdatedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, ManipulationUpdatedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationUpdated)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationUpdated))
        }
    }
    pub fn ManipulationInertiaStarting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ManipulationInertiaStartingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, ManipulationInertiaStartingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationInertiaStarting)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationInertiaStarting))
        }
    }
    pub fn ManipulationCompleted<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ManipulationCompletedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, ManipulationCompletedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).ManipulationCompleted)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveManipulationCompleted))
        }
    }
    pub fn CrossSliding<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<CrossSlidingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, CrossSlidingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CrossSliding)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCrossSliding))
        }
    }
}
impl windows_core::RuntimeType for GestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGestureRecognizer>();
}
unsafe impl windows_core::Interface for GestureRecognizer {
    type Vtable = <IGestureRecognizer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGestureRecognizer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GestureRecognizer {
    const NAME: &'static str = "Microsoft.UI.Input.GestureRecognizer";
}
unsafe impl Send for GestureRecognizer {}
unsafe impl Sync for GestureRecognizer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GestureSettings(pub u32);
impl GestureSettings {
    pub const None: Self = Self(0);
    pub const Tap: Self = Self(1);
    pub const DoubleTap: Self = Self(2);
    pub const Hold: Self = Self(4);
    pub const HoldWithMouse: Self = Self(8);
    pub const RightTap: Self = Self(16);
    pub const Drag: Self = Self(32);
    pub const ManipulationTranslateX: Self = Self(64);
    pub const ManipulationTranslateY: Self = Self(128);
    pub const ManipulationTranslateRailsX: Self = Self(256);
    pub const ManipulationTranslateRailsY: Self = Self(512);
    pub const ManipulationRotate: Self = Self(1024);
    pub const ManipulationScale: Self = Self(2048);
    pub const ManipulationTranslateInertia: Self = Self(4096);
    pub const ManipulationRotateInertia: Self = Self(8192);
    pub const ManipulationScaleInertia: Self = Self(16384);
    pub const CrossSlide: Self = Self(32768);
    pub const ManipulationMultipleFingerPanning: Self = Self(65536);
}
impl windows_core::imp::TypeKind for GestureSettings {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for GestureSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.GestureSettings;u4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.GestureSettings");
}
impl GestureSettings {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GestureSettings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GestureSettings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GestureSettings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for GestureSettings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for GestureSettings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HoldingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(HoldingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl HoldingEventArgs {
    pub fn HoldingState(&self) -> windows_core::Result<HoldingState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HoldingState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for HoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IHoldingEventArgs>();
}
unsafe impl windows_core::Interface for HoldingEventArgs {
    type Vtable = <IHoldingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IHoldingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for HoldingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.HoldingEventArgs";
}
unsafe impl Send for HoldingEventArgs {}
unsafe impl Sync for HoldingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HoldingState(pub i32);
impl HoldingState {
    pub const Started: Self = Self(0);
    pub const Completed: Self = Self(1);
    pub const Canceled: Self = Self(2);
}
impl windows_core::imp::TypeKind for HoldingState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for HoldingState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.HoldingState;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.HoldingState");
}
windows_core::imp::define_interface!(ICharacterReceivedEventArgs, ICharacterReceivedEventArgs_Vtbl, 0x36122718_9263_592b_8d87_8f86543ffc95);
impl windows_core::RuntimeType for ICharacterReceivedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.ICharacterReceivedEventArgs");
}
impl windows_core::RuntimeName for ICharacterReceivedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ICharacterReceivedEventArgs";
}
pub trait ICharacterReceivedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn KeyCode(&self) -> windows_core::Result<u32>;
    fn KeyStatus(&self) -> windows_core::Result<PhysicalKeyStatus>;
}
impl ICharacterReceivedEventArgs_Vtbl {
    pub const fn new<Identity: ICharacterReceivedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: ICharacterReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICharacterReceivedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: ICharacterReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICharacterReceivedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn KeyCode<Identity: ICharacterReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICharacterReceivedEventArgs_Impl::KeyCode(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyStatus<Identity: ICharacterReceivedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PhysicalKeyStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICharacterReceivedEventArgs_Impl::KeyStatus(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICharacterReceivedEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            KeyCode: KeyCode::<Identity, OFFSET>,
            KeyStatus: KeyStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICharacterReceivedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICharacterReceivedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyCode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhysicalKeyStatus) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContextMenuKeyEventArgs, IContextMenuKeyEventArgs_Vtbl, 0xf6025762_9426_541a_b647_037abdbecefc);
impl windows_core::RuntimeType for IContextMenuKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IContextMenuKeyEventArgs");
}
impl windows_core::RuntimeName for IContextMenuKeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IContextMenuKeyEventArgs";
}
pub trait IContextMenuKeyEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IContextMenuKeyEventArgs_Vtbl {
    pub const fn new<Identity: IContextMenuKeyEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IContextMenuKeyEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContextMenuKeyEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IContextMenuKeyEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContextMenuKeyEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IContextMenuKeyEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContextMenuKeyEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuKeyEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICrossSlidingEventArgs, ICrossSlidingEventArgs_Vtbl, 0x7679641f_ba9f_543c_a7c8_6229a98f89ef);
impl windows_core::RuntimeType for ICrossSlidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.ICrossSlidingEventArgs");
}
impl windows_core::RuntimeName for ICrossSlidingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ICrossSlidingEventArgs";
}
pub trait ICrossSlidingEventArgs_Impl: windows_core::IUnknownImpl {
    fn CrossSlidingState(&self) -> windows_core::Result<CrossSlidingState>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
}
impl ICrossSlidingEventArgs_Vtbl {
    pub const fn new<Identity: ICrossSlidingEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CrossSlidingState<Identity: ICrossSlidingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CrossSlidingState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrossSlidingEventArgs_Impl::CrossSlidingState(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: ICrossSlidingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrossSlidingEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: ICrossSlidingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICrossSlidingEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICrossSlidingEventArgs, OFFSET>(),
            CrossSlidingState: CrossSlidingState::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICrossSlidingEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICrossSlidingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CrossSlidingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CrossSlidingState) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDraggingEventArgs, IDraggingEventArgs_Vtbl, 0x3efb1b75_3d3b_550e_963d_0828ca76128a);
impl windows_core::RuntimeType for IDraggingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IDraggingEventArgs");
}
impl windows_core::RuntimeName for IDraggingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IDraggingEventArgs";
}
pub trait IDraggingEventArgs_Impl: windows_core::IUnknownImpl {
    fn DraggingState(&self) -> windows_core::Result<DraggingState>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
}
impl IDraggingEventArgs_Vtbl {
    pub const fn new<Identity: IDraggingEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DraggingState<Identity: IDraggingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut DraggingState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDraggingEventArgs_Impl::DraggingState(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IDraggingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDraggingEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IDraggingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDraggingEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDraggingEventArgs, OFFSET>(),
            DraggingState: DraggingState::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDraggingEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDraggingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DraggingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DraggingState) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnteredMoveSizeEventArgs, IEnteredMoveSizeEventArgs_Vtbl, 0x698d28fe_d325_59e0_9834_b10fc2f7ba67);
impl windows_core::RuntimeType for IEnteredMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IEnteredMoveSizeEventArgs");
}
impl windows_core::RuntimeName for IEnteredMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IEnteredMoveSizeEventArgs";
}
pub trait IEnteredMoveSizeEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32>;
    fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation>;
}
impl IEnteredMoveSizeEventArgs_Vtbl {
    pub const fn new<Identity: IEnteredMoveSizeEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerScreenPoint<Identity: IEnteredMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::PointInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnteredMoveSizeEventArgs_Impl::PointerScreenPoint(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveSizeOperation<Identity: IEnteredMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MoveSizeOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnteredMoveSizeEventArgs_Impl::MoveSizeOperation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IEnteredMoveSizeEventArgs, OFFSET>(),
            PointerScreenPoint: PointerScreenPoint::<Identity, OFFSET>,
            MoveSizeOperation: MoveSizeOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnteredMoveSizeEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnteredMoveSizeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerScreenPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::PointInt32) -> windows_core::HRESULT,
    pub MoveSizeOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MoveSizeOperation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IEnteringMoveSizeEventArgs, IEnteringMoveSizeEventArgs_Vtbl, 0x47c083b2_402b_51ec_8836_d48679fea695);
impl windows_core::RuntimeType for IEnteringMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IEnteringMoveSizeEventArgs");
}
impl windows_core::RuntimeName for IEnteringMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IEnteringMoveSizeEventArgs";
}
pub trait IEnteringMoveSizeEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32>;
    fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation>;
    fn MoveSizeWindowId(&self) -> windows_core::Result<super::WindowId>;
    fn SetMoveSizeWindowId(&self, value: &super::WindowId) -> windows_core::Result<()>;
}
impl IEnteringMoveSizeEventArgs_Vtbl {
    pub const fn new<Identity: IEnteringMoveSizeEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerScreenPoint<Identity: IEnteringMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::PointInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnteringMoveSizeEventArgs_Impl::PointerScreenPoint(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveSizeOperation<Identity: IEnteringMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MoveSizeOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnteringMoveSizeEventArgs_Impl::MoveSizeOperation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveSizeWindowId<Identity: IEnteringMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::WindowId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnteringMoveSizeEventArgs_Impl::MoveSizeWindowId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMoveSizeWindowId<Identity: IEnteringMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::WindowId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnteringMoveSizeEventArgs_Impl::SetMoveSizeWindowId(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IEnteringMoveSizeEventArgs, OFFSET>(),
            PointerScreenPoint: PointerScreenPoint::<Identity, OFFSET>,
            MoveSizeOperation: MoveSizeOperation::<Identity, OFFSET>,
            MoveSizeWindowId: MoveSizeWindowId::<Identity, OFFSET>,
            SetMoveSizeWindowId: SetMoveSizeWindowId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnteringMoveSizeEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnteringMoveSizeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerScreenPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::PointInt32) -> windows_core::HRESULT,
    pub MoveSizeOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MoveSizeOperation) -> windows_core::HRESULT,
    pub MoveSizeWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowId) -> windows_core::HRESULT,
    pub SetMoveSizeWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IExitedMoveSizeEventArgs, IExitedMoveSizeEventArgs_Vtbl, 0xdf12a46e_daee_5dac_a678_d7d5e4d0893a);
impl windows_core::RuntimeType for IExitedMoveSizeEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IExitedMoveSizeEventArgs");
}
impl windows_core::RuntimeName for IExitedMoveSizeEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IExitedMoveSizeEventArgs";
}
pub trait IExitedMoveSizeEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32>;
    fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation>;
}
impl IExitedMoveSizeEventArgs_Vtbl {
    pub const fn new<Identity: IExitedMoveSizeEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerScreenPoint<Identity: IExitedMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::PointInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExitedMoveSizeEventArgs_Impl::PointerScreenPoint(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveSizeOperation<Identity: IExitedMoveSizeEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MoveSizeOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExitedMoveSizeEventArgs_Impl::MoveSizeOperation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IExitedMoveSizeEventArgs, OFFSET>(),
            PointerScreenPoint: PointerScreenPoint::<Identity, OFFSET>,
            MoveSizeOperation: MoveSizeOperation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExitedMoveSizeEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExitedMoveSizeEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerScreenPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::PointInt32) -> windows_core::HRESULT,
    pub MoveSizeOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MoveSizeOperation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusChangedEventArgs, IFocusChangedEventArgs_Vtbl, 0xa039b115_dbdf_594c_9b86_da6aa05c9fa2);
impl windows_core::RuntimeType for IFocusChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IFocusChangedEventArgs");
}
impl windows_core::RuntimeName for IFocusChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IFocusChangedEventArgs";
}
pub trait IFocusChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IFocusChangedEventArgs_Vtbl {
    pub const fn new<Identity: IFocusChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IFocusChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusChangedEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IFocusChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFocusChangedEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusChangedEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusNavigationRequest, IFocusNavigationRequest_Vtbl, 0x6d84bb83_9c84_5112_85e9_8919acf97262);
impl windows_core::RuntimeType for IFocusNavigationRequest {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IFocusNavigationRequest");
}
impl windows_core::RuntimeName for IFocusNavigationRequest {
    const NAME: &'static str = "Microsoft.UI.Input.IFocusNavigationRequest";
}
pub trait IFocusNavigationRequest_Impl: windows_core::IUnknownImpl {
    fn CorrelationId(&self) -> windows_core::Result<windows_core::GUID>;
    fn HintRect(&self) -> windows_core::Result<windows_reference::IReference<windows::Foundation::Rect>>;
    fn Reason(&self) -> windows_core::Result<FocusNavigationReason>;
}
impl IFocusNavigationRequest_Vtbl {
    pub const fn new<Identity: IFocusNavigationRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CorrelationId<Identity: IFocusNavigationRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequest_Impl::CorrelationId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HintRect<Identity: IFocusNavigationRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequest_Impl::HintRect(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Reason<Identity: IFocusNavigationRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FocusNavigationReason) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequest_Impl::Reason(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusNavigationRequest, OFFSET>(),
            CorrelationId: CorrelationId::<Identity, OFFSET>,
            HintRect: HintRect::<Identity, OFFSET>,
            Reason: Reason::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusNavigationRequest as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusNavigationRequest_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CorrelationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub HintRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusNavigationReason) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusNavigationRequestEventArgs, IFocusNavigationRequestEventArgs_Vtbl, 0x35a63426_e271_59f9_a231_0d190314b415);
impl windows_core::RuntimeType for IFocusNavigationRequestEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IFocusNavigationRequestEventArgs");
}
impl windows_core::RuntimeName for IFocusNavigationRequestEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IFocusNavigationRequestEventArgs";
}
pub trait IFocusNavigationRequestEventArgs_Impl: windows_core::IUnknownImpl {
    fn Request(&self) -> windows_core::Result<FocusNavigationRequest>;
    fn Result(&self) -> windows_core::Result<FocusNavigationResult>;
    fn SetResult(&self, value: FocusNavigationResult) -> windows_core::Result<()>;
}
impl IFocusNavigationRequestEventArgs_Vtbl {
    pub const fn new<Identity: IFocusNavigationRequestEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Request<Identity: IFocusNavigationRequestEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequestEventArgs_Impl::Request(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Result<Identity: IFocusNavigationRequestEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut FocusNavigationResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequestEventArgs_Impl::Result(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetResult<Identity: IFocusNavigationRequestEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: FocusNavigationResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFocusNavigationRequestEventArgs_Impl::SetResult(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusNavigationRequestEventArgs, OFFSET>(),
            Request: Request::<Identity, OFFSET>,
            Result: Result::<Identity, OFFSET>,
            SetResult: SetResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusNavigationRequestEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusNavigationRequestEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Result: unsafe extern "system" fn(*mut core::ffi::c_void, *mut FocusNavigationResult) -> windows_core::HRESULT,
    pub SetResult: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationResult) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFocusNavigationRequestStatics, IFocusNavigationRequestStatics_Vtbl, 0x8c4d2ed8_3a63_519e_a827_f57e263bd1ff);
impl windows_core::RuntimeType for IFocusNavigationRequestStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IFocusNavigationRequestStatics");
}
impl windows_core::RuntimeName for IFocusNavigationRequestStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IFocusNavigationRequestStatics";
}
pub trait IFocusNavigationRequestStatics_Impl: windows_core::IUnknownImpl {
    fn Create(&self, reason: FocusNavigationReason) -> windows_core::Result<FocusNavigationRequest>;
    fn CreateWithHintRect(&self, reason: FocusNavigationReason, hintRect: &windows::Foundation::Rect) -> windows_core::Result<FocusNavigationRequest>;
    fn CreateWithHintRectAndId(&self, reason: FocusNavigationReason, hintRect: &windows::Foundation::Rect, correlationId: &windows_core::GUID) -> windows_core::Result<FocusNavigationRequest>;
}
impl IFocusNavigationRequestStatics_Vtbl {
    pub const fn new<Identity: IFocusNavigationRequestStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IFocusNavigationRequestStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: FocusNavigationReason, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequestStatics_Impl::Create(this, reason) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateWithHintRect<Identity: IFocusNavigationRequestStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: FocusNavigationReason, hintrect: windows::Foundation::Rect, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequestStatics_Impl::CreateWithHintRect(this, reason, core::mem::transmute(&hintrect)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateWithHintRectAndId<Identity: IFocusNavigationRequestStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reason: FocusNavigationReason, hintrect: windows::Foundation::Rect, correlationid: windows_core::GUID, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFocusNavigationRequestStatics_Impl::CreateWithHintRectAndId(this, reason, core::mem::transmute(&hintrect), core::mem::transmute(&correlationid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IFocusNavigationRequestStatics, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            CreateWithHintRect: CreateWithHintRect::<Identity, OFFSET>,
            CreateWithHintRectAndId: CreateWithHintRectAndId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFocusNavigationRequestStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFocusNavigationRequestStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationReason, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithHintRect: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationReason, windows::Foundation::Rect, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithHintRectAndId: unsafe extern "system" fn(*mut core::ffi::c_void, FocusNavigationReason, windows::Foundation::Rect, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGestureRecognizer, IGestureRecognizer_Vtbl, 0xcda89afc_6bd0_595c_ba37_545fce5bf016);
impl windows_core::RuntimeType for IGestureRecognizer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IGestureRecognizer");
}
impl windows_core::RuntimeName for IGestureRecognizer {
    const NAME: &'static str = "Microsoft.UI.Input.IGestureRecognizer";
}
pub trait IGestureRecognizer_Impl: windows_core::IUnknownImpl {
    fn AutoProcessInertia(&self) -> windows_core::Result<bool>;
    fn SetAutoProcessInertia(&self, value: bool) -> windows_core::Result<()>;
    fn CrossSlideExact(&self) -> windows_core::Result<bool>;
    fn SetCrossSlideExact(&self, value: bool) -> windows_core::Result<()>;
    fn CrossSlideHorizontally(&self) -> windows_core::Result<bool>;
    fn SetCrossSlideHorizontally(&self, value: bool) -> windows_core::Result<()>;
    fn CrossSlideThresholds(&self) -> windows_core::Result<CrossSlideThresholds>;
    fn SetCrossSlideThresholds(&self, value: &CrossSlideThresholds) -> windows_core::Result<()>;
    fn GestureSettings(&self) -> windows_core::Result<GestureSettings>;
    fn SetGestureSettings(&self, value: GestureSettings) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<bool>;
    fn IsInertial(&self) -> windows_core::Result<bool>;
    fn PivotCenter(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn SetPivotCenter(&self, value: &windows::Foundation::Point) -> windows_core::Result<()>;
    fn PivotRadius(&self) -> windows_core::Result<f32>;
    fn SetPivotRadius(&self, value: f32) -> windows_core::Result<()>;
    fn InertiaExpansionDeceleration(&self) -> windows_core::Result<f32>;
    fn SetInertiaExpansionDeceleration(&self, value: f32) -> windows_core::Result<()>;
    fn InertiaExpansion(&self) -> windows_core::Result<f32>;
    fn SetInertiaExpansion(&self, value: f32) -> windows_core::Result<()>;
    fn InertiaRotationAngle(&self) -> windows_core::Result<f32>;
    fn SetInertiaRotationAngle(&self, value: f32) -> windows_core::Result<()>;
    fn InertiaRotationDeceleration(&self) -> windows_core::Result<f32>;
    fn SetInertiaRotationDeceleration(&self, value: f32) -> windows_core::Result<()>;
    fn InertiaTranslationDeceleration(&self) -> windows_core::Result<f32>;
    fn SetInertiaTranslationDeceleration(&self, value: f32) -> windows_core::Result<()>;
    fn InertiaTranslationDisplacement(&self) -> windows_core::Result<f32>;
    fn SetInertiaTranslationDisplacement(&self, value: f32) -> windows_core::Result<()>;
    fn ManipulationExact(&self) -> windows_core::Result<bool>;
    fn SetManipulationExact(&self, value: bool) -> windows_core::Result<()>;
    fn MouseWheelParameters(&self) -> windows_core::Result<MouseWheelParameters>;
    fn ShowGestureFeedback(&self) -> windows_core::Result<bool>;
    fn SetShowGestureFeedback(&self, value: bool) -> windows_core::Result<()>;
    fn CanBeDoubleTap(&self, value: windows_core::Ref<PointerPoint>) -> windows_core::Result<bool>;
    fn CompleteGesture(&self) -> windows_core::Result<()>;
    fn ProcessDownEvent(&self, value: windows_core::Ref<PointerPoint>) -> windows_core::Result<()>;
    fn ProcessMoveEvents(&self, value: windows_core::Ref<windows_collections::IVector<PointerPoint>>) -> windows_core::Result<()>;
    fn ProcessMouseWheelEvent(&self, value: windows_core::Ref<PointerPoint>, isShiftKeyDown: bool, isControlKeyDown: bool) -> windows_core::Result<()>;
    fn ProcessInertia(&self) -> windows_core::Result<()>;
    fn ProcessUpEvent(&self, value: windows_core::Ref<PointerPoint>) -> windows_core::Result<()>;
    fn Tapped(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, TappedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveTapped(&self, token: i64) -> windows_core::Result<()>;
    fn RightTapped(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, RightTappedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveRightTapped(&self, token: i64) -> windows_core::Result<()>;
    fn Holding(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, HoldingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveHolding(&self, token: i64) -> windows_core::Result<()>;
    fn Dragging(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, DraggingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveDragging(&self, token: i64) -> windows_core::Result<()>;
    fn ManipulationStarted(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, ManipulationStartedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveManipulationStarted(&self, token: i64) -> windows_core::Result<()>;
    fn ManipulationUpdated(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, ManipulationUpdatedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveManipulationUpdated(&self, token: i64) -> windows_core::Result<()>;
    fn ManipulationInertiaStarting(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, ManipulationInertiaStartingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveManipulationInertiaStarting(&self, token: i64) -> windows_core::Result<()>;
    fn ManipulationCompleted(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, ManipulationCompletedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveManipulationCompleted(&self, token: i64) -> windows_core::Result<()>;
    fn CrossSliding(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<GestureRecognizer, CrossSlidingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveCrossSliding(&self, token: i64) -> windows_core::Result<()>;
}
impl IGestureRecognizer_Vtbl {
    pub const fn new<Identity: IGestureRecognizer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AutoProcessInertia<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::AutoProcessInertia(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoProcessInertia<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetAutoProcessInertia(this, value).into()
            }
        }
        unsafe extern "system" fn CrossSlideExact<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::CrossSlideExact(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCrossSlideExact<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetCrossSlideExact(this, value).into()
            }
        }
        unsafe extern "system" fn CrossSlideHorizontally<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::CrossSlideHorizontally(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCrossSlideHorizontally<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetCrossSlideHorizontally(this, value).into()
            }
        }
        unsafe extern "system" fn CrossSlideThresholds<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut CrossSlideThresholds) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::CrossSlideThresholds(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCrossSlideThresholds<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: CrossSlideThresholds) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetCrossSlideThresholds(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GestureSettings<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut GestureSettings) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::GestureSettings(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGestureSettings<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: GestureSettings) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetGestureSettings(this, value).into()
            }
        }
        unsafe extern "system" fn IsActive<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::IsActive(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInertial<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::IsInertial(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PivotCenter<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::PivotCenter(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPivotCenter<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetPivotCenter(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn PivotRadius<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::PivotRadius(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPivotRadius<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetPivotRadius(this, value).into()
            }
        }
        unsafe extern "system" fn InertiaExpansionDeceleration<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::InertiaExpansionDeceleration(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInertiaExpansionDeceleration<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetInertiaExpansionDeceleration(this, value).into()
            }
        }
        unsafe extern "system" fn InertiaExpansion<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::InertiaExpansion(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInertiaExpansion<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetInertiaExpansion(this, value).into()
            }
        }
        unsafe extern "system" fn InertiaRotationAngle<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::InertiaRotationAngle(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInertiaRotationAngle<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetInertiaRotationAngle(this, value).into()
            }
        }
        unsafe extern "system" fn InertiaRotationDeceleration<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::InertiaRotationDeceleration(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInertiaRotationDeceleration<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetInertiaRotationDeceleration(this, value).into()
            }
        }
        unsafe extern "system" fn InertiaTranslationDeceleration<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::InertiaTranslationDeceleration(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInertiaTranslationDeceleration<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetInertiaTranslationDeceleration(this, value).into()
            }
        }
        unsafe extern "system" fn InertiaTranslationDisplacement<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::InertiaTranslationDisplacement(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInertiaTranslationDisplacement<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetInertiaTranslationDisplacement(this, value).into()
            }
        }
        unsafe extern "system" fn ManipulationExact<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::ManipulationExact(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetManipulationExact<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetManipulationExact(this, value).into()
            }
        }
        unsafe extern "system" fn MouseWheelParameters<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::MouseWheelParameters(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ShowGestureFeedback<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::ShowGestureFeedback(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowGestureFeedback<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::SetShowGestureFeedback(this, value).into()
            }
        }
        unsafe extern "system" fn CanBeDoubleTap<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::CanBeDoubleTap(this, core::mem::transmute_copy(&value)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompleteGesture<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::CompleteGesture(this).into()
            }
        }
        unsafe extern "system" fn ProcessDownEvent<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::ProcessDownEvent(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ProcessMoveEvents<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::ProcessMoveEvents(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ProcessMouseWheelEvent<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, isshiftkeydown: bool, iscontrolkeydown: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::ProcessMouseWheelEvent(this, core::mem::transmute_copy(&value), isshiftkeydown, iscontrolkeydown).into()
            }
        }
        unsafe extern "system" fn ProcessInertia<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::ProcessInertia(this).into()
            }
        }
        unsafe extern "system" fn ProcessUpEvent<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::ProcessUpEvent(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Tapped<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::Tapped(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveTapped<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveTapped(this, token).into()
            }
        }
        unsafe extern "system" fn RightTapped<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::RightTapped(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveRightTapped<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveRightTapped(this, token).into()
            }
        }
        unsafe extern "system" fn Holding<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::Holding(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveHolding<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveHolding(this, token).into()
            }
        }
        unsafe extern "system" fn Dragging<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::Dragging(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveDragging<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveDragging(this, token).into()
            }
        }
        unsafe extern "system" fn ManipulationStarted<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::ManipulationStarted(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveManipulationStarted<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveManipulationStarted(this, token).into()
            }
        }
        unsafe extern "system" fn ManipulationUpdated<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::ManipulationUpdated(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveManipulationUpdated<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveManipulationUpdated(this, token).into()
            }
        }
        unsafe extern "system" fn ManipulationInertiaStarting<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::ManipulationInertiaStarting(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveManipulationInertiaStarting<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveManipulationInertiaStarting(this, token).into()
            }
        }
        unsafe extern "system" fn ManipulationCompleted<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::ManipulationCompleted(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveManipulationCompleted<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveManipulationCompleted(this, token).into()
            }
        }
        unsafe extern "system" fn CrossSliding<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGestureRecognizer_Impl::CrossSliding(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveCrossSliding<Identity: IGestureRecognizer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IGestureRecognizer_Impl::RemoveCrossSliding(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IGestureRecognizer, OFFSET>(),
            AutoProcessInertia: AutoProcessInertia::<Identity, OFFSET>,
            SetAutoProcessInertia: SetAutoProcessInertia::<Identity, OFFSET>,
            CrossSlideExact: CrossSlideExact::<Identity, OFFSET>,
            SetCrossSlideExact: SetCrossSlideExact::<Identity, OFFSET>,
            CrossSlideHorizontally: CrossSlideHorizontally::<Identity, OFFSET>,
            SetCrossSlideHorizontally: SetCrossSlideHorizontally::<Identity, OFFSET>,
            CrossSlideThresholds: CrossSlideThresholds::<Identity, OFFSET>,
            SetCrossSlideThresholds: SetCrossSlideThresholds::<Identity, OFFSET>,
            GestureSettings: GestureSettings::<Identity, OFFSET>,
            SetGestureSettings: SetGestureSettings::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
            IsInertial: IsInertial::<Identity, OFFSET>,
            PivotCenter: PivotCenter::<Identity, OFFSET>,
            SetPivotCenter: SetPivotCenter::<Identity, OFFSET>,
            PivotRadius: PivotRadius::<Identity, OFFSET>,
            SetPivotRadius: SetPivotRadius::<Identity, OFFSET>,
            InertiaExpansionDeceleration: InertiaExpansionDeceleration::<Identity, OFFSET>,
            SetInertiaExpansionDeceleration: SetInertiaExpansionDeceleration::<Identity, OFFSET>,
            InertiaExpansion: InertiaExpansion::<Identity, OFFSET>,
            SetInertiaExpansion: SetInertiaExpansion::<Identity, OFFSET>,
            InertiaRotationAngle: InertiaRotationAngle::<Identity, OFFSET>,
            SetInertiaRotationAngle: SetInertiaRotationAngle::<Identity, OFFSET>,
            InertiaRotationDeceleration: InertiaRotationDeceleration::<Identity, OFFSET>,
            SetInertiaRotationDeceleration: SetInertiaRotationDeceleration::<Identity, OFFSET>,
            InertiaTranslationDeceleration: InertiaTranslationDeceleration::<Identity, OFFSET>,
            SetInertiaTranslationDeceleration: SetInertiaTranslationDeceleration::<Identity, OFFSET>,
            InertiaTranslationDisplacement: InertiaTranslationDisplacement::<Identity, OFFSET>,
            SetInertiaTranslationDisplacement: SetInertiaTranslationDisplacement::<Identity, OFFSET>,
            ManipulationExact: ManipulationExact::<Identity, OFFSET>,
            SetManipulationExact: SetManipulationExact::<Identity, OFFSET>,
            MouseWheelParameters: MouseWheelParameters::<Identity, OFFSET>,
            ShowGestureFeedback: ShowGestureFeedback::<Identity, OFFSET>,
            SetShowGestureFeedback: SetShowGestureFeedback::<Identity, OFFSET>,
            CanBeDoubleTap: CanBeDoubleTap::<Identity, OFFSET>,
            CompleteGesture: CompleteGesture::<Identity, OFFSET>,
            ProcessDownEvent: ProcessDownEvent::<Identity, OFFSET>,
            ProcessMoveEvents: ProcessMoveEvents::<Identity, OFFSET>,
            ProcessMouseWheelEvent: ProcessMouseWheelEvent::<Identity, OFFSET>,
            ProcessInertia: ProcessInertia::<Identity, OFFSET>,
            ProcessUpEvent: ProcessUpEvent::<Identity, OFFSET>,
            Tapped: Tapped::<Identity, OFFSET>,
            RemoveTapped: RemoveTapped::<Identity, OFFSET>,
            RightTapped: RightTapped::<Identity, OFFSET>,
            RemoveRightTapped: RemoveRightTapped::<Identity, OFFSET>,
            Holding: Holding::<Identity, OFFSET>,
            RemoveHolding: RemoveHolding::<Identity, OFFSET>,
            Dragging: Dragging::<Identity, OFFSET>,
            RemoveDragging: RemoveDragging::<Identity, OFFSET>,
            ManipulationStarted: ManipulationStarted::<Identity, OFFSET>,
            RemoveManipulationStarted: RemoveManipulationStarted::<Identity, OFFSET>,
            ManipulationUpdated: ManipulationUpdated::<Identity, OFFSET>,
            RemoveManipulationUpdated: RemoveManipulationUpdated::<Identity, OFFSET>,
            ManipulationInertiaStarting: ManipulationInertiaStarting::<Identity, OFFSET>,
            RemoveManipulationInertiaStarting: RemoveManipulationInertiaStarting::<Identity, OFFSET>,
            ManipulationCompleted: ManipulationCompleted::<Identity, OFFSET>,
            RemoveManipulationCompleted: RemoveManipulationCompleted::<Identity, OFFSET>,
            CrossSliding: CrossSliding::<Identity, OFFSET>,
            RemoveCrossSliding: RemoveCrossSliding::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGestureRecognizer as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AutoProcessInertia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAutoProcessInertia: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideExact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCrossSlideExact: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideHorizontally: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetCrossSlideHorizontally: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CrossSlideThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CrossSlideThresholds) -> windows_core::HRESULT,
    pub SetCrossSlideThresholds: unsafe extern "system" fn(*mut core::ffi::c_void, CrossSlideThresholds) -> windows_core::HRESULT,
    pub GestureSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut GestureSettings) -> windows_core::HRESULT,
    pub SetGestureSettings: unsafe extern "system" fn(*mut core::ffi::c_void, GestureSettings) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInertial: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PivotCenter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetPivotCenter: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
    pub PivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetPivotRadius: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaExpansionDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaExpansion: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaRotationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaRotationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaTranslationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaTranslationDeceleration: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub InertiaTranslationDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetInertiaTranslationDisplacement: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub ManipulationExact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetManipulationExact: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub MouseWheelParameters: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ShowGestureFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowGestureFeedback: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub CanBeDoubleTap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CompleteGesture: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessDownEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessMoveEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessMouseWheelEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, bool) -> windows_core::HRESULT,
    pub ProcessInertia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessUpEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Tapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub RightTapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveRightTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Holding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveHolding: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Dragging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDragging: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveManipulationStarted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ManipulationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveManipulationUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ManipulationInertiaStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveManipulationInertiaStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveManipulationCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub CrossSliding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCrossSliding: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IHoldingEventArgs, IHoldingEventArgs_Vtbl, 0x8e449e85_d223_533c_b0b2_bf7c6d10c2db);
impl windows_core::RuntimeType for IHoldingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IHoldingEventArgs");
}
impl windows_core::RuntimeName for IHoldingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IHoldingEventArgs";
}
pub trait IHoldingEventArgs_Impl: windows_core::IUnknownImpl {
    fn HoldingState(&self) -> windows_core::Result<HoldingState>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
}
impl IHoldingEventArgs_Vtbl {
    pub const fn new<Identity: IHoldingEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HoldingState<Identity: IHoldingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut HoldingState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHoldingEventArgs_Impl::HoldingState(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IHoldingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHoldingEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IHoldingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IHoldingEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IHoldingEventArgs, OFFSET>(),
            HoldingState: HoldingState::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IHoldingEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHoldingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HoldingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut HoldingState) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputActivationListener, IInputActivationListener_Vtbl, 0x3b818627_6ce7_5e0d_a0f5_6684fd1aec78);
impl windows_core::RuntimeType for IInputActivationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputActivationListener");
}
impl windows_core::RuntimeName for IInputActivationListener {
    const NAME: &'static str = "Microsoft.UI.Input.IInputActivationListener";
}
pub trait IInputActivationListener_Impl: windows_core::IUnknownImpl {
    fn State(&self) -> windows_core::Result<InputActivationState>;
    fn InputActivationChanged(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputActivationListener, InputActivationListenerActivationChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveInputActivationChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputActivationListener_Vtbl {
    pub const fn new<Identity: IInputActivationListener_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn State<Identity: IInputActivationListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputActivationState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputActivationListener_Impl::State(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InputActivationChanged<Identity: IInputActivationListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputActivationListener_Impl::InputActivationChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveInputActivationChanged<Identity: IInputActivationListener_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputActivationListener_Impl::RemoveInputActivationChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputActivationListener, OFFSET>(),
            State: State::<Identity, OFFSET>,
            InputActivationChanged: InputActivationChanged::<Identity, OFFSET>,
            RemoveInputActivationChanged: RemoveInputActivationChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputActivationListener as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListener_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputActivationState) -> windows_core::HRESULT,
    pub InputActivationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveInputActivationChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputActivationListenerActivationChangedEventArgs, IInputActivationListenerActivationChangedEventArgs_Vtbl, 0x7978526b_00b6_5303_8f7d_55bef36da786);
impl windows_core::RuntimeType for IInputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputActivationListenerActivationChangedEventArgs");
}
impl windows_core::RuntimeName for IInputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IInputActivationListenerActivationChangedEventArgs";
}
pub trait IInputActivationListenerActivationChangedEventArgs_Impl: windows_core::IUnknownImpl {}
impl IInputActivationListenerActivationChangedEventArgs_Vtbl {
    pub const fn new<Identity: IInputActivationListenerActivationChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputActivationListenerActivationChangedEventArgs, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputActivationListenerActivationChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerActivationChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputActivationListenerStatics, IInputActivationListenerStatics_Vtbl, 0xc4249843_f053_5c99_9d51_720ade94224d);
impl windows_core::RuntimeType for IInputActivationListenerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputActivationListenerStatics");
}
impl windows_core::RuntimeName for IInputActivationListenerStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputActivationListenerStatics";
}
pub trait IInputActivationListenerStatics_Impl: windows_core::IUnknownImpl {
    fn GetForWindowId(&self, windowId: &super::WindowId) -> windows_core::Result<InputActivationListener>;
}
impl IInputActivationListenerStatics_Vtbl {
    pub const fn new<Identity: IInputActivationListenerStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindowId<Identity: IInputActivationListenerStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowid: super::WindowId, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputActivationListenerStatics_Impl::GetForWindowId(this, core::mem::transmute(&windowid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputActivationListenerStatics, OFFSET>(),
            GetForWindowId: GetForWindowId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputActivationListenerStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputActivationListenerStatics2, IInputActivationListenerStatics2_Vtbl, 0x7ea26120_9636_5292_a7b1_56544ac51a22);
impl windows_core::RuntimeType for IInputActivationListenerStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputActivationListenerStatics2");
}
impl windows_core::RuntimeName for IInputActivationListenerStatics2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputActivationListenerStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputActivationListenerStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputCursor, IInputCursor_Vtbl, 0x359b15f9_19c2_5714_8432_75176826406b);
impl windows_core::RuntimeType for IInputCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputCursor");
}
impl windows_core::RuntimeName for IInputCursor {
    const NAME: &'static str = "Microsoft.UI.Input.IInputCursor";
}
pub trait IInputCursor_Impl: windows_core::IUnknownImpl {}
impl IInputCursor_Vtbl {
    pub const fn new<Identity: IInputCursor_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputCursor, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputCursor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputCursorFactory, IInputCursorFactory_Vtbl, 0x2f47647b_4be0_53e9_be7e_c38d5459db6b);
impl windows_core::RuntimeType for IInputCursorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputCursorFactory");
}
impl windows_core::RuntimeName for IInputCursorFactory {
    const NAME: &'static str = "Microsoft.UI.Input.IInputCursorFactory";
}
pub trait IInputCursorFactory_Impl: windows_core::IUnknownImpl {}
impl IInputCursorFactory_Vtbl {
    pub const fn new<Identity: IInputCursorFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputCursorFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputCursorFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputCursorStatics, IInputCursorStatics_Vtbl, 0x92f6a552_099f_55fb_8c31_e450284c9643);
impl windows_core::RuntimeType for IInputCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputCursorStatics");
}
impl windows_core::RuntimeName for IInputCursorStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputCursorStatics";
}
pub trait IInputCursorStatics_Impl: windows_core::IUnknownImpl {
    fn CreateFromCoreCursor(&self, cursor: windows_core::Ref<windows::UI::Core::CoreCursor>) -> windows_core::Result<InputCursor>;
}
impl IInputCursorStatics_Vtbl {
    pub const fn new<Identity: IInputCursorStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateFromCoreCursor<Identity: IInputCursorStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cursor: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputCursorStatics_Impl::CreateFromCoreCursor(this, core::mem::transmute_copy(&cursor)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputCursorStatics, OFFSET>(),
            CreateFromCoreCursor: CreateFromCoreCursor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputCursorStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromCoreCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputCustomCursor, IInputCustomCursor_Vtbl, 0x5486f042_7e1a_5dc8_8041_e47b609a5ba1);
impl windows_core::RuntimeType for IInputCustomCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputCustomCursor");
}
impl windows_core::RuntimeName for IInputCustomCursor {
    const NAME: &'static str = "Microsoft.UI.Input.IInputCustomCursor";
}
pub trait IInputCustomCursor_Impl: windows_core::IUnknownImpl {}
impl IInputCustomCursor_Vtbl {
    pub const fn new<Identity: IInputCustomCursor_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputCustomCursor, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputCustomCursor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCustomCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputCustomCursorFactory, IInputCustomCursorFactory_Vtbl, 0x6f402882_66e0_57d3_89d0_aa5e2ff917bc);
impl windows_core::RuntimeType for IInputCustomCursorFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputCustomCursorFactory");
}
impl windows_core::RuntimeName for IInputCustomCursorFactory {
    const NAME: &'static str = "Microsoft.UI.Input.IInputCustomCursorFactory";
}
pub trait IInputCustomCursorFactory_Impl: windows_core::IUnknownImpl {}
impl IInputCustomCursorFactory_Vtbl {
    pub const fn new<Identity: IInputCustomCursorFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputCustomCursorFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputCustomCursorFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputCustomCursorFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputDesktopNamedResourceCursor, IInputDesktopNamedResourceCursor_Vtbl, 0xf40ea93b_0ed7_5b3a_bfe2_14e2b5ad88a3);
impl windows_core::RuntimeType for IInputDesktopNamedResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputDesktopNamedResourceCursor");
}
impl windows_core::RuntimeName for IInputDesktopNamedResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.IInputDesktopNamedResourceCursor";
}
pub trait IInputDesktopNamedResourceCursor_Impl: windows_core::IUnknownImpl {
    fn ModuleName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ResourceName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IInputDesktopNamedResourceCursor_Vtbl {
    pub const fn new<Identity: IInputDesktopNamedResourceCursor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleName<Identity: IInputDesktopNamedResourceCursor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopNamedResourceCursor_Impl::ModuleName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResourceName<Identity: IInputDesktopNamedResourceCursor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopNamedResourceCursor_Impl::ResourceName(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputDesktopNamedResourceCursor, OFFSET>(),
            ModuleName: ModuleName::<Identity, OFFSET>,
            ResourceName: ResourceName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputDesktopNamedResourceCursor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopNamedResourceCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ModuleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputDesktopNamedResourceCursorStatics, IInputDesktopNamedResourceCursorStatics_Vtbl, 0xe8b6d5aa_898b_5e69_b01f_383a0943e3e4);
impl windows_core::RuntimeType for IInputDesktopNamedResourceCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputDesktopNamedResourceCursorStatics");
}
impl windows_core::RuntimeName for IInputDesktopNamedResourceCursorStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputDesktopNamedResourceCursorStatics";
}
pub trait IInputDesktopNamedResourceCursorStatics_Impl: windows_core::IUnknownImpl {
    fn Create(&self, resourceName: &windows_core::HSTRING) -> windows_core::Result<InputDesktopNamedResourceCursor>;
    fn CreateFromModule(&self, moduleName: &windows_core::HSTRING, resourceName: &windows_core::HSTRING) -> windows_core::Result<InputDesktopNamedResourceCursor>;
}
impl IInputDesktopNamedResourceCursorStatics_Vtbl {
    pub const fn new<Identity: IInputDesktopNamedResourceCursorStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IInputDesktopNamedResourceCursorStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcename: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopNamedResourceCursorStatics_Impl::Create(this, core::mem::transmute(&resourcename)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromModule<Identity: IInputDesktopNamedResourceCursorStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modulename: *mut core::ffi::c_void, resourcename: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopNamedResourceCursorStatics_Impl::CreateFromModule(this, core::mem::transmute(&modulename), core::mem::transmute(&resourcename)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputDesktopNamedResourceCursorStatics, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            CreateFromModule: CreateFromModule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputDesktopNamedResourceCursorStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopNamedResourceCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputDesktopResourceCursor, IInputDesktopResourceCursor_Vtbl, 0x1df2777f_7c90_58fc_a7a3_d5736c6510fd);
impl windows_core::RuntimeType for IInputDesktopResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputDesktopResourceCursor");
}
impl windows_core::RuntimeName for IInputDesktopResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.IInputDesktopResourceCursor";
}
pub trait IInputDesktopResourceCursor_Impl: windows_core::IUnknownImpl {
    fn ModuleName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ResourceId(&self) -> windows_core::Result<u32>;
}
impl IInputDesktopResourceCursor_Vtbl {
    pub const fn new<Identity: IInputDesktopResourceCursor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ModuleName<Identity: IInputDesktopResourceCursor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopResourceCursor_Impl::ModuleName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResourceId<Identity: IInputDesktopResourceCursor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopResourceCursor_Impl::ResourceId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputDesktopResourceCursor, OFFSET>(),
            ModuleName: ModuleName::<Identity, OFFSET>,
            ResourceId: ResourceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputDesktopResourceCursor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ModuleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ResourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputDesktopResourceCursorStatics, IInputDesktopResourceCursorStatics_Vtbl, 0xf440dc37_a0b6_56eb_bcec_b024f2233d47);
impl windows_core::RuntimeType for IInputDesktopResourceCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputDesktopResourceCursorStatics");
}
impl windows_core::RuntimeName for IInputDesktopResourceCursorStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputDesktopResourceCursorStatics";
}
pub trait IInputDesktopResourceCursorStatics_Impl: windows_core::IUnknownImpl {
    fn Create(&self, resourceId: u32) -> windows_core::Result<InputDesktopResourceCursor>;
    fn CreateFromModule(&self, moduleName: &windows_core::HSTRING, resourceId: u32) -> windows_core::Result<InputDesktopResourceCursor>;
}
impl IInputDesktopResourceCursorStatics_Vtbl {
    pub const fn new<Identity: IInputDesktopResourceCursorStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IInputDesktopResourceCursorStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourceid: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopResourceCursorStatics_Impl::Create(this, resourceid) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFromModule<Identity: IInputDesktopResourceCursorStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, modulename: *mut core::ffi::c_void, resourceid: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputDesktopResourceCursorStatics_Impl::CreateFromModule(this, core::mem::transmute(&modulename), resourceid) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputDesktopResourceCursorStatics, OFFSET>(),
            Create: Create::<Identity, OFFSET>,
            CreateFromModule: CreateFromModule::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputDesktopResourceCursorStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputDesktopResourceCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromModule: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputFocusController, IInputFocusController_Vtbl, 0x8dfdc26c_8b8d_515d_8ddd_4685b3a540e9);
impl windows_core::RuntimeType for IInputFocusController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputFocusController");
}
impl windows_core::RuntimeName for IInputFocusController {
    const NAME: &'static str = "Microsoft.UI.Input.IInputFocusController";
}
pub trait IInputFocusController_Impl: windows_core::IUnknownImpl {
    fn HasFocus(&self) -> windows_core::Result<bool>;
    fn TrySetFocus(&self) -> windows_core::Result<bool>;
    fn GotFocus(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputFocusController, FocusChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveGotFocus(&self, token: i64) -> windows_core::Result<()>;
    fn LostFocus(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputFocusController, FocusChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveLostFocus(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputFocusController_Vtbl {
    pub const fn new<Identity: IInputFocusController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HasFocus<Identity: IInputFocusController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusController_Impl::HasFocus(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrySetFocus<Identity: IInputFocusController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusController_Impl::TrySetFocus(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GotFocus<Identity: IInputFocusController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusController_Impl::GotFocus(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveGotFocus<Identity: IInputFocusController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputFocusController_Impl::RemoveGotFocus(this, token).into()
            }
        }
        unsafe extern "system" fn LostFocus<Identity: IInputFocusController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusController_Impl::LostFocus(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveLostFocus<Identity: IInputFocusController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputFocusController_Impl::RemoveLostFocus(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputFocusController, OFFSET>(),
            HasFocus: HasFocus::<Identity, OFFSET>,
            TrySetFocus: TrySetFocus::<Identity, OFFSET>,
            GotFocus: GotFocus::<Identity, OFFSET>,
            RemoveGotFocus: RemoveGotFocus::<Identity, OFFSET>,
            LostFocus: LostFocus::<Identity, OFFSET>,
            RemoveLostFocus: RemoveLostFocus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputFocusController as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TrySetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveGotFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub LostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLostFocus: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputFocusController2, IInputFocusController2_Vtbl, 0x5165077c_cd4b_501d_b386_b50682360185);
impl windows_core::RuntimeType for IInputFocusController2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputFocusController2");
}
impl windows_core::RuntimeName for IInputFocusController2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputFocusController2";
}
pub trait IInputFocusController2_Impl: windows_core::IUnknownImpl {
    fn DepartFocus(&self, request: windows_core::Ref<FocusNavigationRequest>) -> windows_core::Result<FocusNavigationResult>;
    fn NavigateFocusRequested(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputFocusController, FocusNavigationRequestEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveNavigateFocusRequested(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputFocusController2_Vtbl {
    pub const fn new<Identity: IInputFocusController2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DepartFocus<Identity: IInputFocusController2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *mut core::ffi::c_void, result__: *mut FocusNavigationResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusController2_Impl::DepartFocus(this, core::mem::transmute_copy(&request)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NavigateFocusRequested<Identity: IInputFocusController2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusController2_Impl::NavigateFocusRequested(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveNavigateFocusRequested<Identity: IInputFocusController2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputFocusController2_Impl::RemoveNavigateFocusRequested(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputFocusController2, OFFSET>(),
            DepartFocus: DepartFocus::<Identity, OFFSET>,
            NavigateFocusRequested: NavigateFocusRequested::<Identity, OFFSET>,
            RemoveNavigateFocusRequested: RemoveNavigateFocusRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputFocusController2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusController2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub DepartFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FocusNavigationResult) -> windows_core::HRESULT,
    pub NavigateFocusRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveNavigateFocusRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputFocusController3, IInputFocusController3_Vtbl, 0xe8e82853_46ce_53b5_9029_a74dabea9d32);
impl windows_core::RuntimeType for IInputFocusController3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputFocusController3");
}
impl windows_core::RuntimeName for IInputFocusController3 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputFocusController3";
}
pub trait IInputFocusController3_Impl: windows_core::IUnknownImpl {
    fn ShouldShowKeyboardCues(&self) -> windows_core::Result<bool>;
}
impl IInputFocusController3_Vtbl {
    pub const fn new<Identity: IInputFocusController3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShouldShowKeyboardCues<Identity: IInputFocusController3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusController3_Impl::ShouldShowKeyboardCues(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputFocusController3, OFFSET>(),
            ShouldShowKeyboardCues: ShouldShowKeyboardCues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputFocusController3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusController3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ShouldShowKeyboardCues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputFocusControllerStatics, IInputFocusControllerStatics_Vtbl, 0xaeb311da_da9b_5a1b_92f4_83ddde933e00);
impl windows_core::RuntimeType for IInputFocusControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputFocusControllerStatics");
}
impl windows_core::RuntimeName for IInputFocusControllerStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputFocusControllerStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputFocusNavigationHost, IInputFocusNavigationHost_Vtbl, 0x53c2a147_932c_5486_a9c6_f6c5a9c65956);
impl windows_core::RuntimeType for IInputFocusNavigationHost {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputFocusNavigationHost");
}
impl windows_core::RuntimeName for IInputFocusNavigationHost {
    const NAME: &'static str = "Microsoft.UI.Input.IInputFocusNavigationHost";
}
pub trait IInputFocusNavigationHost_Impl: windows_core::IUnknownImpl {
    fn ContainsFocus(&self) -> windows_core::Result<bool>;
    fn NavigateFocus(&self, request: windows_core::Ref<FocusNavigationRequest>) -> windows_core::Result<FocusNavigationResult>;
    fn DepartFocusRequested(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputFocusNavigationHost, FocusNavigationRequestEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveDepartFocusRequested(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputFocusNavigationHost_Vtbl {
    pub const fn new<Identity: IInputFocusNavigationHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContainsFocus<Identity: IInputFocusNavigationHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusNavigationHost_Impl::ContainsFocus(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NavigateFocus<Identity: IInputFocusNavigationHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, request: *mut core::ffi::c_void, result__: *mut FocusNavigationResult) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusNavigationHost_Impl::NavigateFocus(this, core::mem::transmute_copy(&request)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DepartFocusRequested<Identity: IInputFocusNavigationHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputFocusNavigationHost_Impl::DepartFocusRequested(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveDepartFocusRequested<Identity: IInputFocusNavigationHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputFocusNavigationHost_Impl::RemoveDepartFocusRequested(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputFocusNavigationHost, OFFSET>(),
            ContainsFocus: ContainsFocus::<Identity, OFFSET>,
            NavigateFocus: NavigateFocus::<Identity, OFFSET>,
            DepartFocusRequested: DepartFocusRequested::<Identity, OFFSET>,
            RemoveDepartFocusRequested: RemoveDepartFocusRequested::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputFocusNavigationHost as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusNavigationHost_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContainsFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub NavigateFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut FocusNavigationResult) -> windows_core::HRESULT,
    pub DepartFocusRequested: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDepartFocusRequested: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputFocusNavigationHostStatics, IInputFocusNavigationHostStatics_Vtbl, 0xc9c62cd1_73db_5aa9_b89d_143509db8f37);
impl windows_core::RuntimeType for IInputFocusNavigationHostStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputFocusNavigationHostStatics");
}
impl windows_core::RuntimeName for IInputFocusNavigationHostStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputFocusNavigationHostStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusNavigationHostStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputFocusNavigationHostStatics2, IInputFocusNavigationHostStatics2_Vtbl, 0x82505f60_ef7b_55d8_8362_8cc2840266a1);
impl windows_core::RuntimeType for IInputFocusNavigationHostStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputFocusNavigationHostStatics2");
}
impl windows_core::RuntimeName for IInputFocusNavigationHostStatics2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputFocusNavigationHostStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputFocusNavigationHostStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputKeyboardSource, IInputKeyboardSource_Vtbl, 0xed61b906_16ad_5df7_a550_5e6f7d2229f7);
impl windows_core::RuntimeType for IInputKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputKeyboardSource");
}
impl windows_core::RuntimeName for IInputKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.IInputKeyboardSource";
}
pub trait IInputKeyboardSource_Impl: windows_core::IUnknownImpl {}
impl IInputKeyboardSource_Vtbl {
    pub const fn new<Identity: IInputKeyboardSource_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputKeyboardSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputKeyboardSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputKeyboardSource2, IInputKeyboardSource2_Vtbl, 0x79d1c9b6_b3c9_5ec2_8a5b_707088787f78);
impl windows_core::RuntimeType for IInputKeyboardSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputKeyboardSource2");
}
impl windows_core::RuntimeName for IInputKeyboardSource2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputKeyboardSource2";
}
pub trait IInputKeyboardSource2_Impl: windows_core::IUnknownImpl {
    fn GetCurrentKeyState(&self, virtualKey: windows::System::VirtualKey) -> windows_core::Result<VirtualKeyStates>;
    fn GetKeyState(&self, virtualKey: windows::System::VirtualKey) -> windows_core::Result<VirtualKeyStates>;
    fn CharacterReceived(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputKeyboardSource, CharacterReceivedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveCharacterReceived(&self, token: i64) -> windows_core::Result<()>;
    fn ContextMenuKey(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputKeyboardSource, ContextMenuKeyEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveContextMenuKey(&self, token: i64) -> windows_core::Result<()>;
    fn KeyDown(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveKeyDown(&self, token: i64) -> windows_core::Result<()>;
    fn KeyUp(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveKeyUp(&self, token: i64) -> windows_core::Result<()>;
    fn SystemKeyDown(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveSystemKeyDown(&self, token: i64) -> windows_core::Result<()>;
    fn SystemKeyUp(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputKeyboardSource, KeyEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveSystemKeyUp(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputKeyboardSource2_Vtbl {
    pub const fn new<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentKeyState<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualkey: windows::System::VirtualKey, result__: *mut VirtualKeyStates) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::GetCurrentKeyState(this, virtualkey) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetKeyState<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualkey: windows::System::VirtualKey, result__: *mut VirtualKeyStates) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::GetKeyState(this, virtualkey) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CharacterReceived<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::CharacterReceived(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveCharacterReceived<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputKeyboardSource2_Impl::RemoveCharacterReceived(this, token).into()
            }
        }
        unsafe extern "system" fn ContextMenuKey<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::ContextMenuKey(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveContextMenuKey<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputKeyboardSource2_Impl::RemoveContextMenuKey(this, token).into()
            }
        }
        unsafe extern "system" fn KeyDown<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::KeyDown(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveKeyDown<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputKeyboardSource2_Impl::RemoveKeyDown(this, token).into()
            }
        }
        unsafe extern "system" fn KeyUp<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::KeyUp(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveKeyUp<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputKeyboardSource2_Impl::RemoveKeyUp(this, token).into()
            }
        }
        unsafe extern "system" fn SystemKeyDown<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::SystemKeyDown(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSystemKeyDown<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputKeyboardSource2_Impl::RemoveSystemKeyDown(this, token).into()
            }
        }
        unsafe extern "system" fn SystemKeyUp<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSource2_Impl::SystemKeyUp(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSystemKeyUp<Identity: IInputKeyboardSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputKeyboardSource2_Impl::RemoveSystemKeyUp(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputKeyboardSource2, OFFSET>(),
            GetCurrentKeyState: GetCurrentKeyState::<Identity, OFFSET>,
            GetKeyState: GetKeyState::<Identity, OFFSET>,
            CharacterReceived: CharacterReceived::<Identity, OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Identity, OFFSET>,
            ContextMenuKey: ContextMenuKey::<Identity, OFFSET>,
            RemoveContextMenuKey: RemoveContextMenuKey::<Identity, OFFSET>,
            KeyDown: KeyDown::<Identity, OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Identity, OFFSET>,
            KeyUp: KeyUp::<Identity, OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Identity, OFFSET>,
            SystemKeyDown: SystemKeyDown::<Identity, OFFSET>,
            RemoveSystemKeyDown: RemoveSystemKeyDown::<Identity, OFFSET>,
            SystemKeyUp: SystemKeyUp::<Identity, OFFSET>,
            RemoveSystemKeyUp: RemoveSystemKeyUp::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputKeyboardSource2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentKeyState: unsafe extern "system" fn(*mut core::ffi::c_void, windows::System::VirtualKey, *mut VirtualKeyStates) -> windows_core::HRESULT,
    pub GetKeyState: unsafe extern "system" fn(*mut core::ffi::c_void, windows::System::VirtualKey, *mut VirtualKeyStates) -> windows_core::HRESULT,
    pub CharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCharacterReceived: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ContextMenuKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveContextMenuKey: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub KeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub KeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SystemKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSystemKeyDown: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub SystemKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSystemKeyUp: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputKeyboardSourceStatics, IInputKeyboardSourceStatics_Vtbl, 0xf4e1563d_8c2e_5bcd_b784_47adeaa3cd7e);
impl windows_core::RuntimeType for IInputKeyboardSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputKeyboardSourceStatics");
}
impl windows_core::RuntimeName for IInputKeyboardSourceStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputKeyboardSourceStatics";
}
pub trait IInputKeyboardSourceStatics_Impl: windows_core::IUnknownImpl {
    fn GetKeyStateForCurrentThread(&self, virtualKey: windows::System::VirtualKey) -> windows_core::Result<windows::UI::Core::CoreVirtualKeyStates>;
}
impl IInputKeyboardSourceStatics_Vtbl {
    pub const fn new<Identity: IInputKeyboardSourceStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetKeyStateForCurrentThread<Identity: IInputKeyboardSourceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, virtualkey: windows::System::VirtualKey, result__: *mut windows::UI::Core::CoreVirtualKeyStates) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSourceStatics_Impl::GetKeyStateForCurrentThread(this, virtualkey) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputKeyboardSourceStatics, OFFSET>(),
            GetKeyStateForCurrentThread: GetKeyStateForCurrentThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputKeyboardSourceStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetKeyStateForCurrentThread: unsafe extern "system" fn(*mut core::ffi::c_void, windows::System::VirtualKey, *mut windows::UI::Core::CoreVirtualKeyStates) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputKeyboardSourceStatics2, IInputKeyboardSourceStatics2_Vtbl, 0x8857518c_2899_5f11_9b64_0ad83234824b);
impl windows_core::RuntimeType for IInputKeyboardSourceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputKeyboardSourceStatics2");
}
impl windows_core::RuntimeName for IInputKeyboardSourceStatics2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputKeyboardSourceStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSourceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputKeyboardSourceStatics3, IInputKeyboardSourceStatics3_Vtbl, 0x34b960c5_a5ae_52af_8566_6d2d55ff52d1);
impl windows_core::RuntimeType for IInputKeyboardSourceStatics3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputKeyboardSourceStatics3");
}
impl windows_core::RuntimeName for IInputKeyboardSourceStatics3 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputKeyboardSourceStatics3";
}
pub trait IInputKeyboardSourceStatics3_Impl: windows_core::IUnknownImpl {
    fn GetForWindowId(&self, windowId: &super::WindowId) -> windows_core::Result<InputKeyboardSource>;
}
impl IInputKeyboardSourceStatics3_Vtbl {
    pub const fn new<Identity: IInputKeyboardSourceStatics3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindowId<Identity: IInputKeyboardSourceStatics3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowid: super::WindowId, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputKeyboardSourceStatics3_Impl::GetForWindowId(this, core::mem::transmute(&windowid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputKeyboardSourceStatics3, OFFSET>(),
            GetForWindowId: GetForWindowId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputKeyboardSourceStatics3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputKeyboardSourceStatics3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputLightDismissAction, IInputLightDismissAction_Vtbl, 0xe8a39502_a860_502f_8c10_3646d43aecf1);
impl windows_core::RuntimeType for IInputLightDismissAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputLightDismissAction");
}
impl windows_core::RuntimeName for IInputLightDismissAction {
    const NAME: &'static str = "Microsoft.UI.Input.IInputLightDismissAction";
}
pub trait IInputLightDismissAction_Impl: windows_core::IUnknownImpl {
    fn Dismissed(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputLightDismissAction, InputLightDismissEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveDismissed(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputLightDismissAction_Vtbl {
    pub const fn new<Identity: IInputLightDismissAction_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Dismissed<Identity: IInputLightDismissAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputLightDismissAction_Impl::Dismissed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveDismissed<Identity: IInputLightDismissAction_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputLightDismissAction_Impl::RemoveDismissed(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputLightDismissAction, OFFSET>(),
            Dismissed: Dismissed::<Identity, OFFSET>,
            RemoveDismissed: RemoveDismissed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputLightDismissAction as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissAction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Dismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputLightDismissActionStatics, IInputLightDismissActionStatics_Vtbl, 0xed9b8def_6496_5169_984d_d44b4e690623);
impl windows_core::RuntimeType for IInputLightDismissActionStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputLightDismissActionStatics");
}
impl windows_core::RuntimeName for IInputLightDismissActionStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputLightDismissActionStatics";
}
pub trait IInputLightDismissActionStatics_Impl: windows_core::IUnknownImpl {
    fn GetForWindowId(&self, windowId: &super::WindowId) -> windows_core::Result<InputLightDismissAction>;
}
impl IInputLightDismissActionStatics_Vtbl {
    pub const fn new<Identity: IInputLightDismissActionStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindowId<Identity: IInputLightDismissActionStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowid: super::WindowId, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputLightDismissActionStatics_Impl::GetForWindowId(this, core::mem::transmute(&windowid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputLightDismissActionStatics, OFFSET>(),
            GetForWindowId: GetForWindowId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputLightDismissActionStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissActionStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputLightDismissActionStatics2, IInputLightDismissActionStatics2_Vtbl, 0xc499a8a8_7182_5179_a28c_eab3f369b6f9);
impl windows_core::RuntimeType for IInputLightDismissActionStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputLightDismissActionStatics2");
}
impl windows_core::RuntimeName for IInputLightDismissActionStatics2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputLightDismissActionStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissActionStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputLightDismissEventArgs, IInputLightDismissEventArgs_Vtbl, 0x078660ee_07ca_5808_b982_e6e899cf098c);
impl windows_core::RuntimeType for IInputLightDismissEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputLightDismissEventArgs");
}
impl windows_core::RuntimeName for IInputLightDismissEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IInputLightDismissEventArgs";
}
pub trait IInputLightDismissEventArgs_Impl: windows_core::IUnknownImpl {}
impl IInputLightDismissEventArgs_Vtbl {
    pub const fn new<Identity: IInputLightDismissEventArgs_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputLightDismissEventArgs, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputLightDismissEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputLightDismissEventArgs2, IInputLightDismissEventArgs2_Vtbl, 0x099de760_b54f_5bab_8b69_66c5546d80d1);
impl windows_core::RuntimeType for IInputLightDismissEventArgs2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputLightDismissEventArgs2");
}
impl windows_core::RuntimeName for IInputLightDismissEventArgs2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputLightDismissEventArgs2";
}
pub trait IInputLightDismissEventArgs2_Impl: windows_core::IUnknownImpl {
    fn Reason(&self) -> windows_core::Result<LightDismissReason>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
}
impl IInputLightDismissEventArgs2_Vtbl {
    pub const fn new<Identity: IInputLightDismissEventArgs2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reason<Identity: IInputLightDismissEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut LightDismissReason) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputLightDismissEventArgs2_Impl::Reason(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IInputLightDismissEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputLightDismissEventArgs2_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IInputLightDismissEventArgs2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputLightDismissEventArgs2_Impl::SetHandled(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputLightDismissEventArgs2, OFFSET>(),
            Reason: Reason::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputLightDismissEventArgs2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputLightDismissEventArgs2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Reason: unsafe extern "system" fn(*mut core::ffi::c_void, *mut LightDismissReason) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputNonClientPointerSource, IInputNonClientPointerSource_Vtbl, 0x471732b4_3d07_5104_b192_ebacf71e86df);
impl windows_core::RuntimeType for IInputNonClientPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputNonClientPointerSource");
}
#[cfg(feature = "UI_Dispatching")]
impl windows_core::RuntimeName for IInputNonClientPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.IInputNonClientPointerSource";
}
#[cfg(feature = "UI_Dispatching")]
pub trait IInputNonClientPointerSource_Impl: windows_core::IUnknownImpl {
    fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue>;
    fn ClearAllRegionRects(&self) -> windows_core::Result<()>;
    fn ClearRegionRects(&self, region: NonClientRegionKind) -> windows_core::Result<()>;
    fn GetRegionRects(&self, region: NonClientRegionKind) -> windows_core::Result<windows_core::Array<windows::Graphics::RectInt32>>;
    fn SetRegionRects(&self, region: NonClientRegionKind, rects: &[windows::Graphics::RectInt32]) -> windows_core::Result<()>;
    fn CaptionTapped(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, NonClientCaptionTappedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveCaptionTapped(&self, token: i64) -> windows_core::Result<()>;
    fn PointerEntered(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, NonClientPointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()>;
    fn PointerExited(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, NonClientPointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()>;
    fn PointerMoved(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, NonClientPointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()>;
    fn PointerPressed(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, NonClientPointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()>;
    fn PointerReleased(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, NonClientPointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()>;
    fn RegionsChanged(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, NonClientRegionsChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveRegionsChanged(&self, token: i64) -> windows_core::Result<()>;
}
#[cfg(feature = "UI_Dispatching")]
impl IInputNonClientPointerSource_Vtbl {
    pub const fn new<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DispatcherQueue<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::DispatcherQueue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClearAllRegionRects<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::ClearAllRegionRects(this).into()
            }
        }
        unsafe extern "system" fn ClearRegionRects<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, region: NonClientRegionKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::ClearRegionRects(this, region).into()
            }
        }
        unsafe extern "system" fn GetRegionRects<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, region: NonClientRegionKind, result_size__: *mut u32, result__: *mut *mut windows::Graphics::RectInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::GetRegionRects(this, region) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(ok_data__);
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRegionRects<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, region: NonClientRegionKind, rects_array_size: u32, rects: *const windows::Graphics::RectInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::SetRegionRects(this, region, core::slice::from_raw_parts(core::mem::transmute_copy(&rects), rects_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn CaptionTapped<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::CaptionTapped(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveCaptionTapped<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::RemoveCaptionTapped(this, token).into()
            }
        }
        unsafe extern "system" fn PointerEntered<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::PointerEntered(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::RemovePointerEntered(this, token).into()
            }
        }
        unsafe extern "system" fn PointerExited<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::PointerExited(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::RemovePointerExited(this, token).into()
            }
        }
        unsafe extern "system" fn PointerMoved<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::PointerMoved(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::RemovePointerMoved(this, token).into()
            }
        }
        unsafe extern "system" fn PointerPressed<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::PointerPressed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::RemovePointerPressed(this, token).into()
            }
        }
        unsafe extern "system" fn PointerReleased<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::PointerReleased(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::RemovePointerReleased(this, token).into()
            }
        }
        unsafe extern "system" fn RegionsChanged<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource_Impl::RegionsChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveRegionsChanged<Identity: IInputNonClientPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource_Impl::RemoveRegionsChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputNonClientPointerSource, OFFSET>(),
            DispatcherQueue: DispatcherQueue::<Identity, OFFSET>,
            ClearAllRegionRects: ClearAllRegionRects::<Identity, OFFSET>,
            ClearRegionRects: ClearRegionRects::<Identity, OFFSET>,
            GetRegionRects: GetRegionRects::<Identity, OFFSET>,
            SetRegionRects: SetRegionRects::<Identity, OFFSET>,
            CaptionTapped: CaptionTapped::<Identity, OFFSET>,
            RemoveCaptionTapped: RemoveCaptionTapped::<Identity, OFFSET>,
            PointerEntered: PointerEntered::<Identity, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, OFFSET>,
            PointerExited: PointerExited::<Identity, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, OFFSET>,
            PointerMoved: PointerMoved::<Identity, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, OFFSET>,
            PointerPressed: PointerPressed::<Identity, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, OFFSET>,
            PointerReleased: PointerReleased::<Identity, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, OFFSET>,
            RegionsChanged: RegionsChanged::<Identity, OFFSET>,
            RemoveRegionsChanged: RemoveRegionsChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputNonClientPointerSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputNonClientPointerSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
    pub ClearAllRegionRects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearRegionRects: unsafe extern "system" fn(*mut core::ffi::c_void, NonClientRegionKind) -> windows_core::HRESULT,
    pub GetRegionRects: unsafe extern "system" fn(*mut core::ffi::c_void, NonClientRegionKind, *mut u32, *mut *mut windows::Graphics::RectInt32) -> windows_core::HRESULT,
    pub SetRegionRects: unsafe extern "system" fn(*mut core::ffi::c_void, NonClientRegionKind, u32, *const windows::Graphics::RectInt32) -> windows_core::HRESULT,
    pub CaptionTapped: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveCaptionTapped: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub RegionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveRegionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputNonClientPointerSource2, IInputNonClientPointerSource2_Vtbl, 0xdd2b10c4_7de6_5c1d_b438_06ddc994058f);
impl windows_core::RuntimeType for IInputNonClientPointerSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputNonClientPointerSource2");
}
impl windows_core::RuntimeName for IInputNonClientPointerSource2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputNonClientPointerSource2";
}
pub trait IInputNonClientPointerSource2_Impl: windows_core::IUnknownImpl {
    fn EnteringMoveSize(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, EnteringMoveSizeEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveEnteringMoveSize(&self, token: i64) -> windows_core::Result<()>;
    fn EnteredMoveSize(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, EnteredMoveSizeEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveEnteredMoveSize(&self, token: i64) -> windows_core::Result<()>;
    fn WindowRectChanging(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, WindowRectChangingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveWindowRectChanging(&self, token: i64) -> windows_core::Result<()>;
    fn WindowRectChanged(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, WindowRectChangedEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveWindowRectChanged(&self, token: i64) -> windows_core::Result<()>;
    fn ExitedMoveSize(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputNonClientPointerSource, ExitedMoveSizeEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveExitedMoveSize(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputNonClientPointerSource2_Vtbl {
    pub const fn new<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnteringMoveSize<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource2_Impl::EnteringMoveSize(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEnteringMoveSize<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource2_Impl::RemoveEnteringMoveSize(this, token).into()
            }
        }
        unsafe extern "system" fn EnteredMoveSize<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource2_Impl::EnteredMoveSize(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEnteredMoveSize<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource2_Impl::RemoveEnteredMoveSize(this, token).into()
            }
        }
        unsafe extern "system" fn WindowRectChanging<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource2_Impl::WindowRectChanging(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveWindowRectChanging<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource2_Impl::RemoveWindowRectChanging(this, token).into()
            }
        }
        unsafe extern "system" fn WindowRectChanged<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource2_Impl::WindowRectChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveWindowRectChanged<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource2_Impl::RemoveWindowRectChanged(this, token).into()
            }
        }
        unsafe extern "system" fn ExitedMoveSize<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSource2_Impl::ExitedMoveSize(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveExitedMoveSize<Identity: IInputNonClientPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputNonClientPointerSource2_Impl::RemoveExitedMoveSize(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputNonClientPointerSource2, OFFSET>(),
            EnteringMoveSize: EnteringMoveSize::<Identity, OFFSET>,
            RemoveEnteringMoveSize: RemoveEnteringMoveSize::<Identity, OFFSET>,
            EnteredMoveSize: EnteredMoveSize::<Identity, OFFSET>,
            RemoveEnteredMoveSize: RemoveEnteredMoveSize::<Identity, OFFSET>,
            WindowRectChanging: WindowRectChanging::<Identity, OFFSET>,
            RemoveWindowRectChanging: RemoveWindowRectChanging::<Identity, OFFSET>,
            WindowRectChanged: WindowRectChanged::<Identity, OFFSET>,
            RemoveWindowRectChanged: RemoveWindowRectChanged::<Identity, OFFSET>,
            ExitedMoveSize: ExitedMoveSize::<Identity, OFFSET>,
            RemoveExitedMoveSize: RemoveExitedMoveSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputNonClientPointerSource2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputNonClientPointerSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub EnteringMoveSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveEnteringMoveSize: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub EnteredMoveSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveEnteredMoveSize: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WindowRectChanging: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveWindowRectChanging: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WindowRectChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveWindowRectChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub ExitedMoveSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveExitedMoveSize: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputNonClientPointerSourceStatics, IInputNonClientPointerSourceStatics_Vtbl, 0x7d0b775c_1903_5dc7_bd2f_7a4b31f0cff2);
impl windows_core::RuntimeType for IInputNonClientPointerSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputNonClientPointerSourceStatics");
}
impl windows_core::RuntimeName for IInputNonClientPointerSourceStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputNonClientPointerSourceStatics";
}
pub trait IInputNonClientPointerSourceStatics_Impl: windows_core::IUnknownImpl {
    fn GetForWindowId(&self, windowId: &super::WindowId) -> windows_core::Result<InputNonClientPointerSource>;
}
impl IInputNonClientPointerSourceStatics_Vtbl {
    pub const fn new<Identity: IInputNonClientPointerSourceStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetForWindowId<Identity: IInputNonClientPointerSourceStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowid: super::WindowId, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputNonClientPointerSourceStatics_Impl::GetForWindowId(this, core::mem::transmute(&windowid)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputNonClientPointerSourceStatics, OFFSET>(),
            GetForWindowId: GetForWindowId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputNonClientPointerSourceStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputNonClientPointerSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputObject, IInputObject_Vtbl, 0x42edbc88_d386_544d_b1b8_68617fe68282);
impl windows_core::RuntimeType for IInputObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputObject");
}
#[cfg(feature = "UI_Dispatching")]
impl windows_core::RuntimeName for IInputObject {
    const NAME: &'static str = "Microsoft.UI.Input.IInputObject";
}
#[cfg(feature = "UI_Dispatching")]
pub trait IInputObject_Impl: windows_core::IUnknownImpl {
    fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue>;
}
#[cfg(feature = "UI_Dispatching")]
impl IInputObject_Vtbl {
    pub const fn new<Identity: IInputObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DispatcherQueue<Identity: IInputObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputObject_Impl::DispatcherQueue(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputObject, OFFSET>(), DispatcherQueue: DispatcherQueue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputObject as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObject_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI_Dispatching")]
    pub DispatcherQueue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "UI_Dispatching"))]
    DispatcherQueue: usize,
}
windows_core::imp::define_interface!(IInputObjectFactory, IInputObjectFactory_Vtbl, 0xf7786bc2_b0b8_5961_9a57_ae199d452106);
impl windows_core::RuntimeType for IInputObjectFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputObjectFactory");
}
impl windows_core::RuntimeName for IInputObjectFactory {
    const NAME: &'static str = "Microsoft.UI.Input.IInputObjectFactory";
}
pub trait IInputObjectFactory_Impl: windows_core::IUnknownImpl {}
impl IInputObjectFactory_Vtbl {
    pub const fn new<Identity: IInputObjectFactory_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputObjectFactory, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputObjectFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputObjectFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputPointerSource, IInputPointerSource_Vtbl, 0x6a6c2764_c3f4_5be5_8447_c9a98766c240);
impl windows_core::RuntimeType for IInputPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPointerSource");
}
impl windows_core::RuntimeName for IInputPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPointerSource";
}
pub trait IInputPointerSource_Impl: windows_core::IUnknownImpl {
    fn Cursor(&self) -> windows_core::Result<InputCursor>;
    fn SetCursor(&self, value: windows_core::Ref<InputCursor>) -> windows_core::Result<()>;
    fn DeviceKinds(&self) -> windows_core::Result<InputPointerSourceDeviceKinds>;
    fn PointerCaptureLost(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerCaptureLost(&self, token: i64) -> windows_core::Result<()>;
    fn PointerEntered(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerEntered(&self, token: i64) -> windows_core::Result<()>;
    fn PointerExited(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerExited(&self, token: i64) -> windows_core::Result<()>;
    fn PointerMoved(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerMoved(&self, token: i64) -> windows_core::Result<()>;
    fn PointerPressed(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerPressed(&self, token: i64) -> windows_core::Result<()>;
    fn PointerReleased(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerReleased(&self, token: i64) -> windows_core::Result<()>;
    fn PointerRoutedAway(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerRoutedAway(&self, token: i64) -> windows_core::Result<()>;
    fn PointerRoutedReleased(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerRoutedReleased(&self, token: i64) -> windows_core::Result<()>;
    fn PointerRoutedTo(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerRoutedTo(&self, token: i64) -> windows_core::Result<()>;
    fn PointerWheelChanged(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemovePointerWheelChanged(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputPointerSource_Vtbl {
    pub const fn new<Identity: IInputPointerSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cursor<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::Cursor(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCursor<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::SetCursor(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DeviceKinds<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputPointerSourceDeviceKinds) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::DeviceKinds(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerCaptureLost(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerCaptureLost(this, token).into()
            }
        }
        unsafe extern "system" fn PointerEntered<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerEntered(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerEntered(this, token).into()
            }
        }
        unsafe extern "system" fn PointerExited<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerExited(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerExited<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerExited(this, token).into()
            }
        }
        unsafe extern "system" fn PointerMoved<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerMoved(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerMoved(this, token).into()
            }
        }
        unsafe extern "system" fn PointerPressed<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerPressed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerPressed(this, token).into()
            }
        }
        unsafe extern "system" fn PointerReleased<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerReleased(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerReleased(this, token).into()
            }
        }
        unsafe extern "system" fn PointerRoutedAway<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerRoutedAway(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerRoutedAway<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerRoutedAway(this, token).into()
            }
        }
        unsafe extern "system" fn PointerRoutedReleased<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerRoutedReleased(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerRoutedReleased<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerRoutedReleased(this, token).into()
            }
        }
        unsafe extern "system" fn PointerRoutedTo<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerRoutedTo(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerRoutedTo<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerRoutedTo(this, token).into()
            }
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource_Impl::PointerWheelChanged(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: IInputPointerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource_Impl::RemovePointerWheelChanged(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputPointerSource, OFFSET>(),
            Cursor: Cursor::<Identity, OFFSET>,
            SetCursor: SetCursor::<Identity, OFFSET>,
            DeviceKinds: DeviceKinds::<Identity, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, OFFSET>,
            PointerEntered: PointerEntered::<Identity, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, OFFSET>,
            PointerExited: PointerExited::<Identity, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, OFFSET>,
            PointerMoved: PointerMoved::<Identity, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, OFFSET>,
            PointerPressed: PointerPressed::<Identity, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, OFFSET>,
            PointerReleased: PointerReleased::<Identity, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, OFFSET>,
            PointerRoutedAway: PointerRoutedAway::<Identity, OFFSET>,
            RemovePointerRoutedAway: RemovePointerRoutedAway::<Identity, OFFSET>,
            PointerRoutedReleased: PointerRoutedReleased::<Identity, OFFSET>,
            RemovePointerRoutedReleased: RemovePointerRoutedReleased::<Identity, OFFSET>,
            PointerRoutedTo: PointerRoutedTo::<Identity, OFFSET>,
            RemovePointerRoutedTo: RemovePointerRoutedTo::<Identity, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPointerSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCursor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeviceKinds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputPointerSourceDeviceKinds) -> windows_core::HRESULT,
    pub PointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerCaptureLost: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerEntered: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerExited: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerMoved: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerPressed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerReleased: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerRoutedAway: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerRoutedAway: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerRoutedReleased: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerRoutedReleased: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerRoutedTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerRoutedTo: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePointerWheelChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPointerSource2, IInputPointerSource2_Vtbl, 0x58757e6e_da80_5ad2_a088_b90e8e407379);
impl windows_core::RuntimeType for IInputPointerSource2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPointerSource2");
}
impl windows_core::RuntimeName for IInputPointerSource2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPointerSource2";
}
pub trait IInputPointerSource2_Impl: windows_core::IUnknownImpl {
    fn TrySetDeviceKinds(&self, DeviceKinds: InputPointerSourceDeviceKinds) -> windows_core::Result<bool>;
    fn ActivationBehavior(&self) -> windows_core::Result<InputPointerActivationBehavior>;
    fn SetActivationBehavior(&self, value: InputPointerActivationBehavior) -> windows_core::Result<()>;
    fn DirectManipulationHitTest(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, PointerEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveDirectManipulationHitTest(&self, token: i64) -> windows_core::Result<()>;
    fn TouchHitTesting(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPointerSource, TouchHitTestingEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveTouchHitTesting(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputPointerSource2_Vtbl {
    pub const fn new<Identity: IInputPointerSource2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TrySetDeviceKinds<Identity: IInputPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicekinds: InputPointerSourceDeviceKinds, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource2_Impl::TrySetDeviceKinds(this, devicekinds) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ActivationBehavior<Identity: IInputPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputPointerActivationBehavior) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource2_Impl::ActivationBehavior(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetActivationBehavior<Identity: IInputPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: InputPointerActivationBehavior) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource2_Impl::SetActivationBehavior(this, value).into()
            }
        }
        unsafe extern "system" fn DirectManipulationHitTest<Identity: IInputPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource2_Impl::DirectManipulationHitTest(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveDirectManipulationHitTest<Identity: IInputPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource2_Impl::RemoveDirectManipulationHitTest(this, token).into()
            }
        }
        unsafe extern "system" fn TouchHitTesting<Identity: IInputPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPointerSource2_Impl::TouchHitTesting(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveTouchHitTesting<Identity: IInputPointerSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPointerSource2_Impl::RemoveTouchHitTesting(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputPointerSource2, OFFSET>(),
            TrySetDeviceKinds: TrySetDeviceKinds::<Identity, OFFSET>,
            ActivationBehavior: ActivationBehavior::<Identity, OFFSET>,
            SetActivationBehavior: SetActivationBehavior::<Identity, OFFSET>,
            DirectManipulationHitTest: DirectManipulationHitTest::<Identity, OFFSET>,
            RemoveDirectManipulationHitTest: RemoveDirectManipulationHitTest::<Identity, OFFSET>,
            TouchHitTesting: TouchHitTesting::<Identity, OFFSET>,
            RemoveTouchHitTesting: RemoveTouchHitTesting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPointerSource2 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSource2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TrySetDeviceKinds: unsafe extern "system" fn(*mut core::ffi::c_void, InputPointerSourceDeviceKinds, *mut bool) -> windows_core::HRESULT,
    pub ActivationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputPointerActivationBehavior) -> windows_core::HRESULT,
    pub SetActivationBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, InputPointerActivationBehavior) -> windows_core::HRESULT,
    pub DirectManipulationHitTest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveDirectManipulationHitTest: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TouchHitTesting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveTouchHitTesting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPointerSourceStatics, IInputPointerSourceStatics_Vtbl, 0xe8a19fd1_a914_533f_9b0f_6bf0065e6781);
impl windows_core::RuntimeType for IInputPointerSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPointerSourceStatics");
}
impl windows_core::RuntimeName for IInputPointerSourceStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPointerSourceStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputPointerSourceStatics2, IInputPointerSourceStatics2_Vtbl, 0x76b37b4a_de02_531a_a9d2_185186159d31);
impl windows_core::RuntimeType for IInputPointerSourceStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPointerSourceStatics2");
}
impl windows_core::RuntimeName for IInputPointerSourceStatics2 {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPointerSourceStatics2";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPointerSourceStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    GetForVisual: usize,
    pub GetForWindowId: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPopupController, IInputPopupController_Vtbl, 0x433ead0d_3822_58ea_90fa_eb3e0b39829b);
impl windows_core::RuntimeType for IInputPopupController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPopupController");
}
impl windows_core::RuntimeName for IInputPopupController {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPopupController";
}
pub trait IInputPopupController_Impl: windows_core::IUnknownImpl {
    fn Mode(&self) -> windows_core::Result<PopupPointerMode>;
    fn SetMode(&self, value: PopupPointerMode) -> windows_core::Result<()>;
    fn LightDismissed(&self, handler: windows_core::Ref<windows::Foundation::TypedEventHandler<InputPopupController, InputLightDismissEventArgs>>) -> windows_core::Result<i64>;
    fn RemoveLightDismissed(&self, token: i64) -> windows_core::Result<()>;
}
impl IInputPopupController_Vtbl {
    pub const fn new<Identity: IInputPopupController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Mode<Identity: IInputPopupController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PopupPointerMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPopupController_Impl::Mode(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMode<Identity: IInputPopupController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: PopupPointerMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPopupController_Impl::SetMode(this, value).into()
            }
        }
        unsafe extern "system" fn LightDismissed<Identity: IInputPopupController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputPopupController_Impl::LightDismissed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveLightDismissed<Identity: IInputPopupController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputPopupController_Impl::RemoveLightDismissed(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputPopupController, OFFSET>(),
            Mode: Mode::<Identity, OFFSET>,
            SetMode: SetMode::<Identity, OFFSET>,
            LightDismissed: LightDismissed::<Identity, OFFSET>,
            RemoveLightDismissed: RemoveLightDismissed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPopupController as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPopupController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Mode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PopupPointerMode) -> windows_core::HRESULT,
    pub SetMode: unsafe extern "system" fn(*mut core::ffi::c_void, PopupPointerMode) -> windows_core::HRESULT,
    pub LightDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveLightDismissed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputPopupControllerStatics, IInputPopupControllerStatics_Vtbl, 0xb7ec6bdb_fea5_581c_966f_8d4238817b3b);
impl windows_core::RuntimeType for IInputPopupControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPopupControllerStatics");
}
impl windows_core::RuntimeName for IInputPopupControllerStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPopupControllerStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPopupControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputPreTranslateKeyboardSource, IInputPreTranslateKeyboardSource_Vtbl, 0x2f327feb_b7e7_5e37_a0cc_37dcabe76588);
impl windows_core::RuntimeType for IInputPreTranslateKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPreTranslateKeyboardSource");
}
impl windows_core::RuntimeName for IInputPreTranslateKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPreTranslateKeyboardSource";
}
pub trait IInputPreTranslateKeyboardSource_Impl: windows_core::IUnknownImpl {}
impl IInputPreTranslateKeyboardSource_Vtbl {
    pub const fn new<Identity: IInputPreTranslateKeyboardSource_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputPreTranslateKeyboardSource, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputPreTranslateKeyboardSource as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPreTranslateKeyboardSource_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputPreTranslateKeyboardSourceStatics, IInputPreTranslateKeyboardSourceStatics_Vtbl, 0x23d584d2_af8c_5a8a_806f_2ba9c5b1a5ec);
impl windows_core::RuntimeType for IInputPreTranslateKeyboardSourceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputPreTranslateKeyboardSourceStatics");
}
impl windows_core::RuntimeName for IInputPreTranslateKeyboardSourceStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputPreTranslateKeyboardSourceStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPreTranslateKeyboardSourceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IInputSystemCursor, IInputSystemCursor_Vtbl, 0x59f538e7_c500_59ab_8b54_0bc6100fd49e);
impl windows_core::RuntimeType for IInputSystemCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputSystemCursor");
}
impl windows_core::RuntimeName for IInputSystemCursor {
    const NAME: &'static str = "Microsoft.UI.Input.IInputSystemCursor";
}
pub trait IInputSystemCursor_Impl: windows_core::IUnknownImpl {
    fn CursorShape(&self) -> windows_core::Result<InputSystemCursorShape>;
}
impl IInputSystemCursor_Vtbl {
    pub const fn new<Identity: IInputSystemCursor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CursorShape<Identity: IInputSystemCursor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputSystemCursorShape) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputSystemCursor_Impl::CursorShape(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputSystemCursor, OFFSET>(), CursorShape: CursorShape::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputSystemCursor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CursorShape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputSystemCursorShape) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputSystemCursorStatics, IInputSystemCursorStatics_Vtbl, 0xd3860bb6_698a_5814_aedd_c2fa8bba5a02);
impl windows_core::RuntimeType for IInputSystemCursorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputSystemCursorStatics");
}
impl windows_core::RuntimeName for IInputSystemCursorStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputSystemCursorStatics";
}
pub trait IInputSystemCursorStatics_Impl: windows_core::IUnknownImpl {
    fn Create(&self, r#type: InputSystemCursorShape) -> windows_core::Result<InputSystemCursor>;
}
impl IInputSystemCursorStatics_Vtbl {
    pub const fn new<Identity: IInputSystemCursorStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Create<Identity: IInputSystemCursorStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: InputSystemCursorShape, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputSystemCursorStatics_Impl::Create(this, r#type) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputSystemCursorStatics, OFFSET>(), Create: Create::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputSystemCursorStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputSystemCursorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, InputSystemCursorShape, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputUnderlyingWindowController, IInputUnderlyingWindowController_Vtbl, 0xa236276e_ba7b_50b1_85cc_c7e263e4cbb4);
impl windows_core::RuntimeType for IInputUnderlyingWindowController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputUnderlyingWindowController");
}
impl windows_core::RuntimeName for IInputUnderlyingWindowController {
    const NAME: &'static str = "Microsoft.UI.Input.IInputUnderlyingWindowController";
}
pub trait IInputUnderlyingWindowController_Impl: windows_core::IUnknownImpl {
    fn WindowId(&self) -> windows_core::Result<super::WindowId>;
    fn OverrideUnderlyingWindow(&self, windowId: &super::WindowId) -> windows_core::Result<()>;
}
impl IInputUnderlyingWindowController_Vtbl {
    pub const fn new<Identity: IInputUnderlyingWindowController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WindowId<Identity: IInputUnderlyingWindowController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::WindowId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputUnderlyingWindowController_Impl::WindowId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OverrideUnderlyingWindow<Identity: IInputUnderlyingWindowController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windowid: super::WindowId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInputUnderlyingWindowController_Impl::OverrideUnderlyingWindow(this, core::mem::transmute(&windowid)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputUnderlyingWindowController, OFFSET>(),
            WindowId: WindowId::<Identity, OFFSET>,
            OverrideUnderlyingWindow: OverrideUnderlyingWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputUnderlyingWindowController as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputUnderlyingWindowController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WindowId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowId) -> windows_core::HRESULT,
    pub OverrideUnderlyingWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowId) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputUnderlyingWindowControllerStatics, IInputUnderlyingWindowControllerStatics_Vtbl, 0xbe701843_653b_5a19_81d9_21b61662e78e);
impl windows_core::RuntimeType for IInputUnderlyingWindowControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IInputUnderlyingWindowControllerStatics");
}
impl windows_core::RuntimeName for IInputUnderlyingWindowControllerStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IInputUnderlyingWindowControllerStatics";
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputUnderlyingWindowControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IKeyEventArgs, IKeyEventArgs_Vtbl, 0x40d5bb74_977e_5194_8039_9f6c44427bbb);
impl windows_core::RuntimeType for IKeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IKeyEventArgs");
}
impl windows_core::RuntimeName for IKeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IKeyEventArgs";
}
pub trait IKeyEventArgs_Impl: windows_core::IUnknownImpl {
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn KeyStatus(&self) -> windows_core::Result<PhysicalKeyStatus>;
    fn Timestamp(&self) -> windows_core::Result<u64>;
    fn VirtualKey(&self) -> windows_core::Result<windows::System::VirtualKey>;
}
impl IKeyEventArgs_Vtbl {
    pub const fn new<Identity: IKeyEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Handled<Identity: IKeyEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IKeyEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IKeyEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn KeyStatus<Identity: IKeyEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PhysicalKeyStatus) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyEventArgs_Impl::KeyStatus(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IKeyEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyEventArgs_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VirtualKey<Identity: IKeyEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKey) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IKeyEventArgs_Impl::VirtualKey(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IKeyEventArgs, OFFSET>(),
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            KeyStatus: KeyStatus::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
            VirtualKey: VirtualKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IKeyEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKeyEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PhysicalKeyStatus) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub VirtualKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKey) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationCompletedEventArgs, IManipulationCompletedEventArgs_Vtbl, 0x0e0249d4_46e4_5559_aee3_fa45ce2a7f56);
impl windows_core::RuntimeType for IManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IManipulationCompletedEventArgs");
}
impl windows_core::RuntimeName for IManipulationCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IManipulationCompletedEventArgs";
}
pub trait IManipulationCompletedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Cumulative(&self) -> windows_core::Result<ManipulationDelta>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn Velocities(&self) -> windows_core::Result<ManipulationVelocities>;
}
impl IManipulationCompletedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationCompletedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cumulative<Identity: IManipulationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IManipulationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Velocities<Identity: IManipulationCompletedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationVelocities) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationCompletedEventArgs_Impl::Velocities(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationCompletedEventArgs, OFFSET>(),
            Cumulative: Cumulative::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            Velocities: Velocities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationCompletedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationCompletedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationVelocities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationInertiaStartingEventArgs, IManipulationInertiaStartingEventArgs_Vtbl, 0xacf9ef71_6e15_56ab_9260_f0d3ce5f66e8);
impl windows_core::RuntimeType for IManipulationInertiaStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IManipulationInertiaStartingEventArgs");
}
impl windows_core::RuntimeName for IManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IManipulationInertiaStartingEventArgs";
}
pub trait IManipulationInertiaStartingEventArgs_Impl: windows_core::IUnknownImpl {
    fn Cumulative(&self) -> windows_core::Result<ManipulationDelta>;
    fn Delta(&self) -> windows_core::Result<ManipulationDelta>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn Velocities(&self) -> windows_core::Result<ManipulationVelocities>;
}
impl IManipulationInertiaStartingEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationInertiaStartingEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cumulative<Identity: IManipulationInertiaStartingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delta<Identity: IManipulationInertiaStartingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingEventArgs_Impl::Delta(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationInertiaStartingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IManipulationInertiaStartingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Velocities<Identity: IManipulationInertiaStartingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationVelocities) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationInertiaStartingEventArgs_Impl::Velocities(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationInertiaStartingEventArgs, OFFSET>(),
            Cumulative: Cumulative::<Identity, OFFSET>,
            Delta: Delta::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            Velocities: Velocities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationInertiaStartingEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationInertiaStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationVelocities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationStartedEventArgs, IManipulationStartedEventArgs_Vtbl, 0x4a616613_eef1_5f1b_a768_0775478d49d4);
impl windows_core::RuntimeType for IManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IManipulationStartedEventArgs");
}
impl windows_core::RuntimeName for IManipulationStartedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IManipulationStartedEventArgs";
}
pub trait IManipulationStartedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Cumulative(&self) -> windows_core::Result<ManipulationDelta>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
}
impl IManipulationStartedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationStartedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cumulative<Identity: IManipulationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IManipulationStartedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationStartedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationStartedEventArgs, OFFSET>(),
            Cumulative: Cumulative::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationStartedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationStartedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IManipulationUpdatedEventArgs, IManipulationUpdatedEventArgs_Vtbl, 0x406e1961_0c98_5fc0_b3d8_116492ef0053);
impl windows_core::RuntimeType for IManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IManipulationUpdatedEventArgs");
}
impl windows_core::RuntimeName for IManipulationUpdatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IManipulationUpdatedEventArgs";
}
pub trait IManipulationUpdatedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Cumulative(&self) -> windows_core::Result<ManipulationDelta>;
    fn Delta(&self) -> windows_core::Result<ManipulationDelta>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn Velocities(&self) -> windows_core::Result<ManipulationVelocities>;
}
impl IManipulationUpdatedEventArgs_Vtbl {
    pub const fn new<Identity: IManipulationUpdatedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Cumulative<Identity: IManipulationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationUpdatedEventArgs_Impl::Cumulative(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Delta<Identity: IManipulationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationDelta) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationUpdatedEventArgs_Impl::Delta(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IManipulationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationUpdatedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IManipulationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationUpdatedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Velocities<Identity: IManipulationUpdatedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ManipulationVelocities) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IManipulationUpdatedEventArgs_Impl::Velocities(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IManipulationUpdatedEventArgs, OFFSET>(),
            Cumulative: Cumulative::<Identity, OFFSET>,
            Delta: Delta::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            Velocities: Velocities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IManipulationUpdatedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IManipulationUpdatedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Cumulative: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub Delta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationDelta) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub Velocities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ManipulationVelocities) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IMouseWheelParameters, IMouseWheelParameters_Vtbl, 0x6d98be40_1d56_51d1_aa0d_f325439cd009);
impl windows_core::RuntimeType for IMouseWheelParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IMouseWheelParameters");
}
impl windows_core::RuntimeName for IMouseWheelParameters {
    const NAME: &'static str = "Microsoft.UI.Input.IMouseWheelParameters";
}
pub trait IMouseWheelParameters_Impl: windows_core::IUnknownImpl {
    fn CharTranslation(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn SetCharTranslation(&self, value: &windows::Foundation::Point) -> windows_core::Result<()>;
    fn DeltaScale(&self) -> windows_core::Result<f32>;
    fn SetDeltaScale(&self, value: f32) -> windows_core::Result<()>;
    fn DeltaRotationAngle(&self) -> windows_core::Result<f32>;
    fn SetDeltaRotationAngle(&self, value: f32) -> windows_core::Result<()>;
    fn PageTranslation(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn SetPageTranslation(&self, value: &windows::Foundation::Point) -> windows_core::Result<()>;
}
impl IMouseWheelParameters_Vtbl {
    pub const fn new<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CharTranslation<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMouseWheelParameters_Impl::CharTranslation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCharTranslation<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMouseWheelParameters_Impl::SetCharTranslation(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn DeltaScale<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMouseWheelParameters_Impl::DeltaScale(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDeltaScale<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMouseWheelParameters_Impl::SetDeltaScale(this, value).into()
            }
        }
        unsafe extern "system" fn DeltaRotationAngle<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMouseWheelParameters_Impl::DeltaRotationAngle(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDeltaRotationAngle<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMouseWheelParameters_Impl::SetDeltaRotationAngle(this, value).into()
            }
        }
        unsafe extern "system" fn PageTranslation<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMouseWheelParameters_Impl::PageTranslation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPageTranslation<Identity: IMouseWheelParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMouseWheelParameters_Impl::SetPageTranslation(this, core::mem::transmute(&value)).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMouseWheelParameters, OFFSET>(),
            CharTranslation: CharTranslation::<Identity, OFFSET>,
            SetCharTranslation: SetCharTranslation::<Identity, OFFSET>,
            DeltaScale: DeltaScale::<Identity, OFFSET>,
            SetDeltaScale: SetDeltaScale::<Identity, OFFSET>,
            DeltaRotationAngle: DeltaRotationAngle::<Identity, OFFSET>,
            SetDeltaRotationAngle: SetDeltaRotationAngle::<Identity, OFFSET>,
            PageTranslation: PageTranslation::<Identity, OFFSET>,
            SetPageTranslation: SetPageTranslation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMouseWheelParameters as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMouseWheelParameters_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CharTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetCharTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
    pub DeltaScale: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDeltaScale: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub DeltaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetDeltaRotationAngle: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub PageTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub SetPageTranslation: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INonClientCaptionTappedEventArgs, INonClientCaptionTappedEventArgs_Vtbl, 0x3d173531_991f_5753_b7e0_14a121c3cd2d);
impl windows_core::RuntimeType for INonClientCaptionTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.INonClientCaptionTappedEventArgs");
}
impl windows_core::RuntimeName for INonClientCaptionTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.INonClientCaptionTappedEventArgs";
}
pub trait INonClientCaptionTappedEventArgs_Impl: windows_core::IUnknownImpl {
    fn Point(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
}
impl INonClientCaptionTappedEventArgs_Vtbl {
    pub const fn new<Identity: INonClientCaptionTappedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Point<Identity: INonClientCaptionTappedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonClientCaptionTappedEventArgs_Impl::Point(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: INonClientCaptionTappedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonClientCaptionTappedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INonClientCaptionTappedEventArgs, OFFSET>(),
            Point: Point::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INonClientCaptionTappedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INonClientCaptionTappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INonClientPointerEventArgs, INonClientPointerEventArgs_Vtbl, 0xa5b44aec_b797_505a_a129_ae4e5271c73c);
impl windows_core::RuntimeType for INonClientPointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.INonClientPointerEventArgs");
}
impl windows_core::RuntimeName for INonClientPointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.INonClientPointerEventArgs";
}
pub trait INonClientPointerEventArgs_Impl: windows_core::IUnknownImpl {
    fn Point(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn RegionKind(&self) -> windows_core::Result<NonClientRegionKind>;
    fn IsPointInRegion(&self) -> windows_core::Result<bool>;
}
impl INonClientPointerEventArgs_Vtbl {
    pub const fn new<Identity: INonClientPointerEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Point<Identity: INonClientPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonClientPointerEventArgs_Impl::Point(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: INonClientPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonClientPointerEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegionKind<Identity: INonClientPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NonClientRegionKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonClientPointerEventArgs_Impl::RegionKind(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPointInRegion<Identity: INonClientPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonClientPointerEventArgs_Impl::IsPointInRegion(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INonClientPointerEventArgs, OFFSET>(),
            Point: Point::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            RegionKind: RegionKind::<Identity, OFFSET>,
            IsPointInRegion: IsPointInRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INonClientPointerEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INonClientPointerEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub RegionKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NonClientRegionKind) -> windows_core::HRESULT,
    pub IsPointInRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(INonClientRegionsChangedEventArgs, INonClientRegionsChangedEventArgs_Vtbl, 0xfe97ee95_1824_51b2_b8eb_10ff0665ce23);
impl windows_core::RuntimeType for INonClientRegionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.INonClientRegionsChangedEventArgs");
}
impl windows_core::RuntimeName for INonClientRegionsChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.INonClientRegionsChangedEventArgs";
}
pub trait INonClientRegionsChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn ChangedRegions(&self) -> windows_core::Result<windows_core::Array<NonClientRegionKind>>;
}
impl INonClientRegionsChangedEventArgs_Vtbl {
    pub const fn new<Identity: INonClientRegionsChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ChangedRegions<Identity: INonClientRegionsChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut NonClientRegionKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match INonClientRegionsChangedEventArgs_Impl::ChangedRegions(this) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(ok_data__);
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, INonClientRegionsChangedEventArgs, OFFSET>(),
            ChangedRegions: ChangedRegions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<INonClientRegionsChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INonClientRegionsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ChangedRegions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut NonClientRegionKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerEventArgs, IPointerEventArgs_Vtbl, 0x865b188c_2ed5_5df8_829f_ac0701d5c51a);
impl windows_core::RuntimeType for IPointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IPointerEventArgs");
}
impl windows_core::RuntimeName for IPointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerEventArgs";
}
pub trait IPointerEventArgs_Impl: windows_core::IUnknownImpl {
    fn CurrentPoint(&self) -> windows_core::Result<PointerPoint>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn KeyModifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers>;
    fn GetIntermediatePoints(&self) -> windows_core::Result<windows_collections::IVector<PointerPoint>>;
    fn GetIntermediateTransformedPoints(&self, transform: windows_core::Ref<IPointerPointTransform>) -> windows_core::Result<windows_collections::IVector<PointerPoint>>;
}
impl IPointerEventArgs_Vtbl {
    pub const fn new<Identity: IPointerEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentPoint<Identity: IPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerEventArgs_Impl::CurrentPoint(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: IPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: IPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPointerEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn KeyModifiers<Identity: IPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerEventArgs_Impl::KeyModifiers(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIntermediatePoints<Identity: IPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerEventArgs_Impl::GetIntermediatePoints(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIntermediateTransformedPoints<Identity: IPointerEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerEventArgs_Impl::GetIntermediateTransformedPoints(this, core::mem::transmute_copy(&transform)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerEventArgs, OFFSET>(),
            CurrentPoint: CurrentPoint::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            KeyModifiers: KeyModifiers::<Identity, OFFSET>,
            GetIntermediatePoints: GetIntermediatePoints::<Identity, OFFSET>,
            GetIntermediateTransformedPoints: GetIntermediateTransformedPoints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeyModifiers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::System::VirtualKeyModifiers) -> windows_core::HRESULT,
    pub GetIntermediatePoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIntermediateTransformedPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPoint, IPointerPoint_Vtbl, 0x0d430ee6_252c_59a4_b2a2_d44264dc6a40);
impl windows_core::RuntimeType for IPointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IPointerPoint");
}
impl windows_core::RuntimeName for IPointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPoint";
}
pub trait IPointerPoint_Impl: windows_core::IUnknownImpl {
    fn FrameId(&self) -> windows_core::Result<u32>;
    fn IsInContact(&self) -> windows_core::Result<bool>;
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn PointerId(&self) -> windows_core::Result<u32>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn Properties(&self) -> windows_core::Result<PointerPointProperties>;
    fn Timestamp(&self) -> windows_core::Result<u64>;
    fn GetTransformedPoint(&self, transform: windows_core::Ref<IPointerPointTransform>) -> windows_core::Result<PointerPoint>;
}
impl IPointerPoint_Vtbl {
    pub const fn new<Identity: IPointerPoint_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FrameId<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::FrameId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInContact<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::IsInContact(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerDeviceType<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerId<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::PointerId(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Properties<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::Properties(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Timestamp<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::Timestamp(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransformedPoint<Identity: IPointerPoint_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transform: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPoint_Impl::GetTransformedPoint(this, core::mem::transmute_copy(&transform)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPoint, OFFSET>(),
            FrameId: FrameId::<Identity, OFFSET>,
            IsInContact: IsInContact::<Identity, OFFSET>,
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            PointerId: PointerId::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            Properties: Properties::<Identity, OFFSET>,
            Timestamp: Timestamp::<Identity, OFFSET>,
            GetTransformedPoint: GetTransformedPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPoint as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPoint_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FrameId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsInContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub PointerId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub Properties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Timestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetTransformedPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointProperties, IPointerPointProperties_Vtbl, 0xd760ed77_4b10_57a5_b3cc_d9bf3413e996);
impl windows_core::RuntimeType for IPointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IPointerPointProperties");
}
impl windows_core::RuntimeName for IPointerPointProperties {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPointProperties";
}
pub trait IPointerPointProperties_Impl: windows_core::IUnknownImpl {
    fn ContactRect(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn IsBarrelButtonPressed(&self) -> windows_core::Result<bool>;
    fn IsCanceled(&self) -> windows_core::Result<bool>;
    fn IsEraser(&self) -> windows_core::Result<bool>;
    fn IsHorizontalMouseWheel(&self) -> windows_core::Result<bool>;
    fn IsInRange(&self) -> windows_core::Result<bool>;
    fn IsInverted(&self) -> windows_core::Result<bool>;
    fn IsLeftButtonPressed(&self) -> windows_core::Result<bool>;
    fn IsMiddleButtonPressed(&self) -> windows_core::Result<bool>;
    fn IsPrimary(&self) -> windows_core::Result<bool>;
    fn IsRightButtonPressed(&self) -> windows_core::Result<bool>;
    fn IsXButton1Pressed(&self) -> windows_core::Result<bool>;
    fn IsXButton2Pressed(&self) -> windows_core::Result<bool>;
    fn MouseWheelDelta(&self) -> windows_core::Result<i32>;
    fn Orientation(&self) -> windows_core::Result<f32>;
    fn PointerUpdateKind(&self) -> windows_core::Result<PointerUpdateKind>;
    fn Pressure(&self) -> windows_core::Result<f32>;
    fn TouchConfidence(&self) -> windows_core::Result<bool>;
    fn Twist(&self) -> windows_core::Result<f32>;
    fn XTilt(&self) -> windows_core::Result<f32>;
    fn YTilt(&self) -> windows_core::Result<f32>;
}
impl IPointerPointProperties_Vtbl {
    pub const fn new<Identity: IPointerPointProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContactRect<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::ContactRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsBarrelButtonPressed<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsBarrelButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCanceled<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsCanceled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEraser<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsEraser(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsHorizontalMouseWheel<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsHorizontalMouseWheel(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInRange<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsInRange(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsInverted<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsInverted(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLeftButtonPressed<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsLeftButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMiddleButtonPressed<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsMiddleButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsPrimary<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsPrimary(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRightButtonPressed<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsRightButtonPressed(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsXButton1Pressed<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsXButton1Pressed(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsXButton2Pressed<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::IsXButton2Pressed(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MouseWheelDelta<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::MouseWheelDelta(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Orientation<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::Orientation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PointerUpdateKind<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerUpdateKind) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::PointerUpdateKind(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Pressure<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::Pressure(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TouchConfidence<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::TouchConfidence(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Twist<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::Twist(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn XTilt<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::XTilt(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn YTilt<Identity: IPointerPointProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointProperties_Impl::YTilt(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPointProperties, OFFSET>(),
            ContactRect: ContactRect::<Identity, OFFSET>,
            IsBarrelButtonPressed: IsBarrelButtonPressed::<Identity, OFFSET>,
            IsCanceled: IsCanceled::<Identity, OFFSET>,
            IsEraser: IsEraser::<Identity, OFFSET>,
            IsHorizontalMouseWheel: IsHorizontalMouseWheel::<Identity, OFFSET>,
            IsInRange: IsInRange::<Identity, OFFSET>,
            IsInverted: IsInverted::<Identity, OFFSET>,
            IsLeftButtonPressed: IsLeftButtonPressed::<Identity, OFFSET>,
            IsMiddleButtonPressed: IsMiddleButtonPressed::<Identity, OFFSET>,
            IsPrimary: IsPrimary::<Identity, OFFSET>,
            IsRightButtonPressed: IsRightButtonPressed::<Identity, OFFSET>,
            IsXButton1Pressed: IsXButton1Pressed::<Identity, OFFSET>,
            IsXButton2Pressed: IsXButton2Pressed::<Identity, OFFSET>,
            MouseWheelDelta: MouseWheelDelta::<Identity, OFFSET>,
            Orientation: Orientation::<Identity, OFFSET>,
            PointerUpdateKind: PointerUpdateKind::<Identity, OFFSET>,
            Pressure: Pressure::<Identity, OFFSET>,
            TouchConfidence: TouchConfidence::<Identity, OFFSET>,
            Twist: Twist::<Identity, OFFSET>,
            XTilt: XTilt::<Identity, OFFSET>,
            YTilt: YTilt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointProperties as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContactRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub IsBarrelButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsCanceled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsEraser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsHorizontalMouseWheel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsInverted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsLeftButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsMiddleButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsPrimary: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsRightButtonPressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton1Pressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsXButton2Pressed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub MouseWheelDelta: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Orientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub PointerUpdateKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerUpdateKind) -> windows_core::HRESULT,
    pub Pressure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub TouchConfidence: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Twist: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub XTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub YTilt: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointStatics, IPointerPointStatics_Vtbl, 0x880d34c2_acfd_51d9_b60a_b408ce17590c);
impl windows_core::RuntimeType for IPointerPointStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IPointerPointStatics");
}
impl windows_core::RuntimeName for IPointerPointStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPointStatics";
}
pub trait IPointerPointStatics_Impl: windows_core::IUnknownImpl {
    fn GetCurrentPoint(&self, pointerId: u32) -> windows_core::Result<PointerPoint>;
}
impl IPointerPointStatics_Vtbl {
    pub const fn new<Identity: IPointerPointStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentPoint<Identity: IPointerPointStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointStatics_Impl::GetCurrentPoint(this, pointerid) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPointStatics, OFFSET>(), GetCurrentPoint: GetCurrentPoint::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetCurrentPoint: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPointTransform, IPointerPointTransform_Vtbl, 0xdb4791bc_994d_54c7_92ef_66ea1de9b43c);
impl windows_core::RuntimeType for IPointerPointTransform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IPointerPointTransform");
}
windows_core::imp::interface_hierarchy!(IPointerPointTransform, windows_core::IUnknown, windows_core::IInspectable);
impl IPointerPointTransform {
    pub fn Inverse(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Inverse)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn TryTransform(&self, inpoint: windows::Foundation::Point, outpoint: &mut windows::Foundation::Point) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryTransform)(windows_core::Interface::as_raw(self), inpoint, outpoint, &mut result__).map(|| result__)
        }
    }
    pub fn TryTransformBounds(&self, inrect: windows::Foundation::Rect, outrect: &mut windows::Foundation::Rect) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TryTransformBounds)(windows_core::Interface::as_raw(self), inrect, outrect, &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IPointerPointTransform {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPointTransform";
}
pub trait IPointerPointTransform_Impl: windows_core::IUnknownImpl {
    fn Inverse(&self) -> windows_core::Result<IPointerPointTransform>;
    fn TryTransform(&self, inPoint: &windows::Foundation::Point, outPoint: &mut windows::Foundation::Point) -> windows_core::Result<bool>;
    fn TryTransformBounds(&self, inRect: &windows::Foundation::Rect, outRect: &mut windows::Foundation::Rect) -> windows_core::Result<bool>;
}
impl IPointerPointTransform_Vtbl {
    pub const fn new<Identity: IPointerPointTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Inverse<Identity: IPointerPointTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointTransform_Impl::Inverse(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TryTransform<Identity: IPointerPointTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inpoint: windows::Foundation::Point, outpoint: *mut windows::Foundation::Point, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointTransform_Impl::TryTransform(this, core::mem::transmute(&inpoint), core::mem::transmute_copy(&outpoint)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TryTransformBounds<Identity: IPointerPointTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inrect: windows::Foundation::Rect, outrect: *mut windows::Foundation::Rect, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPointTransform_Impl::TryTransformBounds(this, core::mem::transmute(&inrect), core::mem::transmute_copy(&outrect)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPointTransform, OFFSET>(),
            Inverse: Inverse::<Identity, OFFSET>,
            TryTransform: TryTransform::<Identity, OFFSET>,
            TryTransformBounds: TryTransformBounds::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPointTransform as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPointTransform_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Inverse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryTransform: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Point, *mut windows::Foundation::Point, *mut bool) -> windows_core::HRESULT,
    pub TryTransformBounds: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect, *mut windows::Foundation::Rect, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPredictor, IPointerPredictor_Vtbl, 0x12c100ec_2100_565f_a60c_f1187f438828);
impl windows_core::RuntimeType for IPointerPredictor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IPointerPredictor");
}
impl windows_core::RuntimeName for IPointerPredictor {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPredictor";
}
pub trait IPointerPredictor_Impl: windows_core::IUnknownImpl {
    fn PredictionTime(&self) -> windows_core::Result<windows_time::TimeSpan>;
    fn SetPredictionTime(&self, value: &windows_time::TimeSpan) -> windows_core::Result<()>;
    fn GetPredictedPoints(&self, point: windows_core::Ref<PointerPoint>) -> windows_core::Result<windows_core::Array<PointerPoint>>;
}
impl IPointerPredictor_Vtbl {
    pub const fn new<Identity: IPointerPredictor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PredictionTime<Identity: IPointerPredictor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_time::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPredictor_Impl::PredictionTime(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPredictionTime<Identity: IPointerPredictor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_time::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPointerPredictor_Impl::SetPredictionTime(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn GetPredictedPoints<Identity: IPointerPredictor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: *mut core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPredictor_Impl::GetPredictedPoints(this, core::mem::transmute_copy(&point)) {
                    Ok(ok__) => {
                        let (ok_data__, ok_data_len__) = ok__.into_abi();
                        result__.write(core::mem::transmute(ok_data__));
                        result_size__.write(ok_data_len__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPredictor, OFFSET>(),
            PredictionTime: PredictionTime::<Identity, OFFSET>,
            SetPredictionTime: SetPredictionTime::<Identity, OFFSET>,
            GetPredictedPoints: GetPredictedPoints::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPredictor as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictor_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PredictionTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::TimeSpan) -> windows_core::HRESULT,
    pub SetPredictionTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_time::TimeSpan) -> windows_core::HRESULT,
    pub GetPredictedPoints: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPointerPredictorStatics, IPointerPredictorStatics_Vtbl, 0x78a8ef30_3e5c_55cd_8f85_65ac09b1a987);
impl windows_core::RuntimeType for IPointerPredictorStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IPointerPredictorStatics");
}
impl windows_core::RuntimeName for IPointerPredictorStatics {
    const NAME: &'static str = "Microsoft.UI.Input.IPointerPredictorStatics";
}
pub trait IPointerPredictorStatics_Impl: windows_core::IUnknownImpl {
    fn CreateForInputPointerSource(&self, inputPointerSource: windows_core::Ref<InputPointerSource>) -> windows_core::Result<PointerPredictor>;
}
impl IPointerPredictorStatics_Vtbl {
    pub const fn new<Identity: IPointerPredictorStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateForInputPointerSource<Identity: IPointerPredictorStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputpointersource: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPointerPredictorStatics_Impl::CreateForInputPointerSource(this, core::mem::transmute_copy(&inputpointersource)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPointerPredictorStatics, OFFSET>(),
            CreateForInputPointerSource: CreateForInputPointerSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPointerPredictorStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPointerPredictorStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateForInputPointerSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRightTappedEventArgs, IRightTappedEventArgs_Vtbl, 0x8ff73b39_887e_50a4_8500_77953039dcb4);
impl windows_core::RuntimeType for IRightTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IRightTappedEventArgs");
}
impl windows_core::RuntimeName for IRightTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IRightTappedEventArgs";
}
pub trait IRightTappedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
}
impl IRightTappedEventArgs_Vtbl {
    pub const fn new<Identity: IRightTappedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerDeviceType<Identity: IRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRightTappedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: IRightTappedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRightTappedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRightTappedEventArgs, OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRightTappedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRightTappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITappedEventArgs, ITappedEventArgs_Vtbl, 0xc3a01bb5_6076_5e0f_871a_9d94a6a8f82b);
impl windows_core::RuntimeType for ITappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.ITappedEventArgs");
}
impl windows_core::RuntimeName for ITappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ITappedEventArgs";
}
pub trait ITappedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType>;
    fn Position(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn TapCount(&self) -> windows_core::Result<u32>;
}
impl ITappedEventArgs_Vtbl {
    pub const fn new<Identity: ITappedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerDeviceType<Identity: ITappedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut PointerDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITappedEventArgs_Impl::PointerDeviceType(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Position<Identity: ITappedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITappedEventArgs_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TapCount<Identity: ITappedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITappedEventArgs_Impl::TapCount(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITappedEventArgs, OFFSET>(),
            PointerDeviceType: PointerDeviceType::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            TapCount: TapCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITappedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITappedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PointerDeviceType) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub TapCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ITouchHitTestingEventArgs, ITouchHitTestingEventArgs_Vtbl, 0xc2196f0c_60ed_5b5d_b919_aa7357e0a25c);
impl windows_core::RuntimeType for ITouchHitTestingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.ITouchHitTestingEventArgs");
}
impl windows_core::RuntimeName for ITouchHitTestingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ITouchHitTestingEventArgs";
}
pub trait ITouchHitTestingEventArgs_Impl: windows_core::IUnknownImpl {
    fn BoundingBox(&self) -> windows_core::Result<windows::Foundation::Rect>;
    fn Handled(&self) -> windows_core::Result<bool>;
    fn SetHandled(&self, value: bool) -> windows_core::Result<()>;
    fn Point(&self) -> windows_core::Result<windows::Foundation::Point>;
    fn GetProximityEvaluation(&self) -> windows_core::Result<ProximityEvaluation>;
    fn SetProximityEvaluation(&self, proximityEvaluation: &ProximityEvaluation) -> windows_core::Result<()>;
    fn EvaluateProximityToRect(&self, controlBoundingBox: &windows::Foundation::Rect) -> windows_core::Result<ProximityEvaluation>;
    fn EvaluateProximityToPolygon(&self, controlVertices: &[windows::Foundation::Point]) -> windows_core::Result<ProximityEvaluation>;
}
impl ITouchHitTestingEventArgs_Vtbl {
    pub const fn new<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BoundingBox<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Rect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITouchHitTestingEventArgs_Impl::BoundingBox(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Handled<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITouchHitTestingEventArgs_Impl::Handled(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetHandled<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITouchHitTestingEventArgs_Impl::SetHandled(this, value).into()
            }
        }
        unsafe extern "system" fn Point<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITouchHitTestingEventArgs_Impl::Point(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProximityEvaluation<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ProximityEvaluation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITouchHitTestingEventArgs_Impl::GetProximityEvaluation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProximityEvaluation<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, proximityevaluation: ProximityEvaluation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITouchHitTestingEventArgs_Impl::SetProximityEvaluation(this, core::mem::transmute(&proximityevaluation)).into()
            }
        }
        unsafe extern "system" fn EvaluateProximityToRect<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controlboundingbox: windows::Foundation::Rect, result__: *mut ProximityEvaluation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITouchHitTestingEventArgs_Impl::EvaluateProximityToRect(this, core::mem::transmute(&controlboundingbox)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EvaluateProximityToPolygon<Identity: ITouchHitTestingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controlvertices_array_size: u32, controlvertices: *const windows::Foundation::Point, result__: *mut ProximityEvaluation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITouchHitTestingEventArgs_Impl::EvaluateProximityToPolygon(this, core::slice::from_raw_parts(core::mem::transmute_copy(&controlvertices), controlvertices_array_size as usize)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ITouchHitTestingEventArgs, OFFSET>(),
            BoundingBox: BoundingBox::<Identity, OFFSET>,
            Handled: Handled::<Identity, OFFSET>,
            SetHandled: SetHandled::<Identity, OFFSET>,
            Point: Point::<Identity, OFFSET>,
            GetProximityEvaluation: GetProximityEvaluation::<Identity, OFFSET>,
            SetProximityEvaluation: SetProximityEvaluation::<Identity, OFFSET>,
            EvaluateProximityToRect: EvaluateProximityToRect::<Identity, OFFSET>,
            EvaluateProximityToPolygon: EvaluateProximityToPolygon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITouchHitTestingEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITouchHitTestingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BoundingBox: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Rect) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Point: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Foundation::Point) -> windows_core::HRESULT,
    pub GetProximityEvaluation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProximityEvaluation) -> windows_core::HRESULT,
    pub SetProximityEvaluation: unsafe extern "system" fn(*mut core::ffi::c_void, ProximityEvaluation) -> windows_core::HRESULT,
    pub EvaluateProximityToRect: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Foundation::Rect, *mut ProximityEvaluation) -> windows_core::HRESULT,
    pub EvaluateProximityToPolygon: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows::Foundation::Point, *mut ProximityEvaluation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowRectChangedEventArgs, IWindowRectChangedEventArgs_Vtbl, 0x8a885d28_d2d9_5dda_9848_cdf247771037);
impl windows_core::RuntimeType for IWindowRectChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IWindowRectChangedEventArgs");
}
impl windows_core::RuntimeName for IWindowRectChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IWindowRectChangedEventArgs";
}
pub trait IWindowRectChangedEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32>;
    fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation>;
    fn OldWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32>;
    fn NewWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32>;
}
impl IWindowRectChangedEventArgs_Vtbl {
    pub const fn new<Identity: IWindowRectChangedEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerScreenPoint<Identity: IWindowRectChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::PointInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangedEventArgs_Impl::PointerScreenPoint(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveSizeOperation<Identity: IWindowRectChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MoveSizeOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangedEventArgs_Impl::MoveSizeOperation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OldWindowRect<Identity: IWindowRectChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::RectInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangedEventArgs_Impl::OldWindowRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NewWindowRect<Identity: IWindowRectChangedEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::RectInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangedEventArgs_Impl::NewWindowRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowRectChangedEventArgs, OFFSET>(),
            PointerScreenPoint: PointerScreenPoint::<Identity, OFFSET>,
            MoveSizeOperation: MoveSizeOperation::<Identity, OFFSET>,
            OldWindowRect: OldWindowRect::<Identity, OFFSET>,
            NewWindowRect: NewWindowRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowRectChangedEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowRectChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerScreenPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::PointInt32) -> windows_core::HRESULT,
    pub MoveSizeOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MoveSizeOperation) -> windows_core::HRESULT,
    pub OldWindowRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::RectInt32) -> windows_core::HRESULT,
    pub NewWindowRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::RectInt32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IWindowRectChangingEventArgs, IWindowRectChangingEventArgs_Vtbl, 0xdb13ed3c_debc_5855_8d70_5936fd813457);
impl windows_core::RuntimeType for IWindowRectChangingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.IWindowRectChangingEventArgs");
}
impl windows_core::RuntimeName for IWindowRectChangingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.IWindowRectChangingEventArgs";
}
pub trait IWindowRectChangingEventArgs_Impl: windows_core::IUnknownImpl {
    fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32>;
    fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation>;
    fn OldWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32>;
    fn NewWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32>;
    fn SetNewWindowRect(&self, value: &windows::Graphics::RectInt32) -> windows_core::Result<()>;
    fn AllowRectChange(&self) -> windows_core::Result<bool>;
    fn SetAllowRectChange(&self, value: bool) -> windows_core::Result<()>;
    fn ShowWindow(&self) -> windows_core::Result<bool>;
    fn SetShowWindow(&self, value: bool) -> windows_core::Result<()>;
}
impl IWindowRectChangingEventArgs_Vtbl {
    pub const fn new<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PointerScreenPoint<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::PointInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangingEventArgs_Impl::PointerScreenPoint(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveSizeOperation<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut MoveSizeOperation) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangingEventArgs_Impl::MoveSizeOperation(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OldWindowRect<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::RectInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangingEventArgs_Impl::OldWindowRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NewWindowRect<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows::Graphics::RectInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangingEventArgs_Impl::NewWindowRect(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNewWindowRect<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows::Graphics::RectInt32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowRectChangingEventArgs_Impl::SetNewWindowRect(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn AllowRectChange<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangingEventArgs_Impl::AllowRectChange(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllowRectChange<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowRectChangingEventArgs_Impl::SetAllowRectChange(this, value).into()
            }
        }
        unsafe extern "system" fn ShowWindow<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowRectChangingEventArgs_Impl::ShowWindow(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetShowWindow<Identity: IWindowRectChangingEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowRectChangingEventArgs_Impl::SetShowWindow(this, value).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IWindowRectChangingEventArgs, OFFSET>(),
            PointerScreenPoint: PointerScreenPoint::<Identity, OFFSET>,
            MoveSizeOperation: MoveSizeOperation::<Identity, OFFSET>,
            OldWindowRect: OldWindowRect::<Identity, OFFSET>,
            NewWindowRect: NewWindowRect::<Identity, OFFSET>,
            SetNewWindowRect: SetNewWindowRect::<Identity, OFFSET>,
            AllowRectChange: AllowRectChange::<Identity, OFFSET>,
            SetAllowRectChange: SetAllowRectChange::<Identity, OFFSET>,
            ShowWindow: ShowWindow::<Identity, OFFSET>,
            SetShowWindow: SetShowWindow::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowRectChangingEventArgs as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowRectChangingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PointerScreenPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::PointInt32) -> windows_core::HRESULT,
    pub MoveSizeOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MoveSizeOperation) -> windows_core::HRESULT,
    pub OldWindowRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::RectInt32) -> windows_core::HRESULT,
    pub NewWindowRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows::Graphics::RectInt32) -> windows_core::HRESULT,
    pub SetNewWindowRect: unsafe extern "system" fn(*mut core::ffi::c_void, windows::Graphics::RectInt32) -> windows_core::HRESULT,
    pub AllowRectChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetAllowRectChange: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ShowWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetShowWindow: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputActivationListener(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputActivationListener, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputActivationListener, InputObject);
impl InputActivationListener {
    pub fn State(&self) -> windows_core::Result<InputActivationState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn InputActivationChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<InputActivationListenerActivationChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, InputActivationListenerActivationChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).InputActivationChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveInputActivationChanged))
        }
    }
    pub fn GetForWindowId(windowid: super::WindowId) -> windows_core::Result<Self> {
        Self::IInputActivationListenerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IInputActivationListenerStatics<R, F: FnOnce(&IInputActivationListenerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputActivationListener, IInputActivationListenerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInputActivationListenerStatics2<R, F: FnOnce(&IInputActivationListenerStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputActivationListener, IInputActivationListenerStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputActivationListener {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputActivationListener>();
}
unsafe impl windows_core::Interface for InputActivationListener {
    type Vtable = <IInputActivationListener as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputActivationListener as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputActivationListener {
    const NAME: &'static str = "Microsoft.UI.Input.InputActivationListener";
}
unsafe impl Send for InputActivationListener {}
unsafe impl Sync for InputActivationListener {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputActivationListenerActivationChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputActivationListenerActivationChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl windows_core::RuntimeType for InputActivationListenerActivationChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputActivationListenerActivationChangedEventArgs>();
}
unsafe impl windows_core::Interface for InputActivationListenerActivationChangedEventArgs {
    type Vtable = <IInputActivationListenerActivationChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputActivationListenerActivationChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputActivationListenerActivationChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.InputActivationListenerActivationChangedEventArgs";
}
unsafe impl Send for InputActivationListenerActivationChangedEventArgs {}
unsafe impl Sync for InputActivationListenerActivationChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputActivationState(pub i32);
impl InputActivationState {
    pub const None: Self = Self(0);
    pub const Deactivated: Self = Self(1);
    pub const Activated: Self = Self(2);
}
impl windows_core::imp::TypeKind for InputActivationState {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputActivationState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.InputActivationState;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.InputActivationState");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputCursor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputCursor, windows::Foundation::IClosable);
impl InputCursor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CreateFromCoreCursor<P0>(cursor: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<windows::UI::Core::CoreCursor>,
    {
        Self::IInputCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromCoreCursor)(windows_core::Interface::as_raw(this), cursor.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputCursorStatics<R, F: FnOnce(&IInputCursorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputCursor, IInputCursorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputCursor>();
}
unsafe impl windows_core::Interface for InputCursor {
    type Vtable = <IInputCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputCursor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCursor";
}
unsafe impl Send for InputCursor {}
unsafe impl Sync for InputCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputCustomCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputCustomCursor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputCustomCursor, windows::Foundation::IClosable, InputCursor);
impl InputCustomCursor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeType for InputCustomCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputCustomCursor>();
}
unsafe impl windows_core::Interface for InputCustomCursor {
    type Vtable = <IInputCustomCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputCustomCursor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputCustomCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputCustomCursor";
}
unsafe impl Send for InputCustomCursor {}
unsafe impl Sync for InputCustomCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputDesktopNamedResourceCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputDesktopNamedResourceCursor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputDesktopNamedResourceCursor, windows::Foundation::IClosable, InputCursor);
impl InputDesktopNamedResourceCursor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ModuleName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ResourceName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Create(resourcename: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IInputDesktopNamedResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(resourcename), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModule(modulename: &windows_core::HSTRING, resourcename: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IInputDesktopNamedResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModule)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(modulename), core::mem::transmute_copy(resourcename), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputDesktopNamedResourceCursorStatics<R, F: FnOnce(&IInputDesktopNamedResourceCursorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputDesktopNamedResourceCursor, IInputDesktopNamedResourceCursorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputDesktopNamedResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputDesktopNamedResourceCursor>();
}
unsafe impl windows_core::Interface for InputDesktopNamedResourceCursor {
    type Vtable = <IInputDesktopNamedResourceCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputDesktopNamedResourceCursor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputDesktopNamedResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopNamedResourceCursor";
}
unsafe impl Send for InputDesktopNamedResourceCursor {}
unsafe impl Sync for InputDesktopNamedResourceCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputDesktopResourceCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputDesktopResourceCursor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputDesktopResourceCursor, windows::Foundation::IClosable, InputCursor);
impl InputDesktopResourceCursor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ModuleName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ModuleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ResourceId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResourceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Create(resourceid: u32) -> windows_core::Result<Self> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), resourceid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateFromModule(modulename: &windows_core::HSTRING, resourceid: u32) -> windows_core::Result<Self> {
        Self::IInputDesktopResourceCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromModule)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(modulename), resourceid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputDesktopResourceCursorStatics<R, F: FnOnce(&IInputDesktopResourceCursorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputDesktopResourceCursor, IInputDesktopResourceCursorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputDesktopResourceCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputDesktopResourceCursor>();
}
unsafe impl windows_core::Interface for InputDesktopResourceCursor {
    type Vtable = <IInputDesktopResourceCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputDesktopResourceCursor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputDesktopResourceCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputDesktopResourceCursor";
}
unsafe impl Send for InputDesktopResourceCursor {}
unsafe impl Sync for InputDesktopResourceCursor {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputFocusController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputFocusController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputFocusController, InputObject);
impl InputFocusController {
    pub fn HasFocus(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TrySetFocus(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrySetFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GotFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<FocusChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, FocusChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).GotFocus)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveGotFocus))
        }
    }
    pub fn LostFocus<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<FocusChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, FocusChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).LostFocus)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveLostFocus))
        }
    }
    pub fn DepartFocus<P0>(&self, request: P0) -> windows_core::Result<FocusNavigationResult>
    where
        P0: windows_core::Param<FocusNavigationRequest>,
    {
        let this = &windows_core::Interface::cast::<IInputFocusController2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DepartFocus)(windows_core::Interface::as_raw(this), request.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn NavigateFocusRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<FocusNavigationRequestEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputFocusController2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, FocusNavigationRequestEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).NavigateFocusRequested)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveNavigateFocusRequested))
        }
    }
    pub fn ShouldShowKeyboardCues(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInputFocusController3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ShouldShowKeyboardCues)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IInputFocusControllerStatics<R, F: FnOnce(&IInputFocusControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputFocusController, IInputFocusControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputFocusController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputFocusController>();
}
unsafe impl windows_core::Interface for InputFocusController {
    type Vtable = <IInputFocusController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputFocusController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputFocusController {
    const NAME: &'static str = "Microsoft.UI.Input.InputFocusController";
}
unsafe impl Send for InputFocusController {}
unsafe impl Sync for InputFocusController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputFocusNavigationHost(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputFocusNavigationHost, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputFocusNavigationHost, InputObject);
impl InputFocusNavigationHost {
    pub fn ContainsFocus(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainsFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn NavigateFocus<P0>(&self, request: P0) -> windows_core::Result<FocusNavigationResult>
    where
        P0: windows_core::Param<FocusNavigationRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NavigateFocus)(windows_core::Interface::as_raw(self), request.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn DepartFocusRequested<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<FocusNavigationRequestEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, FocusNavigationRequestEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).DepartFocusRequested)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDepartFocusRequested))
        }
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IInputFocusNavigationHostStatics<R, F: FnOnce(&IInputFocusNavigationHostStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputFocusNavigationHost, IInputFocusNavigationHostStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInputFocusNavigationHostStatics2<R, F: FnOnce(&IInputFocusNavigationHostStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputFocusNavigationHost, IInputFocusNavigationHostStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputFocusNavigationHost {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputFocusNavigationHost>();
}
unsafe impl windows_core::Interface for InputFocusNavigationHost {
    type Vtable = <IInputFocusNavigationHost as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputFocusNavigationHost as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputFocusNavigationHost {
    const NAME: &'static str = "Microsoft.UI.Input.InputFocusNavigationHost";
}
unsafe impl Send for InputFocusNavigationHost {}
unsafe impl Sync for InputFocusNavigationHost {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputKeyboardSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputKeyboardSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputKeyboardSource, InputObject);
impl InputKeyboardSource {
    pub fn GetCurrentKeyState(&self, virtualkey: windows::System::VirtualKey) -> windows_core::Result<VirtualKeyStates> {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentKeyState)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        }
    }
    pub fn GetKeyState(&self, virtualkey: windows::System::VirtualKey) -> windows_core::Result<VirtualKeyStates> {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKeyState)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        }
    }
    pub fn CharacterReceived<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<CharacterReceivedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, CharacterReceivedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).CharacterReceived)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveCharacterReceived))
        }
    }
    pub fn ContextMenuKey<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ContextMenuKeyEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, ContextMenuKeyEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ContextMenuKey)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveContextMenuKey))
        }
    }
    pub fn KeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<KeyEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, KeyEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyDown))
        }
    }
    pub fn KeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<KeyEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, KeyEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).KeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveKeyUp))
        }
    }
    pub fn SystemKeyDown<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<KeyEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, KeyEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).SystemKeyDown)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveSystemKeyDown))
        }
    }
    pub fn SystemKeyUp<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<KeyEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputKeyboardSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, KeyEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).SystemKeyUp)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveSystemKeyUp))
        }
    }
    pub fn GetKeyStateForCurrentThread(virtualkey: windows::System::VirtualKey) -> windows_core::Result<windows::UI::Core::CoreVirtualKeyStates> {
        Self::IInputKeyboardSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetKeyStateForCurrentThread)(windows_core::Interface::as_raw(this), virtualkey, &mut result__).map(|| result__)
        })
    }
    pub fn GetForWindowId(windowid: super::WindowId) -> windows_core::Result<Self> {
        Self::IInputKeyboardSourceStatics3(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IInputKeyboardSourceStatics<R, F: FnOnce(&IInputKeyboardSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputKeyboardSource, IInputKeyboardSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInputKeyboardSourceStatics2<R, F: FnOnce(&IInputKeyboardSourceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputKeyboardSource, IInputKeyboardSourceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInputKeyboardSourceStatics3<R, F: FnOnce(&IInputKeyboardSourceStatics3) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputKeyboardSource, IInputKeyboardSourceStatics3> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputKeyboardSource>();
}
unsafe impl windows_core::Interface for InputKeyboardSource {
    type Vtable = <IInputKeyboardSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputKeyboardSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputKeyboardSource";
}
unsafe impl Send for InputKeyboardSource {}
unsafe impl Sync for InputKeyboardSource {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputLayoutPolicy(pub i32);
impl InputLayoutPolicy {
    pub const LeftToRight: Self = Self(0);
    pub const RightToLeft: Self = Self(1);
}
impl windows_core::imp::TypeKind for InputLayoutPolicy {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputLayoutPolicy {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.InputLayoutPolicy;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.InputLayoutPolicy");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputLightDismissAction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputLightDismissAction, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputLightDismissAction, InputObject);
impl InputLightDismissAction {
    pub fn Dismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<InputLightDismissEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, InputLightDismissEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Dismissed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveDismissed))
        }
    }
    pub fn GetForWindowId(windowid: super::WindowId) -> windows_core::Result<Self> {
        Self::IInputLightDismissActionStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IInputLightDismissActionStatics<R, F: FnOnce(&IInputLightDismissActionStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputLightDismissAction, IInputLightDismissActionStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInputLightDismissActionStatics2<R, F: FnOnce(&IInputLightDismissActionStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputLightDismissAction, IInputLightDismissActionStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputLightDismissAction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputLightDismissAction>();
}
unsafe impl windows_core::Interface for InputLightDismissAction {
    type Vtable = <IInputLightDismissAction as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputLightDismissAction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputLightDismissAction {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissAction";
}
unsafe impl Send for InputLightDismissAction {}
unsafe impl Sync for InputLightDismissAction {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputLightDismissEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputLightDismissEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl InputLightDismissEventArgs {
    pub fn Reason(&self) -> windows_core::Result<LightDismissReason> {
        let this = &windows_core::Interface::cast::<IInputLightDismissEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Reason)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInputLightDismissEventArgs2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInputLightDismissEventArgs2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for InputLightDismissEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputLightDismissEventArgs>();
}
unsafe impl windows_core::Interface for InputLightDismissEventArgs {
    type Vtable = <IInputLightDismissEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputLightDismissEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputLightDismissEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.InputLightDismissEventArgs";
}
unsafe impl Send for InputLightDismissEventArgs {}
unsafe impl Sync for InputLightDismissEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputNonClientPointerSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputNonClientPointerSource, windows_core::IUnknown, windows_core::IInspectable);
impl InputNonClientPointerSource {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn ClearAllRegionRects(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearAllRegionRects)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub fn ClearRegionRects(&self, region: NonClientRegionKind) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearRegionRects)(windows_core::Interface::as_raw(self), region).ok() }
    }
    pub fn GetRegionRects(&self, region: NonClientRegionKind) -> windows_core::Result<windows_core::Array<windows::Graphics::RectInt32>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetRegionRects)(windows_core::Interface::as_raw(self), region, windows_core::Array::<windows::Graphics::RectInt32>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn SetRegionRects(&self, region: NonClientRegionKind, rects: &[windows::Graphics::RectInt32]) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRegionRects)(windows_core::Interface::as_raw(self), region, rects.len().try_into().unwrap(), rects.as_ptr()).ok() }
    }
    pub fn CaptionTapped<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<NonClientCaptionTappedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, NonClientCaptionTappedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).CaptionTapped)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveCaptionTapped))
        }
    }
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<NonClientPointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, NonClientPointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerEntered)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerEntered))
        }
    }
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<NonClientPointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, NonClientPointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerExited)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerExited))
        }
    }
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<NonClientPointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, NonClientPointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerMoved)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerMoved))
        }
    }
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<NonClientPointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, NonClientPointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerPressed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerPressed))
        }
    }
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<NonClientPointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, NonClientPointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerReleased)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerReleased))
        }
    }
    pub fn RegionsChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<NonClientRegionsChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, NonClientRegionsChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).RegionsChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveRegionsChanged))
        }
    }
    pub fn EnteringMoveSize<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<EnteringMoveSizeEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputNonClientPointerSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, EnteringMoveSizeEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EnteringMoveSize)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEnteringMoveSize))
        }
    }
    pub fn EnteredMoveSize<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<EnteredMoveSizeEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputNonClientPointerSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, EnteredMoveSizeEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).EnteredMoveSize)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveEnteredMoveSize))
        }
    }
    pub fn WindowRectChanging<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<WindowRectChangingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputNonClientPointerSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, WindowRectChangingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).WindowRectChanging)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveWindowRectChanging))
        }
    }
    pub fn WindowRectChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<WindowRectChangedEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputNonClientPointerSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, WindowRectChangedEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).WindowRectChanged)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveWindowRectChanged))
        }
    }
    pub fn ExitedMoveSize<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<ExitedMoveSizeEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputNonClientPointerSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, ExitedMoveSizeEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).ExitedMoveSize)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveExitedMoveSize))
        }
    }
    pub fn GetForWindowId(windowid: super::WindowId) -> windows_core::Result<Self> {
        Self::IInputNonClientPointerSourceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputNonClientPointerSourceStatics<R, F: FnOnce(&IInputNonClientPointerSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputNonClientPointerSource, IInputNonClientPointerSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputNonClientPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputNonClientPointerSource>();
}
unsafe impl windows_core::Interface for InputNonClientPointerSource {
    type Vtable = <IInputNonClientPointerSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputNonClientPointerSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputNonClientPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputNonClientPointerSource";
}
unsafe impl Send for InputNonClientPointerSource {}
unsafe impl Sync for InputNonClientPointerSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputObject(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputObject, windows_core::IUnknown, windows_core::IInspectable);
impl InputObject {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DispatcherQueue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InputObject {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputObject>();
}
unsafe impl windows_core::Interface for InputObject {
    type Vtable = <IInputObject as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputObject as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputObject {
    const NAME: &'static str = "Microsoft.UI.Input.InputObject";
}
unsafe impl Send for InputObject {}
unsafe impl Sync for InputObject {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputPointerActivationBehavior(pub i32);
impl InputPointerActivationBehavior {
    pub const Default: Self = Self(0);
    pub const Activate: Self = Self(1);
    pub const NoActivate: Self = Self(3);
}
impl windows_core::imp::TypeKind for InputPointerActivationBehavior {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputPointerActivationBehavior {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.InputPointerActivationBehavior;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.InputPointerActivationBehavior");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputPointerSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputPointerSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputPointerSource, InputObject);
impl InputPointerSource {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Cursor(&self) -> windows_core::Result<InputCursor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cursor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetCursor<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<InputCursor>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCursor)(windows_core::Interface::as_raw(self), value.param().abi()).ok() }
    }
    pub fn DeviceKinds(&self) -> windows_core::Result<InputPointerSourceDeviceKinds> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceKinds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerCaptureLost<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerCaptureLost)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerCaptureLost))
        }
    }
    pub fn PointerEntered<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerEntered)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerEntered))
        }
    }
    pub fn PointerExited<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerExited)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerExited))
        }
    }
    pub fn PointerMoved<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerMoved)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerMoved))
        }
    }
    pub fn PointerPressed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerPressed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerPressed))
        }
    }
    pub fn PointerReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerReleased)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerReleased))
        }
    }
    pub fn PointerRoutedAway<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerRoutedAway)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerRoutedAway))
        }
    }
    pub fn PointerRoutedReleased<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerRoutedReleased)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerRoutedReleased))
        }
    }
    pub fn PointerRoutedTo<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerRoutedTo)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerRoutedTo))
        }
    }
    pub fn PointerWheelChanged<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).PointerWheelChanged)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemovePointerWheelChanged))
        }
    }
    pub fn TrySetDeviceKinds(&self, devicekinds: InputPointerSourceDeviceKinds) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IInputPointerSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TrySetDeviceKinds)(windows_core::Interface::as_raw(this), devicekinds, &mut result__).map(|| result__)
        }
    }
    pub fn ActivationBehavior(&self) -> windows_core::Result<InputPointerActivationBehavior> {
        let this = &windows_core::Interface::cast::<IInputPointerSource2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ActivationBehavior)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetActivationBehavior(&self, value: InputPointerActivationBehavior) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IInputPointerSource2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetActivationBehavior)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DirectManipulationHitTest<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<PointerEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputPointerSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, PointerEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).DirectManipulationHitTest)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveDirectManipulationHitTest))
        }
    }
    pub fn TouchHitTesting<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<TouchHitTestingEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let this = &windows_core::Interface::cast::<IInputPointerSource2>(self)?;
        let handler = <windows::Foundation::TypedEventHandler<Self, TouchHitTestingEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(this).TouchHitTesting)(windows_core::Interface::as_raw(this), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(this.clone(), token__, windows_core::Interface::vtable(this).RemoveTouchHitTesting))
        }
    }
    pub fn GetForWindowId(windowid: super::WindowId) -> windows_core::Result<Self> {
        Self::IInputPointerSourceStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForWindowId)(windows_core::Interface::as_raw(this), windowid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputPointerSourceStatics<R, F: FnOnce(&IInputPointerSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputPointerSource, IInputPointerSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IInputPointerSourceStatics2<R, F: FnOnce(&IInputPointerSourceStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputPointerSource, IInputPointerSourceStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputPointerSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputPointerSource>();
}
unsafe impl windows_core::Interface for InputPointerSource {
    type Vtable = <IInputPointerSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputPointerSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputPointerSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPointerSource";
}
unsafe impl Send for InputPointerSource {}
unsafe impl Sync for InputPointerSource {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputPointerSourceDeviceKinds(pub u32);
impl InputPointerSourceDeviceKinds {
    pub const None: Self = Self(0);
    pub const Touch: Self = Self(1);
    pub const Pen: Self = Self(2);
    pub const Mouse: Self = Self(4);
}
impl windows_core::imp::TypeKind for InputPointerSourceDeviceKinds {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputPointerSourceDeviceKinds {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.InputPointerSourceDeviceKinds;u4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.InputPointerSourceDeviceKinds");
}
impl InputPointerSourceDeviceKinds {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InputPointerSourceDeviceKinds {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for InputPointerSourceDeviceKinds {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for InputPointerSourceDeviceKinds {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputPopupController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputPopupController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputPopupController, InputObject);
impl InputPopupController {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Mode(&self) -> windows_core::Result<PopupPointerMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Mode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetMode(&self, value: PopupPointerMode) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMode)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn LightDismissed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<InputLightDismissEventArgs>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <windows::Foundation::TypedEventHandler<Self, InputLightDismissEventArgs>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).LightDismissed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveLightDismissed))
        }
    }
    fn IInputPopupControllerStatics<R, F: FnOnce(&IInputPopupControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputPopupController, IInputPopupControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputPopupController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputPopupController>();
}
unsafe impl windows_core::Interface for InputPopupController {
    type Vtable = <IInputPopupController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputPopupController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputPopupController {
    const NAME: &'static str = "Microsoft.UI.Input.InputPopupController";
}
unsafe impl Send for InputPopupController {}
unsafe impl Sync for InputPopupController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputPreTranslateKeyboardSource(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputPreTranslateKeyboardSource, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputPreTranslateKeyboardSource, InputObject);
impl InputPreTranslateKeyboardSource {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    fn IInputPreTranslateKeyboardSourceStatics<R, F: FnOnce(&IInputPreTranslateKeyboardSourceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputPreTranslateKeyboardSource, IInputPreTranslateKeyboardSourceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputPreTranslateKeyboardSource {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputPreTranslateKeyboardSource>();
}
unsafe impl windows_core::Interface for InputPreTranslateKeyboardSource {
    type Vtable = <IInputPreTranslateKeyboardSource as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputPreTranslateKeyboardSource as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputPreTranslateKeyboardSource {
    const NAME: &'static str = "Microsoft.UI.Input.InputPreTranslateKeyboardSource";
}
unsafe impl Send for InputPreTranslateKeyboardSource {}
unsafe impl Sync for InputPreTranslateKeyboardSource {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputSystemCursor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputSystemCursor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputSystemCursor, windows::Foundation::IClosable, InputCursor);
impl InputSystemCursor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CursorShape(&self) -> windows_core::Result<InputSystemCursorShape> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CursorShape)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Create(r#type: InputSystemCursorShape) -> windows_core::Result<Self> {
        Self::IInputSystemCursorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), r#type, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IInputSystemCursorStatics<R, F: FnOnce(&IInputSystemCursorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputSystemCursor, IInputSystemCursorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputSystemCursor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputSystemCursor>();
}
unsafe impl windows_core::Interface for InputSystemCursor {
    type Vtable = <IInputSystemCursor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputSystemCursor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputSystemCursor {
    const NAME: &'static str = "Microsoft.UI.Input.InputSystemCursor";
}
unsafe impl Send for InputSystemCursor {}
unsafe impl Sync for InputSystemCursor {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputSystemCursorShape(pub i32);
impl InputSystemCursorShape {
    pub const Arrow: Self = Self(0);
    pub const Cross: Self = Self(1);
    pub const Hand: Self = Self(3);
    pub const Help: Self = Self(4);
    pub const IBeam: Self = Self(5);
    pub const SizeAll: Self = Self(6);
    pub const SizeNortheastSouthwest: Self = Self(7);
    pub const SizeNorthSouth: Self = Self(8);
    pub const SizeNorthwestSoutheast: Self = Self(9);
    pub const SizeWestEast: Self = Self(10);
    pub const UniversalNo: Self = Self(11);
    pub const UpArrow: Self = Self(12);
    pub const Wait: Self = Self(13);
    pub const Pin: Self = Self(14);
    pub const Person: Self = Self(15);
    pub const AppStarting: Self = Self(16);
}
impl windows_core::imp::TypeKind for InputSystemCursorShape {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for InputSystemCursorShape {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.InputSystemCursorShape;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.InputSystemCursorShape");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputUnderlyingWindowController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputUnderlyingWindowController, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(InputUnderlyingWindowController, InputObject);
impl InputUnderlyingWindowController {
    #[cfg(feature = "UI_Dispatching")]
    pub fn DispatcherQueue(&self) -> windows_core::Result<super::Dispatching::DispatcherQueue> {
        let this = &windows_core::Interface::cast::<IInputObject>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DispatcherQueue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn WindowId(&self) -> windows_core::Result<super::WindowId> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OverrideUnderlyingWindow(&self, windowid: super::WindowId) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OverrideUnderlyingWindow)(windows_core::Interface::as_raw(self), windowid).ok() }
    }
    fn IInputUnderlyingWindowControllerStatics<R, F: FnOnce(&IInputUnderlyingWindowControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InputUnderlyingWindowController, IInputUnderlyingWindowControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for InputUnderlyingWindowController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputUnderlyingWindowController>();
}
unsafe impl windows_core::Interface for InputUnderlyingWindowController {
    type Vtable = <IInputUnderlyingWindowController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputUnderlyingWindowController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputUnderlyingWindowController {
    const NAME: &'static str = "Microsoft.UI.Input.InputUnderlyingWindowController";
}
unsafe impl Send for InputUnderlyingWindowController {}
unsafe impl Sync for InputUnderlyingWindowController {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KeyEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(KeyEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl KeyEventArgs {
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn KeyStatus(&self) -> windows_core::Result<PhysicalKeyStatus> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn VirtualKey(&self) -> windows_core::Result<windows::System::VirtualKey> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VirtualKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for KeyEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IKeyEventArgs>();
}
unsafe impl windows_core::Interface for KeyEventArgs {
    type Vtable = <IKeyEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IKeyEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for KeyEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.KeyEventArgs";
}
unsafe impl Send for KeyEventArgs {}
unsafe impl Sync for KeyEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LightDismissReason(pub i32);
impl LightDismissReason {
    pub const Programmatic: Self = Self(0);
    pub const WindowMoved: Self = Self(1);
    pub const ActivationChanged: Self = Self(2);
    pub const KeyboardEvent: Self = Self(3);
    pub const PointerEvent: Self = Self(4);
    pub const VisibilityChanged: Self = Self(5);
}
impl windows_core::imp::TypeKind for LightDismissReason {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for LightDismissReason {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.LightDismissReason;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.LightDismissReason");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationCompletedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationCompletedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationCompletedEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationCompletedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationCompletedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationCompletedEventArgs {
    type Vtable = <IManipulationCompletedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationCompletedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationCompletedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationCompletedEventArgs";
}
unsafe impl Send for ManipulationCompletedEventArgs {}
unsafe impl Sync for ManipulationCompletedEventArgs {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ManipulationDelta {
    pub Translation: windows::Foundation::Point,
    pub Scale: f32,
    pub Rotation: f32,
    pub Expansion: f32,
}
impl windows_core::imp::TypeKind for ManipulationDelta {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ManipulationDelta {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Input.ManipulationDelta;struct(Windows.Foundation.Point;f4;f4);f4;f4;f4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.ManipulationDelta");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationInertiaStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationInertiaStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationInertiaStartingEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Delta(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationInertiaStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationInertiaStartingEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationInertiaStartingEventArgs {
    type Vtable = <IManipulationInertiaStartingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationInertiaStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationInertiaStartingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationInertiaStartingEventArgs";
}
unsafe impl Send for ManipulationInertiaStartingEventArgs {}
unsafe impl Sync for ManipulationInertiaStartingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationStartedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationStartedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationStartedEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationStartedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationStartedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationStartedEventArgs {
    type Vtable = <IManipulationStartedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationStartedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationStartedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationStartedEventArgs";
}
unsafe impl Send for ManipulationStartedEventArgs {}
unsafe impl Sync for ManipulationStartedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManipulationUpdatedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(ManipulationUpdatedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl ManipulationUpdatedEventArgs {
    pub fn Cumulative(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Cumulative)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Delta(&self) -> windows_core::Result<ManipulationDelta> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Delta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Velocities(&self) -> windows_core::Result<ManipulationVelocities> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Velocities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for ManipulationUpdatedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IManipulationUpdatedEventArgs>();
}
unsafe impl windows_core::Interface for ManipulationUpdatedEventArgs {
    type Vtable = <IManipulationUpdatedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IManipulationUpdatedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for ManipulationUpdatedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.ManipulationUpdatedEventArgs";
}
unsafe impl Send for ManipulationUpdatedEventArgs {}
unsafe impl Sync for ManipulationUpdatedEventArgs {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ManipulationVelocities {
    pub Linear: windows::Foundation::Point,
    pub Angular: f32,
    pub Expansion: f32,
}
impl windows_core::imp::TypeKind for ManipulationVelocities {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ManipulationVelocities {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Input.ManipulationVelocities;struct(Windows.Foundation.Point;f4;f4);f4;f4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.ManipulationVelocities");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MouseWheelParameters(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(MouseWheelParameters, windows_core::IUnknown, windows_core::IInspectable);
impl MouseWheelParameters {
    pub fn CharTranslation(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CharTranslation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetCharTranslation(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCharTranslation)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn DeltaScale(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeltaScale)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeltaScale(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeltaScale)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn DeltaRotationAngle(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeltaRotationAngle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetDeltaRotationAngle(&self, value: f32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetDeltaRotationAngle)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn PageTranslation(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PageTranslation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPageTranslation(&self, value: windows::Foundation::Point) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPageTranslation)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for MouseWheelParameters {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IMouseWheelParameters>();
}
unsafe impl windows_core::Interface for MouseWheelParameters {
    type Vtable = <IMouseWheelParameters as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IMouseWheelParameters as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for MouseWheelParameters {
    const NAME: &'static str = "Microsoft.UI.Input.MouseWheelParameters";
}
unsafe impl Send for MouseWheelParameters {}
unsafe impl Sync for MouseWheelParameters {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MoveSizeOperation(pub i32);
impl MoveSizeOperation {
    pub const Move: Self = Self(0);
    pub const SizeBottom: Self = Self(1);
    pub const SizeBottomLeft: Self = Self(2);
    pub const SizeBottomRight: Self = Self(3);
    pub const SizeLeft: Self = Self(4);
    pub const SizeRight: Self = Self(5);
    pub const SizeTop: Self = Self(6);
    pub const SizeTopLeft: Self = Self(7);
    pub const SizeTopRight: Self = Self(8);
}
impl windows_core::imp::TypeKind for MoveSizeOperation {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for MoveSizeOperation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.MoveSizeOperation;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.MoveSizeOperation");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonClientCaptionTappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NonClientCaptionTappedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl NonClientCaptionTappedEventArgs {
    pub fn Point(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Point)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NonClientCaptionTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INonClientCaptionTappedEventArgs>();
}
unsafe impl windows_core::Interface for NonClientCaptionTappedEventArgs {
    type Vtable = <INonClientCaptionTappedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INonClientCaptionTappedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NonClientCaptionTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientCaptionTappedEventArgs";
}
unsafe impl Send for NonClientCaptionTappedEventArgs {}
unsafe impl Sync for NonClientCaptionTappedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonClientPointerEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NonClientPointerEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl NonClientPointerEventArgs {
    pub fn Point(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Point)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn RegionKind(&self) -> windows_core::Result<NonClientRegionKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegionKind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPointInRegion(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPointInRegion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for NonClientPointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INonClientPointerEventArgs>();
}
unsafe impl windows_core::Interface for NonClientPointerEventArgs {
    type Vtable = <INonClientPointerEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INonClientPointerEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NonClientPointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientPointerEventArgs";
}
unsafe impl Send for NonClientPointerEventArgs {}
unsafe impl Sync for NonClientPointerEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NonClientRegionKind(pub i32);
impl NonClientRegionKind {
    pub const Close: Self = Self(0);
    pub const Maximize: Self = Self(1);
    pub const Minimize: Self = Self(2);
    pub const Icon: Self = Self(3);
    pub const Caption: Self = Self(4);
    pub const TopBorder: Self = Self(5);
    pub const LeftBorder: Self = Self(6);
    pub const BottomBorder: Self = Self(7);
    pub const RightBorder: Self = Self(8);
    pub const Passthrough: Self = Self(9);
}
impl windows_core::imp::TypeKind for NonClientRegionKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for NonClientRegionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.NonClientRegionKind;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.NonClientRegionKind");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NonClientRegionsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(NonClientRegionsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl NonClientRegionsChangedEventArgs {
    pub fn ChangedRegions(&self) -> windows_core::Result<windows_core::Array<NonClientRegionKind>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).ChangedRegions)(windows_core::Interface::as_raw(self), windows_core::Array::<NonClientRegionKind>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
}
impl windows_core::RuntimeType for NonClientRegionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, INonClientRegionsChangedEventArgs>();
}
unsafe impl windows_core::Interface for NonClientRegionsChangedEventArgs {
    type Vtable = <INonClientRegionsChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <INonClientRegionsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for NonClientRegionsChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.NonClientRegionsChangedEventArgs";
}
unsafe impl Send for NonClientRegionsChangedEventArgs {}
unsafe impl Sync for NonClientRegionsChangedEventArgs {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PhysicalKeyStatus {
    pub RepeatCount: u32,
    pub ScanCode: u32,
    pub IsExtendedKey: bool,
    pub IsMenuKeyDown: bool,
    pub WasKeyDown: bool,
    pub IsKeyReleased: bool,
}
impl windows_core::imp::TypeKind for PhysicalKeyStatus {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PhysicalKeyStatus {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Input.PhysicalKeyStatus;u4;u4;b1;b1;b1;b1)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.PhysicalKeyStatus");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerDeviceType(pub i32);
impl PointerDeviceType {
    pub const Touch: Self = Self(0);
    pub const Pen: Self = Self(1);
    pub const Mouse: Self = Self(2);
    pub const Touchpad: Self = Self(3);
}
impl windows_core::imp::TypeKind for PointerDeviceType {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PointerDeviceType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.PointerDeviceType;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.PointerDeviceType");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl PointerEventArgs {
    pub fn CurrentPoint(&self) -> windows_core::Result<PointerPoint> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPoint)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn KeyModifiers(&self) -> windows_core::Result<windows::System::VirtualKeyModifiers> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyModifiers)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetIntermediatePoints(&self) -> windows_core::Result<windows_collections::IVector<PointerPoint>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntermediatePoints)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetIntermediateTransformedPoints<P0>(&self, transform: P0) -> windows_core::Result<windows_collections::IVector<PointerPoint>>
    where
        P0: windows_core::Param<IPointerPointTransform>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntermediateTransformedPoints)(windows_core::Interface::as_raw(self), transform.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for PointerEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerEventArgs>();
}
unsafe impl windows_core::Interface for PointerEventArgs {
    type Vtable = <IPointerEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.PointerEventArgs";
}
unsafe impl Send for PointerEventArgs {}
unsafe impl Sync for PointerEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPoint(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerPoint, windows_core::IUnknown, windows_core::IInspectable);
impl PointerPoint {
    pub fn FrameId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FrameId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInContact(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInContact)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Properties(&self) -> windows_core::Result<PointerPointProperties> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Properties)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Timestamp(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Timestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetTransformedPoint<P0>(&self, transform: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IPointerPointTransform>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformedPoint)(windows_core::Interface::as_raw(self), transform.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn GetCurrentPoint(pointerid: u32) -> windows_core::Result<Self> {
        Self::IPointerPointStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCurrentPoint)(windows_core::Interface::as_raw(this), pointerid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IPointerPointStatics<R, F: FnOnce(&IPointerPointStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerPoint, IPointerPointStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerPoint>();
}
unsafe impl windows_core::Interface for PointerPoint {
    type Vtable = <IPointerPoint as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPoint as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPoint {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPoint";
}
unsafe impl Send for PointerPoint {}
unsafe impl Sync for PointerPoint {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPointProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerPointProperties, windows_core::IUnknown, windows_core::IInspectable);
impl PointerPointProperties {
    pub fn ContactRect(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContactRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsBarrelButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBarrelButtonPressed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsCanceled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCanceled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsEraser(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsEraser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsHorizontalMouseWheel(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsHorizontalMouseWheel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInRange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInRange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsInverted(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInverted)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsLeftButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLeftButtonPressed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsMiddleButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMiddleButtonPressed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsPrimary(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsPrimary)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsRightButtonPressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRightButtonPressed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsXButton1Pressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsXButton1Pressed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsXButton2Pressed(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsXButton2Pressed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MouseWheelDelta(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MouseWheelDelta)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Orientation(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Orientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn PointerUpdateKind(&self) -> windows_core::Result<PointerUpdateKind> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerUpdateKind)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Pressure(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Pressure)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TouchConfidence(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TouchConfidence)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Twist(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Twist)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn XTilt(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).XTilt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn YTilt(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).YTilt)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for PointerPointProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerPointProperties>();
}
unsafe impl windows_core::Interface for PointerPointProperties {
    type Vtable = <IPointerPointProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPointProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPointProperties {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPointProperties";
}
unsafe impl Send for PointerPointProperties {}
unsafe impl Sync for PointerPointProperties {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PointerPredictor(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(PointerPredictor, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(PointerPredictor, windows::Foundation::IClosable);
impl PointerPredictor {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<windows::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn PredictionTime(&self) -> windows_core::Result<windows_time::TimeSpan> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PredictionTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetPredictionTime(&self, value: windows_time::TimeSpan) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPredictionTime)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn GetPredictedPoints<P0>(&self, point: P0) -> windows_core::Result<windows_core::Array<PointerPoint>>
    where
        P0: windows_core::Param<PointerPoint>,
    {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetPredictedPoints)(windows_core::Interface::as_raw(self), point.param().abi(), windows_core::Array::<PointerPoint>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn CreateForInputPointerSource<P0>(inputpointersource: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<InputPointerSource>,
    {
        Self::IPointerPredictorStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateForInputPointerSource)(windows_core::Interface::as_raw(this), inputpointersource.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    fn IPointerPredictorStatics<R, F: FnOnce(&IPointerPredictorStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<PointerPredictor, IPointerPredictorStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for PointerPredictor {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IPointerPredictor>();
}
unsafe impl windows_core::Interface for PointerPredictor {
    type Vtable = <IPointerPredictor as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IPointerPredictor as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for PointerPredictor {
    const NAME: &'static str = "Microsoft.UI.Input.PointerPredictor";
}
unsafe impl Send for PointerPredictor {}
unsafe impl Sync for PointerPredictor {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PointerUpdateKind(pub i32);
impl PointerUpdateKind {
    pub const Other: Self = Self(0);
    pub const LeftButtonPressed: Self = Self(1);
    pub const LeftButtonReleased: Self = Self(2);
    pub const RightButtonPressed: Self = Self(3);
    pub const RightButtonReleased: Self = Self(4);
    pub const MiddleButtonPressed: Self = Self(5);
    pub const MiddleButtonReleased: Self = Self(6);
    pub const XButton1Pressed: Self = Self(7);
    pub const XButton1Released: Self = Self(8);
    pub const XButton2Pressed: Self = Self(9);
    pub const XButton2Released: Self = Self(10);
}
impl windows_core::imp::TypeKind for PointerUpdateKind {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PointerUpdateKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.PointerUpdateKind;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.PointerUpdateKind");
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PopupPointerMode(pub i32);
impl PopupPointerMode {
    pub const Default: Self = Self(0);
    pub const Modal: Self = Self(1);
    pub const LightDismiss: Self = Self(2);
}
impl windows_core::imp::TypeKind for PopupPointerMode {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for PopupPointerMode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.PopupPointerMode;i4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.PopupPointerMode");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ProximityEvaluation {
    pub Score: i32,
    pub AdjustedPoint: windows::Foundation::Point,
}
impl windows_core::imp::TypeKind for ProximityEvaluation {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for ProximityEvaluation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Microsoft.UI.Input.ProximityEvaluation;i4;struct(Windows.Foundation.Point;f4;f4))");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.ProximityEvaluation");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RightTappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RightTappedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl RightTappedEventArgs {
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RightTappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRightTappedEventArgs>();
}
unsafe impl windows_core::Interface for RightTappedEventArgs {
    type Vtable = <IRightTappedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRightTappedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RightTappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.RightTappedEventArgs";
}
unsafe impl Send for RightTappedEventArgs {}
unsafe impl Sync for RightTappedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TappedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TappedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TappedEventArgs {
    pub fn PointerDeviceType(&self) -> windows_core::Result<PointerDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Position(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn TapCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TapCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TappedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITappedEventArgs>();
}
unsafe impl windows_core::Interface for TappedEventArgs {
    type Vtable = <ITappedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITappedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TappedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.TappedEventArgs";
}
unsafe impl Send for TappedEventArgs {}
unsafe impl Sync for TappedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TouchHitTestingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(TouchHitTestingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl TouchHitTestingEventArgs {
    pub fn BoundingBox(&self) -> windows_core::Result<windows::Foundation::Rect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BoundingBox)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Handled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetHandled)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Point(&self) -> windows_core::Result<windows::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Point)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn GetProximityEvaluation(&self) -> windows_core::Result<ProximityEvaluation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProximityEvaluation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetProximityEvaluation(&self, proximityevaluation: ProximityEvaluation) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProximityEvaluation)(windows_core::Interface::as_raw(self), proximityevaluation).ok() }
    }
    pub fn EvaluateProximityToRect(&self, controlboundingbox: windows::Foundation::Rect) -> windows_core::Result<ProximityEvaluation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EvaluateProximityToRect)(windows_core::Interface::as_raw(self), controlboundingbox, &mut result__).map(|| result__)
        }
    }
    pub fn EvaluateProximityToPolygon(&self, controlvertices: &[windows::Foundation::Point]) -> windows_core::Result<ProximityEvaluation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EvaluateProximityToPolygon)(windows_core::Interface::as_raw(self), controlvertices.len().try_into().unwrap(), controlvertices.as_ptr(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for TouchHitTestingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ITouchHitTestingEventArgs>();
}
unsafe impl windows_core::Interface for TouchHitTestingEventArgs {
    type Vtable = <ITouchHitTestingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ITouchHitTestingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for TouchHitTestingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.TouchHitTestingEventArgs";
}
unsafe impl Send for TouchHitTestingEventArgs {}
unsafe impl Sync for TouchHitTestingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VirtualKeyStates(pub u32);
impl VirtualKeyStates {
    pub const None: Self = Self(0);
    pub const Down: Self = Self(1);
    pub const Locked: Self = Self(2);
}
impl windows_core::imp::TypeKind for VirtualKeyStates {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for VirtualKeyStates {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Microsoft.UI.Input.VirtualKeyStates;u4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Microsoft.UI.Input.VirtualKeyStates");
}
impl VirtualKeyStates {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VirtualKeyStates {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VirtualKeyStates {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VirtualKeyStates {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for VirtualKeyStates {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for VirtualKeyStates {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowRectChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowRectChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowRectChangedEventArgs {
    pub fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerScreenPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OldWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OldWindowRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn NewWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewWindowRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for WindowRectChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowRectChangedEventArgs>();
}
unsafe impl windows_core::Interface for WindowRectChangedEventArgs {
    type Vtable = <IWindowRectChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowRectChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowRectChangedEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.WindowRectChangedEventArgs";
}
unsafe impl Send for WindowRectChangedEventArgs {}
unsafe impl Sync for WindowRectChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WindowRectChangingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(WindowRectChangingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl WindowRectChangingEventArgs {
    pub fn PointerScreenPoint(&self) -> windows_core::Result<windows::Graphics::PointInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PointerScreenPoint)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn MoveSizeOperation(&self) -> windows_core::Result<MoveSizeOperation> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveSizeOperation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn OldWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OldWindowRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn NewWindowRect(&self) -> windows_core::Result<windows::Graphics::RectInt32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NewWindowRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetNewWindowRect(&self, value: windows::Graphics::RectInt32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNewWindowRect)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn AllowRectChange(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowRectChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetAllowRectChange(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetAllowRectChange)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn ShowWindow(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShowWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetShowWindow(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetShowWindow)(windows_core::Interface::as_raw(self), value).ok() }
    }
}
impl windows_core::RuntimeType for WindowRectChangingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IWindowRectChangingEventArgs>();
}
unsafe impl windows_core::Interface for WindowRectChangingEventArgs {
    type Vtable = <IWindowRectChangingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IWindowRectChangingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for WindowRectChangingEventArgs {
    const NAME: &'static str = "Microsoft.UI.Input.WindowRectChangingEventArgs";
}
unsafe impl Send for WindowRectChangingEventArgs {}
unsafe impl Sync for WindowRectChangingEventArgs {}
