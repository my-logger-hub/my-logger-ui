mod render_log_ball;
pub use render_log_ball::*;
mod icons;
pub use icons::*;
mod input_bool;
pub use input_bool::*;
mod input_string;
pub use input_string::*;
mod input_i64;
pub use input_i64::*;

#[derive(Debug, Clone)]
pub enum InputValue<T: Clone> {
    Value(T),
    InvalidValue,
}
