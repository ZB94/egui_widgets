use crate::layer::LogRecord;
use crossbeam_channel::Receiver;
use egui::{
    CollapsingHeader, Color32, ComboBox, Grid, Response, RichText, ScrollArea, SelectableLabel,
    TextEdit, TextStyle, Ui, Widget,
};
use std::{cmp::Ordering, collections::HashMap};
use tracing::Level;

pub struct EguiLog {
    max_size: usize,
    receiver: Receiver<LogRecord>,
    log_list: Vec<LogRecord>,
    pub filter_level: Option<Level>,
    pub filter_span_data: String,
    pub filter_data: String,
    pub filter_message: String,
    pub display_info: DisplayInfo,
}

impl EguiLog {
    pub(crate) fn new(max_size: usize, receiver: Receiver<LogRecord>) -> Self {
        let log_list = Vec::with_capacity(max_size);
        Self {
            max_size,
            receiver,
            log_list,
            filter_level: None,
            filter_span_data: String::new(),
            filter_data: String::new(),
            filter_message: String::new(),
            display_info: DisplayInfo::default(),
        }
    }

    /// 更新日志记录。
    ///
    /// 返回本次更新接收到的日志记录的最小级别
    pub fn update(&mut self) -> Option<Level> {
        let mut ret: Option<Level> = None;

        let recv_count = self.receiver.len().min(self.max_size);
        if recv_count > 0 {
            if recv_count >= self.max_size {
                self.log_list.clear();
            } else if self.log_list.len() == self.max_size {
                self.log_list.rotate_left(recv_count);
                let _ = self.log_list.split_off(self.max_size - recv_count);
            }

            while let Ok(record) = self.receiver.try_recv() {
                if let Some(l) = &mut ret {
                    *l = (*l).min(record.level);
                } else {
                    ret = Some(record.level);
                }

                self.log_list.push(record);
                if self.log_list.len() == self.max_size {
                    break;
                }
            }
        }

        ret
    }
}

impl EguiLog {
    fn ui_filter(&mut self, ui: &mut Ui) {
        CollapsingHeader::new(&self.display_info.filter)
            .id_source(ui.auto_id_with("filter"))
            .default_open(false)
            .show(ui, |ui| {
                Grid::new(ui.auto_id_with("filter grid"))
                    .num_columns(2)
                    .show(ui, |ui| {
                        ui.label(&self.display_info.level);
                        ui.horizontal(|ui| {
                            let mut checked = self.filter_level.is_some();
                            if ui.checkbox(&mut checked, "").clicked() {
                                self.filter_level = checked.then_some(Level::TRACE);
                            }

                            if let Some(level) = &mut self.filter_level {
                                let (text, color) = level_info(*level, ui);

                                ComboBox::from_id_source(ui.auto_id_with("filter level"))
                                    .selected_text(RichText::new(text).color(color))
                                    .show_ui(ui, |ui| {
                                        for l in [
                                            Level::TRACE,
                                            Level::DEBUG,
                                            Level::INFO,
                                            Level::WARN,
                                            Level::ERROR,
                                        ] {
                                            let (text, color) = level_info(l, ui);
                                            if ui
                                                .add(SelectableLabel::new(
                                                    level == &l,
                                                    RichText::new(text).color(color),
                                                ))
                                                .clicked()
                                            {
                                                *level = l;
                                            }
                                        }
                                    });
                            }
                        });
                        ui.end_row();

                        ui.label(&self.display_info.span_data);
                        ui.text_edit_singleline(&mut self.filter_span_data);
                        ui.end_row();

                        ui.label(&self.display_info.data);
                        ui.text_edit_singleline(&mut self.filter_data);
                        ui.end_row();

                        ui.label(&self.display_info.message);
                        ui.text_edit_singleline(&mut self.filter_message);
                        ui.end_row();
                    });
            });
    }
}

