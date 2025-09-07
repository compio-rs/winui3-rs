#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    ISwapChainPanelNative,
    ISwapChainPanelNative_Vtbl,
    0x63aad0b8_7c24_40ff_85a8_640d944cc325
);
windows_core::imp::interface_hierarchy!(ISwapChainPanelNative, windows_core::IUnknown);

impl ISwapChainPanelNative {
    pub unsafe fn SetSwapChain<P0>(&self, swapChain: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SetSwapChain)(
                windows_core::Interface::as_raw(self),
                swapChain.param().abi(),
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSwapChain: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait ISwapChainPanelNative_Impl: windows_core::IUnknownImpl {
    fn SetSwapChain(
        &self,
        swapChain: windows_core::Ref<'_, windows::Win32::Graphics::Dxgi::IDXGISwapChain>,
    ) -> windows_core::Result<()>;
}
impl ISwapChainPanelNative_Vtbl {
    pub const fn new<Identity: ISwapChainPanelNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSwapChain<
            Identity: ISwapChainPanelNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            swapChain: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISwapChainPanelNative_Impl::SetSwapChain(
                    this,
                    core::mem::transmute_copy(&swapChain),
                ) {
                    Ok(()) => windows_core::HRESULT(0),
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSwapChain: SetSwapChain::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISwapChainPanelNative as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISwapChainPanelNative {}
unsafe impl Send for ISwapChainPanelNative {}
unsafe impl Sync for ISwapChainPanelNative {}

windows_core::imp::define_interface!(
    IWindowNative,
    IWindowNative_Vtbl,
    0xeecdbf0e_bae9_4cb6_a68e_9598e1cb57bb
);
windows_core::imp::interface_hierarchy!(IWindowNative, windows_core::IUnknown);

impl IWindowNative {
    pub unsafe fn get_WindowHandle(
        &self,
    ) -> windows_core::Result<windows::Win32::Foundation::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowHandle)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .ok()?;
            Ok(result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowNative_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_WindowHandle: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows::Win32::Foundation::HWND,
    ) -> windows_core::HRESULT,
}
pub trait IWindowNative_Impl: windows_core::IUnknownImpl {
    fn get_WindowHandle(&self) -> windows_core::Result<windows::Win32::Foundation::HWND>;
}
impl IWindowNative_Vtbl {
    pub const fn new<Identity: IWindowNative_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_WindowHandle<
            Identity: IWindowNative_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            hWnd: *mut windows::Win32::Foundation::HWND,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowNative_Impl::get_WindowHandle(this) {
                    Ok(result) => {
                        *hWnd = result;
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_WindowHandle: get_WindowHandle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowNative as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowNative {}
unsafe impl Send for IWindowNative {}
unsafe impl Sync for IWindowNative {}
