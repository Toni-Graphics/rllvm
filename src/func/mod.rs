//! Functions
//! 
//! ## Example
//! 
//! ```rust
//! use std::error::Error;
//! use rllvm::{contxt::{contxt::Context, jit::JitFunction}, ir::{ir::*, r#type::Type}};
//! use target_lexicon::Triple;
//! 
//! fn main() -> Result<(), Box<dyn Error>>{
//!     let mut contxt = Context::new( Triple::host() )?;
//!     let func = contxt.add_function("main", vec![], Type::u32);
//! 
//!     func.ir.push( Return::new(5) );
//! 
//!     unsafe {
//!         let mut func: JitFunction<unsafe extern "C" fn() -> u32> = contxt.get_jit_function("main")?;
//!         let out = func.call();
//! 
//!         println!("main() -> {}", out);
//! 
//!         assert_eq!(out, 5);
//!     }
//! 
//!     Ok(())
//! }
//! ```

pub mod asmfunc;
pub mod func;

pub use func::Function;
pub use asmfunc::AsmFunction;