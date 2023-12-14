use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use crate::foundation::geometry::{Density, Size};
use crate::foundation::modifier::{Modifier, ModifierNodeImpl};
use crate::foundation::modifier::ModifierNodeElement;
use std::rc::Rc;
use auto_delegate::Delegate;
use compose_foundation_macro::ModifierElement;
use crate::foundation::constraint::Constraints;
use crate::foundation::measurable::Measurable;
use crate::foundation::measure_result::MeasureResult;
use crate::foundation::measure_scope::{MeasureScope, MeasureScopeLayoutAction};
use crate::foundation::modifier_node::LayoutModifierNode;

#[derive(Debug, Clone)]
pub struct GraphicsLayerScope {
    density: Density,
    size: Size<f32>,
    alpha: f32,
    scale_x: f32,
    scale_y: f32,
    translation_x: f32,
    translation_y: f32,
    clip: bool,
}

impl Deref for GraphicsLayerScope {
    type Target = Density;

    fn deref(&self) -> &Self::Target {
        &self.density
    }
}

impl GraphicsLayerScope {
    pub(crate) fn new() -> Self {
        GraphicsLayerScope {
            density: Density::default(),
            size: Size::UNSPECIFIC,
            alpha: 1.0,
            scale_x: 1.0,
            scale_y: 1.0,
            translation_x: 0.0,
            translation_y: 0.0,
            clip: false,
        }
    }

    pub(crate) fn reset(&mut self) {
        self.density = Density::default();
        self.alpha = 1.0;
        self.scale_x = 1.0;
        self.scale_y = 1.0;
        self.translation_x = 0.0;
        self.translation_y = 0.0;
    }

    pub(crate) fn set_density(&mut self, density: Density) {
        self.density = density;
    }

    pub(crate) fn set_size(&mut self, size: Size<f32>) {
        self.size = size;
    }

    pub fn set_translation_x(&mut self, translation_x: f32) {
        self.translation_x = translation_x;
    }

    pub fn set_translation_y(&mut self, translation_y: f32) {
        self.translation_y = translation_y;
    }

    pub fn set_scale_x(&mut self, scale_x: f32) {
        self.scale_x = scale_x;
    }

    pub fn set_scale_y(&mut self, scale_y: f32) {
        self.scale_y = scale_y;
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn get_size(&self) -> Size<f32> {
        self.size
    }

    pub fn get_translation_x(&self) -> f32 {
        self.translation_x
    }

    pub fn get_translation_y(&self) -> f32 {
        self.translation_y
    }

    pub fn get_scale_x(&self) -> f32 {
        self.scale_x
    }

    pub fn get_scale_y(&self) -> f32 {
        self.scale_y
    }

    pub fn get_alpha(&self) -> f32 {
        self.alpha
    }

    pub fn get_density(&self) -> Density {
        self.density
    }

    pub fn set_clip(&mut self) {
        self.clip = true;
    }

    pub fn get_clip(&self) -> bool {
        self.clip
    }
}

impl Modifier {
    pub fn graphics_layer(self, graphics_scope: impl Fn(&mut GraphicsLayerScope) + 'static) -> Modifier {
        self.then(block_graphics_layer_element(Rc::new(graphics_scope)))
    }
}

#[derive(Delegate, ModifierElement)]
#[Impl(Layout)]
struct BlockGraphicsLayerModifier {
    block: Rc<dyn Fn(&mut GraphicsLayerScope)>,

    #[to(ModifierNode)]
    node_impl: ModifierNodeImpl,
}

impl Debug for BlockGraphicsLayerModifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockGraphicsLayerModifier").finish()
    }
}

impl LayoutModifierNode for BlockGraphicsLayerModifier {
    fn measure(&self, measure_scope: &mut dyn MeasureScope, measurable: &mut dyn Measurable, constraint: &Constraints) -> MeasureResult {
        let (size, placeable) = measurable.measure(constraint);
        let block = self.block.clone();
        measure_scope.layout(size, move |scope| {
            scope.place_with_layer(&placeable, 0, 0, 0f32, block.clone())
        })
    }
}

fn block_graphics_layer_element(block: Rc<dyn Fn(&mut GraphicsLayerScope)>) -> Modifier {
    let block_for_update = block.clone();
    ModifierNodeElement(
        "BlockGraphicsLayerElement",
        move || BlockGraphicsLayerModifier { block: block.clone(), node_impl: Default::default() },
        move |modifier: &mut BlockGraphicsLayerModifier| {
            modifier.block = block_for_update.clone();
        },
    )
}