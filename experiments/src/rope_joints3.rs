// rope joint demo from rapier source

use rapier3d::prelude::*;
use rapier_testbed3d::Testbed;
use std::time::SystemTime;
use nalgebra::distance;
use nalgebra::geometry::OPoint;

pub fn init_world(testbed: &mut Testbed) {
    /*
     * World
     */
    let mut bodies = RigidBodySet::new();
    let mut colliders = ColliderSet::new();
    let mut impulse_joints = ImpulseJointSet::new();
    let multibody_joints = MultibodyJointSet::new();

    /*
     * Ground
     */
    let ground_size = 0.75;
    let ground_height = 0.1;

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![0.0, -ground_height, 0.0]);
    let floor_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_size);
    colliders.insert_with_parent(collider, floor_handle, &mut bodies);

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![
        -ground_size - ground_height,
        ground_height,
        0.0
    ]);
    let floor_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_height, ground_height, ground_size);
    colliders.insert_with_parent(collider, floor_handle, &mut bodies);

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![
        ground_size + ground_height,
        ground_height,
        0.0
    ]);
    let floor_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_height, ground_height, ground_size);
    colliders.insert_with_parent(collider, floor_handle, &mut bodies);

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![
        0.0,
        ground_height,
        -ground_size - ground_height
    ]);
    let floor_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_height);
    colliders.insert_with_parent(collider, floor_handle, &mut bodies);

    let rigid_body = RigidBodyBuilder::fixed().translation(vector![
        0.0,
        ground_height,
        ground_size + ground_height
    ]);
    let floor_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(ground_size, ground_height, ground_height);
    colliders.insert_with_parent(collider, floor_handle, &mut bodies);

    /*
     * Character we will control manually.
     */

    let rigid_body =
        RigidBodyBuilder::kinematic_position_based().translation(vector![0.0, 0.3, 0.0]);
    let character_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::cuboid(0.15, 0.3, 0.15);
    colliders.insert_with_parent(collider, character_handle, &mut bodies);

    testbed.set_initial_body_color(character_handle, [255. / 255., 131. / 255., 244.0 / 255.]);

    /*
     * Tethered Ball
     */
    let rad = 0.04;

    let rigid_body =
        RigidBodyBuilder::new(RigidBodyType::Dynamic).translation(vector![2.0, 1.0, 0.0])
        .gravity_scale(0.0).linvel(vector![0.0,-1.0,0.0]);
    let child_handle = bodies.insert(rigid_body);
    let collider = ColliderBuilder::ball(rad);
    colliders.insert_with_parent(collider, child_handle, &mut bodies);

    let joint = RopeJointBuilder::new()
        .local_anchor2(point![0.0, 0.0, 0.0])
        .limits([2.0, 2.0]);
    let joint_handle = impulse_joints.insert(character_handle, child_handle, joint, true);

    let mut now = SystemTime::now();

    let mut lim = 2.0;

    testbed.add_callback(move |_, physics_state, _, _| {
        // if now.elapsed().unwrap().as_secs() >= 1 {
        //     let ropejoint = physics_state.impulse_joints.get_mut(joint_handle).unwrap();
        //     ropejoint.data.set_limits(JointAxis::X, [1.0,1.0]);
        //     ropejoint.data.set_limits(JointAxis::Y, [1.0,1.0]);
        //     ropejoint.data.set_limits(JointAxis::Z, [1.0,1.0]);
        // }
        let char = physics_state.bodies.get_mut(character_handle).unwrap();
        let char_t = *char.translation();
        let char_point = OPoint::from(char_t);
        let child = physics_state.bodies.get_mut(child_handle).unwrap();
        let child_t = *child.translation();
        let child_point = OPoint::from(child_t);
        let dist = distance(&char_point,&child_point);
        println!("{dist}");
        let new_lim = dist / 3.0_f32.sqrt();
        if new_lim < lim {
            lim = new_lim;
        }
        // println!("{lim}");

        // if lim > 0.0 {lim -= 0.001};
        let ropejoint = physics_state.impulse_joints.get_mut(joint_handle).unwrap();
        ropejoint.data.set_limits(JointAxis::X, [lim,lim]);
        ropejoint.data.set_limits(JointAxis::Y, [lim,lim]);
        ropejoint.data.set_limits(JointAxis::Z, [lim,lim]);

        if now.elapsed().unwrap().as_secs() >= 2 {
            let char = physics_state.bodies.get_mut(character_handle).unwrap();
            char.apply_impulse((child_t-char_t).normalize() * 0.00001, true);
            let child = physics_state.bodies.get_mut(child_handle).unwrap();
            child.apply_impulse((char_t-child_t).normalize() * 0.00001, true);
        }
    });

    /*
     * Set up the testbed.
     */
    testbed.set_world(bodies, colliders, impulse_joints, multibody_joints);
    testbed.set_character_body(character_handle);
    testbed.look_at(point![10.0, 10.0, 10.0], point![0.0, 0.0, 0.0]);
}
