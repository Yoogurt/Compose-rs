use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};
use crate::foundation::modifier::ModifierNode;

#[derive(PartialEq, Copy, Clone, Debug)]
struct DistanceInLayer {
    distance: f32,
    is_in_layer: bool,
}

impl PartialOrd for DistanceInLayer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let this_is_in_lauer = self.is_in_layer;
        let other_is_in_layer = other.is_in_layer;

        if this_is_in_lauer != other_is_in_layer {
            return if this_is_in_lauer {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            };
        }

        let distance_diff = self.distance - other.distance;
        distance_diff.partial_cmp(&0.0)
    }
}

pub(crate) struct HitTestResult {
    values: HashMap<usize, Rc<RefCell<dyn ModifierNode>>>,
    distance_from_edge_and_in_layer: HashMap<usize, DistanceInLayer>,
    size: usize,
    hit_depth: i32,
}

impl HitTestResult {
    pub(crate) fn new() -> Self {
        Self {
            values: HashMap::new(),
            distance_from_edge_and_in_layer: HashMap::new(),
            size: 0,
            hit_depth: -1,
        }
    }

    pub(crate) fn accept_hits(&mut self) {
        self.hit_depth = self.size as i32 - 1;
    }

    pub(crate) fn sibling_hits(&mut self, mut block: impl FnMut(&mut Self)) {
        let depth = self.hit_depth;
        block(self);
        self.hit_depth = depth;
    }

    pub(crate) fn hit(&mut self, node: &Rc<RefCell<dyn ModifierNode>>, is_in_layer: bool, child_hit_test: impl Fn()) {}

    fn resize_to_hit_depth(&mut self) {
        self.size = (self.hit_depth + 1) as usize;
    }

    fn hit_in_minimum_touch_target(&mut self, node: &Rc<RefCell<dyn ModifierNode>>,
                                   distance_from_edge: f32,
                                   is_in_layer: bool, child_hit_test: impl Fn()) {
        let start_depth = self.hit_depth;
        self.hit_depth += 1;
        self.values.insert(self.hit_depth as usize, node.clone());
        self.distance_from_edge_and_in_layer.insert(self.hit_depth as usize, DistanceInLayer {
            distance: distance_from_edge,
            is_in_layer,
        });

        self.resize_to_hit_depth();
        child_hit_test();
        self.hit_depth = start_depth;
    }

    pub(crate) fn is_not_empty(&self) -> bool {
        self.size != 0
    }

    pub(crate) fn clear(&mut self) {
        self.values.clear();
        self.distance_from_edge_and_in_layer.clear();
        self.hit_depth = -1;
        self.resize_to_hit_depth();
    }

    fn find_best_hit_distance(&self) -> DistanceInLayer {
        let mut best_distance = DistanceInLayer { distance: f32::MAX, is_in_layer: false };

        for i in (self.hit_depth + 1) as usize..self.size {
            let distance = self.distance_from_edge_and_in_layer.get(&i).unwrap();
            best_distance = if distance < &best_distance { *distance } else { best_distance };

            if best_distance.distance < 0f32 && best_distance.is_in_layer {
                return best_distance;
            }
        }

        best_distance
    }

    pub(crate) fn has_hit(&self) -> bool {
        let distance = self.find_best_hit_distance();
        distance.distance < 0f32 && distance.is_in_layer
    }
}

