#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
#[inline]
pub unsafe fn D3D11CreateDevice<P0>(padapter: P0, drivertype: super::Direct3D::D3D_DRIVER_TYPE, software: Option<super::super::Foundation::HMODULE>, flags: D3D11_CREATE_DEVICE_FLAG, pfeaturelevels: Option<&[super::Direct3D::D3D_FEATURE_LEVEL]>, sdkversion: u32, ppdevice: Option<*mut Option<ID3D11Device>>, pfeaturelevel: Option<*mut super::Direct3D::D3D_FEATURE_LEVEL>, ppimmediatecontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::Dxgi::IDXGIAdapter>,
{
    windows_core::link!("d3d11.dll" "system" fn D3D11CreateDevice(padapter : *mut core::ffi::c_void, drivertype : super::Direct3D::D3D_DRIVER_TYPE, software : super::super::Foundation::HMODULE, flags : D3D11_CREATE_DEVICE_FLAG, pfeaturelevels : *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels : u32, sdkversion : u32, ppdevice : *mut *mut core::ffi::c_void, pfeaturelevel : *mut super::Direct3D::D3D_FEATURE_LEVEL, ppimmediatecontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { D3D11CreateDevice(padapter.param().abi(), drivertype, software.unwrap_or(core::mem::zeroed()) as _, flags, pfeaturelevels.map_or(core::ptr::null(), |slice| slice.as_ptr()), pfeaturelevels.map_or(0, |slice| slice.len().try_into().unwrap()), sdkversion, ppdevice.unwrap_or(core::mem::zeroed()) as _, pfeaturelevel.unwrap_or(core::mem::zeroed()) as _, ppimmediatecontext.unwrap_or(core::mem::zeroed()) as _).ok() }
}
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 32;
pub const D3D11_CREATE_DEVICE_DEBUG: D3D11_CREATE_DEVICE_FLAG = 2;
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: D3D11_CREATE_DEVICE_FLAG = 64;
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: D3D11_CREATE_DEVICE_FLAG = 256;
pub type D3D11_CREATE_DEVICE_FLAG = u32;
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: D3D11_CREATE_DEVICE_FLAG = 128;
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: D3D11_CREATE_DEVICE_FLAG = 8;
pub const D3D11_CREATE_DEVICE_SINGLETHREADED: D3D11_CREATE_DEVICE_FLAG = 1;
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: D3D11_CREATE_DEVICE_FLAG = 4;
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: D3D11_CREATE_DEVICE_FLAG = 2048;
pub const D3D11_SDK_VERSION: u32 = 7;
windows_core::imp::define_interface!(ID3D11Device, ID3D11Device_Vtbl, 0xdb6f6ddb_ac77_4e88_8253_819df9bbf140);
windows_core::imp::interface_hierarchy!(ID3D11Device, windows_core::IUnknown);
impl ID3D11Device {
    pub unsafe fn CreateDeferredContext(&self, contextflags: u32, ppdeferredcontext: Option<*mut Option<ID3D11DeviceContext>>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CreateDeferredContext)(windows_core::Interface::as_raw(self), contextflags, ppdeferredcontext.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn OpenSharedResource<T>(&self, hresource: super::super::Foundation::HANDLE, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenSharedResource)(windows_core::Interface::as_raw(self), hresource, &T::IID, result__ as *mut _ as *mut _).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckFormatSupport)(windows_core::Interface::as_raw(self), format, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckMultisampleQualityLevels)(windows_core::Interface::as_raw(self), format, samplecount, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn GetFeatureLevel(&self) -> super::Direct3D::D3D_FEATURE_LEVEL {
        unsafe { (windows_core::Interface::vtable(self).GetFeatureLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCreationFlags)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceRemovedReason)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetImmediateContext(&self) -> windows_core::Result<ID3D11DeviceContext> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImmediateContext)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetExceptionMode)(windows_core::Interface::as_raw(self), raiseflags).ok() }
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetExceptionMode)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11Device_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CreateBuffer: usize,
    CreateTexture1D: usize,
    CreateTexture2D: usize,
    CreateTexture3D: usize,
    CreateShaderResourceView: usize,
    CreateUnorderedAccessView: usize,
    CreateRenderTargetView: usize,
    CreateDepthStencilView: usize,
    CreateInputLayout: usize,
    CreateVertexShader: usize,
    CreateGeometryShader: usize,
    CreateGeometryShaderWithStreamOutput: usize,
    CreatePixelShader: usize,
    CreateHullShader: usize,
    CreateDomainShader: usize,
    CreateComputeShader: usize,
    CreateClassLinkage: usize,
    CreateBlendState: usize,
    CreateDepthStencilState: usize,
    CreateRasterizerState: usize,
    CreateSamplerState: usize,
    CreateQuery: usize,
    CreatePredicate: usize,
    CreateCounter: usize,
    pub CreateDeferredContext: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenSharedResource: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::HANDLE, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckFormatSupport: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckFormatSupport: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CheckMultisampleQualityLevels: unsafe extern "system" fn(*mut core::ffi::c_void, super::Dxgi::Common::DXGI_FORMAT, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CheckMultisampleQualityLevels: usize,
    CheckCounterInfo: usize,
    CheckCounter: usize,
    CheckFeatureSupport: usize,
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub GetFeatureLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::Direct3D::D3D_FEATURE_LEVEL,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    GetFeatureLevel: usize,
    pub GetCreationFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetDeviceRemovedReason: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetImmediateContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub SetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetExceptionMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
unsafe impl Send for ID3D11Device {}
unsafe impl Sync for ID3D11Device {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl windows_core::RuntimeName for ID3D11Device {}
windows_core::imp::define_interface!(ID3D11DeviceChild, ID3D11DeviceChild_Vtbl, 0x1841e5c8_16b0_489b_bcc8_44cfb0d5deae);
windows_core::imp::interface_hierarchy!(ID3D11DeviceChild, windows_core::IUnknown);
impl ID3D11DeviceChild {
    pub unsafe fn GetDevice(&self) -> windows_core::Result<ID3D11Device> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDevice)(windows_core::Interface::as_raw(self), &mut result__);
            windows_core::imp::Type::from_abi(result__)
        }
    }
    pub unsafe fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: Option<*mut core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateData)(windows_core::Interface::as_raw(self), guid, pdatasize as _, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateData)(windows_core::Interface::as_raw(self), guid, datasize, pdata.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetPrivateDataInterface<P1>(&self, guid: *const windows_core::GUID, pdata: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrivateDataInterface)(windows_core::Interface::as_raw(self), guid, pdata.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceChild_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void),
    pub GetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrivateDataInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
unsafe impl Send for ID3D11DeviceChild {}
unsafe impl Sync for ID3D11DeviceChild {}
pub trait ID3D11DeviceChild_Impl: windows_core::IUnknownImpl {
    fn GetDevice(&self, ppdevice: windows_core::OutRef<ID3D11Device>);
    fn GetPrivateData(&self, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateData(&self, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetPrivateDataInterface(&self, guid: *const windows_core::GUID, pdata: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl ID3D11DeviceChild_Vtbl {
    pub const fn new<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevice<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdevice: *mut *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::GetDevice(this, core::mem::transmute_copy(&ppdevice));
            }
        }
        unsafe extern "system" fn GetPrivateData<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdatasize: *mut u32, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::GetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdatasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateData<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, datasize: u32, pdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::SetPrivateData(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&pdata)).into()
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ID3D11DeviceChild_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guid: *const windows_core::GUID, pdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ID3D11DeviceChild_Impl::SetPrivateDataInterface(this, core::mem::transmute_copy(&guid), core::mem::transmute_copy(&pdata)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDevice: GetDevice::<Identity, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, OFFSET>,
            SetPrivateData: SetPrivateData::<Identity, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ID3D11DeviceChild as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ID3D11DeviceChild {}
windows_core::imp::define_interface!(ID3D11DeviceContext, ID3D11DeviceContext_Vtbl, 0xc0bfa96c_e089_44fb_8eaf_26f8796190da);
impl core::ops::Deref for ID3D11DeviceContext {
    type Target = ID3D11DeviceChild;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ID3D11DeviceContext, windows_core::IUnknown, ID3D11DeviceChild);
impl ID3D11DeviceContext {
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexed)(windows_core::Interface::as_raw(self), indexcount, startindexlocation, basevertexlocation);
        }
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Draw)(windows_core::Interface::as_raw(self), vertexcount, startvertexlocation);
        }
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawIndexedInstanced)(windows_core::Interface::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation);
        }
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawInstanced)(windows_core::Interface::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation);
        }
    }
    pub unsafe fn DrawAuto(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DrawAuto)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Dispatch)(windows_core::Interface::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz);
        }
    }
    pub unsafe fn RSSetScissorRects(&self, prects: Option<&[super::super::Foundation::RECT]>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSSetScissorRects)(windows_core::Interface::as_raw(self), prects.map_or(0, |slice| slice.len().try_into().unwrap()), prects.map_or(core::ptr::null(), |slice| slice.as_ptr()));
        }
    }
    pub unsafe fn RSGetScissorRects(&self, pnumrects: *mut u32, prects: Option<*mut super::super::Foundation::RECT>) {
        unsafe {
            (windows_core::Interface::vtable(self).RSGetScissorRects)(windows_core::Interface::as_raw(self), pnumrects as _, prects.unwrap_or(core::mem::zeroed()) as _);
        }
    }
    pub unsafe fn ClearState(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearState)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Flush(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetContextFlags(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetContextFlags)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ID3D11DeviceContext_Vtbl {
    pub base__: ID3D11DeviceChild_Vtbl,
    VSSetConstantBuffers: usize,
    PSSetShaderResources: usize,
    PSSetShader: usize,
    PSSetSamplers: usize,
    VSSetShader: usize,
    pub DrawIndexed: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, i32),
    pub Draw: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32),
    Map: usize,
    Unmap: usize,
    PSSetConstantBuffers: usize,
    IASetInputLayout: usize,
    IASetVertexBuffers: usize,
    IASetIndexBuffer: usize,
    pub DrawIndexedInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, i32, u32),
    pub DrawInstanced: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32),
    GSSetConstantBuffers: usize,
    GSSetShader: usize,
    IASetPrimitiveTopology: usize,
    VSSetShaderResources: usize,
    VSSetSamplers: usize,
    Begin: usize,
    End: usize,
    GetData: usize,
    SetPredication: usize,
    GSSetShaderResources: usize,
    GSSetSamplers: usize,
    OMSetRenderTargets: usize,
    OMSetRenderTargetsAndUnorderedAccessViews: usize,
    OMSetBlendState: usize,
    OMSetDepthStencilState: usize,
    SOSetTargets: usize,
    pub DrawAuto: unsafe extern "system" fn(*mut core::ffi::c_void),
    DrawIndexedInstancedIndirect: usize,
    DrawInstancedIndirect: usize,
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32),
    DispatchIndirect: usize,
    RSSetState: usize,
    RSSetViewports: usize,
    pub RSSetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::super::Foundation::RECT),
    CopySubresourceRegion: usize,
    CopyResource: usize,
    UpdateSubresource: usize,
    CopyStructureCount: usize,
    ClearRenderTargetView: usize,
    ClearUnorderedAccessViewUint: usize,
    ClearUnorderedAccessViewFloat: usize,
    ClearDepthStencilView: usize,
    GenerateMips: usize,
    SetResourceMinLOD: usize,
    GetResourceMinLOD: usize,
    ResolveSubresource: usize,
    ExecuteCommandList: usize,
    HSSetShaderResources: usize,
    HSSetShader: usize,
    HSSetSamplers: usize,
    HSSetConstantBuffers: usize,
    DSSetShaderResources: usize,
    DSSetShader: usize,
    DSSetSamplers: usize,
    DSSetConstantBuffers: usize,
    CSSetShaderResources: usize,
    CSSetUnorderedAccessViews: usize,
    CSSetShader: usize,
    CSSetSamplers: usize,
    CSSetConstantBuffers: usize,
    VSGetConstantBuffers: usize,
    PSGetShaderResources: usize,
    PSGetShader: usize,
    PSGetSamplers: usize,
    VSGetShader: usize,
    PSGetConstantBuffers: usize,
    IAGetInputLayout: usize,
    IAGetVertexBuffers: usize,
    IAGetIndexBuffer: usize,
    GSGetConstantBuffers: usize,
    GSGetShader: usize,
    IAGetPrimitiveTopology: usize,
    VSGetShaderResources: usize,
    VSGetSamplers: usize,
    GetPredication: usize,
    GSGetShaderResources: usize,
    GSGetSamplers: usize,
    OMGetRenderTargets: usize,
    OMGetRenderTargetsAndUnorderedAccessViews: usize,
    OMGetBlendState: usize,
    OMGetDepthStencilState: usize,
    SOGetTargets: usize,
    RSGetState: usize,
    RSGetViewports: usize,
    pub RSGetScissorRects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut super::super::Foundation::RECT),
    HSGetShaderResources: usize,
    HSGetShader: usize,
    HSGetSamplers: usize,
    HSGetConstantBuffers: usize,
    DSGetShaderResources: usize,
    DSGetShader: usize,
    DSGetSamplers: usize,
    DSGetConstantBuffers: usize,
    CSGetShaderResources: usize,
    CSGetUnorderedAccessViews: usize,
    CSGetShader: usize,
    CSGetSamplers: usize,
    CSGetConstantBuffers: usize,
    pub ClearState: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void),
    GetType: usize,
    pub GetContextFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    FinishCommandList: usize,
}
unsafe impl Send for ID3D11DeviceContext {}
unsafe impl Sync for ID3D11DeviceContext {}
impl windows_core::RuntimeName for ID3D11DeviceContext {}
