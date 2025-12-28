use godot::prelude::*;

mod entity;
mod main_game;
mod hud;

struct GameExtension;

#[gdextension]
unsafe impl ExtensionLibrary for GameExtension {}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
