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
	sound::{PlaybackState, static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings}},
    tween::{Tween},
    CommandError
};
use mint::{Vector3, Quaternion};
use std::collections::HashMap;

use shared::shared_components::*;
type Entity = DefaultKey;

use slotmap::{SecondaryMap, DefaultKey, SparseSecondaryMap};

#[derive(Clone, Copy)]
struct VecEntry {
    id: EmitterId,
    source: Option<Entity>
}

impl VecEntry {
    pub fn new(id: EmitterId, source: Option<Entity>) -> VecEntry {
        VecEntry{
            id: id,
            source: source,
        }
    }
}

/**
 * Single audio player for the game.
 * Ties an emitter to each sound, which is inefficient. Too bad!
 */
pub struct AudioPlayer {
    manager: AudioManager,
    listener: ListenerHandle,
    scene: SpatialSceneHandle,
    source_map: HashMap<String, StaticSoundData>,
    
    sound_vec: Vec<Sound>, // non-thruster sounds
    thrusters: SparseSecondaryMap<Entity, Sound>, // thruster sounds
}

struct Sound {
    handle: StaticSoundHandle,
    emitter: EmitterHandle,
    source: Option<Entity>,
}

impl AudioPlayer {
    pub fn new() -> Option<AudioPlayer> {
        let mut manager: AudioManager;
        match AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()) {
            Ok(am) => manager = am,
            Err(e) => {
                eprintln!("Error loading audio manager: {e}");
                return None;
            } 
        }
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
            source_map: HashMap::new(),
            sound_vec: vec![],
            thrusters: SparseSecondaryMap::new(),
        };
        // TODO: load from config file
        player.source_map.insert("fire".to_string(), StaticSoundData::from_file("resources/audio/blast0.ogg", StaticSoundSettings::default()).unwrap());
        player.source_map.insert("thruster".to_string(), 
            StaticSoundData::from_file("resources/audio/thruster.ogg", 
            StaticSoundSettings::default().loop_region(2.0..3.0)).unwrap());
        Some(player)
    }

    /**
     * Plays a sound and places its static sound handle in a list of sounds.
     */
    pub fn play_sound(&mut self, name: &String, x: f32, y: f32, z: f32, source: Option<Entity>) -> Result<(),Box<dyn std::error::Error>> {
        let emitter = self.scene.add_emitter(Vector3{x, y, z}, EmitterSettings::default().persist_until_sounds_finish(true))?;
        let sound_handle = self.manager.play(self.source_map[name]
            .with_modified_settings(|settings| settings.output_destination(&emitter)))?;
        let sound = Sound{handle: sound_handle, emitter: emitter, source: source};

        // thruster cannot be passed without a source. or else.
        if name == "thruster" {
            self.thrusters.insert(source.unwrap(), sound);
        } else {
            self.sound_vec.push(sound);
        }
        
        Ok(())
    }

    /**
     * Stops the thruster sound of a given player.
     */
    pub fn stop_thruster (&mut self, player: Entity) {
        if self.thrusters.contains_key(player) {
            self.thrusters[player].handle.stop(Tween::default()).unwrap();
        }
    }

    // Takes player position and updates listener position
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
        for sound in &mut self.sound_vec {
            match sound.source {
                Some(source) => {
                    let pos = &pos_map[source];
                    sound.emitter.set_position(Vector3{x:pos.x, y:pos.y, z:pos.z}, Tween::default()).unwrap();
                }
                None => ()
            }
        }
        for (player, thruster) in &mut self.thrusters {
            let pos = &pos_map[player];
            thruster.emitter.set_position(Vector3{x:pos.x, y:pos.y, z:pos.z}, Tween::default()).unwrap();
        }
    }

    /**
     * Removes old emitters for sounds that have stopped playing.
     */
    pub fn prune_emitters(&mut self) {
        self.sound_vec.retain(|s| s.handle.state() != PlaybackState::Stopped);
    }
}