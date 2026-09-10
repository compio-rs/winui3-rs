#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Certificate(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Certificate, windows_core::IUnknown, windows_core::IInspectable);
impl Certificate {
    pub fn BuildChainAsync<P0>(&self, certificates: P0) -> windows_core::Result<windows_future::IAsyncOperation<CertificateChain>>
    where
        P0: windows_core::Param<windows_collections::IIterable<Self>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuildChainAsync)(windows_core::Interface::as_raw(self), certificates.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SerialNumber(&self) -> windows_core::Result<windows_core::Array<u8>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).SerialNumber)(windows_core::Interface::as_raw(self), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetHashValue(&self) -> windows_core::Result<windows_core::Array<u8>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetHashValue)(windows_core::Interface::as_raw(self), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetHashValueWithAlgorithm(&self, hashalgorithmname: &windows_core::HSTRING) -> windows_core::Result<windows_core::Array<u8>> {
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(self).GetHashValueWithAlgorithm)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(hashalgorithmname), windows_core::Array::<u8>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn Subject(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Subject)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Issuer(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Issuer)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn HasPrivateKey(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasPrivateKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn IsStronglyProtected(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsStronglyProtected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ValidFrom(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidFrom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ValidTo(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ValidTo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn EnhancedKeyUsages(&self) -> windows_core::Result<windows_collections::IVectorView<windows_core::HSTRING>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnhancedKeyUsages)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SetFriendlyName(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFriendlyName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)).ok() }
    }
    pub fn FriendlyName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsSecurityDeviceBound(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSecurityDeviceBound)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn KeyAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SignatureAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignatureAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SignatureHashAlgorithmName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SignatureHashAlgorithmName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn IsPerUser(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsPerUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StoreName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn KeyStorageProviderName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<ICertificate3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeyStorageProviderName)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn ICertificateFactory<R, F: FnOnce(&ICertificateFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Certificate, ICertificateFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Certificate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificate>();
}
unsafe impl windows_core::Interface for Certificate {
    type Vtable = <ICertificate as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICertificate as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Certificate {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.Certificate";
}
unsafe impl Send for Certificate {}
unsafe impl Sync for Certificate {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertificateChain(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CertificateChain, windows_core::IUnknown, windows_core::IInspectable);
impl CertificateChain {
    pub fn GetCertificates(&self, includeroot: bool) -> windows_core::Result<windows_collections::IVectorView<Certificate>> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCertificates)(windows_core::Interface::as_raw(self), includeroot, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for CertificateChain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICertificateChain>();
}
unsafe impl windows_core::Interface for CertificateChain {
    type Vtable = <ICertificateChain as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICertificateChain as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CertificateChain {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.CertificateChain";
}
unsafe impl Send for CertificateChain {}
unsafe impl Sync for CertificateChain {}
windows_core::imp::define_interface!(ICertificate, ICertificate_Vtbl, 0x333f740c_04d8_43b3_b278_8c5fcc9be5a0);
impl windows_core::RuntimeType for ICertificate {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Security.Cryptography.Certificates.ICertificate");
}
impl windows_core::RuntimeName for ICertificate {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub BuildChainAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    BuildChainWithParametersAsync: usize,
    pub SerialNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetHashValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub GetHashValueWithAlgorithm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    GetCertificateBlob: usize,
    pub Subject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Issuer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasPrivateKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub IsStronglyProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ValidFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
    pub ValidTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
    pub EnhancedKeyUsages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificate2, ICertificate2_Vtbl, 0x17b8374c_8a25_4d96_a492_8fc29ac4fda6);
impl windows_core::RuntimeType for ICertificate2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Security.Cryptography.Certificates.ICertificate2");
}
impl windows_core::RuntimeName for ICertificate2 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate2";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSecurityDeviceBound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    KeyUsages: usize,
    pub KeyAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SignatureAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SignatureHashAlgorithmName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificate3, ICertificate3_Vtbl, 0xbe51a966_ae5f_4652_ace7_c6d7e7724cf3);
impl windows_core::RuntimeType for ICertificate3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Security.Cryptography.Certificates.ICertificate3");
}
impl windows_core::RuntimeName for ICertificate3 {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificate3";
}
pub trait ICertificate3_Impl: windows_core::IUnknownImpl {
    fn IsPerUser(&self) -> windows_core::Result<bool>;
    fn StoreName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn KeyStorageProviderName(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl ICertificate3_Vtbl {
    pub const fn new<Identity: ICertificate3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsPerUser<Identity: ICertificate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertificate3_Impl::IsPerUser(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StoreName<Identity: ICertificate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertificate3_Impl::StoreName(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyStorageProviderName<Identity: ICertificate3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICertificate3_Impl::KeyStorageProviderName(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, ICertificate3, OFFSET>(),
            IsPerUser: IsPerUser::<Identity, OFFSET>,
            StoreName: StoreName::<Identity, OFFSET>,
            KeyStorageProviderName: KeyStorageProviderName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICertificate3 as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificate3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsPerUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub StoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeyStorageProviderName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateChain, ICertificateChain_Vtbl, 0x20bf5385_3691_4501_a62c_fd97278b31ee);
impl windows_core::RuntimeType for ICertificateChain {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Security.Cryptography.Certificates.ICertificateChain");
}
impl windows_core::RuntimeName for ICertificateChain {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateChain";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateChain_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    Validate: usize,
    ValidateWithParameters: usize,
    pub GetCertificates: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICertificateFactory, ICertificateFactory_Vtbl, 0x17b4221c_4baf_44a2_9608_04fb62b16942);
impl windows_core::RuntimeType for ICertificateFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Security.Cryptography.Certificates.ICertificateFactory");
}
impl windows_core::RuntimeName for ICertificateFactory {
    const NAME: &'static str = "Windows.Security.Cryptography.Certificates.ICertificateFactory";
}
#[repr(C)]
#[doc(hidden)]
pub struct ICertificateFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
