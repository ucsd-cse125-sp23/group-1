use nalgebra::*;
use rapier3d::prelude::*;

struct Player {
    handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    camera_vec: Vector3<f32>,
    firing: bool,
}

fn new_player(rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) -> Player {
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 0.0, 0.0])
        .build();
    let handle = rigid_body_set.insert(rigid_body);
    let collider = ColliderBuilder::capsule_y(1.0, 1.0).build();
    let collider_handle = collider_set.insert_with_parent(collider, handle, rigid_body_set);
    let player = Player {
        handle,
        collider_handle,
        camera_vec: vector![0.0, 0.0, -1.0],
        firing: false,
    };
    player
}

fn player_fire(player: &Player, rigid_body_set: &mut RigidBodySet) {
    if player.firing {
        let player_rigid_body = rigid_body_set.get_mut(player.handle).unwrap();
        let impulse = -10.0 * player.camera_vec;
        player_rigid_body.apply_impulse(impulse, true);
    }
}

fn player_get_pos(player: &Player, rigid_body_set: &mut RigidBodySet) -> Vector3<f32> {
    let player_rigid_body = rigid_body_set.get_mut(player.handle).unwrap();
    *player_rigid_body.translation()
}

fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let mut player = new_player(&mut rigid_body_set, &mut collider_set);

    let gravity = vector![0.0, 0.0, 0.0];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    for i in 0..200 {

        if i == 10 {
            player.firing = true;
        } else {
            player.firing = false;
        }
        
        player_fire(&player, &mut rigid_body_set);      

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        let player_pos = player_get_pos(&player, &mut rigid_body_set);

        println!(
            "{}",
            player_pos.z
        );
    }
}
