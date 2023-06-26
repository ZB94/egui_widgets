use egui::{Id, Ui};

pub trait ListViewItem {
    type Data<'a>: Copy;

    fn title() -> &'static str;

    /// 在列表中显示的名称
    fn label(&self, _data: Self::Data<'_>) -> String;

    /// 在该列表中的唯一标识
    fn id(&self, _data: Self::Data<'_>) -> Id;

    /// 被选中时显示的UI
    fn selected_ui(&self, ui: &mut Ui, _data: Self::Data<'_>);

    /// 是否符合搜索条件
    fn on_search(&self, text: &str, _data: Self::Data<'_>) -> bool;
}
