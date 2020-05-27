// ggez imports

use ggez::event::{self, EventHandler};
use ggez::nalgebra as na;
use ggez::{graphics, Context, ContextBuilder, GameResult};

// nphysics imports

use nalgebra::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::math::{Force, ForceType};
use nphysics2d::object::{
    Body, BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet,
    Ground, RigidBody, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

use rand::Rng;
use rand::rngs::ThreadRng;

// Constants

const BLUE: graphics::Color = graphics::Color {
    r: 0.01,
    g: 0.33,
    b: 0.98,
    a: 1.0,
};

// const GREEN: graphics::Color = graphics::Color {
//     r: 0.01,
//     g: 0.98,
//     b: 0.43,
//     a: 1.0,
// };

const BALL_RAD: f32 = 10.0;
const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

const FLOATER_CNT: u32 = 15;

// Structures & enums

fn initialize_walls(bodies: &mut DefaultBodySet<f32>, colliders: &mut DefaultColliderSet<f32>) {
    // Ground & Ceiling

    let ground_size = WIN_WIDTH;
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 1.0)));

    let ground_handle = bodies.insert(Ground::new());
    let co = ColliderDesc::new(ground_shape.clone())
        .translation(Vector2::new(0.0, WIN_HEIGHT))
        .build(BodyPartHandle(ground_handle, 0));
    colliders.insert(co);
    let ceil_handle = bodies.insert(Ground::new());
    let ceil_co = ColliderDesc::new(ground_shape.clone())
        .translation(Vector2::new(0.0, 0.0))
        .build(BodyPartHandle(ceil_handle, 0));
    colliders.insert(ceil_co);

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

}

fn get_rand_x(rng: &mut ThreadRng) -> f32 {
    rng.gen_range(0.0, WIN_WIDTH)
}

fn get_rand_y(rng: &mut ThreadRng) -> f32 {
    rng.gen_range(0.0, WIN_HEIGHT)
}

struct Floater {
    handle: DefaultBodyHandle,
    target: Option<Vector2<f32>>,
}

impl Floater {
    fn new(
        bodies: &mut DefaultBodySet<f32>,
        colliders: &mut DefaultColliderSet<f32>,
        pos: Vector2<f32>,
    ) -> Self {
        let ball_shape = ShapeHandle::new(Ball::new(BALL_RAD));
        let rigid_body = RigidBodyDesc::new()
            .translation(pos)
            .linear_damping(1.0)
            .build();

        let ball = bodies.insert(rigid_body);
        let co = ColliderDesc::new(ball_shape.clone())
            .density(0.01)
            .build(BodyPartHandle(ball, 0));
        colliders.insert(co);

        Floater {
            handle: ball,
            target: None,
        }
    }

    fn set_target(&mut self, target: Vector2<f32>) {
        self.target = Some(target);
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
    floaters: Vec<Floater>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.

        let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0));
        let geometrical_world = DefaultGeometricalWorld::new();
        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();

        // Floater(s)
        let mut floaters = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..FLOATER_CNT {
            let mut new_float = Floater::new(
                &mut bodies,
                &mut colliders,
                Vector2::new(
                    get_rand_x(&mut rng),
                    get_rand_y(&mut rng),
                ),
            );
            new_float.set_target(Vector2::new(get_rand_x(&mut rng), get_rand_y(&mut rng)));
            floaters.push(new_float);
        }


        initialize_walls(&mut bodies, &mut colliders);

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
            floaters,
        }
    }

    fn get_floater_body(&self, handle: DefaultBodyHandle) -> &RigidBody<f32> {
        self.physics
            .bodies
            .rigid_body(handle)
            .expect("Ball not found")
    }

    fn get_floater_body_mut(&mut self, handle: DefaultBodyHandle) -> &mut RigidBody<f32> {
        self.physics
            .bodies
            .rigid_body_mut(handle)
            .expect("Ball not found")
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        let mut floaters_to_force = Vec::new();
        for floater in self.floaters.iter() {
            if let Some(target) = floater.target {
                floaters_to_force.push((floater.handle.clone(), target));
            }
        }

        for (handle, target) in floaters_to_force {
            let body = self.get_floater_body_mut(handle);
            let translation = body.position().translation.vector;
            body.apply_force(0, &Force::new(target - translation, 0.0), ForceType::Force, true);
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
        for floater in self.floaters.iter() {
            let ball_body = self.get_floater_body(floater.handle);
            let pos = ball_body.position();
            let translation = pos.translation.vector;
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(translation[0], translation[1]),
                BALL_RAD,
                0.001,
                BLUE,
            )?;

            graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        }

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
