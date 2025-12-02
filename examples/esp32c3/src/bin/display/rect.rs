use embedded_graphics::geometry::{Point, Size};
use embedded_graphics::primitives::Rectangle;

/// Extension trait for rectangles.
pub trait RectangleExt {
    /// Returns the rectangle's area that only has pixels to the left of the specified area.
    fn left_of(&self, other: &Self) -> Self;

    /// Returns the rectangle's area that only has pixels to the right of the specified area.
    fn right_of(&self, other: &Self) -> Self;

    /// Returns the rectangle's area that only has pixels above the specified area.
    fn above(&self, other: &Self) -> Self;

    /// Returns the rectangle's area that only has pixels below the specified area.
    fn below(&self, other: &Self) -> Self;
}

impl RectangleExt for Rectangle {
    fn left_of(&self, other: &Self) -> Self {
        let top_left = self.top_left;
        let width = other.top_left.x.saturating_sub(self.top_left.x);
        let width = width.try_into().unwrap_or_default();
        let size = Size::new(width, self.size.height);
        let size = self.size.component_min(size);

        Self { top_left, size }
    }

    fn right_of(&self, other: &Self) -> Self {
        let right = other.top_left.x.saturating_add_unsigned(other.size.width);
        let top_left = Point::new(right, self.top_left.y);
        let top_left = self.top_left.component_max(top_left);
        let width = right.saturating_sub(self.top_left.x);
        let width = width.try_into().unwrap_or_default();
        let size = Size::new(width, Default::default());
        let size = self.size.saturating_sub(size);

        Self { top_left, size }
    }

    fn above(&self, other: &Self) -> Self {
        let top_left = self.top_left;
        let height = other.top_left.y.saturating_sub(self.top_left.y);
        let height = height.try_into().unwrap_or_default();
        let size = Size::new(self.size.width, height);
        let size = self.size.component_min(size);

        Self { top_left, size }
    }

    fn below(&self, other: &Self) -> Self {
        let bottom = other.top_left.y.saturating_add_unsigned(other.size.height);
        let top_left = Point::new(self.top_left.x, bottom);
        let top_left = self.top_left.component_max(top_left);
        let height = bottom.saturating_sub(self.top_left.y);
        let height = height.try_into().unwrap_or_default();
        let size = Size::new(Default::default(), height);
        let size = self.size.saturating_sub(size);

        Self { top_left, size }
    }
}
