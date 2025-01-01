#![allow(non_snake_case)]

#[rustfmt::skip]
mod bindings {
    pub mod Interop;
    pub mod WinUI;
}

pub mod WUX {
    pub use crate::bindings::Interop;
}

pub mod bootstrap;

pub use bindings::WinUI::Graphics;
pub use bindings::WinUI::Windows;
pub use bindings::WinUI::UI;

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
