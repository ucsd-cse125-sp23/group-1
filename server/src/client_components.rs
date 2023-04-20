use slotmap::{SlotMap, SecondaryMap, DefaultKey};

pub struct PlayerWeaponComponent {
    cooldown: i8,
}