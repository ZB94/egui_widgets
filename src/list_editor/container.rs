use super::ListEditorItem;

pub trait ListEditorContainer<W: ListEditorItem> {
    fn retain_mut<F: FnMut(&mut W) -> bool>(&mut self, f: F);

    fn add(&mut self, i: W);

    fn append(&mut self, o: Vec<W>);
}

impl<W: ListEditorItem> ListEditorContainer<W> for Vec<W> {
    #[inline]
    fn retain_mut<F: FnMut(&mut W) -> bool>(&mut self, f: F) {
        self.retain_mut(f);
    }

    #[inline]
    fn add(&mut self, i: W) {
        self.push(i);
    }

    #[inline]
    fn append(&mut self, mut o: Vec<W>) {
        self.append(&mut o);
    }
}
