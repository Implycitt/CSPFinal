use bevy::prelude::*;

pub struct HitPointsPlugin;

impl Plugin for HitPointsPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component)]
pub struct HitPoints {
    pub current: u32,
    pub max: u32,
}

impl Default for HitPoints {
    fn default() -> Self {
        Self::set(1)
    }
}

impl HitPoints {
    pub fn set(val: u32) -> Self {
        Self {
            current: val,
            max: val,
        }
    }

    pub fn sub(&mut self, val: u32) {
        self.current = self.current.saturating_sub(val);
    }

    pub fn is_zero(&self) -> bool {
        self.current == 0
    }
}
