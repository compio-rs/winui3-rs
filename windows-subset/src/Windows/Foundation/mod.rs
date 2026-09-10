#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Deferral(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Deferral, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Deferral, IClosable);
impl Deferral {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Complete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Complete)(windows_core::Interface::as_raw(self)).ok() }
    }
    fn IDeferralFactory<R, F: FnOnce(&IDeferralFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Deferral, IDeferralFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Deferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDeferral>();
}
unsafe impl windows_core::Interface for Deferral {
    type Vtable = <IDeferral as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDeferral as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Deferral {
    const NAME: &'static str = "Windows.Foundation.Deferral";
}
unsafe impl Send for Deferral {}
unsafe impl Sync for Deferral {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventHandler<T>(windows_core::IUnknown, core::marker::PhantomData<T>)
where
    T: windows_core::RuntimeType + 'static;
unsafe impl<T: windows_core::RuntimeType + 'static> windows_core::Interface for EventHandler<T> {
    type Vtable = EventHandler_Vtbl<T>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<T: windows_core::RuntimeType + 'static> windows_core::RuntimeType for EventHandler<T> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({9de1c535-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(T::SIGNATURE).push_slice(b")");
}
impl<T: windows_core::RuntimeType + 'static> EventHandler<T> {
    pub fn new<F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&EventHandlerBox::<T, F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
        P1: windows_core::Param<T>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct EventHandler_Vtbl<T>
where
    T: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: windows_core::imp::AbiType<T>) -> windows_core::HRESULT,
    T: core::marker::PhantomData<T>,
}
struct EventHandlerBox<T, F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(T, fn() -> F)>)
where
    T: windows_core::RuntimeType + 'static;
impl<T: windows_core::RuntimeType + 'static, F: Fn(windows_core::Ref<windows_core::IInspectable>, windows_core::Ref<T>) -> windows_core::Result<()> + Send + 'static> EventHandlerBox<T, F> {
    const VTABLE: EventHandler_Vtbl<T> = EventHandler_Vtbl::<T> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<EventHandler<T>, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<EventHandler<T>, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<EventHandler<T>, F>::Release,
        },
        Invoke: Self::Invoke,
        T: core::marker::PhantomData::<T>,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, args: windows_core::imp::AbiType<T>) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<EventHandler<T>, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
        }
    }
}
windows_core::imp::define_interface!(IClosable, IClosable_Vtbl, 0x30d5a829_7fa4_4026_83bb_d75bae4ea99e);
impl windows_core::RuntimeType for IClosable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IClosable");
}
windows_core::imp::interface_hierarchy!(IClosable, windows_core::IUnknown, windows_core::IInspectable);
impl IClosable {
    pub fn Close(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)).ok() }
    }
}
impl windows_core::RuntimeName for IClosable {
    const NAME: &'static str = "Windows.Foundation.IClosable";
}
pub trait IClosable_Impl: windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
}
impl IClosable_Vtbl {
    pub const fn new<Identity: IClosable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IClosable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClosable_Impl::Close(this).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IClosable, OFFSET>(), Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClosable as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClosable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeferral, IDeferral_Vtbl, 0xd6269732_3b7f_46a7_b40b_4fdca2a2c693);
