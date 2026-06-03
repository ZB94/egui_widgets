use eframe::App;
use egui_widgets::OptionValue;

fn main() {
    let _ = eframe::run_native(
        "OptionValue Example",
        Default::default(),
        Box::new(|_| Ok(Box::new(Application { option_value: None }))),
    );
}

struct Application {
    option_value: Option<String>,
}

impl App for Application {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add(OptionValue::new_full(
                &mut self.option_value,
                "Option String",
                |ui, value| ui.text_edit_singleline(value).changed(),
                || "default value".to_string(),
            ));
        });
    }
}
