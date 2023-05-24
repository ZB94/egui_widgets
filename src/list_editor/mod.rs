use egui::{
    collapsing_header::CollapsingState,
    emath::{self, remap, Align},
    epaint::{pos2, vec2, Rect, Shape, Stroke},
    Layout, Response, Sense, TextStyle, Ui, Widget,
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

mod container;
mod item;

pub use container::ListEditorContainer;
pub use item::ListEditorItem;

static UI_TEXT: RwLock<UiText> = RwLock::new(UiText::DEFAULT);

#[derive(Debug, Clone, Copy)]
pub struct UiText {
    pub add: &'static str,
    pub reset: &'static str,
    pub filter: &'static str,
    pub delete: &'static str,
    pub copy: &'static str,
}

#[derive(Debug)]
pub struct ListEditor<'a, W: ListEditorItem, C: ListEditorContainer<W>> {
    pub container: &'a mut C,
    pub data: W::Data<'a>,
    pub default_open: bool,
}

impl<'a, W: ListEditorItem + 'static, C: ListEditorContainer<W>> ListEditor<'a, W, C> {
    pub fn new(container: &'a mut C, data: W::Data<'a>) -> Self {
        Self {
            container,
            data,
            default_open: false,
        }
    }

    pub fn default_open(mut self) -> Self {
        self.default_open = true;
        self
    }

    /// 设置界面上UI的文字
    pub fn set_ui_text(ui_text: UiText) {
        *UI_TEXT.write() = ui_text;
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ListEditorUiData<W: ListEditorItem> {
    pub new: W,
    pub search: String,
}

impl<'a, W: ListEditorItem + 'static, C: ListEditorContainer<W>> Widget for ListEditor<'a, W, C> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            let ui_text = *UI_TEXT.read();

            let ListEditor {
                container,
                data,
                default_open,
                ..
            } = self;
            let id = ui.next_auto_id();

            let data_id = id.with("data");
            let mut ui_data: ListEditorUiData<W> = ui.data_mut(|d| {
                d.get_temp(data_id).unwrap_or_else(|| ListEditorUiData {
                    new: W::new(data).unwrap_or_default(),
                    search: String::new(),
                })
            });

            let mut state =
                CollapsingState::load_with_default_open(ui.ctx(), id.with("new"), default_open);
            let resp = ui.horizontal_top(|ui| {
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button(ui_text.add).clicked() {
                        let mut new = W::new(data).unwrap_or_default();
                        std::mem::swap(&mut ui_data.new, &mut new);
                        container.add(new);
                    }

                    if ui.button(ui_text.reset).clicked() {
                        ui_data.new = W::new(data).unwrap_or_default();
                    }

                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        if paint_title(ui_data.new.new_title(data), ui, state.openness(ui.ctx()))
                            .clicked()
                        {
                            state.toggle(ui);
                        }
                    });
                });
            });
            state.show_body_indented(&resp.response, ui, |ui| ui_data.new.ui(ui, data));

            ui.separator();

            ui.horizontal(|ui| {
                ui.label(ui_text.filter);
                ui.text_edit_singleline(&mut ui_data.search);
            });

            let mut new_list = vec![];
            let mut idx = 0;
            container.retain_mut(|w| {
                let id = id.with(idx);
                idx += 1;

                if !ui_data.search.is_empty() && !w.on_search(&ui_data.search, data) {
                    return true;
                }

                let mut remove = false;
                let mut state = CollapsingState::load_with_default_open(ui.ctx(), id, false);
                let resp = ui.horizontal_top(|ui| {
                    ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                        if ui.button(ui_text.delete).clicked() {
                            remove = true;
                        }

                        if ui.button(ui_text.copy).clicked() {
                            new_list.push(w.clone());
                        }

                        ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                            if paint_title(w.title(data), ui, state.openness(ui.ctx())).clicked() {
                                state.toggle(ui);
                            }
                        });
                    });
                });

                state.show_body_indented(&resp.response, ui, |ui| w.ui(ui, data));

                !remove
            });

            container.append(new_list);

            ui.data_mut(|d| d.insert_temp(data_id, ui_data));
        })
        .response
    }
}

pub fn paint_title(text: String, ui: &mut Ui, openness: f32) -> Response {
    let icon_width = 10.0;
    let text_offset_x = icon_width + ui.spacing().item_spacing.x;

    // 先准备绘制标题
    let font_id = ui.style().text_styles[&TextStyle::Button].clone();
    let galley = ui.painter().layout(
        text,
        font_id,
        ui.visuals().text_color(),
        ui.available_width() - text_offset_x,
    );

    let sense = Sense::hover().union(Sense::click());

    let size = vec2(icon_width, galley.size().y.max(icon_width));
    let mut response = ui.allocate_response(size, sense);
    let mut rect_min = response.rect.min;
    rect_min.x += text_offset_x;

    for row in &galley.rows {
        let rect = Rect::from_min_size(rect_min, row.rect.size());
        response = response.union(ui.allocate_rect(rect, sense));

        rect_min.y += rect.height();
    }

    let rect = response.rect;

    let visuals = ui.style().interact(&response);

    // Draw a pointy triangle arrow:
    let rect = Rect::from_center_size(
        pos2(
            rect.left() + (ui.spacing().indent) / 2.0,
            rect.top() + rect.height() / 2.0,
        ),
        vec2(icon_width, icon_width) * 0.75,
    );
    let rect = rect.expand(visuals.expansion);
    let mut points = vec![rect.left_top(), rect.right_top(), rect.center_bottom()];
    use std::f32::consts::TAU;
    let rotation = emath::Rot2::from_angle(remap(openness, 0.0..=1.0, -TAU / 4.0..=0.0));
    for p in &mut points {
        *p = rect.center() + rotation * (*p - rect.center());
    }

    ui.painter().add(Shape::convex_polygon(
        points,
        visuals.fg_stroke.color,
        Stroke::NONE,
    ));

    let pos = pos2(
        rect.right() + ui.spacing().item_spacing.x,
        response.rect.top(),
    );
    ui.painter()
        .galley_with_color(pos, galley, visuals.fg_stroke.color);

    response
}

impl UiText {
    pub const DEFAULT: UiText = UiText {
        add: "Add",
        reset: "Reset",
        filter: "Filter",
        delete: "Delete",
        copy: "Copy",
    };
}

impl Default for UiText {
    fn default() -> Self {
        Self::DEFAULT
    }
}
