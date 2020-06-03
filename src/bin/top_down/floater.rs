use ggez::graphics::Color;
use nalgebra::Vector2;
use nphysics2d::object::DefaultBodyHandle;

pub const BLUE: Color = Color {
    r: 0.01,
    g: 0.33,
    b: 0.98,
    a: 1.0,
};

/* -- Eventually we'll be able to choose a color -- */
pub const GREEN: Color = Color {
    r: 0.01,
    g: 0.98,
    b: 0.43,
    a: 1.0,
};

pub const RED: Color = Color {
    r: 0.98,
    g: 0.01,
    b: 0.35,
    a: 1.0,
};

pub trait PhysicsBall {
    fn get_handle(&self) -> DefaultBodyHandle;
    fn size() -> f32;
    fn color() -> Color;
}

pub struct Floater {
    handle: DefaultBodyHandle,
    pub target: Option<Vector2<f32>>,
}

impl Floater {
    pub fn new(handle: DefaultBodyHandle) -> Self {
        Floater {
            handle,
            target: None,
        }
    }

    pub fn set_target(&mut self, target: Vector2<f32>) {
        self.target = Some(target);
    }
}

impl PhysicsBall for Floater {
    fn get_handle(&self) -> DefaultBodyHandle {
        self.handle
    }

    fn size() -> f32 {
        10.0
    }

    fn color() -> Color {
        BLUE
    }
}

pub struct BeachBall {
    handle: DefaultBodyHandle,
}

impl BeachBall {
    pub fn new(handle: DefaultBodyHandle) -> Self {
        BeachBall { handle }
    }
}

impl PhysicsBall for BeachBall {
    fn get_handle(&self) -> DefaultBodyHandle {
        self.handle
    }

    fn size() -> f32 {
        45.0
    }

    fn color() -> Color {
        RED
    }
}
