use kira::{
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
    spatial::{
		scene::{SpatialSceneSettings, SpatialSceneHandle},
		listener::{ListenerSettings, ListenerHandle},
		emitter::{EmitterHandle, EmitterSettings, EmitterDistances},
	},
	sound::{PlaybackState, static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings}},
    tween::{Tween, Easing},
    CommandError
};
use mint::{Vector3, Quaternion};

use std::{collections::HashMap};
use std::time::Duration;

use shared::shared_components::*;
type Entity = DefaultKey;

use slotmap::{SecondaryMap, DefaultKey, SparseSecondaryMap};

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
    music: Option<StaticSoundHandle>
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
            music: None
        };
        // TODO: load from config file
        player.source_map.insert("fire".to_string(),
            StaticSoundData::from_file("resources/audio/blast0.ogg", 
            StaticSoundSettings::default()).unwrap());

        player.source_map.insert("hit".to_string(),
            StaticSoundData::from_file("resources/audio/hit.ogg", 
            StaticSoundSettings::default()).unwrap());

        player.source_map.insert("thruster".to_string(),
            StaticSoundData::from_file("resources/audio/thruster.ogg", 
            StaticSoundSettings::default()
            .loop_region(0.5..2.5)
            .volume(0.5)
            .fade_in_tween(Tween{
                start_time:kira::StartTime::Immediate, 
                duration:Duration::from_millis(10), 
                easing:Easing::Linear}))
        .unwrap());
        player.source_map.insert("death".to_string(),
            StaticSoundData::from_file("resources/audio/bell2.ogg", 
            StaticSoundSettings::default().volume(0.8))
        .unwrap());
        
        player.source_map.insert("lobby".to_string(),
            StaticSoundData::from_file("resources/audio/lobby.ogg",
            StaticSoundSettings::default().loop_region(0. ..90.))
        .unwrap());

        player.source_map.insert("throw".to_string(),
            StaticSoundData::from_file("resources/audio/throw.ogg", 
            StaticSoundSettings::default()).unwrap());
        
        player.source_map.insert("release".to_string(),
            StaticSoundData::from_file("resources/audio/release.ogg", 
            StaticSoundSettings::default()).unwrap());

        player.source_map.insert("reload".to_string(),
            StaticSoundData::from_file("resources/audio/reload.ogg", 
            StaticSoundSettings::default()).unwrap());

        player.source_map.insert("attach".to_string(),
            StaticSoundData::from_file("resources/audio/attach.ogg", 
            StaticSoundSettings::default()).unwrap());

        Some(player)
    }

    /**
     * Plays a sound and places its static sound handle in a list of sounds.
     */
    pub fn play_sound(&mut self, name: &String, x: f32, y: f32, z: f32, source: Option<Entity>) -> Result<(),Box<dyn std::error::Error>> {
        let emitter = self.scene.add_emitter(Vector3{x, y, z}, EmitterSettings::default()
            .persist_until_sounds_finish(true)
            .distances(EmitterDistances{min_distance:shared::ATT_MIN, max_distance:shared::ATT_MAX})
            .attenuation_function(Easing::Linear))?;
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
     * Plays a sound once, just for the player.
     */
    pub fn play_static(&mut self, name: &String) -> Result<StaticSoundHandle,Box<dyn std::error::Error>> {
        let handle = self.manager.play(self.source_map[name].clone())?;
        Ok(handle)
    }

    /**
     * Plays a sound and then puts it in the music slot.
     */
    pub fn play_music(&mut self, name: &String) {
        if self.music.is_some() {
            self.music.as_mut().unwrap().stop(Tween::default()).unwrap();
        }
        let handle = self.play_static(name).unwrap();
        self.music = Some(handle);
    }

    /**
     * Stops the currently playing music track.
     */
    pub fn stop_music(&mut self) {
        if self.music.is_some() {
            self.music.as_mut().unwrap().stop(Tween::default()).unwrap();
            self.music = None;
        }
    }

    /**
     * Stops the thruster sound of a given player.
     */
    pub fn stop_thruster (&mut self, player: Entity) {
        if self.thrusters.contains_key(player) {
            self.thrusters[player].handle.stop(Tween::default()).unwrap();
        }
    }

    pub fn stop_all_sounds(&mut self) {
        for (_, sound) in &mut self.thrusters {
            sound.handle.stop(Tween::default()).unwrap();
        }
        self.thrusters.clear();
        self.sound_vec.clear();
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