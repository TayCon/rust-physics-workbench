mod physics;
mod floater;

use physics::PhysicsStruct;
use floater::Floater;

use ggez::event::{self, EventHandler};
use ggez::nalgebra as na;
use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::input::mouse::MouseButton;

use nalgebra::Vector2;

use rand::Rng;

use std::collections::HashSet;
use uuid::Uuid;

// Constants
const BALL_RAD: f32 = 10.0;
const WIN_WIDTH: f32 = 800.0;
const WIN_HEIGHT: f32 = 600.0;

const FLOATER_CNT: u32 = 15;

// Structures & enums

struct MyGame {
    physics: PhysicsStruct,
    floaters: Vec<Floater>,
    selected: HashSet<Uuid>
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut physics = PhysicsStruct::new((WIN_WIDTH, WIN_HEIGHT));

        // Floater(s)
        let mut floaters = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..FLOATER_CNT {
            let handle =
                physics.create_ball(BALL_RAD, Vector2::new(WIN_WIDTH / 2.0, WIN_HEIGHT / 2.0));
            let mut new_float = Floater::new(handle);
            new_float.set_target(Vector2::new(
                rng.gen_range(0.0, WIN_WIDTH),
                rng.gen_range(0.0, WIN_HEIGHT),
            ));
            floaters.push(new_float);
        }

        let selected = HashSet::new();
        MyGame { physics, floaters, selected}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        let mut floaters_to_force = Vec::new();
        for floater in self.floaters.iter() {
            if let Some(target) = floater.target {
                floaters_to_force.push((floater.get_handle().clone(), target));
            }
        }

        for (handle, target) in floaters_to_force {
            let translation = self.physics.get_pos_of(handle);
            self.physics.apply_force(handle, target - translation);
        }

        self.physics.step();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        for floater in self.floaters.iter() {
            let translation = self.physics.get_pos_of(floater.get_handle());
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(translation[0], translation[1]),
                BALL_RAD,
                0.001,
                floater.color,
            )?;

            graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;
        }

        graphics::present(ctx)
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match button {
            MouseButton::Left => {
                
            },
            MouseButton::Right => {

            },
            _ => ()
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
