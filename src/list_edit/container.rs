use super::ListEditItem;

pub trait ListEditContainer<W: ListEditItem> {
    fn retain_mut<F: FnMut(&mut W) -> bool>(&mut self, f: F);

    fn add(&mut self, i: W);

    fn append(&mut self, o: Vec<W>);

    fn len(&self) -> usize;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<W: ListEditItem> ListEditContainer<W> for Vec<W> {
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

    #[inline]
    fn len(&self) -> usize {
        Vec::len(self)
    }
}
