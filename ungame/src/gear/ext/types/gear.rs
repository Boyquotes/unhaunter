use bevy::prelude::*;

use super::super::systemparam::gearstuff::GearStuff;
use uncore::{
    components::board::position::Position,
    types::gear::{equipmentposition::EquipmentPosition, spriteid::GearSpriteID},
};

use super::{traits::GearUsable, uncore_gearkind::GearKind};

/// A wrapper struct for holding a `GearKind`.
#[derive(Debug, Default, Clone)]
pub struct Gear {
    /// The type of gear being held.
    pub kind: GearKind,
    pub data: Option<Box<dyn GearUsable>>,
}

impl Gear {
    /// Creates a new empty Gear
    pub fn none() -> Self {
        Self {
            kind: GearKind::None,
            data: None,
        }
    }

    /// Creates a new Gear of the specified Kind
    pub fn new_from_kind(kind: GearKind, data: Box<dyn GearUsable>) -> Self {
        Self {
            kind,
            data: Some(data),
        }
    }

    /// Takes the content of the current Gear and returns it, leaving None.
    pub fn take(&mut self) -> Self {
        let mut new = Self::none();
        std::mem::swap(&mut new, self);
        new
    }
}

impl GearUsable for Gear {
    fn get_display_name(&self) -> &'static str {
        match &self.data {
            Some(x) => x.get_display_name(),
            None => "",
        }
    }

    fn get_description(&self) -> &'static str {
        match &self.data {
            Some(x) => x.get_description(),
            None => "",
        }
    }

    fn get_status(&self) -> String {
        match &self.data {
            Some(x) => x.get_status(),
            None => "".to_string(),
        }
    }

    fn set_trigger(&mut self, gs: &mut GearStuff) {
        let sound_file = "sounds/switch-on-1.ogg";
        gs.play_audio_nopos(sound_file.into(), 0.6);
        let ni = |s| warn!("Trigger not implemented for {:?}", s);
        match &mut self.data {
            Some(x) => x.set_trigger(gs),
            None => ni(&self),
        }
    }

    fn get_sprite_idx(&self) -> GearSpriteID {
        match &self.data {
            Some(x) => x.get_sprite_idx(),
            None => GearSpriteID::None,
        }
    }

    fn box_clone(&self) -> Box<dyn GearUsable> {
        unimplemented!();
    }

    fn update(&mut self, gs: &mut GearStuff, pos: &Position, ep: &EquipmentPosition) {
        if let Some(x) = &mut self.data {
            x.update(gs, pos, ep)
        }
    }
}

impl From<GearKind> for Gear {
    fn from(value: GearKind) -> Self {
        use crate::gear::prelude::*;
        match value {
            GearKind::Thermometer => Thermometer::default().into(),
            GearKind::EMFMeter => EMFMeter::default().into(),
            GearKind::Recorder => Recorder::default().into(),
            GearKind::Flashlight => Flashlight::default().into(),
            GearKind::GeigerCounter => GeigerCounter::default().into(),
            GearKind::UVTorch => UVTorch::default().into(),
            GearKind::IonMeter => IonMeter::default().into(),
            GearKind::SpiritBox => SpiritBox::default().into(),
            GearKind::ThermalImager => ThermalImager::default().into(),
            GearKind::RedTorch => RedTorch::default().into(),
            GearKind::Photocam => Photocam::default().into(),
            GearKind::Compass => Compass::default().into(),
            GearKind::EStaticMeter => EStaticMeter::default().into(),
            GearKind::Videocam => Videocam::default().into(),
            GearKind::MotionSensor => MotionSensor::default().into(),
            GearKind::RepellentFlask => RepellentFlask::default().into(),
            GearKind::QuartzStone => QuartzStoneData::default().into(),
            GearKind::Salt => SaltData::default().into(),
            GearKind::SageBundle => SageBundleData::default().into(),
            GearKind::None => Gear::none(),
        }
    }
}
