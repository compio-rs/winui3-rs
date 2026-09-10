#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasicProperties(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(BasicProperties, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(BasicProperties, IStorageItemExtraProperties);
impl BasicProperties {
    pub fn Size(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Size)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn DateModified(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DateModified)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn ItemDate(&self) -> windows_core::Result<windows_time::DateTime> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemDate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn RetrievePropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RetrievePropertiesAsync)(windows_core::Interface::as_raw(this), propertiestoretrieve.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SavePropertiesAsync<P0>(&self, propertiestosave: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>,
    {
        let this = &windows_core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SavePropertiesAsync)(windows_core::Interface::as_raw(this), propertiestosave.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        let this = &windows_core::Interface::cast::<IStorageItemExtraProperties>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SavePropertiesAsyncOverloadDefault)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for BasicProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBasicProperties>();
}
unsafe impl windows_core::Interface for BasicProperties {
    type Vtable = <IBasicProperties as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBasicProperties as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for BasicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.BasicProperties";
}
windows_core::imp::define_interface!(IBasicProperties, IBasicProperties_Vtbl, 0xd05d55db_785e_4a66_be02_9beec58aea81);
impl windows_core::RuntimeType for IBasicProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.FileProperties.IBasicProperties");
}
impl windows_core::RuntimeName for IBasicProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.IBasicProperties";
}
pub trait IBasicProperties_Impl: windows_core::IUnknownImpl {
    fn Size(&self) -> windows_core::Result<u64>;
    fn DateModified(&self) -> windows_core::Result<windows_time::DateTime>;
    fn ItemDate(&self) -> windows_core::Result<windows_time::DateTime>;
}
impl IBasicProperties_Vtbl {
    pub const fn new<Identity: IBasicProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Size<Identity: IBasicProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicProperties_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DateModified<Identity: IBasicProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_time::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicProperties_Impl::DateModified(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemDate<Identity: IBasicProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_time::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBasicProperties_Impl::ItemDate(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBasicProperties, OFFSET>(),
            Size: Size::<Identity, OFFSET>,
            DateModified: DateModified::<Identity, OFFSET>,
            ItemDate: ItemDate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBasicProperties as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBasicProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub DateModified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
    pub ItemDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_time::DateTime) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStorageItemExtraProperties, IStorageItemExtraProperties_Vtbl, 0xc54361b2_54cd_432b_bdbc_4b19c4b470d7);
impl windows_core::RuntimeType for IStorageItemExtraProperties {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Storage.FileProperties.IStorageItemExtraProperties");
}
windows_core::imp::interface_hierarchy!(IStorageItemExtraProperties, windows_core::IUnknown, windows_core::IInspectable);
impl IStorageItemExtraProperties {
    pub fn RetrievePropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_core::HSTRING>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RetrievePropertiesAsync)(windows_core::Interface::as_raw(self), propertiestoretrieve.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SavePropertiesAsync<P0>(&self, propertiestosave: P0) -> windows_core::Result<windows_future::IAsyncAction>
    where
        P0: windows_core::Param<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SavePropertiesAsync)(windows_core::Interface::as_raw(self), propertiestosave.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn SavePropertiesAsyncOverloadDefault(&self) -> windows_core::Result<windows_future::IAsyncAction> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SavePropertiesAsyncOverloadDefault)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IStorageItemExtraProperties {
    const NAME: &'static str = "Windows.Storage.FileProperties.IStorageItemExtraProperties";
}
pub trait IStorageItemExtraProperties_Impl: windows_core::IUnknownImpl {
    fn RetrievePropertiesAsync(&self, propertiesToRetrieve: windows_core::Ref<windows_collections::IIterable<windows_core::HSTRING>>) -> windows_core::Result<windows_future::IAsyncOperation<windows_collections::IMap<windows_core::HSTRING, windows_core::IInspectable>>>;
    fn SavePropertiesAsync(&self, propertiesToSave: windows_core::Ref<windows_collections::IIterable<windows_collections::IKeyValuePair<windows_core::HSTRING, windows_core::IInspectable>>>) -> windows_core::Result<windows_future::IAsyncAction>;
    fn SavePropertiesAsyncOverloadDefault(&self) -> windows_core::Result<windows_future::IAsyncAction>;
}
impl IStorageItemExtraProperties_Vtbl {
    pub const fn new<Identity: IStorageItemExtraProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RetrievePropertiesAsync<Identity: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiestoretrieve: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemExtraProperties_Impl::RetrievePropertiesAsync(this, core::mem::transmute_copy(&propertiestoretrieve)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SavePropertiesAsync<Identity: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertiestosave: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemExtraProperties_Impl::SavePropertiesAsync(this, core::mem::transmute_copy(&propertiestosave)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SavePropertiesAsyncOverloadDefault<Identity: IStorageItemExtraProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorageItemExtraProperties_Impl::SavePropertiesAsyncOverloadDefault(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStorageItemExtraProperties, OFFSET>(),
            RetrievePropertiesAsync: RetrievePropertiesAsync::<Identity, OFFSET>,
            SavePropertiesAsync: SavePropertiesAsync::<Identity, OFFSET>,
            SavePropertiesAsyncOverloadDefault: SavePropertiesAsyncOverloadDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorageItemExtraProperties as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemExtraProperties_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RetrievePropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SavePropertiesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SavePropertiesAsyncOverloadDefault: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
