use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
    spatial::{
		scene::{SpatialSceneSettings, SpatialSceneHandle},
		listener::{ListenerSettings, ListenerHandle},
		emitter::{EmitterHandle, EmitterId, EmitterSettings},
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings}, 
    tween::{Tween},
    CommandError
};
use mint::{Vector3, Quaternion};
use std::collections::HashMap;
use std::time::{Instant, Duration};

use shared::shared_components::*;
use shared::shared_functions::*;
use shared::*;

type Entity = DefaultKey;

use slotmap::{SecondaryMap, DefaultKey};

#[derive(Clone, Copy)]
struct VecEntry {
    id: EmitterId,
    start: Instant,
    source: Option<Entity>
}

impl VecEntry {
    pub fn new(id: EmitterId, source: Option<Entity>) -> VecEntry {
        VecEntry{
            id: id,
            start: Instant::now(),
            source: source,
        }
    }
    pub fn expired(self) -> bool {
        return self.start.elapsed() >= Duration::from_secs(shared::EMITTER_LIFETIME);
    }
}

pub struct AudioPlayer {
    manager: AudioManager,
    listener: ListenerHandle,
    scene: SpatialSceneHandle,
    sound_map: HashMap<String, StaticSoundData>,
    e_map: HashMap<EmitterId, EmitterHandle>,
    sound_vec: Vec<VecEntry>,
}

impl AudioPlayer {
    pub fn default() -> AudioPlayer {
        let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let mut scene = manager.add_spatial_scene(SpatialSceneSettings::default()).unwrap();
        let listener = scene.add_listener(
            Vector3{x:0.,y:0.,z:0.}, 
            Quaternion{v: Vector3{x:0., y:0., z:0.}, s:1.}, 
            ListenerSettings::default())
        .unwrap();
        let mut player = AudioPlayer {
            manager: manager,
            scene: scene,
            listener: listener,
            sound_map: HashMap::new(),
            e_map: HashMap::new(),
            sound_vec: vec![],
        };
        // TODO: load from config file
        player.sound_map.insert("fire".to_string(), StaticSoundData::from_file("resources/audio/blast0.ogg", StaticSoundSettings::default()).unwrap());
        player
    }

    pub fn play_sound(&mut self, name: &String, x: f32, y: f32, z: f32, source: Option<Entity>) -> Result<EmitterId,Box<dyn std::error::Error>> {
        let emitter = self.scene.add_emitter(Vector3{x, y, z}, EmitterSettings::default().persist_until_sounds_finish(true))?;
        let id = emitter.id();
        self.manager.play(self.sound_map[name].with_settings(StaticSoundSettings::new().output_destination(&emitter)))?;
        self.e_map.insert(id, emitter);
        self.sound_vec.push(VecEntry::new(id, source),);
        // println!("{}", self.manager.num_sounds());
        Ok(id)
    }

    // TODO: stop looped sound function
    pub fn stop_sound() {

    }
    // called periodically
    pub fn move_listener(&mut self, x: f32, y: f32, z: f32, qx: f32, qy: f32, qz: f32, qw: f32) -> Result<(),CommandError> {
        self.listener.set_position(Vector3{x:x, y:y, z:z}, 
            Tween::default()
            // Tween {
            //     start_time:kira::StartTime::Immediate,
            //     duration:Duration::ZERO,
            //     easing:Linear
            // }
        )?;
        self.listener.set_orientation(Quaternion{v:Vector3{x:qx, y:qy, z:qz}, s:qw}, 
            Tween::default()
            // Tween {
            //     start_time:kira::StartTime::Immediate,
            //     duration:Duration::ZERO,
            //     easing:Linear
            // }
        )?;
        
        Ok(())
    }

    /**
     * Removes old emitters and updates positions of emitters tied to entities.
     */
    pub fn update_emitters(&mut self, pos_map: &SecondaryMap<Entity, PositionComponent>) {
        self.prune_emitters();
        for sound in &self.sound_vec {
            match sound.source {
                Some(source) => {
                    let pos = &pos_map[source];
                    match self.e_map.get_mut(&sound.id) {
                        Some(handle) => {
                            handle.set_position(Vector3{x:pos.x, y:pos.y, z:pos.z}, Tween::default()).unwrap();
                        }
                        None => {eprintln!("Emitter not found!")} // THIS SHOULD NEVER HAPPEN
                    }
                }
                None => ()
            }
        }
    }

    /**
     * Removes old emitters for sounds based on max. sound effect length.
     * Janky, but the emitter doesn't expose a function to say when it's done explicitly.
     * (Or it does and we're too dumb to find it.)
     */
    pub fn prune_emitters(&mut self) {
        while self.sound_vec.len() != 0 && self.sound_vec[0].expired() {
            self.e_map.remove(&self.sound_vec[0].id);
            self.sound_vec.pop();
        }
    }
}