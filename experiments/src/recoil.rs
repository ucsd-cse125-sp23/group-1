use rapier3d::prelude::*;
use rapier_testbed3d::Testbed;

use std::time::SystemTime;

pub fn init_world(testbed: &mut Testbed) {
    // world
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();

    // player
    let player_rigid_body = RigidBodyBuilder::dynamic().gravity_scale(0.0).build();
    let player_handle = bodies.insert(player_rigid_body);
    let player_collider = ColliderBuilder::cone(1.0, 1.0).build();
    colliders.insert_with_parent(player_collider, player_handle, &mut bodies);

    // let now = SystemTime::now();

    testbed.add_callback(move |mut graphics, physics_state, physics_events, run_state| {

    });

    testbed.set_world(bodies, colliders, impulse_joints, multibody_joints);
    testbed.look_at(point![0.0, 0.0, 30.0], point![0.0, 0.0, 0.0]);
}