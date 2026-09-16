#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
    unsafe { RoInitialize(inittype).ok() }
}
windows_core::imp::define_interface!(IBufferByteAccess, IBufferByteAccess_Vtbl, 0x905a0fef_bc53_11df_8c49_001e4fc686da);
windows_core::imp::interface_hierarchy!(IBufferByteAccess, windows_core::IUnknown);
impl IBufferByteAccess {
    pub unsafe fn Buffer(&self) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IBufferByteAccess_Impl: windows_core::IUnknownImpl {
    fn Buffer(&self) -> windows_core::Result<*mut u8>;
}
impl IBufferByteAccess_Vtbl {
    pub const fn new<Identity: IBufferByteAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Buffer<Identity: IBufferByteAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBufferByteAccess_Impl::Buffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Buffer: Buffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBufferByteAccess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBufferByteAccess {}
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0;
pub type RO_INIT_TYPE = i32;
