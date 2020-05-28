use ggez::graphics::Color;
use nalgebra::Vector2;
use nphysics2d::object::DefaultBodyHandle;

const BLUE: Color = Color {
    r: 0.01,
    g: 0.33,
    b: 0.98,
    a: 1.0,
};

/* -- Eventually we'll be able to choose a color -- */
// const GREEN: graphics::Color = graphics::Color {
//     r: 0.01,
//     g: 0.98,
//     b: 0.43,
//     a: 1.0,
// };

pub struct Floater {
    handle: DefaultBodyHandle,
    pub target: Option<Vector2<f32>>,
    pub color: Color,
}

impl Floater {
    pub fn new(handle: DefaultBodyHandle) -> Self {
        Floater {
            handle,
            target: None,
            color: BLUE,
        }
    }

    pub fn set_target(&mut self, target: Vector2<f32>) {
        self.target = Some(target);
    }

    pub fn get_handle(&self) -> DefaultBodyHandle {
        self.handle
    }
}
