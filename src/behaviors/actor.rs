use super::{attack::AttackBehavior, movement::MovementBehavior, wait::WaitBehavior};

pub type ActorBehavior = (MovementBehavior, AttackBehavior, WaitBehavior);
