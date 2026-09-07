mod bootstrap;
pub use bootstrap::*;

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;
pub use bindings::*;

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub(crate) mod internal;

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod native;
pub use native::*;

pub enum ApartmentType {
    MultiThreaded,
    SingleThreaded,
}

#[inline]
pub fn init_apartment(apartment_type: ApartmentType) -> windows_core::Result<()> {
    let roinit = match apartment_type {
        ApartmentType::MultiThreaded => internal::RO_INIT_MULTITHREADED,
        ApartmentType::SingleThreaded => internal::RO_INIT_SINGLETHREADED,
    };
    unsafe { internal::RoInitialize(roinit).ok() }
}
