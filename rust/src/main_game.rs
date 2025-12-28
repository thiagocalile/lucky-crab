use godot::prelude::*;
use godot::classes::{Node, Marker2D, Timer, PathFollow2D, PackedScene, AudioStreamPlayer2D};
use godot::classes::RigidBody2D;
use crate::entity::{Player, Mob, Bullet};
use crate::hud::HUD;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Main {
    mob_scene: OnReady<Gd<PackedScene>>,
    bullet_scene: OnReady<Gd<PackedScene>>,
    player: OnReady<Gd<Player>>,
    hud: OnReady<Gd<HUD>>,

    score: u32,
    base: Base<Node>
}

#[godot_api]
impl INode for Main {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            // FIX: Use OnReady::new for scenes and OnReady::node for existing nodes
            mob_scene: OnReady::new(|| load::<PackedScene>("res://mob.tscn")),
            bullet_scene: OnReady::new(|| load::<PackedScene>("res://bullet.tscn")),
            player: OnReady::node("Player"),
            hud: OnReady::node("HUD"),
            score: 0,
            base
        }
    }

    fn ready(&mut self) {
        let game = self.to_gd();

        // FIX: Connect signals using the string-based API to bypass macro bugs
        (*self.player).connect("hit", &game.callable("game_over"));
        (*self.hud).connect("start_game", &game.callable("new_game"));

        self.start_timer().connect("timeout", &game.callable("on_start_timer_timeout"));
        self.mob_timer().connect("timeout", &game.callable("on_mob_timer_timeout"));
        self.bullet_timer().connect("timeout", &game.callable("on_bullet_timer_timeout"));
        self.score_timer().connect("timeout", &game.callable("on_score_timer_timeout"));
    }
}

#[godot_api]
impl Main {
    #[func]
    fn game_over(&mut self) {
        self.score_timer().stop();
        self.mob_timer().stop();
        self.hud.bind_mut().show_gameover();
        self.bgm().stop();
        self.game_over_sound().play();

    }

    #[func]
    fn new_game(&mut self) {
        self.score = 0;
        let start_pos = self.start_position().get_position();
        self.player.bind_mut().start(start_pos);

        self.start_timer().start();

        self.hud.bind_mut().set_score(self.score);
        self.hud.bind_mut().show_text("Get Ready".into());

        // FIX: Pass strings directly to engine-generic methods
        self.base()
            .get_tree()
            .unwrap()
            .call_group("mobs", "queue_free", &[]);

        self.base()
            .get_tree()
            .unwrap()
            .call_group("bullets", "queue_free", &[]);

        self.bgm().play();

    }

    #[func]
    fn on_score_timer_timeout(&mut self) {
        self.score += 1;
        self.hud.bind_mut().set_score(self.score);
    }

    #[func]
    fn on_start_timer_timeout(&mut self) {
        self.score_timer().start();
        self.mob_timer().start();
        self.bullet_timer().start();
    }

    #[func]
    fn on_mob_timer_timeout(&mut self) {
        let mut mob_spawn_location = self.base().get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        let mut mob_scene = self.mob_scene.instantiate_as::<Mob>();

        // Ensure floating point for progress
        let progress = rand::random_range(0.0..f32::MAX);
        mob_spawn_location.set_progress(progress);
        mob_scene.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + std::f32::consts::PI;
        direction += rand::random_range((-std::f32::consts::PI / 4.0)..std::f32::consts::PI / 4.0);
        mob_scene.set_rotation(direction);

        let velocity = Vector2::new(rand::random_range(150.0..250.0), 0.0);
        mob_scene.set_linear_velocity(velocity.rotated(direction));

        mob_scene.add_to_group("mobs");
        
        self.base_mut().add_child(&mob_scene);
    }

    #[func]
    fn on_bullet_timer_timeout(&mut self) {
        let player_pos = self.player.get_global_position();
        let shooters = self.base().get_tree().unwrap().get_nodes_in_group("shooters");
        for shooter_node in shooters.iter_shared() {
            // Cast the generic Node to a RigidBody2D so we can get its position
            let shooter = shooter_node.cast::<RigidBody2D>();
            let shooter_pos = shooter.get_global_position();

            // 3. Instantiate the bullet
            let mut bullet = self.bullet_scene.instantiate_as::<Bullet>();
            
            // Set starting position to the shooter
            bullet.set_global_position(shooter_pos);

            // 4. Calculate direction toward the player's position at this moment
            let direction = (player_pos - shooter_pos).normalized();
            
            // Set bullet rotation to face the direction of travel
            bullet.set_rotation(direction.angle());

            // 5. Set velocity
            let bullet_speed = 250.0;
            bullet.set_linear_velocity(direction * bullet_speed);

            // Add to scene (adding to base_mut ensures it's a child of Main)
            self.base_mut().add_child(&bullet);
        }


    }

    // --- Helper Getters ---
    fn start_position(&self) -> Gd<Marker2D> {
        self.base().get_node_as::<Marker2D>("StartPosition")
    }

    fn score_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("ScoreTimer")       
    }

    fn start_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("StartTimer")
    }

    fn mob_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("MobTimer")
    }

    fn bullet_timer(&self) -> Gd<Timer> {
        self.base().get_node_as::<Timer>("BulletTimer")
    }

    fn bgm(&self) -> Gd<AudioStreamPlayer2D> {
        self.base().get_node_as::<AudioStreamPlayer2D>("Music")
    }

    fn game_over_sound(&self) -> Gd<AudioStreamPlayer2D> {
        self.base().get_node_as::<AudioStreamPlayer2D>("DeathSound")
    }
}