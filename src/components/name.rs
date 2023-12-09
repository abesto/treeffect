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

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
