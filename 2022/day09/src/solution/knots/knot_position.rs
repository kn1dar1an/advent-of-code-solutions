#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct KnotPosition {
    pub x_pos: i32,
    pub y_pos: i32,
}

impl KnotPosition {
    pub fn new() -> Self {
        Self { x_pos: 0, y_pos: 0 }
    }

    pub fn distance_to(&self, other: &KnotPosition) -> (i32, i32) {
        let distance_x: i32 = self.x_pos - other.x_pos;
        let distance_y: i32 = self.y_pos - other.y_pos;

        (distance_x, distance_y)
    }

    pub fn to_string(&self) -> String {
        format!("x: {}, y: {}", self.x_pos, self.y_pos)
    }
}
