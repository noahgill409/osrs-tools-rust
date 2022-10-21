use crate::data::{Stance, StyleName, DT};

/// Style and collection type

pub struct Style {
    pub name: StyleName,
    pub dt: DT,
    pub stance: Stance,
    pub attack_speed_mod: Option<i8>,
    pub attack_range_mod: Option<i8>,
}

pub struct StyleCollection {
    pub styles: Vec<Style>,
    pub default: usize,
}

/// Implementation

impl Style {
    pub fn new(
        name: StyleName,
        dt: DT,
        stance: Stance,
        attack_speed_mod: Option<i8>,
        attack_range_mod: Option<i8>,
    ) -> Self {
        Self {
            name,
            dt,
            stance,
            attack_speed_mod,
            attack_range_mod,
        }
    }
}

impl StyleCollection {
    pub fn new(styles: Vec<Style>, default: usize) -> Self {
        match styles.len() > default {
            true => return Self { styles, default },
            false => panic!("Default index must point to a valid style in styles"),
        }
    }

    pub fn default_style(&self) -> &Style {
        &self.styles.get(self.default).unwrap()
    }
}
