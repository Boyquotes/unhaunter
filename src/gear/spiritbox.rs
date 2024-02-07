use bevy::prelude::*;
use rand::Rng;

use crate::board::Position;

use super::{on_off, playergear::EquipmentPosition, Gear, GearKind, GearSpriteID, GearUsable};

#[derive(Component, Debug, Clone, Default)]
pub struct SpiritBox {
    pub enabled: bool,
    pub mode_frame: u32,
    pub ghost_answer: bool,
    pub last_change_secs: f32,
}

impl GearUsable for SpiritBox {
    fn get_sprite_idx(&self) -> GearSpriteID {
        match self.enabled {
            true => match self.ghost_answer {
                true => match self.mode_frame % 2 {
                    0 => GearSpriteID::SpiritBoxAns1,
                    _ => GearSpriteID::SpiritBoxAns2,
                },
                false => match self.mode_frame % 3 {
                    0 => GearSpriteID::SpiritBoxScan1,
                    1 => GearSpriteID::SpiritBoxScan2,
                    _ => GearSpriteID::SpiritBoxScan3,
                },
            },
            false => GearSpriteID::SpiritBoxOff,
        }
    }

    fn get_display_name(&self) -> &'static str {
        "Spirit Box"
    }

    fn get_status(&self) -> String {
        let name = self.get_display_name();
        let on_s = on_off(self.enabled);
        let msg = if self.enabled {
            "Scanning..".to_string()
        } else {
            "".to_string()
        };
        format!("{name}: {on_s}\n{msg}")
    }

    fn set_trigger(&mut self, _gs: &mut super::GearStuff) {
        self.enabled = !self.enabled;
    }

    fn box_clone(&self) -> Box<dyn GearUsable> {
        Box::new(self.clone())
    }
    fn update(&mut self, gs: &mut super::GearStuff, _pos: &Position, _ep: &EquipmentPosition) {
        let sec = gs.time.elapsed_seconds();
        let delta = sec - self.last_change_secs;
        self.mode_frame = (sec * 4.0).round() as u32;
        if !self.enabled {
            return;
        }
        let mut rng = rand::thread_rng();

        if self.ghost_answer {
            if delta > 3.0 {
                self.ghost_answer = false;
            }
        } else if delta > 0.3 {
            self.last_change_secs = sec;
            gs.play_audio("sounds/effects-radio-scan.ogg".into(), 0.3);
            let r = rng.gen_range(0..100);

            self.ghost_answer = true;
            match r {
                0 => gs.play_audio("sounds/effects-radio-answer1.ogg".into(), 0.7),
                1 => gs.play_audio("sounds/effects-radio-answer2.ogg".into(), 0.7),
                2 => gs.play_audio("sounds/effects-radio-answer3.ogg".into(), 0.7),
                3 => gs.play_audio("sounds/effects-radio-answer4.ogg".into(), 0.4),
                _ => self.ghost_answer = false,
            }
        }
    }
}

impl From<SpiritBox> for Gear {
    fn from(value: SpiritBox) -> Self {
        Gear::new_from_kind(GearKind::SpiritBox(value))
    }
}
