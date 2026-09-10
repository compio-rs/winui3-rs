#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
pub const DXGI_ERROR_DEVICE_REMOVED: windows_core::HRESULT = windows_core::HRESULT(0x887A0005_u32 as _);
pub const DXGI_ERROR_DEVICE_RESET: windows_core::HRESULT = windows_core::HRESULT(0x887A0007_u32 as _);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_MATRIX_3X2_F {
    pub _11: f32,
    pub _12: f32,
    pub _21: f32,
    pub _22: f32,
    pub _31: f32,
    pub _32: f32,
}
pub type DXGI_SCALING = i32;
pub const DXGI_SCALING_ASPECT_RATIO_STRETCH: DXGI_SCALING = 2;
pub const DXGI_SCALING_NONE: DXGI_SCALING = 1;
pub const DXGI_SCALING_STRETCH: DXGI_SCALING = 0;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Format: Common::DXGI_FORMAT,
    pub Stereo: windows_core::BOOL,
    pub SampleDesc: Common::DXGI_SAMPLE_DESC,
    pub BufferUsage: DXGI_USAGE,
    pub BufferCount: u32,
    pub Scaling: DXGI_SCALING,
    pub SwapEffect: DXGI_SWAP_EFFECT,
    pub AlphaMode: Common::DXGI_ALPHA_MODE,
    pub Flags: DXGI_SWAP_CHAIN_FLAG,
}
pub type DXGI_SWAP_CHAIN_FLAG = i32;
pub type DXGI_SWAP_EFFECT = i32;
pub const DXGI_SWAP_EFFECT_DISCARD: DXGI_SWAP_EFFECT = 0;
pub const DXGI_SWAP_EFFECT_FLIP_DISCARD: DXGI_SWAP_EFFECT = 4;
pub const DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL: DXGI_SWAP_EFFECT = 3;
pub const DXGI_SWAP_EFFECT_SEQUENTIAL: DXGI_SWAP_EFFECT = 1;
pub type DXGI_USAGE = u32;
pub const DXGI_USAGE_BACK_BUFFER: DXGI_USAGE = 64;
pub const DXGI_USAGE_DISCARD_ON_PRESENT: DXGI_USAGE = 512;
pub const DXGI_USAGE_READ_ONLY: DXGI_USAGE = 256;
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: DXGI_USAGE = 32;
pub const DXGI_USAGE_SHADER_INPUT: DXGI_USAGE = 16;
pub const DXGI_USAGE_SHARED: DXGI_USAGE = 128;
pub const DXGI_USAGE_UNORDERED_ACCESS: DXGI_USAGE = 1024;
windows_core::imp::define_interface!(IDXGIAdapter, IDXGIAdapter_Vtbl, 0x2411e7e1_12ac_4ccf_bd14_9798e8534dc0);
impl core::ops::Deref for IDXGIAdapter {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIAdapter, windows_core::IUnknown, IDXGIObject);
impl IDXGIAdapter {
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const windows_core::GUID) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckInterfaceSupport)(windows_core::Interface::as_raw(self), interfacename, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIAdapter_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    EnumOutputs: usize,
    GetDesc: usize,
    pub CheckInterfaceSupport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut i64) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIAdapter {}
unsafe impl Sync for IDXGIAdapter {}
impl windows_core::RuntimeName for IDXGIAdapter {}
windows_core::imp::define_interface!(IDXGIDevice, IDXGIDevice_Vtbl, 0x54ec77fa_1377_44e6_8c32_88fd5f44c84c);
impl core::ops::Deref for IDXGIDevice {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice, windows_core::IUnknown, IDXGIObject);
impl IDXGIDevice {
    pub unsafe fn GetAdapter(&self) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdapter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGPUThreadPriority)(windows_core::Interface::as_raw(self), priority).ok() }
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGPUThreadPriority)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    CreateSurface: usize,
    QueryResourceResidency: usize,
    pub SetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetGPUThreadPriority: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice {}
unsafe impl Sync for IDXGIDevice {}
impl windows_core::RuntimeName for IDXGIDevice {}
windows_core::imp::define_interface!(IDXGIDevice1, IDXGIDevice1_Vtbl, 0x77db970f_6276_48ba_ba28_070143b4392c);
impl core::ops::Deref for IDXGIDevice1 {
    type Target = IDXGIDevice;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDevice1, windows_core::IUnknown, IDXGIObject, IDXGIDevice);
