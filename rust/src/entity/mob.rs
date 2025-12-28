use godot::prelude::*;
use godot::classes::{IRigidBody2D, RigidBody2D, AnimatedSprite2D};
use rand::seq::{IndexedRandom};


#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
        base: Base<RigidBody2D>
}

#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }

    fn ready(&mut self){
        let mut sprite = self.base().get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let anime_names = sprite.get_sprite_frames().unwrap().get_animation_names().to_vec();
        let mut rng = rand::rng();
        let animation = anime_names.choose(&mut rng).unwrap();

        // É mais fácil manter essa lógica aqui do que ficar jogando mais coisa pra main()
        if rand::random_bool(0.2) {

            self.base_mut().add_to_group("shooters");

            let shooter_hue_shift = Color::from_hsv(0.8, 0.7, 1.0);
            self.base_mut().set_modulate(shooter_hue_shift);

        }


        sprite.set_animation(animation.arg());

        sprite.play();
    }
}

#[godot_api]
impl Mob {
    #[func]
    fn on_visibility_screen_exit(&mut self) {
        self.base_mut().queue_free();
    }
}