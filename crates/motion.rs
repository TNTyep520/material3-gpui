//! MD3 运动（Motion）系统。
//!
//! 移植自 [m3fx](https://github.com/Glavo/m3fx) 的 `animation` 包
//! 与 `internal` 中的动画实现（Apache-2.0，© 2026 Glavo），
//! 对齐 Compose Material 3 的运动模型。
//!
//! - [`scheme::MotionScheme`]：按语义角色（effects/spatial × fast/default/slow）
//!   归组的运动规格，提供 standard / expressive 两套默认；
//! - [`easing`]：MD3 全部缓动曲线；
//! - [`spring`]：阻尼弹簧解析求解器；
//! - [`duration`]：运动时长令牌；
//! - [`animatable`]：可重定向弹簧值运行时与组件逐帧驱动器。
//!
//! 用法（组件内）：
//! ```ignore
//! use material3_gpui::motion::{Animatable, AnimationDriver, AnimatedComponent, MotionRole};
//!
//! struct MyControl {
//!     opacity: Animatable,
//!     driver: AnimationDriver,
//! }
//!
//! impl AnimatedComponent for MyControl {
//!     fn step(&mut self, now: Instant) -> bool {
//!         self.opacity.tick(now)
//!     }
//!     fn driver_mut(&mut self) -> &mut AnimationDriver { &mut self.driver }
//! }
//! ```

#[path = "motion/Animatable.rs"]
pub mod animatable;
#[path = "motion/Duration.rs"]
pub mod duration;
#[path = "motion/Easing.rs"]
pub mod easing;
#[path = "motion/Scheme.rs"]
pub mod scheme;
#[path = "motion/Spring.rs"]
pub mod spring;

pub use animatable::{Animatable, AnimatedComponent, AnimationDriver, lerp_color};
pub use duration::ms;
pub use easing::Easing;
pub use scheme::{MotionRole, MotionScheme, MotionSchemeBuilder, MotionSpec};
pub use spring::SpringParameters;
