#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreDragInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDragInfo, windows_core::IUnknown, windows_core::IInspectable);
impl CoreDragInfo {
    pub fn Data(&self) -> windows_core::Result<super::super::DataPackageView> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Data)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<super::super::super::super::Foundation::Point> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Position)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn AllowedOperations(&self) -> windows_core::Result<super::super::DataPackageOperation> {
        let this = &windows_core::Interface::cast::<ICoreDragInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AllowedOperations)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreDragInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDragInfo>();
}
unsafe impl windows_core::Interface for CoreDragInfo {
    type Vtable = <ICoreDragInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreDragInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragInfo";
}
unsafe impl Send for CoreDragInfo {}
unsafe impl Sync for CoreDragInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreDragUIOverride(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreDragUIOverride, windows_core::IUnknown, windows_core::IInspectable);
impl CoreDragUIOverride {
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmap<P0>(&self, softwarebitmap: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContentFromSoftwareBitmap)(windows_core::Interface::as_raw(self), softwarebitmap.param().abi()).ok() }
    }
    #[cfg(feature = "Graphics_Imaging")]
    pub fn SetContentFromSoftwareBitmapWithAnchorPoint<P0>(&self, softwarebitmap: P0, anchorpoint: super::super::super::super::Foundation::Point) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContentFromSoftwareBitmapWithAnchorPoint)(windows_core::Interface::as_raw(self), softwarebitmap.param().abi(), anchorpoint).ok() }
    }
    pub fn IsContentVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsContentVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsContentVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsContentVisible)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Caption(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Caption)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SetCaption(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetCaption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsCaptionVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCaptionVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsCaptionVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsCaptionVisible)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn IsGlyphVisible(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsGlyphVisible)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn SetIsGlyphVisible(&self, value: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsGlyphVisible)(windows_core::Interface::as_raw(self), value).ok() }
    }
    pub fn Clear(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)).ok() }
    }
}
impl windows_core::RuntimeType for CoreDragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreDragUIOverride>();
}
unsafe impl windows_core::Interface for CoreDragUIOverride {
    type Vtable = <ICoreDragUIOverride as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreDragUIOverride as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.CoreDragUIOverride";
}
unsafe impl Send for CoreDragUIOverride {}
unsafe impl Sync for CoreDragUIOverride {}
windows_core::imp::define_interface!(ICoreDragInfo, ICoreDragInfo_Vtbl, 0x48353a8b_cb50_464e_9575_cd4e3a7ab028);
impl windows_core::RuntimeType for ICoreDragInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo");
}
impl windows_core::RuntimeName for ICoreDragInfo {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    Modifiers: usize,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::super::Foundation::Point) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragInfo2, ICoreDragInfo2_Vtbl, 0xc54691e5_e6fb_4d74_b4b1_8a3c17f25e9e);
impl windows_core::RuntimeType for ICoreDragInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2");
}
impl windows_core::RuntimeName for ICoreDragInfo2 {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragInfo2";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AllowedOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::DataPackageOperation) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreDragUIOverride, ICoreDragUIOverride_Vtbl, 0x89a85064_3389_4f4f_8897_7e8a3ffb3c93);
impl windows_core::RuntimeType for ICoreDragUIOverride {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride");
}
#[cfg(feature = "Graphics_Imaging")]
impl windows_core::RuntimeName for ICoreDragUIOverride {
    const NAME: &'static str = "Windows.ApplicationModel.DataTransfer.DragDrop.Core.ICoreDragUIOverride";
}
#[cfg(feature = "Graphics_Imaging")]
pub trait ICoreDragUIOverride_Impl: windows_core::IUnknownImpl {
    fn SetContentFromSoftwareBitmap(&self, softwareBitmap: windows_core::Ref<super::super::super::super::Graphics::Imaging::SoftwareBitmap>) -> windows_core::Result<()>;
    fn SetContentFromSoftwareBitmapWithAnchorPoint(&self, softwareBitmap: windows_core::Ref<super::super::super::super::Graphics::Imaging::SoftwareBitmap>, anchorPoint: &super::super::super::super::Foundation::Point) -> windows_core::Result<()>;
    fn IsContentVisible(&self) -> windows_core::Result<bool>;
    fn SetIsContentVisible(&self, value: bool) -> windows_core::Result<()>;
    fn Caption(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetCaption(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn IsCaptionVisible(&self) -> windows_core::Result<bool>;
    fn SetIsCaptionVisible(&self, value: bool) -> windows_core::Result<()>;
    fn IsGlyphVisible(&self) -> windows_core::Result<bool>;
    fn SetIsGlyphVisible(&self, value: bool) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "Graphics_Imaging")]
impl ICoreDragUIOverride_Vtbl {
    pub const fn new<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetContentFromSoftwareBitmap<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, softwarebitmap: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreDragUIOverride_Impl::SetContentFromSoftwareBitmap(this, core::mem::transmute_copy(&softwarebitmap)).into()
            }
        }
        unsafe extern "system" fn SetContentFromSoftwareBitmapWithAnchorPoint<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, softwarebitmap: *mut core::ffi::c_void, anchorpoint: super::super::super::super::Foundation::Point) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreDragUIOverride_Impl::SetContentFromSoftwareBitmapWithAnchorPoint(this, core::mem::transmute_copy(&softwarebitmap), core::mem::transmute(&anchorpoint)).into()
            }
        }
        unsafe extern "system" fn IsContentVisible<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreDragUIOverride_Impl::IsContentVisible(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsContentVisible<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreDragUIOverride_Impl::SetIsContentVisible(this, value).into()
            }
        }
        unsafe extern "system" fn Caption<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreDragUIOverride_Impl::Caption(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCaption<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreDragUIOverride_Impl::SetCaption(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn IsCaptionVisible<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreDragUIOverride_Impl::IsCaptionVisible(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsCaptionVisible<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreDragUIOverride_Impl::SetIsCaptionVisible(this, value).into()
            }
        }
        unsafe extern "system" fn IsGlyphVisible<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICoreDragUIOverride_Impl::IsGlyphVisible(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsGlyphVisible<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreDragUIOverride_Impl::SetIsGlyphVisible(this, value).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ICoreDragUIOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICoreDragUIOverride_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICoreDragUIOverride, OFFSET>(),
            SetContentFromSoftwareBitmap: SetContentFromSoftwareBitmap::<Identity, OFFSET>,
            SetContentFromSoftwareBitmapWithAnchorPoint: SetContentFromSoftwareBitmapWithAnchorPoint::<Identity, OFFSET>,
            IsContentVisible: IsContentVisible::<Identity, OFFSET>,
            SetIsContentVisible: SetIsContentVisible::<Identity, OFFSET>,
            Caption: Caption::<Identity, OFFSET>,
            SetCaption: SetCaption::<Identity, OFFSET>,
            IsCaptionVisible: IsCaptionVisible::<Identity, OFFSET>,
            SetIsCaptionVisible: SetIsCaptionVisible::<Identity, OFFSET>,
            IsGlyphVisible: IsGlyphVisible::<Identity, OFFSET>,
            SetIsGlyphVisible: SetIsGlyphVisible::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICoreDragUIOverride as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreDragUIOverride_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmap: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub SetContentFromSoftwareBitmapWithAnchorPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::super::super::super::Foundation::Point) -> windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    SetContentFromSoftwareBitmapWithAnchorPoint: usize,
    pub IsContentVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsContentVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Caption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsCaptionVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsCaptionVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub IsGlyphVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetIsGlyphVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
