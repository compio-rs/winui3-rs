use std::{ffi::c_void, ops::Deref, ptr::NonNull};

use windows::Win32::Foundation::E_NOINTERFACE;
use windows_core::{
    factory, imp::WeakRefCount, ComObject, ComObjectInner, ComObjectInterface, IInspectable,
    IInspectable_Vtbl, IUnknownImpl, Interface, InterfaceRef, Result, RuntimeName, Type, TypeKind,
    GUID, HRESULT,
};

#[repr(transparent)]
pub struct Compose<T> {
    inner: T,
}

pub type CreateInstanceFn = unsafe extern "system" fn(
    this: *mut c_void,
    outer: *mut c_void,
    base: *mut *mut c_void,
    result: *mut *mut c_void,
) -> HRESULT;

pub trait ChildClass: ComObjectInner {
    type BaseType: RuntimeName + TypeKind + Type<Self::BaseType, Abi = *mut c_void>;
    type FactoryInterface: Interface;

    fn create_interface_fn(
        vtable: &<Self::FactoryInterface as Interface>::Vtable,
    ) -> CreateInstanceFn;

    fn identity_vtable(vtable: &mut Self::Outer) -> &mut &'static IInspectable_Vtbl;

    fn ref_count(vtable: &Self::Outer) -> &WeakRefCount;

    fn into_outer(self) -> Self::Outer;
}

impl<T: ChildClass> Compose<T>
where
    T::Outer: ComObjectInterface<IInspectable>,
{
    pub fn compose(inner: T) -> Result<T::BaseType> {
        Self::compose_with(inner, &factory::<T::BaseType, T::FactoryInterface>()?)
    }

    pub fn compose_with(inner: T, factory: &T::FactoryInterface) -> Result<T::BaseType> {
        let this = Self { inner };
        unsafe {
            let outer: IInspectable = this.into();
            let outer__ = outer.as_raw();
            let r#impl = outer__ as *mut Compose_Impl<T>;
            let base__ = &mut (*r#impl).base;
            let mut result__ = std::ptr::null_mut();
            T::create_interface_fn(factory.vtable())(
                factory.as_raw(),
                outer__,
                base__ as *mut _ as _,
                &mut result__,
            )
            .and_then(|| Type::from_abi(result__))
        }
    }

    /// Get the base object. You should query the *Overrides interface.
    /// # Safety
    /// The vtable should be created by `Compose::compose*`.
    pub unsafe fn base(vtable: &T::Outer) -> &IInspectable {
        (*(vtable as *const T::Outer as *const Compose_Impl<T>))
            .base
            .as_ref()
            .unwrap_unchecked()
    }
}

impl<T: ChildClass> Compose<T> {
    #[inline(always)]
    fn into_outer(self) -> Compose_Impl<T> {
        let mut vtable = self.inner.into_outer();
        *T::identity_vtable(&mut vtable) = &Compose_Impl::<T>::VTABLE_IDENTITY;
        Compose_Impl { vtable, base: None }
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub struct Compose_Impl<T: ComObjectInner> {
    vtable: T::Outer,
    base: Option<IInspectable>,
}

impl<T: ComObjectInner> Deref for Compose_Impl<T>
where
    T::Outer: Deref,
{
    type Target = <T::Outer as Deref>::Target;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.vtable.deref()
    }
}

impl<T: ChildClass> Compose_Impl<T> {
    const VTABLE_IDENTITY: IInspectable_Vtbl =
        IInspectable_Vtbl::new::<Compose_Impl<T>, T::BaseType, 0>();
}

impl<T: ChildClass> IUnknownImpl for Compose_Impl<T> {
    type Impl = Compose<T>;

    #[inline(always)]
    fn get_impl(&self) -> &Self::Impl {
        unsafe { &*(self.vtable.get_impl() as *const T as *const Compose<T>) }
    }

    #[inline(always)]
    fn get_impl_mut(&mut self) -> &mut Self::Impl {
        unsafe { &mut *(self.vtable.get_impl_mut() as *mut T as *mut Compose<T>) }
    }

    #[inline(always)]
    fn into_inner(self) -> Self::Impl {
        Compose {
            inner: self.vtable.into_inner(),
        }
    }

    #[inline(always)]
    unsafe fn QueryInterface(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        let res = self.vtable.QueryInterface(iid, interface);
        if res == E_NOINTERFACE {
            if let Some(base) = &self.base {
                return base.query(iid, interface);
            }
        }
        res
    }

    #[inline(always)]
    fn AddRef(&self) -> u32 {
        T::ref_count(&self.vtable).add_ref()
    }

    #[inline(always)]
    unsafe fn Release(self_: *mut Self) -> u32 {
        let remaining = T::ref_count(&(*self_).vtable).release();
        if remaining == 0 {
            _ = Box::from_raw(self_);
        }
        remaining
    }

    #[inline(always)]
    fn is_reference_count_one(&self) -> bool {
        self.vtable.is_reference_count_one()
    }

    #[inline(always)]
    unsafe fn GetTrustLevel(&self, value: *mut i32) -> HRESULT {
        self.vtable.GetTrustLevel(value)
    }

    #[inline(always)]
    fn to_object(&self) -> ComObject<Self::Impl> {
        self.AddRef();
        unsafe { ComObject::from_raw(NonNull::from(self)) }
    }
}

impl<T: ChildClass> ComObjectInner for Compose<T> {
    type Outer = Compose_Impl<T>;

    fn into_object(self) -> ComObject<Self> {
        let boxed = Box::new(self.into_outer());
        unsafe {
            let ptr = Box::into_raw(boxed);
            ComObject::from_raw(NonNull::new_unchecked(ptr))
        }
    }
}

impl<T: ChildClass> From<Compose<T>> for IInspectable
where
    T::Outer: ComObjectInterface<IInspectable>,
{
    #[inline(always)]
    fn from(value: Compose<T>) -> Self {
        let com_object = ComObject::new(value);
        com_object.into_interface()
    }
}

impl<T: ChildClass> ComObjectInterface<IInspectable> for Compose_Impl<T>
where
    T::Outer: ComObjectInterface<IInspectable>,
{
    #[inline(always)]
    fn as_interface_ref(&self) -> InterfaceRef<'_, IInspectable> {
        self.vtable.as_interface_ref()
    }
}
