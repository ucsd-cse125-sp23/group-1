use rapier3d::prelude::*;
use rapier_testbed3d::Testbed;

use std::f32::consts::PI;
use std::time::SystemTime;
use nalgebra::*;

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

    let mut now = SystemTime::now();
    let mut iter = 0;

    testbed.add_callback(move |_, physics_state, _, _| {
        let player_rigid_body = physics_state.bodies.get_mut(player_handle).unwrap();

        if now.elapsed().unwrap().as_secs() >= 1 {
            // 1 sec
            match iter{
                0 => {
                    let player_rot = player_rigid_body.rotation();
                    player_rigid_body.set_rotation(UnitQuaternion::from_euler_angles(0.0, 0.0, PI) * player_rot, true);
                }
                1 => {
                    let player_rot = player_rigid_body.rotation();
                    player_rigid_body.set_rotation(UnitQuaternion::from_euler_angles(0.0, 0.0, PI / 3.0) * player_rot, true);
                }
                _ => {

                }
            }
            iter = (iter + 1) % 2;
            now = SystemTime::now();
        }

        let player_rot = player_rigid_body.rotation();
        let impulse = player_rot.transform_vector(&Vector3::new(0.0,-0.5,0.0));
        player_rigid_body.apply_impulse(impulse, true);
    });

    testbed.set_world(bodies, colliders, impulse_joints, multibody_joints);
    testbed.look_at(point![-15.0, -15.0, 50.0], point![-15.0, -15.0, 0.0]);
}