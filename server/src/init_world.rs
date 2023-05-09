use crate::ecs::*;
use rapier3d::{dynamics::RigidBodySet, geometry::{ColliderSet,SharedShape}};
use nalgebra::{Isometry3,Translation3,UnitQuaternion};

pub fn init_world(ecs: &mut ECS, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
    ecs.spawn_prop(rigid_body_set, collider_set, "cube".to_string(), "cube".to_string(), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true, SharedShape::cuboid(1.0,1.0,1.0), 1.0, 0.0);
    ecs.spawn_prop(rigid_body_set, collider_set, "cube".to_string(), "cube".to_string(), 0.0, 5.0, 0.0, 0.0, 0.0, 0.0, true, SharedShape::cuboid(1.0,1.0,1.0), 1.0, 0.0);
    ecs.spawn_prop(rigid_body_set, collider_set, "cube".to_string(), "cube".to_string(), 0.0, -5.0, 0.0, 0.0, 0.0, 0.0, true, SharedShape::cuboid(1.0,1.0,1.0), 1.0, 0.0);
    ecs.spawn_prop(rigid_body_set, collider_set, "cube".to_string(), "cube".to_string(), 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, true, SharedShape::cuboid(1.0,1.0,1.0), 1.0, 0.0);
    ecs.spawn_prop(rigid_body_set, collider_set, "cube".to_string(), "cube".to_string(), -5.0, 0.0, 0.0, 0.0, 0.0, 0.0, true, SharedShape::cuboid(1.0,1.0,1.0), 1.0, 0.0);
}

pub fn init_player_spawns(ecs: &mut ECS) {
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(0.0, 0.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(0.0, 5.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(0.0, -5.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(5.0, 0.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
    ecs.spawnpoints.push(Isometry3::from_parts(Translation3::new(-5.0, 0.0, 3.0), UnitQuaternion::from_euler_angles(0.0,0.0,0.0)));
}