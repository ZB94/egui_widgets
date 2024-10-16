use egui::{Event, PointerButton, PopupCloseBehavior, ScrollArea, TextEdit, Widget};

pub struct SelectEdit<'a, S, L>
where
    S: ToString,
    L: Iterator<Item = S>,
{
    pub text: &'a mut String,
    pub iter: L,
    pub filter: bool,
    pub hint_text: Option<&'a str>,
}

impl<'a, S, L> SelectEdit<'a, S, L>
where
    S: ToString,
    L: Iterator<Item = S>,
{
    pub fn new(text: &'a mut String, iter: L) -> Self {
        Self {
            text,
            iter,
            filter: false,
            hint_text: None,
        }
    }

    pub fn filter(self) -> Self {
        Self {
            filter: true,
            ..self
        }
    }

    pub fn hint_text(self, hint_text: impl Into<Option<&'a str>>) -> Self {
        Self {
            hint_text: hint_text.into(),
            ..self
        }
    }
}

impl<'a, S, L> Widget for SelectEdit<'a, S, L>
where
    S: ToString,
    L: Iterator<Item = S>,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut text_edit = TextEdit::singleline(self.text);

        if let Some(hint_text) = self.hint_text {
            text_edit = text_edit.hint_text(hint_text);
        }

        let mut resp = ui.add(text_edit);
        let mut changed = resp.changed();

        let popup_id = ui.auto_id_with(module_path!()).with("select editor popup");

        let mut press = false;
        egui::popup_below_widget(
            ui,
            popup_id,
            &resp,
            PopupCloseBehavior::CloseOnClick,
            |ui| {
                ScrollArea::vertical().max_height(100.0).show(ui, |ui| {
                    for item in self.iter {
                        let text = item.to_string();
                        if self.text.is_empty() || !self.filter || text.contains(self.text.as_str())
                        {
                            let r = ui.selectable_value(self.text, text.clone(), text);

                            if !press
                                && ui.ctx().input(|state| {
                                    state.events.iter().any(|e| {
                                        let Event::PointerButton {
                                            button: PointerButton::Primary,
                                            pressed: true,
                                            pos,
                                            ..
                                        } = e
                                        else {
                                            return false;
                                        };

                                        r.rect.contains(*pos)
                                    })
                                })
                            {
                                press = true;
                            }

                            changed = r.clicked() || changed;
                        }
                    }
                });
            },
        );

        if !press && resp.lost_focus() {
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
