use egui::{Ui, Widget, WidgetText};

pub struct OptionValue<'a, T, L, F, N>
where
    L: Into<WidgetText>,
    F: FnMut(&mut Ui, &mut T) -> bool,
    N: Fn() -> T,
{
    pub value: &'a mut Option<T>,
    /// 要显示的标签
    pub label: L,
    /// 当`value`为`some`时要显示的UI，返回值是否被改变
    pub ui_some: F,
    /// some值的初始化方法
    pub new: N,
}

impl<'a, T, L, F, N> OptionValue<'a, T, L, F, N>
where
    L: Into<WidgetText>,
    F: FnMut(&mut Ui, &mut T) -> bool,
    N: Fn() -> T,
{
    pub fn new_full(value: &'a mut Option<T>, label: L, ui_some: F, new: N) -> Self {
        Self {
            value,
            label,
            ui_some,
            new,
        }
    }
}

impl<'a, T, L, F> OptionValue<'a, T, L, F, fn() -> T>
where
    T: Default,
    L: Into<WidgetText>,
    F: FnMut(&mut Ui, &mut T) -> bool,
{
    pub fn new(value: &'a mut Option<T>, label: L, ui_some: F) -> Self {
        Self::new_full(value, label, ui_some, Default::default)
    }
}

impl<'a, T, L, F, N> Widget for OptionValue<'a, T, L, F, N>
where
    L: Into<WidgetText>,
    F: FnMut(&mut Ui, &mut T) -> bool,
    N: Fn() -> T,
{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let OptionValue {
            value,
            label,
            mut ui_some,
            new,
        } = self;

        let mut resp = ui.horizontal_top(|ui| {
            let mut checked = value.is_some();

            let mut changed = if ui.checkbox(&mut checked, label).clicked() {
                *value = checked.then(new);
                true
            } else {
                false
            };

            if let Some(d) = value {
                changed = ui_some(ui, d) || changed;
            }

            changed
        });

        if resp.inner {
            resp.response.mark_changed();
        }

        resp.response
    }
}
