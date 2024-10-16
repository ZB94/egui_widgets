use egui::{Id, Label, RichText, ScrollArea, TextEdit};
pub use item::ListViewItem;
use parking_lot::RwLock;

mod item;

static SEARCH_HINT_TEXT: RwLock<&'static str> = RwLock::new("search");

/// 设置搜索框为空时显示的占位文字
pub fn set_search_hint_text(text: &'static str) {
    *SEARCH_HINT_TEXT.write() = text;
}

#[derive(Debug)]
pub struct ListView<'a, W: ListViewItem + 'a, L: Iterator<Item = &'a W>> {
    pub container: L,
    pub data: W::Data<'a>,
    pub height: f32,
}

impl<'a, W: ListViewItem + 'a, L: Iterator<Item = &'a W>> ListView<'a, W, L> {
    pub fn new(container: L, data: W::Data<'a>) -> Self {
        Self {
            container,
            data,
            height: 200.0,
        }
    }

    /// 列表区域的最大高度。不包括标题和选中项目UI
    pub fn max_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl<'a, W: ListViewItem + 'a, L: Iterator<Item = &'a W>> ListView<'a, W, L> {
    pub fn show(self, ui: &mut egui::Ui) -> egui::InnerResponse<Option<&'a W>> {
        let mut selected_item = None;

        let mut resp = ui.vertical(|ui| {
            let ListView {
                container,
                data,
                height,
            } = self;

            let resp = ui.group(|ui| {
                let base_id = ui.auto_id_with("list viewer");
                let search_id = base_id.with("search");
                let selected_id = base_id.with("selected");

                let mut search: String = ui.data_mut(|d| d.get_temp(search_id)).unwrap_or_default();
                let mut selected: Option<Id> =
                    ui.data_mut(|d| d.get_temp(selected_id)).unwrap_or_default();
                let old_selected = selected;

                ui.horizontal_top(|ui| {
                    ui.add(Label::new(RichText::new(W::title()).strong()));
                    ui.add(TextEdit::singleline(&mut search).hint_text(*SEARCH_HINT_TEXT.read()));
                });

                ui.separator();

                ScrollArea::vertical()
                    .id_salt(base_id.with("list"))
                    .max_height(height)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        for item in container {
                            let id = item.id(data);
                            let checked = selected == Some(id);

                            if checked {
                                selected_item = Some(item);
                            }

                            if (search.is_empty() || item.on_search(&search, data))
                                && ui.selectable_label(checked, item.label(data)).clicked()
                                && !checked
                            {
                                selected = Some(id);
                                selected_item = Some(item);
                            }
                        }
                    });

                if let Some(item) = selected_item {
                    ui.separator();
                    item.selected_ui(ui, data);
                }

                ui.data_mut(|d| {
                    d.insert_temp(search_id, search);
                    d.insert_temp(selected_id, selected);
                });

                old_selected != selected
            });

            resp.inner
        });

        if resp.inner {
            resp.response.mark_changed();
        }

        egui::InnerResponse::new(selected_item, resp.response)
    }
}
