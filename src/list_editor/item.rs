use egui::Ui;

pub trait ListEditorItem: Clone + Default + Send + Sync {
    type Data<'a>: Copy;

    fn new_title(&self, _data: Self::Data<'_>) -> String;

    fn title(&self, _data: Self::Data<'_>) -> String;

    fn ui(&mut self, ui: &mut Ui, _data: Self::Data<'_>);

    fn new(_data: Self::Data<'_>) -> Option<Self>;

    fn on_search(&self, text: &str, _data: Self::Data<'_>) -> bool;
}
