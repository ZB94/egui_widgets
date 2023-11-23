use eframe::App;
use egui::{CentralPanel, DragValue, Grid, ScrollArea};
use egui_widgets::list_edit::{ListEdit, ListEditItem};

fn main() {
    let _ = eframe::run_native(
        "ListEdit Example",
        Default::default(),
        Box::new(|_| {
            Box::new(Application {
                list: (0..5)
                    .map(|id| Item {
                        id,
                        name: format!("{id:#04X}"),
                    })
                    .collect(),
            })
        }),
    );
}

struct Application {
    list: Vec<Item>,
}

impl App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| ui.add(ListEdit::new(&mut self.list, ())));
        });
    }
}

#[derive(Debug, Clone, Default)]
struct Item {
    pub id: i64,
    pub name: String,
}

impl ListEditItem for Item {
    type Data<'a> = ();

    fn new_title(&self, _data: Self::Data<'_>, _index: usize) -> String {
        "new item".to_string()
    }

    fn title(&self, _data: Self::Data<'_>, _index: usize) -> String {
        self.name.clone()
    }

    fn ui(&mut self, ui: &mut egui::Ui, _data: Self::Data<'_>, _index: usize) {
        Grid::new(ui.auto_id_with("grid"))
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("id");
                ui.add(DragValue::new(&mut self.id).clamp_range(0..=500));
                ui.end_row();

                ui.label("name");
                ui.text_edit_singleline(&mut self.name);
                ui.end_row();
            });
    }

    fn new(_data: Self::Data<'_>, _index: usize) -> Option<Self> {
        Some(Default::default())
    }

    fn on_search(&self, text: &str, _data: Self::Data<'_>, _index: usize) -> bool {
        self.name.contains(text)
    }
}
