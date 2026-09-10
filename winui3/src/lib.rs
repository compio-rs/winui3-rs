mod bootstrap;
pub use bootstrap::*;

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
pub mod Microsoft;

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "native")]
mod native;
#[cfg(feature = "native")]
pub use native::*;

pub enum ApartmentType {
    MultiThreaded,
    SingleThreaded,
}

#[inline]
pub fn init_apartment(apartment_type: ApartmentType) -> windows_core::Result<()> {
    use windows::Win32::System::WinRT;

    let roinit = match apartment_type {
        ApartmentType::MultiThreaded => WinRT::RO_INIT_MULTITHREADED,
        ApartmentType::SingleThreaded => WinRT::RO_INIT_SINGLETHREADED,
    };
    unsafe { WinRT::RoInitialize(roinit) }
}
