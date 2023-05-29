use egui::{ScrollArea, Widget};

pub struct SelectEditor<'a, S, L>
where
    S: ToString,
    L: Iterator<Item = S>,
{
    pub text: &'a mut String,
    pub iter: L,
    pub filter: bool,
}

impl<'a, S, L> SelectEditor<'a, S, L>
where
    S: ToString,
    L: Iterator<Item = S>,
{
    pub fn new(text: &'a mut String, iter: L) -> Self {
        Self {
            text,
            iter,
            filter: false,
        }
    }

    pub fn filter(self) -> Self {
        Self {
            filter: true,
            ..self
        }
    }
}

impl<'a, S, L> Widget for SelectEditor<'a, S, L>
where
    S: ToString,
    L: Iterator<Item = S>,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut resp = ui.text_edit_singleline(self.text);
        let mut changed = resp.changed();

        let popup_id = ui.auto_id_with(module_path!()).with("select editor popup");

        egui::popup_below_widget(ui, popup_id, &resp, |ui| {
            ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                for item in self.iter {
                    let text = item.to_string();
                    if self.text.is_empty() || !self.filter || text.contains(self.text.as_str()) {
                        changed =
                            ui.selectable_value(self.text, text.clone(), text).clicked() || changed;
                    }
                }
            });
        });

        if resp.lost_focus() {
            ui.memory_mut(|mem| mem.close_popup())
        } else if resp.has_focus() && !ui.memory(|mem| mem.is_popup_open(popup_id)) {
            ui.memory_mut(|mem| mem.open_popup(popup_id));
        }

        if changed {
            resp.mark_changed();
        }

        resp
    }
}
