use std::cell::RefCell;
use std::rc::Weak;
use crate::foundation::{Constraint, DelegatingLayoutNodeWrapper, DelegatingLayoutNodeWrapperImpl, LayoutNode, LayoutNodeWrapper, Measurable, Measured, Modifier, Placeable, PlaceAction};
use crate::foundation::geometry::{IntOffset, IntSize};

impl LayoutNodeWrapper for DelegatingLayoutNodeWrapperImpl {
    fn attach(&mut self, layout_node: Weak<RefCell<LayoutNode>>) {
        self.layout_node_wrapper_impl.attach(layout_node)
    }

    fn layout_node(&self) -> Weak<RefCell<LayoutNode>> {
        self.layout_node_wrapper_impl.layout_node()
    }
}

impl Placeable for DelegatingLayoutNodeWrapperImpl {
    fn get_width(&self) -> usize {
        self.layout_node_wrapper_impl.get_width()
    }

    fn get_height(&self) -> usize {
        self.layout_node_wrapper_impl.get_height()
    }

    fn set_measured_size(&mut self, size: IntSize) {
        self.layout_node_wrapper_impl.set_measured_size(size)
    }

    fn get_measured_size(&self) -> IntSize {
        self.layout_node_wrapper_impl.get_measured_size()
    }

    fn place_at(&mut self, position: IntOffset, z_index: f32, place_action: PlaceAction) {
        self.layout_node_wrapper_impl.place_at(position, z_index, place_action);

        self.on_place();
    }

    fn get_measurement_constraint(&self) -> &Constraint {
        self.layout_node_wrapper_impl.get_measurement_constraint()
    }
}

impl Measured for DelegatingLayoutNodeWrapperImpl {
    fn get_measured_width(&self) -> usize {
        self.layout_node_wrapper_impl.get_measured_width()
    }

    fn get_measured_height(&self) -> usize {
        self.layout_node_wrapper_impl.get_measured_height()
    }
}

impl Measurable for DelegatingLayoutNodeWrapperImpl {
    fn measure(&mut self, constraint: &Constraint) -> &mut dyn Placeable {
        self.layout_node_wrapper_impl.measure(constraint)
    }
}

impl DelegatingLayoutNodeWrapper for DelegatingLayoutNodeWrapperImpl {
    fn set_modifier_to(&mut self, modifier: Modifier) {
        self.modifier = modifier;
    }
}

impl DelegatingLayoutNodeWrapperImpl {
    pub(crate) fn new() -> Self {
        todo!()
        // DelegatingLayoutNodeWrapperImpl {
        //     wrapped: Rc::new(RefCell::new(())),
        //     modifier: Default::default(),
        //     layout_node_wrapper_impl: LayoutNodeWrapperImpl {},
        // }
    }
}