impl windows_core::RuntimeType for IDeferral {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IDeferral");
}
impl windows_core::RuntimeName for IDeferral {
    const NAME: &'static str = "Windows.Foundation.IDeferral";
}
pub trait IDeferral_Impl: IClosable_Impl {
    fn Complete(&self) -> windows_core::Result<()>;
}
impl IDeferral_Vtbl {
    pub const fn new<Identity: IDeferral_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Complete<Identity: IDeferral_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDeferral_Impl::Complete(this).into()
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IDeferral, OFFSET>(), Complete: Complete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDeferral as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferral_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Complete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDeferralFactory, IDeferralFactory_Vtbl, 0x65a1ecc5_3fb5_4832_8ca9_f061b281d13a);
impl windows_core::RuntimeType for IDeferralFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IDeferralFactory");
}
impl windows_core::RuntimeName for IDeferralFactory {
    const NAME: &'static str = "Windows.Foundation.IDeferralFactory";
}
#[repr(C)]
#[doc(hidden)]
pub struct IDeferralFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IMemoryBufferReference, IMemoryBufferReference_Vtbl, 0xfbc4dd29_245b_11e4_af98_689423260cf8);
impl windows_core::RuntimeType for IMemoryBufferReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IMemoryBufferReference");
}
windows_core::imp::interface_hierarchy!(IMemoryBufferReference, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IMemoryBufferReference, IClosable);
impl IMemoryBufferReference {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Capacity)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Closed<F>(&self, handler: F) -> windows_core::Result<windows_core::EventRevoker>
    where
        F: Fn(windows_core::Ref<Self>, windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<()> + Send + 'static,
    {
        let handler = <TypedEventHandler<Self, windows_core::IInspectable>>::new(move |a0, a1| handler(a0, a1));
        unsafe {
            let mut result__ = core::mem::zeroed();
            let token__ = (windows_core::Interface::vtable(self).Closed)(windows_core::Interface::as_raw(self), windows_core::Interface::as_raw(&handler), &mut result__).map(|| result__)?;
            Ok(windows_core::EventRevoker::new(self.clone(), token__, windows_core::Interface::vtable(self).RemoveClosed))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IMemoryBufferReference {
    const NAME: &'static str = "Windows.Foundation.IMemoryBufferReference";
}
pub trait IMemoryBufferReference_Impl: IClosable_Impl {
    fn Capacity(&self) -> windows_core::Result<u32>;
    fn Closed(&self, handler: windows_core::Ref<TypedEventHandler<IMemoryBufferReference, windows_core::IInspectable>>) -> windows_core::Result<i64>;
    fn RemoveClosed(&self, token: i64) -> windows_core::Result<()>;
}
impl IMemoryBufferReference_Vtbl {
    pub const fn new<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capacity<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMemoryBufferReference_Impl::Capacity(this) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Closed<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMemoryBufferReference_Impl::Closed(this, core::mem::transmute_copy(&handler)) {
                    Ok(ok__) => {
                        result__.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveClosed<Identity: IMemoryBufferReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, token: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMemoryBufferReference_Impl::RemoveClosed(this, token).into()
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IMemoryBufferReference, OFFSET>(),
            Capacity: Capacity::<Identity, OFFSET>,
            Closed: Closed::<Identity, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMemoryBufferReference as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Closed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveClosed: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IStringable, IStringable_Vtbl, 0x96369f54_8eb6_48f0_abce_c1b211e627c3);
impl windows_core::RuntimeType for IStringable {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IStringable");
}
windows_core::imp::interface_hierarchy!(IStringable, windows_core::IUnknown, windows_core::IInspectable);
impl IStringable {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToString)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
pub trait IStringable_Impl: windows_core::IUnknownImpl {
    fn ToString(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IStringable_Vtbl {
    pub const fn new<Identity: IStringable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ToString<Identity: IStringable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringable_Impl::ToString(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IStringable, OFFSET>(), ToString: ToString::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStringable as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriEscapeStatics, IUriEscapeStatics_Vtbl, 0xc1d432ba_c824_4452_a7fd_512bc3bbe9a1);
impl windows_core::RuntimeType for IUriEscapeStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriEscapeStatics");
}
impl windows_core::RuntimeName for IUriEscapeStatics {
    const NAME: &'static str = "Windows.Foundation.IUriEscapeStatics";
}
pub trait IUriEscapeStatics_Impl: windows_core::IUnknownImpl {
    fn UnescapeComponent(&self, toUnescape: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>;
    fn EscapeComponent(&self, toEscape: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>;
}
impl IUriEscapeStatics_Vtbl {
    pub const fn new<Identity: IUriEscapeStatics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnescapeComponent<Identity: IUriEscapeStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tounescape: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriEscapeStatics_Impl::UnescapeComponent(this, core::mem::transmute(&tounescape)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EscapeComponent<Identity: IUriEscapeStatics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, toescape: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriEscapeStatics_Impl::EscapeComponent(this, core::mem::transmute(&toescape)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUriEscapeStatics, OFFSET>(),
            UnescapeComponent: UnescapeComponent::<Identity, OFFSET>,
            EscapeComponent: EscapeComponent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriEscapeStatics as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriEscapeStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnescapeComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EscapeComponent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriRuntimeClass, IUriRuntimeClass_Vtbl, 0x9e365e57_48b2_4160_956f_c7385120bbfc);
impl windows_core::RuntimeType for IUriRuntimeClass {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClass");
}
impl windows_core::RuntimeName for IUriRuntimeClass {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClass";
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClass_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Domain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Extension: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Fragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Host: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Password: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Path: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    QueryParsed: usize,
    pub RawUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SchemeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UserName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Port: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Suspicious: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Equals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CombineUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriRuntimeClassFactory, IUriRuntimeClassFactory_Vtbl, 0x44a9796f_723e_4fdf_a218_033e75b0c084);
impl windows_core::RuntimeType for IUriRuntimeClassFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClassFactory");
}
impl windows_core::RuntimeName for IUriRuntimeClassFactory {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassFactory";
}
pub trait IUriRuntimeClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateUri(&self, uri: &windows_core::HSTRING) -> windows_core::Result<Uri>;
    fn CreateWithRelativeUri(&self, baseUri: &windows_core::HSTRING, relativeUri: &windows_core::HSTRING) -> windows_core::Result<Uri>;
}
impl IUriRuntimeClassFactory_Vtbl {
    pub const fn new<Identity: IUriRuntimeClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateUri<Identity: IUriRuntimeClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriRuntimeClassFactory_Impl::CreateUri(this, core::mem::transmute(&uri)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateWithRelativeUri<Identity: IUriRuntimeClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, baseuri: *mut core::ffi::c_void, relativeuri: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriRuntimeClassFactory_Impl::CreateWithRelativeUri(this, core::mem::transmute(&baseuri), core::mem::transmute(&relativeuri)) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUriRuntimeClassFactory, OFFSET>(),
            CreateUri: CreateUri::<Identity, OFFSET>,
            CreateWithRelativeUri: CreateWithRelativeUri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriRuntimeClassFactory as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateWithRelativeUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUriRuntimeClassWithAbsoluteCanonicalUri, IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl, 0x758d9661_221c_480f_a339_50656673f46f);
impl windows_core::RuntimeType for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri");
}
impl windows_core::RuntimeName for IUriRuntimeClassWithAbsoluteCanonicalUri {
    const NAME: &'static str = "Windows.Foundation.IUriRuntimeClassWithAbsoluteCanonicalUri";
}
pub trait IUriRuntimeClassWithAbsoluteCanonicalUri_Impl: windows_core::IUnknownImpl {
    fn AbsoluteCanonicalUri(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn DisplayIri(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
    pub const fn new<Identity: IUriRuntimeClassWithAbsoluteCanonicalUri_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AbsoluteCanonicalUri<Identity: IUriRuntimeClassWithAbsoluteCanonicalUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriRuntimeClassWithAbsoluteCanonicalUri_Impl::AbsoluteCanonicalUri(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisplayIri<Identity: IUriRuntimeClassWithAbsoluteCanonicalUri_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUriRuntimeClassWithAbsoluteCanonicalUri_Impl::DisplayIri(this) {
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
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IUriRuntimeClassWithAbsoluteCanonicalUri, OFFSET>(),
            AbsoluteCanonicalUri: AbsoluteCanonicalUri::<Identity, OFFSET>,
            DisplayIri: DisplayIri::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUriRuntimeClassWithAbsoluteCanonicalUri as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUriRuntimeClassWithAbsoluteCanonicalUri_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AbsoluteCanonicalUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisplayIri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl windows_core::imp::TypeKind for Point {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Point {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.Point");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub X: f32,
    pub Y: f32,
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::imp::TypeKind for Rect {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Rect {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Rect;f4;f4;f4;f4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.Rect");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub Width: f32,
    pub Height: f32,
}
impl windows_core::imp::TypeKind for Size {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Size {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Size;f4;f4)");
    const NAME: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.Size");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TypedEventHandler<TSender, TResult>(windows_core::IUnknown, core::marker::PhantomData<TSender>, core::marker::PhantomData<TResult>)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
unsafe impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> windows_core::Interface for TypedEventHandler<TSender, TResult> {
    type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
    const IID: windows_core::GUID = windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> windows_core::RuntimeType for TypedEventHandler<TSender, TResult> {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new().push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}").push_slice(b";").push_other(TSender::SIGNATURE).push_slice(b";").push_other(TResult::SIGNATURE).push_slice(b")");
}
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static> TypedEventHandler<TSender, TResult> {
    pub fn new<F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()> + Send + 'static>(invoke: F) -> Self {
        let com = windows_core::imp::DelegateBox::<Self, F>::new(&TypedEventHandlerBox::<TSender, TResult, F>::VTABLE, invoke);
        unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
    }
    pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<TSender>,
        P1: windows_core::Param<TResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self), sender.param().abi(), args.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct TypedEventHandler_Vtbl<TSender, TResult>
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static,
{
    base__: windows_core::IUnknown_Vtbl,
    Invoke: unsafe extern "system" fn(this: *mut core::ffi::c_void, sender: windows_core::imp::AbiType<TSender>, args: windows_core::imp::AbiType<TResult>) -> windows_core::HRESULT,
    TSender: core::marker::PhantomData<TSender>,
    TResult: core::marker::PhantomData<TResult>,
}
struct TypedEventHandlerBox<TSender, TResult, F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()> + Send + 'static>(core::marker::PhantomData<(TSender, TResult, fn() -> F)>)
where
    TSender: windows_core::RuntimeType + 'static,
    TResult: windows_core::RuntimeType + 'static;
impl<TSender: windows_core::RuntimeType + 'static, TResult: windows_core::RuntimeType + 'static, F: Fn(windows_core::Ref<TSender>, windows_core::Ref<TResult>) -> windows_core::Result<()> + Send + 'static> TypedEventHandlerBox<TSender, TResult, F> {
    const VTABLE: TypedEventHandler_Vtbl<TSender, TResult> = TypedEventHandler_Vtbl::<TSender, TResult> {
        base__: windows_core::IUnknown_Vtbl {
            QueryInterface: windows_core::imp::DelegateBox::<TypedEventHandler<TSender, TResult>, F>::QueryInterface,
            AddRef: windows_core::imp::DelegateBox::<TypedEventHandler<TSender, TResult>, F>::AddRef,
            Release: windows_core::imp::DelegateBox::<TypedEventHandler<TSender, TResult>, F>::Release,
        },
        Invoke: Self::Invoke,
        TSender: core::marker::PhantomData::<TSender>,
        TResult: core::marker::PhantomData::<TResult>,
    };
    unsafe extern "system" fn Invoke(this: *mut core::ffi::c_void, sender: windows_core::imp::AbiType<TSender>, args: windows_core::imp::AbiType<TResult>) -> windows_core::HRESULT {
        unsafe {
            let this = &mut *(this as *mut *mut core::ffi::c_void as *mut windows_core::imp::DelegateBox<TypedEventHandler<TSender, TResult>, F>);
            (this.invoke)(core::mem::transmute_copy(&sender), core::mem::transmute_copy(&args)).into()
        }
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Uri(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Uri, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(Uri, IStringable);
impl Uri {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IStringable>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UnescapeComponent(tounescape: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnescapeComponent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tounescape), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn EscapeComponent(toescape: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        Self::IUriEscapeStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EscapeComponent)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(toescape), &mut result__).map(|| core::mem::transmute(result__))
        })
    }
    pub fn AbsoluteUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AbsoluteUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisplayUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Domain(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Domain)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Extension(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Extension)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Fragment(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Fragment)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Host(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Host)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Password(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Password)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Path(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Path)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Query(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn RawUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RawUri)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn SchemeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SchemeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn UserName(&self) -> windows_core::Result<windows_core::HSTRING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UserName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Port(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Port)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Suspicious(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Suspicious)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub fn Equals<P0>(&self, puri: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Equals)(windows_core::Interface::as_raw(self), puri.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn CombineUri(&self, relativeuri: &windows_core::HSTRING) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CombineUri)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(relativeuri), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub fn CreateUri(uri: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(uri), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn CreateWithRelativeUri(baseuri: &windows_core::HSTRING, relativeuri: &windows_core::HSTRING) -> windows_core::Result<Self> {
        Self::IUriRuntimeClassFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateWithRelativeUri)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(baseuri), core::mem::transmute_copy(relativeuri), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        })
    }
    pub fn AbsoluteCanonicalUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AbsoluteCanonicalUri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DisplayIri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IUriRuntimeClassWithAbsoluteCanonicalUri>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DisplayIri)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    fn IUriEscapeStatics<R, F: FnOnce(&IUriEscapeStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriEscapeStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IUriRuntimeClassFactory<R, F: FnOnce(&IUriRuntimeClassFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Uri, IUriRuntimeClassFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Uri {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUriRuntimeClass>();
}
unsafe impl windows_core::Interface for Uri {
    type Vtable = <IUriRuntimeClass as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUriRuntimeClass as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Uri {
    const NAME: &'static str = "Windows.Foundation.Uri";
}
unsafe impl Send for Uri {}
unsafe impl Sync for Uri {}
