use eframe::App;
use egui::CentralPanel;
use egui_widgets::SelectEdit;

fn main() {
    let _ = eframe::run_native(
        "SelectEdit Example",
        Default::default(),
        Box::new(|_| {
            Box::new(Application {
                text: Default::default(),
            })
        }),
    );
}

struct Application {
    text: String,
}

impl App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(SelectEdit::new(
                &mut self.text,
                ('a'..='z')
                    .enumerate()
                    .map(|c| c.1.to_string().repeat(c.0 + 1)),
            ));

            ui.add(
                SelectEdit::new(
                    &mut self.text,
                    ('a'..='z')
                        .enumerate()
                        .map(|c| c.1.to_string().repeat(c.0 + 1)),
                )
                .filter(),
            );
        });
    }
}
