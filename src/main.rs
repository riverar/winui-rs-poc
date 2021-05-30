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

use windows::{IInspectable, implement, initialize_sta};

#[implement(
    extend Microsoft::UI::Xaml::Application,
    override OnLaunched
)]
struct App {
    window: Option<Window>
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

pub unsafe fn mdd_bootstrap_initialize() -> u32 {
    #[link(name = "Microsoft.ProjectReunion.Bootstrap")]
    extern "system" {
        fn MddBootstrapInitialize(majorMinorVersion: u32,
            versionTag: *const u16,
            minVersion: u64) -> u32;
    }
    let version = "preview";
    let mut wide_version: Vec<u16> = version.encode_utf16().collect();
    wide_version.push(0);

    MddBootstrapInitialize(8u32, wide_version.as_ptr(), 0u64)
}

fn main() -> windows::Result<()> {
    initialize_sta()?;
    unsafe { println!("MddBootstrapInitialize initialize, hr={}", mdd_bootstrap_initialize()) }
    
    Application::Start(ApplicationInitializationCallback::new(|_| {
        App{window: None}.new()?;
        Ok(())
    }))
}
