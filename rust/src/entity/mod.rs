/// entity/mod.rs
mod player;
mod mob;
mod bullet;

// vamos deixar os itens de player visiveis para o escopo acima
pub use player::*;
pub use mob::*;
pub use bullet::*;
