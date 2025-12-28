use godot::prelude::*;
use godot::classes::{CanvasLayer, ICanvasLayer, Label, Button, Timer};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct HUD {
    base: Base<CanvasLayer>
}

#[godot_api]
impl ICanvasLayer for HUD {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let hud_gd = self.to_gd();
        
        let on_pressed = hud_gd.callable("on_start_button_pressed");
        self.start_button().connect("pressed", &on_pressed);

        let on_timeout = hud_gd.callable("on_message_timer_timeout");
        self.message_timer().connect("timeout", &on_timeout);
    }
}

#[godot_api]
impl HUD {
    #[signal]
    fn start_game();

    #[func]
    fn on_start_button_pressed(&mut self) {
        self.start_button().hide();
        self.base_mut().emit_signal("start_game", &[]);
    }

    #[func]
    fn on_message_timer_timeout(&mut self) {
        self.message_label().hide();
    }

    #[func]
    pub fn set_score(&mut self, score: u32) {
        // FIX: Pass a reference to the string. &String satisfies AsArg<GString>.
        self.score_label().set_text(&score.to_string());
    }

    #[func]
    pub fn show_text(&mut self, text: GString) {
        let mut msg_label = self.message_label();
        // FIX: Borrow the GString. &GString satisfies AsArg<GString>.
        msg_label.set_text(&text);
        msg_label.show();
        self.message_timer().start();
    }

    #[func]
    pub fn show_gameover(&mut self) {
        // show_text expects an owned GString, so .into() is correct here
        self.show_text("Game Over!".into());

        let mut timer = self.base().get_tree().unwrap().create_timer(2.0).unwrap();
        timer.connect("timeout", &self.to_gd().callable("show_start_button"));
    }

    #[func]
    fn show_start_button(&mut self) {
        let mut msg = self.message_label();
        // FIX: Passing a &str directly is the simplest way and satisfies AsArg.
        msg.set_text("Dodge the Creeps!");
        msg.show();
        self.start_button().show();
    }

    fn message_label(&self) -> Gd<Label> { self.base().get_node_as::<Label>("Message") }
    fn score_label(&self) -> Gd<Label> { self.base().get_node_as::<Label>("ScoreLabel") }
    fn start_button(&self) -> Gd<Button> { self.base().get_node_as::<Button>("StartButton") }
    fn message_timer(&self) -> Gd<Timer> { self.base().get_node_as::<Timer>("MessageTimer") }
}