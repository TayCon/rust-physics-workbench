use nalgebra::Vector2;
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::math::{Force, ForceType};
use nphysics2d::object::{
    Body, BodyPartHandle, ColliderDesc, DefaultBodyHandle, DefaultBodySet, DefaultColliderSet,
    Ground, RigidBodyDesc,
};
use nphysics2d::world::{DefaultGeometricalWorld, DefaultMechanicalWorld};

pub struct PhysicsStruct {
    mechanical_world: DefaultMechanicalWorld<f32>,
    geometrical_world: DefaultGeometricalWorld<f32>,
    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    joint_constraints: DefaultJointConstraintSet<f32>,
    force_generators: DefaultForceGeneratorSet<f32>,
}

impl PhysicsStruct {
    pub fn new(dims: (f32, f32)) -> Self {
        let mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 0.0));
        let geometrical_world = DefaultGeometricalWorld::new();
        let mut bodies = DefaultBodySet::new();
        let mut colliders = DefaultColliderSet::new();
        let joint_constraints = DefaultJointConstraintSet::new();
        let force_generators = DefaultForceGeneratorSet::new();

        PhysicsStruct::initialize_walls(&mut bodies, &mut colliders, dims);

        PhysicsStruct {
            mechanical_world,
            geometrical_world,
            bodies,
            colliders,
            joint_constraints,
            force_generators,
        }
    }

    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraints,
            &mut self.force_generators,
        );
    }

    pub fn create_ball(
        &mut self,
        radius: f32,
        start_pos: Vector2<f32>,
        density: f32,
    ) -> DefaultBodyHandle {
        let ball_shape = ShapeHandle::new(Ball::new(radius));
        let rigid_body = RigidBodyDesc::new()
            .translation(start_pos)
            .linear_damping(1.0)
            .build();

        let ball = self.bodies.insert(rigid_body);
        let co = ColliderDesc::new(ball_shape.clone())
            .density(density)
            .build(BodyPartHandle(ball, 0));
        self.colliders.insert(co);

        ball
    }

    pub fn apply_force(&mut self, object_handle: DefaultBodyHandle, force_vector: Vector2<f32>) {
        let body = self
            .bodies
            .rigid_body_mut(object_handle)
            .expect("Object not found");
        body.apply_force(0, &Force::new(force_vector, 0.0), ForceType::Force, true);
    }

    pub fn get_pos_of(&self, object_handle: DefaultBodyHandle) -> Vector2<f32> {
        let body = self
            .bodies
            .rigid_body(object_handle)
            .expect("Object not found");

        body.position().translation.vector
    }

    pub fn floater_contains(
        &self,
        object_handle: DefaultBodyHandle,
        radius: f32,
        query: Vector2<f32>,
    ) -> bool {
        let body = self
            .bodies
            .rigid_body(object_handle)
            .expect("Object not found");

        let pos = body.position().translation.vector;
        let diff = query - pos;

        diff.norm() < radius
    }

    fn initialize_walls(
        bodies: &mut DefaultBodySet<f32>,
        colliders: &mut DefaultColliderSet<f32>,
        dims: (f32, f32),
    ) {
        // Ground & Ceiling
        let win_width = dims.0;
        let win_height = dims.1;

        let ground_size = win_width;
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(ground_size, 1.0)));

        let ground_handle = bodies.insert(Ground::new());
        let co = ColliderDesc::new(ground_shape.clone())
            .translation(Vector2::new(0.0, win_height))
            .build(BodyPartHandle(ground_handle, 0));
        colliders.insert(co);
        let ceil_handle = bodies.insert(Ground::new());
        let ceil_co = ColliderDesc::new(ground_shape.clone())
            .translation(Vector2::new(0.0, 0.0))
            .build(BodyPartHandle(ceil_handle, 0));
        colliders.insert(ceil_co);

        // Walls

        let wall_size = win_height;
        let wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(1.0, wall_size)));
        let wall_handle_l = bodies.insert(Ground::new());
        let left_wall_co = ColliderDesc::new(wall_shape.clone())
            .translation(Vector2::new(0.0, 0.0))
            .build(BodyPartHandle(wall_handle_l, 0));
        let wall_handle_r = bodies.insert(Ground::new());
        let right_wall_co = ColliderDesc::new(wall_shape.clone())
            .translation(Vector2::new(win_width, 0.0))
            .build(BodyPartHandle(wall_handle_r, 0));
        colliders.insert(left_wall_co);
        colliders.insert(right_wall_co);
    }
}
