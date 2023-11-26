use auto_delegate::{delegate, Delegate};

#[delegate]
pub(crate) trait Applier<N> {
    fn get_current(&self) -> &N;
    fn on_begin_changes(&self) {}
    fn on_end_changes(&self) {}
    fn down(&mut self, node: N);
    fn up(&mut self);
    fn clear(&mut self);

    fn insert_top_down(&self) {
        unimplemented!()
    }

    fn insert_bottom_up(&self) {
        unimplemented!()
    }

    fn remove(&self, index: usize, count: usize) {
        unimplemented!()
    }

    fn r#move(&self, from: usize, to: usize, count: usize) {
        unimplemented!()
    }
}

#[derive(Delegate)]
pub(crate) struct AbstractApplier<T> {
    pub(crate) root: T,
    current: T,
    stack: Vec<T>,
}

impl<T> AbstractApplier<T> where T: Clone {
    pub(crate) fn new(root: T) -> Self {
        Self {
            current: root.clone(),
            root,
            stack: vec![],
        }
    }
}

impl<T> Applier<T> for AbstractApplier<T> where T: Clone {
    fn get_current(&self) -> &T {
        &self.current
    }

    fn down(&mut self, mut node: T) {
        std::mem::swap(&mut node, &mut self.current);
        self.stack.push(node);
    }

    fn up(&mut self) {
        self.current = self.stack.pop().unwrap();
    }

    fn clear(&mut self) {
        self.stack.clear();
        self.current = self.root.clone();
    }
}