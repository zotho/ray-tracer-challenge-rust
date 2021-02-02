#[cfg(test)]
use super::Pattern;
#[cfg(test)]
use crate::{Color, Matrix, Point, IDENTITY};
#[cfg(test)]
use uuid::Uuid;
#[cfg(test)]
use serde::{Serialize, Deserialize};
#[cfg(test)]
use typetag;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
#[cfg(test)]
pub struct TestPattern {
    id: Uuid,
    pub transform: Matrix,
}

#[cfg(test)]
impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern {
            id: Uuid::new_v4(),
            transform: IDENTITY,
        }
    }
}

#[cfg(test)]
#[typetag::serde]
impl Pattern for TestPattern {
    fn id(&self) -> Uuid {
        self.id
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }

    fn transform(&self) -> Matrix {
        self.transform
    }
}

#[cfg(test)]
impl Default for TestPattern {
    fn default() -> Self {
        Self::new()
    }
}
