use bevy::prelude::*;

use super::{on_off, GearSpriteID, GearUsable};

#[derive(Component, Debug, Clone, Default, PartialEq, Eq)]
pub struct EStaticMeter {
    pub enabled: bool,
}

impl GearUsable for EStaticMeter {
    fn get_sprite_idx(&self) -> GearSpriteID {
        match self.enabled {
            true => GearSpriteID::EStaticMeter,
            false => GearSpriteID::EStaticMeter,
        }
    }

    fn get_display_name(&self) -> &'static str {
        "Electrostatic Meter"
    }

    fn get_status(&self) -> String {
        let name = self.get_display_name();
        let on_s = on_off(self.enabled);
        let msg = if self.enabled {
            "Reading: 200V/m".to_string()
        } else {
            "".to_string()
        };
        format!("{name}: {on_s}\n{msg}")
    }

    fn set_trigger(&mut self) {
        self.enabled = !self.enabled;
    }

    fn box_clone(&self) -> Box<dyn GearUsable> {
        Box::new(self.clone())
    }
}
