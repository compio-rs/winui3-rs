pub mod bootstrap;

#[rustfmt::skip]
pub mod Microsoft;

#[rustfmt::skip]
pub mod Windows;

#[cfg(feature = "native")]
mod native;

#[cfg(feature = "native")]
pub use native::*;

mod compose;
pub use compose::*;

pub enum ApartmentType {
    MultiThreaded,
    SingleThreaded,
}

#[inline]
pub fn init_apartment(apartment_type: ApartmentType) -> windows_core::Result<()> {
    let roinit = match apartment_type {
        ApartmentType::MultiThreaded => windows::Win32::System::WinRT::RO_INIT_MULTITHREADED,
        ApartmentType::SingleThreaded => windows::Win32::System::WinRT::RO_INIT_SINGLETHREADED,
    };
    unsafe { windows::Win32::System::WinRT::RoInitialize(roinit) }
}
