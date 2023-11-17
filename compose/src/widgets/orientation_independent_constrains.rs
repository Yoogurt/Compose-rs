use crate::foundation::constraint::Constraints;
use crate::widgets::row_column_measurement_helper::LayoutOrientation;

#[derive(Clone)]
pub(crate) struct OrientationIndependentConstrains {
    value: Constraints,
}

impl OrientationIndependentConstrains {
    pub fn new(main_axis_min: usize, main_axis_max: usize, cross_axis_min: usize, cross_axis_max: usize) -> Self {
        Self {
            value: Constraints::new(main_axis_min..=main_axis_max, cross_axis_min..=cross_axis_max)
        }
    }

    pub fn new_with_constrains(value: Constraints) -> Self {
        Self { value }
    }

    pub fn main_axis_min(&self) -> usize {
        self.value.min_width
    }
    pub fn main_axis_max(&self) -> usize {
        self.value.max_width
    }
    pub fn cross_axis_min(&self) -> usize {
        self.value.min_height
    }
    pub fn cross_axis_max(&self) -> usize {
        self.value.max_height
    }

    pub fn new_with_orientation(constraints: Constraints, orientation: LayoutOrientation) -> Self {
        Self::new(
            match orientation {
                LayoutOrientation::Horizontal => constraints.min_width,
                LayoutOrientation::Vertical => constraints.min_height,
            },
            match orientation {
                LayoutOrientation::Horizontal => constraints.max_width,
                LayoutOrientation::Vertical => constraints.max_height,
            },
            match orientation {
                LayoutOrientation::Horizontal => constraints.min_height,
                LayoutOrientation::Vertical => constraints.min_width,
            },
            match orientation {
                LayoutOrientation::Horizontal => constraints.max_height,
                LayoutOrientation::Vertical => constraints.max_width,
            },
        )
    }

    pub fn to_box_constrains(&self, orientation: LayoutOrientation) -> Constraints {
        match orientation {
            LayoutOrientation::Horizontal => Constraints::new(self.main_axis_min()..=self.main_axis_max(), self.cross_axis_min()..=self.cross_axis_max()),
            LayoutOrientation::Vertical => Constraints::new(self.cross_axis_min()..=self.cross_axis_max(), self.main_axis_min()..=self.main_axis_max()),
        }
    }

    pub fn stretch_cross_axis(&self) -> Self {
        Self::new(
            self.main_axis_min(),
            self.main_axis_max(),
            if self.cross_axis_max() != Constraints::INFINITE { self.cross_axis_max() } else { self.cross_axis_min() },
            self.cross_axis_max(),
        )
    }

    pub fn max_height(&self, orientation: LayoutOrientation) -> usize {
        match orientation {
            LayoutOrientation::Horizontal => self.cross_axis_max(),
            LayoutOrientation::Vertical => self.main_axis_max(),
        }
    }

    pub fn copy(&self, main_axis_min: usize, main_axis_max: usize, cross_axis_min: usize, cross_axis_max: usize) -> Self {
        Self::new(main_axis_min, main_axis_max, cross_axis_min, cross_axis_max)
    }
}