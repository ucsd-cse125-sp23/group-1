use rapier3d::prelude::*;
use rapier_testbed3d::Testbed;

use std::f32::consts::PI;

pub fn init_world(testbed: &mut Testbed) {
    // world
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();

    // ground
    let rigid_body = RigidBodyBuilder::fixed().translation(vector![0.0, 0.0, 0.0]).rotation(vector![0.0,0.0,-PI/4.0]).build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(10.0,1.0,10.0).restitution(1.0).restitution_combine_rule(CoefficientCombineRule::Max);
    colliders.insert_with_parent(collider, handle, &mut bodies);

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![16.0, 0.0, 0.0]).rotation(vector![0.0,0.0,PI/4.0]).build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(10.0,1.0,10.0).restitution(1.0).restitution_combine_rule(CoefficientCombineRule::Max);
    colliders.insert_with_parent(collider, handle, &mut bodies);

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![16.0, 16.0, 0.0]).rotation(vector![0.0,0.0,-PI/4.0]).build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(10.0,1.0,10.0).restitution(1.0).restitution_combine_rule(CoefficientCombineRule::Max);
    colliders.insert_with_parent(collider, handle, &mut bodies);

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![0.0, 16.0, 0.0]).rotation(vector![0.0,0.0,PI/4.0]).build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(10.0,1.0,10.0).restitution(1.0).restitution_combine_rule(CoefficientCombineRule::Max);
    colliders.insert_with_parent(collider, handle, &mut bodies);

    // ball
    let rigid_body = RigidBodyBuilder::dynamic().translation(vector![0.0, 10.0, 0.0]).linvel(vector![0.0, -10.0, 0.0]).gravity_scale(0.0).build();
    let handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::ball(1.0).restitution(1.0).build();
    colliders.insert_with_parent(collider, handle, &mut bodies);

    testbed.set_world(bodies, colliders, impulse_joints, multibody_joints);
    testbed.look_at(point![8.0, 8.0, 50.0], point![8.0, 8.0, 0.0]);
}