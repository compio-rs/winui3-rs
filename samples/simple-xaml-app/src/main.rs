use windows_core::{h, Result};

use winui3::{
    bootstrap::PackageDependency,
    Microsoft::UI::Xaml::{
        Application, ApplicationInitializationCallback, LaunchActivatedEventArgs, Window,
    },
    XamlApp, XamlAppOverrides,
};

fn main() -> Result<()> {
    winui3::init_apartment(winui3::ApartmentType::SingleThreaded)?;

    let _dependency = PackageDependency::initialize()?;

    Application::Start(&ApplicationInitializationCallback::new(|_| {
        let _app = XamlApp::compose(App)?;
        Ok(())
    }))
}

pub struct App;

impl XamlAppOverrides for App {
    fn OnLaunched(
        &self,
        _base: &Application,
        _args: Option<&LaunchActivatedEventArgs>,
    ) -> Result<()> {
        let window = Window::new()?;
        window.SetTitle(h!("Simple XAML app"))?;
        window.Activate()
    }
}
