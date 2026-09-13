use godot::{
    classes::{
        AudioEffectReverb, AudioServer, AudioStream, AudioStreamPlaybackPolyphonic,
        AudioStreamPlayer, AudioStreamPlayer2D, Tween, object::ConnectFlags,
    },
    prelude::*,
    tools::try_get_autoload_by_name,
};

use crate::{
    entities::audio_tirgger::AudioEffectType, managers::save::SaveManager, message::Message,
};

#[derive(Default, Debug)]
enum Track {
    #[default]
    Unset,
    First,
    Last,
}

#[derive(GodotConvert, Debug, Default, Export, Var, PartialEq, Eq, Clone)]
#[godot(via = GString)]
pub enum UIAudio {
    #[default]
    Bloop,
    Error,
    Select,
    Success,
    Woosh,
}

#[derive(GodotClass)]
#[class(base = Node, init)]
pub(crate) struct AudioManager {
    base: Base<Node>,
    #[init(node = "%Music")]
    music: OnReady<Gd<AudioStreamPlayer>>,
    #[init(node = "%Music1")]
    music_bak: OnReady<Gd<AudioStreamPlayer>>,
    #[init(node = "%Ui")]
    ui: OnReady<Gd<AudioStreamPlayer>>,
    #[init(val = Track::default())]
    track_index: Track,

    #[export_group(name = "UI Sound")]
    #[export]
    ui_bloop: Option<Gd<AudioStream>>,
    #[export]
    ui_error: Option<Gd<AudioStream>>,
    #[export]
    ui_select: Option<Gd<AudioStream>>,
    #[export]
    ui_success: Option<Gd<AudioStream>>,
    #[export]
    ui_woosh: Option<Gd<AudioStream>>,

    #[init(val = Array::default())]
    tweens: Array<Gd<Tween>>,

    #[init(val = vec![])]
    spatials: Vec<Gd<AudioStreamPlayer2D>>,
}

#[godot_api]
impl INode for AudioManager {
    fn ready(&mut self) {
        Message::singleton()
            .signals()
            .play_music()
            .connect_other(&*self, Self::on_play_audio);
        Message::singleton()
            .signals()
            .play_ui_audio()
            .connect_other(&*self, Self::play_ui_audio);
        self.signals()
            .recyle_spatial()
            .connect_self(Self::recyle_player);
        self.init_volume();
    }
}

#[godot_api]
impl AudioManager {
    #[signal]
    fn recyle_spatial(ap: InstanceId);

    fn recyle_player(&mut self, id: InstanceId) {
        if let Ok(ref player) = Gd::<AudioStreamPlayer2D>::try_from_instance_id(id) {
            if let Some(ref mut parent) = player.get_parent() {
                godot_print!("回收空间特效播放器： {id}");
                parent.remove_child(player);
                self.spatials.push(player.clone());
            }
        }
    }

    fn init_volume(&self) {
        if let Ok(mut save_helper) = try_get_autoload_by_name::<SaveManager>("SaveHelper") {
            let (music_volume, sfx_volume, ui_volume) = save_helper.bind_mut().get_volume();
            AudioServer::singleton().set_bus_volume_linear(2, music_volume as f32);
            AudioServer::singleton().set_bus_volume_linear(3, sfx_volume as f32);
            AudioServer::singleton().set_bus_volume_linear(4, ui_volume as f32);
        } else {
            godot_print!("init sound volume, but save manager not found!");
        }
    }

