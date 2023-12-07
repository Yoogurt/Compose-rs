use crate::foundation::modifier_node::ParentDataModifierNode;
use crate::foundation::modifier::ModifierNodeImpl;
use auto_delegate::Delegate;
use std::any::Any;
use std::rc::Rc;
use std::fmt::Debug;
use compose_foundation_macro::ModifierElement;
use crate::foundation::delegatable_node::{DelegatableKind, DelegatableNode};
use crate::foundation::geometry::Density;
use crate::foundation::measurable::Measurable;
use crate::foundation::modifier::{Modifier, ModifierNodeElement};
use crate::foundation::oop::AnyConverter;
use crate::foundation::parent_data::ExtractParentData;
use crate::foundation::utils::option_extension::OptionalInstanceConverter;
use crate::foundation::utils::rc_wrapper::WrapWithRcRefCell;

struct LayoutId {
    layout_id: Rc<dyn Any>,
}

impl Modifier {
    pub fn layout_id<LayoutId>(self, layout_id: LayoutId) -> Modifier where LayoutId: Any + 'static {
        self.then(layout_id_element(Rc::new(layout_id)))
    }
}

pub(crate) trait ParentDataLayoutId {
    fn layout_id(&self) -> Option<Rc<dyn Any>>;
}

impl<T> ParentDataLayoutId for T where T: ?Sized + Measurable {
    fn layout_id(&self) -> Option<Rc<dyn Any>> {
        self.cast::<LayoutId>().map(|layout_id| layout_id.layout_id.clone())
    }
}

#[derive(Debug, Delegate, ModifierElement)]
#[Impl(ParentData)]
struct LayoutIdModifier {
    layout_id: Rc<dyn Any>,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}

impl ParentDataModifierNode for LayoutIdModifier {
    fn modify_parent_data(&mut self, density: Density, parent_data: Option<Box<dyn Any>>) -> Option<Box<dyn Any>> {
        let mut parent_data = parent_data.cast_or(|| {
            LayoutId {
                layout_id: self.layout_id.clone()
            }
        });
        parent_data.layout_id = self.layout_id.clone();
        Some(parent_data)
    }
}

fn layout_id_element(layout_id: Rc<dyn Any>) -> Modifier {
    let layout_id_for_update = layout_id.clone();
    ModifierNodeElement(
        move || {
            LayoutIdModifier {
                layout_id: layout_id.clone(),
                node_impl: Default::default(),
            }
        },
        move |layout_id: &mut LayoutIdModifier| {
            layout_id.layout_id = layout_id_for_update.clone();
        },
    )
}