impl Widget for &mut EguiLog {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            self.ui_filter(ui);

            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    Grid::new(ui.auto_id_with("log grid"))
                        .max_col_width(ui.available_width())
                        .num_columns(5)
                        .striped(true)
                        .show(ui, |ui| {
                            ui.heading(&self.display_info.level);
                            ui.heading(&self.display_info.time);
                            ui.heading(&self.display_info.span_data);
                            ui.heading(&self.display_info.data);
                            ui.heading(&self.display_info.message);
                            ui.end_row();

                            for log in self.log_list.iter().rev() {
                                if self.filter_level.map(|l| l >= log.level).unwrap_or(true)
                                    && (self.filter_span_data.is_empty()
                                        || log.span_data.iter().any(|(s, map)| {
                                            s.contains(&self.filter_span_data)
                                                || map.iter().any(|(k, v)| {
                                                    k.contains(&self.filter_span_data)
                                                        || v.contains(&self.filter_span_data)
                                                })
                                        }))
                                    && (self.filter_data.is_empty()
                                        || log.data.iter().any(|(k, v)| {
                                            k.contains(&self.filter_data)
                                                || v.contains(&self.filter_data)
                                        }))
                                    && (self.filter_message.is_empty()
                                        || log.message.contains(&self.filter_message))
                                {
                                    ui_level(ui, log.level);
                                    ui.label(&log.time);
                                    ui.vertical(|ui| {
                                        for (span_idx, (span, map)) in
                                            log.span_data.iter().enumerate()
                                        {
                                            CollapsingHeader::new(*span)
                                                .id_source(ui.auto_id_with(span_idx))
                                                .default_open(false)
                                                .show(ui, |ui| {
                                                    ui_map(ui, map);
                                                });
                                        }
                                    });

                                    ui_map(ui, &log.data);

                                    ui_text(ui, &log.message);
                                    ui.end_row();
                                }
                            }
                        });
                });
        })
        .response
    }
}

fn level_info(level: Level, ui: &Ui) -> (&'static str, Color32) {
    match level {
        Level::TRACE => ("Trace", Color32::LIGHT_GRAY),
        Level::DEBUG => ("Debug", Color32::GRAY),
        Level::INFO => ("Info", ui.style().visuals.text_color()),
        Level::WARN => ("Warn", Color32::YELLOW),
        Level::ERROR => ("Error", Color32::RED),
    }
}

fn ui_level(ui: &mut Ui, level: Level) {
    let (label, color) = level_info(level, ui);

    ui.colored_label(color, label);
}

fn ui_map(ui: &mut Ui, map: &HashMap<&'static str, String>) {
    ui.vertical(|ui| {
        for (k, v) in map {
            ui.horizontal(|ui| {
                ui.label(*k);
                ui.label("=");
                ui_text(ui, v.as_str());
            });
        }
    });
}

fn ui_text(ui: &mut Ui, mut text: &str) {
    let font_id = &ui.style().text_styles[&TextStyle::Body];

    // 8.0 为TextEdit外边距
    let (lines, width) = ui.fonts(|font| {
        text.lines()
            .map(|line| line.chars().map(|c| font.glyph_width(font_id, c)).sum())
            .enumerate()
            .max_by(|l: &(_, f32), r| l.1.partial_cmp(&r.1).unwrap_or(Ordering::Equal))
            .map(|(l, w)| (l + 1, w + 8.0 + font.glyph_width(font_id, ' ')))
            .unwrap()
    });

    ui.add(
        TextEdit::multiline(&mut text)
            .desired_width(width)
            .desired_rows(lines),
    );
}

#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub filter: String,
    pub level: String,
    pub time: String,
    pub span_data: String,
    pub data: String,
    pub message: String,
}

impl Default for DisplayInfo {
    fn default() -> Self {
        Self {
            filter: "Filter".to_string(),
            level: "Level".to_string(),
            time: "Time".to_string(),
            span_data: "Span Data".to_string(),
            data: "Data".to_string(),
            message: "Message".to_string(),
        }
    }
}
