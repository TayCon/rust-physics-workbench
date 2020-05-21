// ggez imports
use ggez::event::{self, EventHandler};
use ggez::nalgebra as na;
use ggez::{graphics, Context, ContextBuilder, GameResult};

// nphysics imports
use nalgebra::Vector2;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::math::Velocity;
use nphysics2d::object::{DefaultBodyHandle, DefaultBodySet, DefaultColliderSet, RigidBodyDesc};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

const BLUE: graphics::Color = graphics::Color {
    r: 0.01,
    g: 0.33,
    b: 0.98,
    a: 1.0,
};

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
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 9.81 * 4.0));
        let geometrical_world = DefaultGeometricalWorld::new();
        let mut bodies = DefaultBodySet::new();
        let colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();

        let rigid_body = RigidBodyDesc::new()
            .translation(Vector2::new(300.0, 300.0))
            .velocity(Velocity::linear(20.0, -100.0))
            .mass(50.0)
            .build();

        let ball = bodies.insert(rigid_body);

        let physics = PhysicsStruct {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
        };

        MyGame { physics, ball }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
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
        let ball_body = self
            .physics
            .bodies
            .rigid_body(self.ball)
            .expect("Ball not found");
        let pos = ball_body.position();
        let translation = pos.translation.vector;
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(translation[0], translation[1]),
            25.0,
            0.01,
            BLUE,
        )?;

        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

        graphics::present(ctx)
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
