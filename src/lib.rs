//! # olimp - Opinionated Lisp IMPlementation
//!
//! Olimp is a lisp implementation that compiles the source code to a byte format
//! which then is interpreted by it's own interpreter.
//!
//! # Example
//!
//! ```rust
//! use olimp::OlimpBuilder;
//! use olimp::include_math;
//!
//! let (compiler, runtime) = OlimpBuilder::new()
//!     .magic("abcdfgh")
//!     .include_builtins(include_math)
//!     .build();
//! ```
//!
//! The magic is a seven byte sequence that is saved in the compiled code to
//! ensure that the file has been compiled for the correct version of the
//! interpreter.
//!
//! The Runtime is only constructed if a vec of bytes are present. These must be
//! set with the .add_program() function. If this was not done, there will be
//! `None` returned for the runtime.

mod builder;
mod compiler;
mod fn_collector;
mod interpreter;

pub use crate::builder::OlimpBuilder;
pub use crate::compiler::compiler::Compiler;
pub use crate::interpreter::runtime::Runtime;

pub use crate::interpreter::builtins::math::include_math;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const MAGIC: &str = env!("COMMIT_HASH");
