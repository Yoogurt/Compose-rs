use crate::foundation::bridge::root_measure_policy::root_measure_policy;
use crate::foundation::canvas::Canvas;
use crate::foundation::constraint::Constraints;

use crate::foundation::measure_and_layout_delegate::MeasureAndLayoutDelegate;



use crate::foundation::composer::Composer;

pub struct MacOSComposeView {
    measure_and_layout_delegate: MeasureAndLayoutDelegate,
}

impl Drop for MacOSComposeView {
    fn drop(&mut self) {
        Composer::detach_root_layout_node();
    }
}

impl MacOSComposeView {
    pub fn new() -> MacOSComposeView {
        let result = MacOSComposeView {
            measure_and_layout_delegate: MeasureAndLayoutDelegate::new(),
        };

        result
            .measure_and_layout_delegate
            .update_root_measure_policy(root_measure_policy());

        if !Composer::attach_root_layout_node(result.measure_and_layout_delegate.root.clone()) {
            panic!("unable to create multiple compose view in single thread");
        }

        result
    }

    pub fn set_content(&self, content: impl FnOnce()) {
        Composer::start_root();
        content();
        Composer::end_root();
    }

    pub fn dispatch_measure(&mut self, width: usize, height: usize) {
        let constraint = Constraints::new(0..=width, 0..=height);
        self.measure_and_layout_delegate
            .update_root_constraints(constraint);
        self.measure_and_layout_delegate.measure_only();
    }

    pub fn dispatch_draw(&mut self, _canvas: &dyn Canvas) {}
}
