use egui::Ui;

pub trait ListEditItem: Clone + Default + Send + Sync {
    type Data<'a>: Copy;

    fn new_title(&self, _data: Self::Data<'_>) -> String;

    fn title(&self, _data: Self::Data<'_>) -> String;

    fn ui(&mut self, ui: &mut Ui, _data: Self::Data<'_>);

    fn new(_data: Self::Data<'_>) -> Option<Self>;

    fn on_search(&self, text: &str, _data: Self::Data<'_>) -> bool;
}

impl ListEditItem for String {
    type Data<'a> = StringData<'a>;

    fn new_title(&self, data: Self::Data<'_>) -> String {
        data.new_title.to_string()
    }

    fn title(&self, _data: Self::Data<'_>) -> String {
        self.clone()
    }

    fn ui(&mut self, ui: &mut Ui, data: Self::Data<'_>) {
        if data.multiline {
            ui.text_edit_multiline(self);
        } else {
            ui.text_edit_singleline(self);
        }
    }

    fn new(data: Self::Data<'_>) -> Option<Self> {
        data.default.map(ToString::to_string)
    }

    fn on_search(&self, text: &str, _data: Self::Data<'_>) -> bool {
        self.contains(text)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StringData<'a> {
    pub new_title: &'a str,
    pub multiline: bool,
    pub default: Option<&'a str>,
}
