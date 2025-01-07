pub mod bootstrap;

#[rustfmt::skip]
pub mod Microsoft;

#[rustfmt::skip]
pub mod Windows;

pub enum ApartmentType {
    MultiThreaded,
    SingleThreaded,
}

#[inline]
pub fn init_apartment(apartment_type: ApartmentType) -> windows_core::Result<()> {
    use windows::Win32::System::WinRT::{
        RoInitialize, RO_INIT_MULTITHREADED, RO_INIT_SINGLETHREADED,
    };
    let roinit = match apartment_type {
        ApartmentType::MultiThreaded => RO_INIT_MULTITHREADED,
        ApartmentType::SingleThreaded => RO_INIT_SINGLETHREADED,
    };
    unsafe { RoInitialize(roinit) }
}
