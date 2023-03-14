use dll_syringe::process::Process;
use eframe::egui;
use thiserror::Error;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(400.0, 140.0)),
        resizable: false,
        centered: true,
        vsync: true,
        always_on_top: true,

        // does not wrork, i'll take time later to understand why
        icon_data: Some(eframe::IconData {
            #[rustfmt::skip]
            rgba: vec![
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0,
            ],
            width: 8,
            height: 8,
        }),

        default_theme: eframe::Theme::Dark,

        ..Default::default()
    };
    eframe::run_native(
        "Simple GUI injector",
        options,
        Box::new(|cc| Box::<Injector>::new(Injector::new(cc))),
    )
}

struct Injector {
    target_name: String,
    dll_path: String,
    dll_path_button_text: String,
    injection_output: InjectionOutput,
}

#[derive(Debug)]
enum InjectionOutput {
    None,
    Ok(String),
    Err(InjectionError),
}

#[derive(Error, Debug)]
pub enum InjectionError {
    #[error("Target not found")]
    InvalidTargetName, // #[from] io::Error
    #[error("The selected process is dead")]
    Deadprocess,
    #[error("Illegal path")]
    IllegalPath,
    #[error("i/o error")]
    Io(std::io::Error),
    #[error("The target is unsupported")]
    UnsupportedTarget,
    #[error("The remote code ran into an io error")]
    RemoteIo(std::io::Error),
    #[error("The remote code ran into an exception")]
    RemoteException(dll_syringe::error::ExceptionCode),
    #[error("The process is inaccessible")]
    ProcessInaccessible,
    #[error("The goblins diddn't do their jobs")]
    Goblin,
    #[error("unknown data store error")]
    Unknown(String),
}

impl Injector {
    fn new(cc: &eframe::CreationContext) -> Self {
        use egui::{
            FontFamily::{Monospace, Proportional},
            FontId, TextStyle,
        };

        let mut style = (*cc.egui_ctx.style()).clone();
        style.text_styles = [
            (TextStyle::Heading, FontId::new(25.0, Proportional)),
            (TextStyle::Body, FontId::new(16.0, Proportional)),
            (TextStyle::Monospace, FontId::new(12.0, Monospace)),
            (TextStyle::Button, FontId::new(16.0, Proportional)),
            (TextStyle::Small, FontId::new(8.0, Proportional)),
        ]
        .into();
        cc.egui_ctx.set_style(style);
        Self {
            target_name: String::new(),
            dll_path: String::new(),
            dll_path_button_text: String::from("Choose a path"),
            injection_output: InjectionOutput::None,
        }
    }
    fn inject(&mut self) {
        use dll_syringe::{process::OwnedProcess, Syringe};

        if self.target_name == String::new() {
            self.injection_output = InjectionOutput::Err(InjectionError::InvalidTargetName);
            return;
        }

        let target_process = match OwnedProcess::find_first_by_name(&self.target_name) {
            Some(op) => {
                if !op.is_alive() {
                    self.injection_output = InjectionOutput::Err(InjectionError::Deadprocess);
                    return;
                }
                op
            }
            None => {
                self.injection_output = InjectionOutput::Err(InjectionError::InvalidTargetName);
                return;
            }
        };

        let syringe = Syringe::for_process(target_process);

        use dll_syringe::error::InjectError;
        match syringe.inject(&self.dll_path) {
            Ok(_) => self.injection_output = InjectionOutput::Ok(String::from("Success")),
            Err(e) => {
                let injection_error = match e {
                    InjectError::IllegalPath(_constains_nul) => InjectionError::IllegalPath,
                    InjectError::Io(e) => InjectionError::Io(e),
                    InjectError::UnsupportedTarget => InjectionError::UnsupportedTarget,
                    InjectError::RemoteIo(e) => InjectionError::RemoteIo(e),
                    InjectError::RemoteException(ec) => InjectionError::RemoteException(ec),
                    InjectError::ProcessInaccessible => InjectionError::ProcessInaccessible,
                    InjectError::Goblin(_e) => InjectionError::Goblin,
                };

                self.injection_output = InjectionOutput::Err(injection_error)
            }
        }
    }
}

impl eframe::App for Injector {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Target name: ");
                ui.text_edit_singleline(&mut self.target_name)
            });
            ui.horizontal(|ui| {
                ui.label("dll path: ");

                if ui.button(&self.dll_path_button_text).clicked() {
                    let dll_file = rfd::AsyncFileDialog::new()
                        .add_filter("Dll files", &["dll"])
                        .set_directory(std::env::current_dir().unwrap())
                        .pick_file();

                    let path = futures::executor::block_on(dll_file);

                    if let Some(..) = path {
                        self.dll_path_button_text = path.as_ref().unwrap().file_name();
                        self.dll_path = path
                            .unwrap()
                            .path()
                            .as_os_str()
                            .to_str()
                            .unwrap()
                            .to_string();
                    }
                }
            });

            if ui.button("inject").clicked() {
                self.inject()
            }

            use egui::widget_text::{RichText, WidgetText};
            match &self.injection_output {
                InjectionOutput::None => {}
                InjectionOutput::Ok(s) => {
                    ui.add(egui::Label::new(WidgetText::RichText(
                        RichText::new(s).color(egui::Color32::GREEN),
                    )));
                }
                InjectionOutput::Err(e) => {
                    ui.add(egui::Label::new(WidgetText::RichText(
                        RichText::new(format!("{e}")).color(egui::Color32::RED),
                    )));
                }
            }
        });
    }
}

impl std::fmt::Display for InjectionOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
