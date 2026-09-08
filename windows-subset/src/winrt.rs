pub mod Windows {
    pub mod Foundation {
        #[repr(transparent)]
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct TypedEventHandler<TSender, TResult>(
            windows_core::IUnknown,
            core::marker::PhantomData<TSender>,
            core::marker::PhantomData<TResult>,
        )
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static;
        unsafe impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        > windows_core::Interface for TypedEventHandler<TSender, TResult>
        {
            type Vtable = TypedEventHandler_Vtbl<TSender, TResult>;
            const IID: windows_core::GUID =
                windows_core::GUID::from_signature(<Self as windows_core::RuntimeType>::SIGNATURE);
        }
        impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        > windows_core::RuntimeType for TypedEventHandler<TSender, TResult>
        {
            const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::new()
                .push_slice(b"pinterface({9de1c534-6ae1-11e0-84e1-18a905bcc53f}")
                .push_slice(b";")
                .push_other(TSender::SIGNATURE)
                .push_slice(b";")
                .push_other(TResult::SIGNATURE)
                .push_slice(b")");
        }
        impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        > TypedEventHandler<TSender, TResult>
        {
            pub fn new<
                F: Fn(
                        windows_core::Ref<TSender>,
                        windows_core::Ref<TResult>,
                    ) -> windows_core::Result<()>
                    + Send
                    + 'static,
            >(
                invoke: F,
            ) -> Self {
                let com = windows_core::imp::DelegateBox::<Self, F>::new(
                    &TypedEventHandlerBox::<TSender, TResult, F>::VTABLE,
                    invoke,
                );
                unsafe { core::mem::transmute(windows_core::imp::box_new(com)) }
            }
            pub fn Invoke<P0, P1>(&self, sender: P0, args: P1) -> windows_core::Result<()>
            where
                P0: windows_core::Param<TSender>,
                P1: windows_core::Param<TResult>,
            {
                unsafe {
                    (windows_core::Interface::vtable(self).Invoke)(
                        windows_core::Interface::as_raw(self),
                        sender.param().abi(),
                        args.param().abi(),
                    )
                    .ok()
                }
            }
        }
        #[repr(C)]
        pub struct TypedEventHandler_Vtbl<TSender, TResult>
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
        {
            base__: windows_core::IUnknown_Vtbl,
            Invoke: unsafe extern "system" fn(
                this: *mut core::ffi::c_void,
                sender: windows_core::imp::AbiType<TSender>,
                args: windows_core::imp::AbiType<TResult>,
            ) -> windows_core::HRESULT,
            TSender: core::marker::PhantomData<TSender>,
            TResult: core::marker::PhantomData<TResult>,
        }
        struct TypedEventHandlerBox<
            TSender,
            TResult,
            F: Fn(
                    windows_core::Ref<TSender>,
                    windows_core::Ref<TResult>,
                ) -> windows_core::Result<()>
                + Send
                + 'static,
        >(core::marker::PhantomData<(TSender, TResult, fn() -> F)>)
        where
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static;
        impl<
            TSender: windows_core::RuntimeType + 'static,
            TResult: windows_core::RuntimeType + 'static,
            F: Fn(
                    windows_core::Ref<TSender>,
                    windows_core::Ref<TResult>,
                ) -> windows_core::Result<()>
                + Send
                + 'static,
        > TypedEventHandlerBox<TSender, TResult, F>
        {
            const VTABLE: TypedEventHandler_Vtbl<TSender, TResult> =
                TypedEventHandler_Vtbl::<TSender, TResult> {
                    base__: windows_core::IUnknown_Vtbl {
                        QueryInterface: windows_core::imp::DelegateBox::<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >::QueryInterface,
                        AddRef: windows_core::imp::DelegateBox::<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >::AddRef,
                        Release: windows_core::imp::DelegateBox::<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >::Release,
                    },
                    Invoke: Self::Invoke,
                    TSender: core::marker::PhantomData::<TSender>,
                    TResult: core::marker::PhantomData::<TResult>,
                };
            unsafe extern "system" fn Invoke(
                this: *mut core::ffi::c_void,
                sender: windows_core::imp::AbiType<TSender>,
                args: windows_core::imp::AbiType<TResult>,
            ) -> windows_core::HRESULT {
                unsafe {
                    let this = &mut *(this as *mut *mut core::ffi::c_void
                        as *mut windows_core::imp::DelegateBox<
                            TypedEventHandler<TSender, TResult>,
                            F,
                        >);
                    (this.invoke)(
                        core::mem::transmute_copy(&sender),
                        core::mem::transmute_copy(&args),
                    )
                    .into()
                }
            }
        }
    }
    pub mod UI {
        #[repr(C)]
        #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
        pub struct Color {
            pub A: u8,
            pub R: u8,
            pub G: u8,
            pub B: u8,
        }
        impl windows_core::imp::TypeKind for Color {
            type TypeKind = windows_core::imp::CopyType;
        }
        impl windows_core::RuntimeType for Color {
            const SIGNATURE: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.UI.Color;u1;u1;u1;u1)");
            const NAME: windows_core::imp::ConstBuffer =
                windows_core::imp::ConstBuffer::from_slice(b"Windows.UI.Color");
        }
        pub mod ViewManagement {
            windows_core::imp::define_interface!(
                IUISettings,
                IUISettings_Vtbl,
                0x85361600_1c63_4627_bcb1_3a89e0bc9c55
            );
            impl windows_core::RuntimeType for IUISettings {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings",
                    );
            }
            impl windows_core::RuntimeName for IUISettings {
                const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings";
            }
            #[repr(C)]
            pub struct IUISettings_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
            }
            windows_core::imp::define_interface!(
                IUISettings3,
                IUISettings3_Vtbl,
                0x03021be4_5254_4781_8194_5168f7d06d7b
            );
            impl windows_core::RuntimeType for IUISettings3 {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_interface::<Self>();
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.IUISettings3",
                    );
            }
            impl windows_core::RuntimeName for IUISettings3 {
                const NAME: &'static str = "Windows.UI.ViewManagement.IUISettings3";
            }
            pub trait IUISettings3_Impl: windows_core::IUnknownImpl {
                fn GetColorValue(
                    &self,
                    desiredColor: UIColorType,
                ) -> windows_core::Result<super::Color>;
                fn ColorValuesChanged(
                    &self,
                    handler: windows_core::Ref<
                        super::super::Foundation::TypedEventHandler<
                            UISettings,
                            windows_core::IInspectable,
                        >,
                    >,
                ) -> windows_core::Result<i64>;
                fn RemoveColorValuesChanged(&self, token: i64) -> windows_core::Result<()>;
            }
            impl IUISettings3_Vtbl {
                pub const fn new<Identity: IUISettings3_Impl, const OFFSET: isize>() -> Self {
                    unsafe extern "system" fn GetColorValue<
                        Identity: IUISettings3_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        desiredcolor: UIColorType,
                        result__: *mut super::Color,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            match IUISettings3_Impl::GetColorValue(this, desiredcolor) {
                                Ok(ok__) => {
                                    result__.write(ok__);
                                    windows_core::HRESULT(0)
                                }
                                Err(err) => err.into(),
                            }
                        }
                    }
                    unsafe extern "system" fn ColorValuesChanged<
                        Identity: IUISettings3_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        handler: *mut core::ffi::c_void,
                        result__: *mut i64,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            match IUISettings3_Impl::ColorValuesChanged(
                                this,
                                core::mem::transmute_copy(&handler),
                            ) {
                                Ok(ok__) => {
                                    result__.write(ok__);
                                    windows_core::HRESULT(0)
                                }
                                Err(err) => err.into(),
                            }
                        }
                    }
                    unsafe extern "system" fn RemoveColorValuesChanged<
                        Identity: IUISettings3_Impl,
                        const OFFSET: isize,
                    >(
                        this: *mut core::ffi::c_void,
                        token: i64,
                    ) -> windows_core::HRESULT {
                        unsafe {
                            let this: &Identity =
                                &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                            IUISettings3_Impl::RemoveColorValuesChanged(this, token).into()
                        }
                    }
                    Self {
                        base__: windows_core::IInspectable_Vtbl::new::<
                            Identity,
                            IUISettings3,
                            OFFSET,
                        >(),
                        GetColorValue: GetColorValue::<Identity, OFFSET>,
                        ColorValuesChanged: ColorValuesChanged::<Identity, OFFSET>,
                        RemoveColorValuesChanged: RemoveColorValuesChanged::<Identity, OFFSET>,
                    }
                }
                pub fn matches(iid: &windows_core::GUID) -> bool {
                    iid == &<IUISettings3 as windows_core::Interface>::IID
                }
            }
            #[repr(C)]
            pub struct IUISettings3_Vtbl {
                pub base__: windows_core::IInspectable_Vtbl,
                pub GetColorValue: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    UIColorType,
                    *mut super::Color,
                )
                    -> windows_core::HRESULT,
                pub ColorValuesChanged: unsafe extern "system" fn(
                    *mut core::ffi::c_void,
                    *mut core::ffi::c_void,
                    *mut i64,
                )
                    -> windows_core::HRESULT,
                pub RemoveColorValuesChanged:
                    unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
            }
            #[repr(transparent)]
            #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
            pub struct UIColorType(pub i32);
            impl UIColorType {
                pub const Background: Self = Self(0);
                pub const Foreground: Self = Self(1);
                pub const AccentDark3: Self = Self(2);
                pub const AccentDark2: Self = Self(3);
                pub const AccentDark1: Self = Self(4);
                pub const Accent: Self = Self(5);
                pub const AccentLight1: Self = Self(6);
                pub const AccentLight2: Self = Self(7);
                pub const AccentLight3: Self = Self(8);
                pub const Complement: Self = Self(9);
            }
            impl windows_core::imp::TypeKind for UIColorType {
                type TypeKind = windows_core::imp::CopyType;
            }
            impl windows_core::RuntimeType for UIColorType {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"enum(Windows.UI.ViewManagement.UIColorType;i4)",
                    );
                const NAME: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::from_slice(
                        b"Windows.UI.ViewManagement.UIColorType",
                    );
            }
            #[repr(transparent)]
            #[derive(Clone, Debug, Eq, PartialEq)]
            pub struct UISettings(windows_core::IUnknown);
            windows_core::imp::interface_hierarchy!(
                UISettings,
                windows_core::IUnknown,
                windows_core::IInspectable
            );
            impl UISettings {
                pub fn new() -> windows_core::Result<Self> {
                    Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
                }
                fn IActivationFactory<
                    R,
                    F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>,
                >(
                    callback: F,
                ) -> windows_core::Result<R> {
                    static SHARED: windows_core::imp::FactoryCache<
                        UISettings,
                        windows_core::imp::IGenericFactory,
                    > = windows_core::imp::FactoryCache::new();
                    SHARED.call(callback)
                }
                pub fn GetColorValue(
                    &self,
                    desiredcolor: UIColorType,
                ) -> windows_core::Result<super::Color> {
                    let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        (windows_core::Interface::vtable(this).GetColorValue)(
                            windows_core::Interface::as_raw(this),
                            desiredcolor,
                            &mut result__,
                        )
                        .map(|| result__)
                    }
                }
                pub fn ColorValuesChanged<F>(
                    &self,
                    handler: F,
                ) -> windows_core::Result<windows_core::EventRevoker>
                where
                    F: Fn(
                            windows_core::Ref<Self>,
                            windows_core::Ref<windows_core::IInspectable>,
                        ) -> windows_core::Result<()>
                        + Send
                        + 'static,
                {
                    let this = &windows_core::Interface::cast::<IUISettings3>(self)?;
                    let handler = <super::super::Foundation::TypedEventHandler<
                        Self,
                        windows_core::IInspectable,
                    >>::new(move |a0, a1| handler(a0, a1));
                    unsafe {
                        let mut result__ = core::mem::zeroed();
                        let token__ = (windows_core::Interface::vtable(this).ColorValuesChanged)(
                            windows_core::Interface::as_raw(this),
                            windows_core::Interface::as_raw(&handler),
                            &mut result__,
                        )
                        .map(|| result__)?;
                        Ok(windows_core::EventRevoker::new(
                            this.clone(),
                            token__,
                            windows_core::Interface::vtable(this).RemoveColorValuesChanged,
                        ))
                    }
                }
            }
            impl windows_core::RuntimeType for UISettings {
                const SIGNATURE: windows_core::imp::ConstBuffer =
                    windows_core::imp::ConstBuffer::for_class::<Self, IUISettings>();
            }
            unsafe impl windows_core::Interface for UISettings {
                type Vtable = <IUISettings as windows_core::Interface>::Vtable;
                const IID: windows_core::GUID = <IUISettings as windows_core::Interface>::IID;
            }
            impl windows_core::RuntimeName for UISettings {
                const NAME: &'static str = "Windows.UI.ViewManagement.UISettings";
            }
            unsafe impl Send for UISettings {}
            unsafe impl Sync for UISettings {}
        }
    }
}
