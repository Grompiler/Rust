// comme ca on import qu'un truc dans le main avec mod systems; et on a tout
// The declaration in main will look for a file named `systems.rs` or `systems/mod.rs` and will
// insert its contents inside a module named `systems` under the main scope
mod paddle;

mod move_balls;
mod bounce;

mod winner;

pub use self::paddle::PaddleSystem; // pour utiliser directement PaddleSystem,
pub use self::move_balls::MoveBallsSystem; // MoveBallsSystem,
pub use self::bounce::BounceSystem; // BounceSystem, ... dans le main
pub use self::winner::WinnerSystem;
