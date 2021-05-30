#![windows_subsystem = "console"]

use std::{convert::{TryFrom}};

use bindings::{
    Microsoft,
    Microsoft::UI::Xaml::{
        Application,
        ApplicationInitializationCallback,
        Controls::Button,
        HorizontalAlignment,
        LaunchActivatedEventArgs,
        Window
    }
};

use windows::{HRESULT, IInspectable, implement, initialize_sta};

#[implement(
    extend Microsoft::UI::Xaml::Application,
    override OnLaunched
)]
struct App {
    window: Option<Window>
}

#[repr(C)]
struct PackageVersion {
    pub revision: u16,
    pub build: u16,
    pub minor: u16,
    pub major: u16
}

#[allow(non_snake_case)]
impl App {
    fn OnLaunched(&mut self, _: &Option<LaunchActivatedEventArgs>) -> windows::Result<()> {
        let window = Window::new()?;
        window.SetTitle("WinUI Desktop (Rust)")?;
        
        let button = Button::new()?;
        button.SetContent(IInspectable::try_from("Click Me")?);
        button.SetHorizontalAlignment(HorizontalAlignment::Center);
        
        window.SetContent(button);

        let result = window.Activate();
        self.window = Some(window);
        result
    }
}

pub unsafe fn mdd_bootstrap_initialize() -> windows::HRESULT {
    #[link(name = "Microsoft.ProjectReunion.Bootstrap")]
    extern "system" {
        fn MddBootstrapInitialize(major_minor_version: u32,
            version_tag: *const u16,
            min_version: PackageVersion) -> windows::HRESULT;
    }
    MddBootstrapInitialize(8, windows::HSTRING::from("preview").as_wide().as_ptr(), PackageVersion {
        major: 0, minor: 0, build: 0, revision: 0
    })
}

fn main() -> windows::Result<()> {
    initialize_sta()?;
    unsafe { println!("MddBootstrapInitialize initialize, hr={}", windows::HRESULT::message(&mdd_bootstrap_initialize())) }
    
    Application::Start(ApplicationInitializationCallback::new(|_| {
        App{window: None}.new()?;
        Ok(())
    }))
}
