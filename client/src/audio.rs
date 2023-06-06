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

#[derive(Clone, Copy)]
struct VecEntry {
    id: EmitterId,
    start: Instant,
}

impl VecEntry {
    pub fn new(id: EmitterId) -> VecEntry {
        VecEntry{
            id: id,
            start: Instant::now(),
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

    pub fn play_sound(&mut self, name: &String, x: f32, y: f32, z: f32) -> Result<EmitterId,Box<dyn std::error::Error>> {
        let emitter = self.scene.add_emitter(Vector3{x, y, z}, EmitterSettings::default().persist_until_sounds_finish(true))?;
        let id = emitter.id();
        self.manager.play(self.sound_map[name].with_settings(StaticSoundSettings::new().output_destination(&emitter)))?;
        self.e_map.insert(id, emitter);
        self.sound_vec.push(VecEntry::new(id));
        Ok(id)
    }

    // called once per frame
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

    pub fn move_emitter(&mut self, id: &EmitterId, x: f32, y: f32, z: f32) {
        match self.e_map.get_mut(id) {
            Some(handle) => {
                handle.set_position(Vector3{x, y, z}, Tween::default()).unwrap();
            }
            None => {eprintln!("Emitter not found!")}
        }
    }

    pub fn prune_emitters(&mut self) {
        while (&self.sound_vec[0]).expired() {
            self.sound_vec.pop();
        }
    }
}