impl IDXGIDevice1 {
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency).ok() }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDevice1_Vtbl {
    pub base__: IDXGIDevice_Vtbl,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDevice1 {}
unsafe impl Sync for IDXGIDevice1 {}
impl windows_core::RuntimeName for IDXGIDevice1 {}
windows_core::imp::define_interface!(IDXGIDeviceSubObject, IDXGIDeviceSubObject_Vtbl, 0x3d3e0379_f9de_4d58_bb6c_18d62992f1a6);
impl core::ops::Deref for IDXGIDeviceSubObject {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDeviceSubObject, windows_core::IUnknown, IDXGIObject);
impl IDXGIDeviceSubObject {
    pub unsafe fn GetDevice<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDeviceSubObject_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIDeviceSubObject {}
unsafe impl Sync for IDXGIDeviceSubObject {}
pub trait IDXGIDeviceSubObject_Impl: IDXGIObject_Impl {
    fn GetDevice(&self, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIDeviceSubObject_Vtbl {
    pub const fn new<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: IDXGIDeviceSubObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDeviceSubObject_Impl::GetDevice(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppdevice)).into()
            }
        }
        Self { base__: IDXGIObject_Vtbl::new::<Identity, OFFSET>(), GetDevice: GetDevice::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as windows_core::Interface>::IID || iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDeviceSubObject {}
windows_core::imp::define_interface!(IDXGIFactory, IDXGIFactory_Vtbl, 0x7b7166ec_21c7_44ae_b21a_c9ae321ae369);
impl core::ops::Deref for IDXGIFactory {
    type Target = IDXGIObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory, windows_core::IUnknown, IDXGIObject);
impl IDXGIFactory {
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdapters)(windows_core::Interface::as_raw(self), adapter, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetWindowAssociation(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWindowAssociation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateSoftwareAdapter(&self, module: super::super::Foundation::HMODULE) -> windows_core::Result<IDXGIAdapter> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSoftwareAdapter)(windows_core::Interface::as_raw(self), module, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory_Vtbl {
    pub base__: IDXGIObject_Vtbl,
    pub EnumAdapters: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    MakeWindowAssociation: usize,
    pub GetWindowAssociation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    CreateSwapChain: usize,
    pub CreateSoftwareAdapter: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HMODULE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIFactory {}
unsafe impl Sync for IDXGIFactory {}
impl windows_core::RuntimeName for IDXGIFactory {}
windows_core::imp::define_interface!(IDXGIFactory1, IDXGIFactory1_Vtbl, 0x770aae78_f26f_4dba_a829_253c83d1b387);
impl core::ops::Deref for IDXGIFactory1 {
    type Target = IDXGIFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory1, windows_core::IUnknown, IDXGIObject, IDXGIFactory);
impl IDXGIFactory1 {
    pub unsafe fn IsCurrent(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsCurrent)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory1_Vtbl {
    pub base__: IDXGIFactory_Vtbl,
    EnumAdapters1: usize,
    pub IsCurrent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
unsafe impl Send for IDXGIFactory1 {}
unsafe impl Sync for IDXGIFactory1 {}
impl windows_core::RuntimeName for IDXGIFactory1 {}
windows_core::imp::define_interface!(IDXGIFactory2, IDXGIFactory2_Vtbl, 0x50c83a1c_e072_4c48_87b0_3630fa36a6d0);
impl core::ops::Deref for IDXGIFactory2 {
    type Target = IDXGIFactory1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIFactory2, windows_core::IUnknown, IDXGIObject, IDXGIFactory, IDXGIFactory1);
impl IDXGIFactory2 {
    pub unsafe fn IsWindowedStereoEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsWindowedStereoEnabled)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RegisterStereoStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterStereoStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterStereoStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterStereoStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
    pub unsafe fn RegisterOcclusionStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusWindow)(windows_core::Interface::as_raw(self), windowhandle, wmsg, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterOcclusionStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterOcclusionStatusEvent)(windows_core::Interface::as_raw(self), hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).UnregisterOcclusionStatus)(windows_core::Interface::as_raw(self), dwcookie);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIFactory2_Vtbl {
    pub base__: IDXGIFactory1_Vtbl,
    pub IsWindowedStereoEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    CreateSwapChainForHwnd: usize,
    CreateSwapChainForCoreWindow: usize,
    GetSharedResourceAdapterLuid: usize,
    pub RegisterStereoStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, *mut u32) -> windows_core::HRESULT,
    pub RegisterStereoStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterStereoStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub RegisterOcclusionStatusWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HWND, u32, *mut u32) -> windows_core::HRESULT,
    pub RegisterOcclusionStatusEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *mut u32) -> windows_core::HRESULT,
    pub UnregisterOcclusionStatus: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    CreateSwapChainForComposition: usize,
}
unsafe impl Send for IDXGIFactory2 {}
unsafe impl Sync for IDXGIFactory2 {}
impl windows_core::RuntimeName for IDXGIFactory2 {}
windows_core::imp::define_interface!(IDXGIObject, IDXGIObject_Vtbl, 0xaec22fb8_76f3_4639_9be0_28eb43a67a2e);
windows_core::imp::interface_hierarchy!(IDXGIObject, windows_core::IUnknown);
impl IDXGIObject {
    pub unsafe fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), name, datasize, pdata).ok() }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, name: *const windows_core::GUID, punknown: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), name, punknown.param().abi()).ok() }
    }
    pub unsafe fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), name, pdatasize as _, pdata as _).ok() }
    }
    pub unsafe fn GetParent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetParent)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGIObject {}
