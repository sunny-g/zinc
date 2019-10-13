//!
//! Transpiler output statements.
//!

mod debug;
mod r#let;
mod r#loop;
mod require;
mod r#struct;
mod r#type;

pub use self::debug::Output as DebugOutput;
pub use self::r#let::Output as LetOutput;
pub use self::r#loop::Output as LoopOutput;
pub use self::r#struct::Output as StructOutput;
pub use self::r#type::Output as TypeOutput;
pub use self::require::Output as RequireOutput;
