mod new;
pub mod util;
mod bump;
mod r#override;

pub use new::{new, Template};
pub use bump::bump;
pub use r#override::{run_override, Override, strings_into_overrides};