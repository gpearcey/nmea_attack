
#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Default for Point {
    fn default () -> Point {
        Point{x: 0, y: 0, z:0}
    }
}
    