    fn on_play_audio(&mut self, audio: Gd<AudioStream>, effect_type: AudioEffectType) {
        self.kill_all_tween();
        let current_player = self.get_track_player(&self.track_index);
        // godot_print!(
        //     "audio: 播放music: {:?} -> {:?}",
        //     audio.get_path(),
        //     current_player.get_name()
        // );
        if let Some(current_stream) = current_player.get_stream() {
            if current_stream.get_path() == audio.get_path() {
                // godot_print!("audio: 已经在播放了: {:?}", audio);
                return;
            }
            // godot_print!("audio: 路径不同")
        } else {
            // godot_print!("audio: 没有播放呢");
        }
        let next_track_index = self.get_next_track(&self.track_index);
        let mut next_player = self.get_track_player(&next_track_index);
        //fadeout current;
        let current_tween = self.fade_track(current_player, 0.0);
        //set next
        next_player.set_stream(&audio);
        next_player.set_volume_linear(0.0);
        next_player.play();
        //fade in next
        let next_tween = self.fade_track(next_player, 1.0);
        self.set_effect(effect_type);
        self.track_index = next_track_index;
        self.tweens.push(&current_tween);
        self.tweens.push(&next_tween);
    }

    fn kill_all_tween(&mut self) {
        for mut tween in self.tweens.iter_shared() {
            tween.stop();
        }
        self.tweens.clear();
    }

    fn get_next_track(&self, track: &Track) -> Track {
        match track {
            Track::First => Track::Last,
            _ => Track::First,
        }
    }

    fn get_track_player(&self, track: &Track) -> Gd<AudioStreamPlayer> {
        match track {
            Track::Unset => self.music.clone(),
            Track::First => self.music_bak.clone(),
            Track::Last => self.music.clone(),
        }
    }

    fn fade_track(&mut self, track: Gd<AudioStreamPlayer>, to: f64) -> Gd<Tween> {
        // godot_print!("fade music: {:?} -> {}", track.get_name(), to);
        let mut tween = self.base_mut().create_tween();
        tween.tween_property(&track, "volume_linear", &to.to_variant(), 0.5);
        tween
    }

    fn set_effect(&mut self, effect_type: AudioEffectType) {
        if let Some(reverb_fx) = AudioServer::singleton().get_bus_effect(1, 0)
            && let Ok(mut reverb) = reverb_fx.try_cast::<AudioEffectReverb>()
        {
            AudioServer::singleton().set_bus_effect_enabled(1, 0, true);
            match effect_type {
                AudioEffectType::None => {
                    AudioServer::singleton().set_bus_effect_enabled(1, 0, false);
                }
                AudioEffectType::SMALL => {
                    reverb.set_room_size(0.2);
                }
                AudioEffectType::MEDUIM => {
                    reverb.set_room_size(0.5);
                }
                AudioEffectType::LARGE => {
                    reverb.set_room_size(0.8);
                }
            };
        }
    }

    fn play_ui_audio(&mut self, audio_type: UIAudio) {
        if let Some(stream) = match audio_type {
            UIAudio::Bloop => &self.ui_bloop,
            UIAudio::Error => &self.ui_error,
            UIAudio::Select => &self.ui_select,
            UIAudio::Woosh => &self.ui_woosh,
            UIAudio::Success => &self.ui_success,
        } {
            // godot_print!("播放ui: {}", stream.get_path());
            self.ui.play();
            if let Some(player) = self.ui.get_stream_playback()
                && let Ok(mut player) = player.try_cast::<AudioStreamPlaybackPolyphonic>()
            {
                player.play_stream(stream);
            }
        }
    }
    pub fn play_spatial_sound(&mut self, audio: Gd<AudioStream>, pos: Vector2) {
        let mut ap = self
            .spatials
            .pop()
            .unwrap_or(AudioStreamPlayer2D::new_alloc());
        self.base_mut().add_child(&ap);
        ap.set_bus("SFX");
        ap.set_global_position(pos);
        ap.set_stream(&audio);
        ap.play();
        let instance_id = self.base().instance_id();
        let ap_id = ap.instance_id();
        ap.connect_flags(
            "finished",
            &Callable::from_fn("once_spatial_trigger", move |_| {
                let node = Gd::<AudioManager>::from_instance_id(instance_id);
                node.signals().recyle_spatial().emit(ap_id);
            }),
            ConnectFlags::ONE_SHOT,
        );
    }
}
