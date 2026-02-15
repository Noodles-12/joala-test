use eframe::egui;
use egui_snarl::{Snarl, NodeId};
use egui_snarl::ui::{SnarlViewer, PinInfo};

use core::hash::BuildHasherDefault;

#[derive(Default)]
struct MyApp {
    snarl: Snarl<String>,
}

#[derive(Hash, PartialEq, Eq, Default)]
struct MyHasher{}

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
        PinInfo::circle().with_fill(egui::Color32::WHITE)
    }

    fn outputs(&mut self, node: &String) -> usize {
        1
    }

    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        snarl: &mut Snarl<String>,
    ) -> impl egui_snarl::ui::SnarlPin + 'static {
         ui.label("in");
        PinInfo::square()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("Menu Bar").show(ctx, |ui| {
            if ui.button("Add Node").clicked() {
                self.snarl.insert_node(egui::pos2(0.0, 0.0), "Node".to_string());
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.snarl.show(
                &mut MyViewer{}, 
                &egui_snarl::ui::SnarlStyle::default(), 
                MyHasher::default(),
                ui);
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
