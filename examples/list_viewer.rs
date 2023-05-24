use eframe::App;
use egui::CentralPanel;
use egui_widgets::list_viewer::{ListViewer, ListViewerItem};

fn main() {
    let _ = eframe::run_native(
        "ListViewer Example",
        Default::default(),
        Box::new(|_| {
            Box::new(Application {
                list: (0..100)
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
            ListViewer::new(self.list.iter(), ()).show(ui);
        });
    }
}

pub struct Item {
    pub id: i64,
    pub name: String,
}

impl ListViewerItem for Item {
    type Data<'a> = ();

    fn title() -> &'static str {
        "List Viewer Example"
    }

    fn label(&self, _data: Self::Data<'_>) -> String {
        self.name.clone()
    }

    fn id(&self, _data: Self::Data<'_>) -> egui::Id {
        egui::Id::new(self.id)
    }

    fn selected_ui(&self, ui: &mut egui::Ui, _data: Self::Data<'_>) {
        ui.horizontal(|ui| {
            ui.label("id");
            ui.text_edit_singleline(&mut self.id.to_string().as_str());
        });
    }

    fn on_search(&self, text: &str, _data: Self::Data<'_>) -> bool {
        self.name.contains(text)
    }
}
