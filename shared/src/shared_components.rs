use serde::{Deserialize, Serialize};
use slotmap::{SlotMap, SecondaryMap, DefaultKey};
use std::net::{TcpStream};
use std::{str};
use std::io::Read;
type Entity = DefaultKey;

// client -> server component

#[derive(Serialize, Deserialize)]
pub struct PlayerInputComponent {
    pub lmb_clicked: bool,
    pub rmb_clicked: bool,
    pub w_pressed: bool,
    pub a_pressed: bool,
    pub s_pressed: bool,
    pub d_pressed: bool,
    pub r_pressed: bool,
    pub shift_pressed: bool,
    pub ctrl_pressed: bool,
    pub camera_front_x: f32,
    pub camera_front_y: f32,
    pub camera_front_z: f32,
}

impl PlayerInputComponent {
    pub fn default() -> PlayerInputComponent{
        PlayerInputComponent{
            lmb_clicked: false,
            rmb_clicked: false,
            w_pressed: false,
            a_pressed: false,
            s_pressed: false,
            d_pressed: false,
            shift_pressed: false,
            ctrl_pressed: false,
            r_pressed: false,
            camera_front_x: 0.0,
            camera_front_y: 0.0,
            camera_front_z: -1.0,
        }
    }
}

// server -> client components

#[derive(Serialize, Deserialize)]
pub struct ClientECS {
    pub name_components: SlotMap<Entity, String>,
    pub position_components: SecondaryMap<Entity, PositionComponent>,
    pub model_components: SecondaryMap<Entity, ModelComponent>,
    pub health_components: SecondaryMap<Entity, HealthComponent>,
    pub players: Vec<Entity>,
    pub renderables: Vec<Entity>,
    pub game_ended: bool,
}

impl ClientECS {
    pub fn default() -> ClientECS{
        ClientECS{
            name_components: SlotMap::new(),
            position_components: SecondaryMap::new(),
            model_components: SecondaryMap::new(),
            health_components: SecondaryMap::new(),
            players: vec![],
            renderables: vec![],
            game_ended: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LobbyECS {
    pub name_components: SlotMap<Entity, String>,
    pub players: Vec<Entity>,
    pub start_game: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub qx: f32,
    pub qy: f32,
    pub qz: f32,
    pub qw: f32,
}

impl PositionComponent {
    pub fn default() -> PositionComponent{
        PositionComponent {
            x:  (0.0),
            y:  (0.0),
            z:  (0.0),
            qx: (0.0),
            qy: (0.0),
            qz: (0.0),
            qw: (1.0)
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ModelComponent {
    pub modelname: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerWeaponComponent {
    pub cooldown: i16,
    pub ammo: u8,
    pub reloading: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HealthComponent {
    pub alive: bool,
    pub health: u8,
}

impl HealthComponent {
    pub fn default() -> HealthComponent{
        HealthComponent {
            alive : true,
            health : 1,
        }
    }
}

pub fn read_data(stream: &mut TcpStream) -> String {
    let mut size_buf = [0 as u8; 4];
    let mut size:u32 = 0;
    match stream.peek(&mut size_buf) {
        Ok(4) => {
            size = u32::from_be_bytes(size_buf);
        },
        Ok(_) => {},
        Err(_) => {}
    }

    let s_size = size.try_into().unwrap();
    let mut read_buf = vec![0 as u8; s_size];
    let mut message : &str = "";
    match stream.peek(&mut read_buf) {
        Ok(bytes_read) if bytes_read == s_size => {
            stream.read_exact(&mut read_buf).expect("read_exact did not read the same amount of bytes as peek");
            message = str::from_utf8(&read_buf[4..]).expect("Error converting buffer to string");
        },
        Ok(_) => {},
        Err(_) => {},
    }
    return message.to_string();
}