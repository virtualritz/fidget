//! Infrastructure and algorithms for complex closed-form implicit surfaces.
//!
//! ```
//! use fidget::context::Context;
//! use fidget::bind::eval;
//! use fidget::render::render2d::{render, RenderConfig};
//!
//! let (shape, ctx) = eval("sqrt(x*x + y*y) - 1").unwrap();
//! let tape = ctx.get_tape(shape, u8::MAX);
//! let cfg = RenderConfig {
//!     image_size: 32,
//!     ..RenderConfig::default()
//! };
//!
//! // This will print
//! //           XXXXXXXXXX
//! //       XXXXXXXXXXXXXXXXXX
//! //     XXXXXXXXXXXXXXXXXXXXXX
//! //   XXXXXXXXXXXXXXXXXXXXXXXXXX
//! //   XXXXXXXXXXXXXXXXXXXXXXXXXX
//! // XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
//! // XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
//! // XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
//! // XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
//! // XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
//! //   XXXXXXXXXXXXXXXXXXXXXXXXXX
//! //   XXXXXXXXXXXXXXXXXXXXXXXXXX
//! //     XXXXXXXXXXXXXXXXXXXXXX
//! //       XXXXXXXXXXXXXXXXXX
//! //           XXXXXXXXXX
//! ```
pub use fidget_core::*;

/// 2D and 3D rendering
pub mod render {
    pub use fidget_render::*;
}

/// Rhai bindings
pub mod bind {
    pub use fidget_rhai::*;
}
