// ggez imports

use ggez::event::{self, EventHandler, KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{graphics, Context, ContextBuilder, GameResult};

// nphysics imports

use nalgebra::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::math::{Force, ForceType, Velocity};
use nphysics2d::object::{
    Body, BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet,
    Ground, RigidBody, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

// Constants

const BLUE: graphics::Color = graphics::Color {
    r: 0.01,
    g: 0.33,
    b: 0.98,
    a: 1.0,
};

const GREEN: graphics::Color = graphics::Color {
    r: 0.01,
    g: 0.98,
    b: 0.43,
    a: 1.0,
};

const BALL_RAD: f32 = 20.0;
const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

// Structures & enums

enum Action {
    Up,
    Left,
    Right,
    Hover,
}

impl Action {
    pub fn from_keycode(key: KeyCode) -> Option<Action> {
        match key {
            KeyCode::Up => Some(Action::Up),
            KeyCode::Left => Some(Action::Left),
            KeyCode::Right => Some(Action::Right),
            KeyCode::C => Some(Action::Hover),
            _ => None,
        }
    }
}

struct PhysicsStruct {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    bodies: DefaultBodySet<f32>,
    colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,
}

struct MyGame {
    physics: PhysicsStruct,
    ball: DefaultBodyHandle,
    hover_on: bool,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.

        let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 300.0));
        let geometrical_world = DefaultGeometricalWorld::new();
        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();

        // Ball

        let ball_shape = ShapeHandle::new(Ball::new(BALL_RAD));
        let rigid_body = RigidBodyDesc::new()
            .translation(Vector2::new(300.0, 300.0))
            // .velocity(Velocity::linear(300.0, -200.0))
            .linear_damping(1.0)
            .build();

        let ball = bodies.insert(rigid_body);
        let co = ColliderDesc::new(ball_shape.clone())
            .density(0.005)
            .build(BodyPartHandle(ball, 0));
        colliders.insert(co);

        // Ground

        let ground_size = WIN_WIDTH;
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 1.0)));

        let ground_handle = bodies.insert(Ground::new());
        let co = ColliderDesc::new(ground_shape)
            .translation(Vector2::new(0.0, WIN_HEIGHT))
            .build(BodyPartHandle(ground_handle, 0));
        colliders.insert(co);

        // Walls

        let wall_size = WIN_HEIGHT;
        let wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(1.0, wall_size)));
        let wall_handle_l = bodies.insert(Ground::new());
        let left_wall_co = ColliderDesc::new(wall_shape.clone())
            .translation(Vector2::new(0.0, 0.0))
            .build(BodyPartHandle(wall_handle_l, 0));
        let wall_handle_r = bodies.insert(Ground::new());
        let right_wall_co = ColliderDesc::new(wall_shape.clone())
            .translation(Vector2::new(WIN_WIDTH, 0.0))
            .build(BodyPartHandle(wall_handle_r, 0));
        colliders.insert(left_wall_co);
        colliders.insert(right_wall_co);

        // Return val

        let physics = PhysicsStruct {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
        };

        MyGame {
            physics,
            ball,
            hover_on: false,
        }
    }

    fn get_ball(&self) -> &RigidBody<f32> {
        self.physics
            .bodies
            .rigid_body(self.ball)
            .expect("Ball not found")
    }

    fn get_ball_mut(&mut self) -> &mut RigidBody<f32> {
        self.physics
            .bodies
            .rigid_body_mut(self.ball)
            .expect("Ball not found")
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        if self.hover_on {
            let hover_force: f32 = -1875.0;
            let ball_body = self.get_ball_mut();
            ball_body.apply_force(
                0,
                &Force::new(Vector2::new(0.0, hover_force), 0.0),
                ForceType::Force,
                true,
            );
        }

        self.physics.mechanical_world.step(
            &mut self.physics.geometrical_world,
            &mut self.physics.bodies,
            &mut self.physics.colliders,
            &mut self.physics.joint_constraints,
            &mut self.physics.force_generators,
        );

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        let ball_body = self.get_ball();
        let pos = ball_body.position();
        let translation = pos.translation.vector;
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(translation[0], translation[1]),
            BALL_RAD,
            0.001,
            if self.hover_on { GREEN } else { BLUE },
        )?;

        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        let impulse: f32 = 1000.0;
        let force_up = Vector2::new(0.0, -impulse);
        let force_left = Vector2::new(-impulse, 0.0);
        let force_right = Vector2::new(impulse, 0.0);

        if let Some(act) = Action::from_keycode(keycode) {
            let ball_body = self.get_ball_mut();
            match act {
                Action::Up => {
                    ball_body.apply_force(0, &Force::new(force_up, 0.0), ForceType::Impulse, true);
                }
                Action::Left => {
                    ball_body.apply_force(0, &Force::new(force_left, 0.0), ForceType::Impulse, true);
                }
                Action::Right => {
                    ball_body.apply_force(0, &Force::new(force_right, 0.0), ForceType::Impulse, true);
                }
                Action::Hover => {
                    self.hover_on = !self.hover_on;
                }
            }
        }
    }
}

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
