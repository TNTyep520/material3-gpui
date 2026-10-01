// Copyright (c) 2026 TNTyep520
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/animation/M3Motion.java

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