unsafe impl Sync for IDXGIObject {}
pub trait IDXGIObject_Impl: windows_core::IUnknownImpl {
    fn SetPrivateData(&self, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, name: *const windows_core::GUID, punknown: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPrivateData(&self, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetParent(&self, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: IDXGIObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, punknown: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&punknown)).into()
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetPrivateData(this, core::mem::transmute_copy(&name), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn GetParent<Identity: IDXGIObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIObject_Impl::GetParent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppparent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            GetParent: GetParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIObject {}
windows_core::imp::define_interface!(IDXGISurface, IDXGISurface_Vtbl, 0xcafcb56c_6ac3_4889_bf47_9e23bbd260ec);
impl core::ops::Deref for IDXGISurface {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISurface, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISurface {
    pub unsafe fn Unmap(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unmap)(windows_core::Interface::as_raw(self)).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISurface_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    GetDesc: usize,
    Map: usize,
    pub Unmap: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISurface {}
unsafe impl Sync for IDXGISurface {}
impl windows_core::RuntimeName for IDXGISurface {}
windows_core::imp::define_interface!(IDXGISwapChain, IDXGISwapChain_Vtbl, 0x310d36a0_d2e7_4c0a_aa04_6a9d23b8886a);
impl core::ops::Deref for IDXGISwapChain {
    type Target = IDXGIDeviceSubObject;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject);
impl IDXGISwapChain {
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffer, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: DXGI_SWAP_CHAIN_FLAG) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ResizeBuffers)(windows_core::Interface::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok() }
    }
    pub unsafe fn GetLastPresentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastPresentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain_Vtbl {
    pub base__: IDXGIDeviceSubObject_Vtbl,
    Present: usize,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    SetFullscreenState: usize,
    GetFullscreenState: usize,
    GetDesc: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub ResizeBuffers: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, Common::DXGI_FORMAT, DXGI_SWAP_CHAIN_FLAG) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    ResizeBuffers: usize,
    ResizeTarget: usize,
    GetContainingOutput: usize,
    GetFrameStatistics: usize,
    pub GetLastPresentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain {}
unsafe impl Sync for IDXGISwapChain {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain {}
windows_core::imp::define_interface!(IDXGISwapChain1, IDXGISwapChain1_Vtbl, 0x790a45f7_0d42_4876_983a_0a55cfe6f4aa);
impl core::ops::Deref for IDXGISwapChain1 {
    type Target = IDXGISwapChain;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain1, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain);
impl IDXGISwapChain1 {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc1(&self) -> windows_core::Result<DXGI_SWAP_CHAIN_DESC1> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDesc1)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetHwnd(&self) -> windows_core::Result<super::super::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHwnd)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCoreWindow)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    pub unsafe fn IsTemporaryMonoSupported(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsTemporaryMonoSupported)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain1_Vtbl {
    pub base__: IDXGISwapChain_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDesc1: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_SWAP_CHAIN_DESC1) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDesc1: usize,
    GetFullscreenDesc: usize,
    pub GetHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::HWND) -> windows_core::HRESULT,
    pub GetCoreWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    Present1: usize,
    pub IsTemporaryMonoSupported: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    GetRestrictToOutput: usize,
    SetBackgroundColor: usize,
    GetBackgroundColor: usize,
    SetRotation: usize,
    GetRotation: usize,
}
unsafe impl Send for IDXGISwapChain1 {}
unsafe impl Sync for IDXGISwapChain1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain1 {}
windows_core::imp::define_interface!(IDXGISwapChain2, IDXGISwapChain2_Vtbl, 0xa8be2ac4_199f_4946_b331_79599fb98de7);
impl core::ops::Deref for IDXGISwapChain2 {
    type Target = IDXGISwapChain1;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGISwapChain2, windows_core::IUnknown, IDXGIObject, IDXGIDeviceSubObject, IDXGISwapChain, IDXGISwapChain1);
impl IDXGISwapChain2 {
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSourceSize)(windows_core::Interface::as_raw(self), width, height).ok() }
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetSourceSize)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _).ok() }
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMaximumFrameLatency)(windows_core::Interface::as_raw(self), maxlatency).ok() }
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumFrameLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        unsafe { (windows_core::Interface::vtable(self).GetFrameLatencyWaitableObject)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix).ok() }
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetMatrixTransform)(windows_core::Interface::as_raw(self), pmatrix as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGISwapChain2_Vtbl {
    pub base__: IDXGISwapChain1_Vtbl,
    pub SetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetSourceSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub SetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMaximumFrameLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFrameLatencyWaitableObject: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::super::Foundation::HANDLE,
    pub SetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
    pub GetMatrixTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DXGI_MATRIX_3X2_F) -> windows_core::HRESULT,
}
unsafe impl Send for IDXGISwapChain2 {}
unsafe impl Sync for IDXGISwapChain2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IDXGISwapChain2 {}
