use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new<S: ToString>(name: S) -> Self {
        Name {
            name: name.to_string(),
        }
    }
}
