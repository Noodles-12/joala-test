use eframe::egui;
use egui_snarl::{Snarl, NodeId};
use egui_snarl::ui::SnarlViewer;

#[derive(Default)]
struct MyApp {
    snarl: Snarl<String>,
}

struct MyViewer{}

impl SnarlViewer<String> for MyViewer {
    fn title(&mut self, node: &String) -> String {
        node.to_string()
    }

    fn inputs(&mut self, node: &String) -> usize {
        1
    }

    fn show_input(
        &mut self,
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<String>,
    ) -> impl egui_snarl::ui::SnarlPin + 'static {
        todo!()
    }

    fn outputs(&mut self, node: &String) -> usize {
        todo!()
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<String>,
    ) -> impl egui_snarl::ui::SnarlPin + 'static {
        todo!()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Snarl Example",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
