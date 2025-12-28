/*
*   Uma implementação que é basicamente uma cópia do mob, mas sem a necessidade
*   de mudar sprites.
*
*/

use godot::prelude::*;
use godot::classes::{IRigidBody2D, RigidBody2D, AnimatedSprite2D};

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Bullet {
        base: Base<RigidBody2D>
}

#[godot_api]
impl IRigidBody2D for Bullet {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self){
        self.base_mut().add_to_group("bullets");

        // Tecnicamente, não é animada, mas estava com preguiça e copiei o mob
        let mut sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        sprite.play();
    }
}

#[godot_api]
impl Bullet {
    #[func]
    fn on_visibility_screen_exit(&mut self) {
        self.base_mut().queue_free();
    }
}