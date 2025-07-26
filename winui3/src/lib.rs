pub mod bootstrap;

#[rustfmt::skip]
pub mod Microsoft;

#[rustfmt::skip]
pub mod Windows;

#[cfg(feature = "XamlApp")]
mod xaml_app;

#[cfg(feature = "XamlNavigation")]
mod xaml_page;

#[cfg(feature = "XamlNavigation")]
mod xaml_types;

#[cfg(feature = "XamlApp")]
pub use xaml_app::{XamlApp, XamlAppOverrides};

#[cfg(feature = "XamlNavigation")]
pub use xaml_page::{XamlPage, XamlPageOverrides};

#[cfg(feature = "XamlNavigation")]
pub use xaml_types::XamlCustomType;

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

pub trait Activatable {
    fn activate() -> windows_core::Result<windows_core::IInspectable>;
}

#[cfg(feature = "UI_Xaml_Interop")]
pub fn xaml_typename<T: AsRef<str>>(type_name: T) -> Windows::UI::Xaml::Interop::TypeName {
    Windows::UI::Xaml::Interop::TypeName {
        Name: windows_core::HSTRING::from(type_name.as_ref()),
        Kind: Windows::UI::Xaml::Interop::TypeKind::Custom,
    }
}
