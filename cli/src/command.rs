mod new;
pub mod util;
mod bump;
mod r#override;

pub use new::{new, Template};
pub use bump::bump;
pub use r#override::{add_override, clear_overrides, Override, strings_into_overrides};