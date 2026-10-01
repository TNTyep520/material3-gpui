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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/animation/M3MotionScheme.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/animation/M3MotionSpec.java

use crate::tokens::{ExpressiveMotionTokens, StandardMotionTokens};
use std::time::Duration;

use super::duration as dur;
use super::easing::{self, Easing};
use super::spring::SpringParameters;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MotionRole {
    FastEffects,

    DefaultEffects,

    SlowEffects,

    FastSpatial,

    DefaultSpatial,

    SlowSpatial,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionSpec {
    pub spring: SpringParameters,

    pub fallback_duration: Duration,

    pub fallback_easing: Easing,
}

impl MotionSpec {
    pub const fn spring(
        spring: SpringParameters,
        fallback_duration: Duration,
        fallback_easing: Easing,
    ) -> Self {
        Self {
            spring,
            fallback_duration,
            fallback_easing,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MotionScheme {
    specs: [MotionSpec; 6],
}

impl MotionScheme {
    pub fn expressive() -> Self {
        let standard = Self::standard();
        standard
            .rebuild()
            .role(
                MotionRole::FastEffects,
                MotionSpec {
                    spring: SpringParameters::new(
                        f64::from(ExpressiveMotionTokens::SPRING_FAST_EFFECTS_DAMPING),
                        f64::from(ExpressiveMotionTokens::SPRING_FAST_EFFECTS_STIFFNESS),
                    ),
                    ..*standard.spec(MotionRole::FastEffects)
                },
            )
            .role(
                MotionRole::DefaultEffects,
                MotionSpec {
                    spring: SpringParameters::new(
                        f64::from(ExpressiveMotionTokens::SPRING_DEFAULT_EFFECTS_DAMPING),
                        f64::from(ExpressiveMotionTokens::SPRING_DEFAULT_EFFECTS_STIFFNESS),
                    ),
                    ..*standard.spec(MotionRole::DefaultEffects)
                },
            )
            .role(
                MotionRole::SlowEffects,
                MotionSpec {
                    spring: SpringParameters::new(
                        f64::from(ExpressiveMotionTokens::SPRING_SLOW_EFFECTS_DAMPING),
                        f64::from(ExpressiveMotionTokens::SPRING_SLOW_EFFECTS_STIFFNESS),
                    ),
                    ..*standard.spec(MotionRole::SlowEffects)
                },
            )
            .role(
                MotionRole::FastSpatial,
                MotionSpec {
                    spring: SpringParameters::new(
                        f64::from(ExpressiveMotionTokens::SPRING_FAST_SPATIAL_DAMPING),
                        f64::from(ExpressiveMotionTokens::SPRING_FAST_SPATIAL_STIFFNESS),
                    ),
                    fallback_easing: easing::EXPRESSIVE_FAST_SPATIAL,
                    ..*standard.spec(MotionRole::FastSpatial)
                },
            )
            .role(
                MotionRole::DefaultSpatial,
                MotionSpec {
                    spring: SpringParameters::new(
                        f64::from(ExpressiveMotionTokens::SPRING_DEFAULT_SPATIAL_DAMPING),
                        f64::from(ExpressiveMotionTokens::SPRING_DEFAULT_SPATIAL_STIFFNESS),
                    ),
                    fallback_easing: easing::EXPRESSIVE_DEFAULT_SPATIAL,
                    ..*standard.spec(MotionRole::DefaultSpatial)
                },
            )
            .role(
                MotionRole::SlowSpatial,
                MotionSpec {
                    spring: SpringParameters::new(
                        f64::from(ExpressiveMotionTokens::SPRING_SLOW_SPATIAL_DAMPING),
                        f64::from(ExpressiveMotionTokens::SPRING_SLOW_SPATIAL_STIFFNESS),
                    ),
                    fallback_easing: easing::EXPRESSIVE_SLOW_SPATIAL,
                    ..*standard.spec(MotionRole::SlowSpatial)
                },
            )
            .build()
    }

    pub fn standard() -> Self {
        Self {
            specs: [
                MotionSpec::spring(
                    SpringParameters::new(
                        f64::from(StandardMotionTokens::SPRING_FAST_EFFECTS_DAMPING),
                        f64::from(StandardMotionTokens::SPRING_FAST_EFFECTS_STIFFNESS),
                    ),
                    dur::SHORT3,
                    easing::FAST_EFFECTS,
                ),
                MotionSpec::spring(
                    SpringParameters::new(
                        f64::from(StandardMotionTokens::SPRING_DEFAULT_EFFECTS_DAMPING),
                        f64::from(StandardMotionTokens::SPRING_DEFAULT_EFFECTS_STIFFNESS),
                    ),
                    dur::SHORT4,
                    easing::DEFAULT_EFFECTS,
                ),
                MotionSpec::spring(
                    SpringParameters::new(
                        f64::from(StandardMotionTokens::SPRING_SLOW_EFFECTS_DAMPING),
                        f64::from(StandardMotionTokens::SPRING_SLOW_EFFECTS_STIFFNESS),
                    ),
                    dur::MEDIUM2,
                    easing::SLOW_EFFECTS,
                ),
                MotionSpec::spring(
                    SpringParameters::new(
                        f64::from(StandardMotionTokens::SPRING_FAST_SPATIAL_DAMPING),
                        f64::from(StandardMotionTokens::SPRING_FAST_SPATIAL_STIFFNESS),
                    ),
                    dur::MEDIUM3,
                    easing::STANDARD_SPATIAL,
                ),
                MotionSpec::spring(
                    SpringParameters::new(
                        f64::from(StandardMotionTokens::SPRING_DEFAULT_SPATIAL_DAMPING),
                        f64::from(StandardMotionTokens::SPRING_DEFAULT_SPATIAL_STIFFNESS),
                    ),
                    dur::LONG2,
                    easing::STANDARD_SPATIAL,
                ),
                MotionSpec::spring(
                    SpringParameters::new(
                        f64::from(StandardMotionTokens::SPRING_SLOW_SPATIAL_DAMPING),
                        f64::from(StandardMotionTokens::SPRING_SLOW_SPATIAL_STIFFNESS),
                    ),
                    Duration::from_millis(750),
                    easing::STANDARD_SPATIAL,
                ),
            ],
        }
    }

    pub fn spec(&self, role: MotionRole) -> &MotionSpec {
        let index = match role {
            MotionRole::FastEffects => 0,
            MotionRole::DefaultEffects => 1,
            MotionRole::SlowEffects => 2,
            MotionRole::FastSpatial => 3,
            MotionRole::DefaultSpatial => 4,
            MotionRole::SlowSpatial => 5,
        };
        &self.specs[index]
    }

    pub fn builder() -> MotionSchemeBuilder {
        MotionSchemeBuilder::new(Self::standard())
    }

    pub fn rebuild(self) -> MotionSchemeBuilder {
        MotionSchemeBuilder::new(self)
    }
}

#[derive(Clone, Debug)]
pub struct MotionSchemeBuilder {
    scheme: MotionScheme,
}

impl MotionSchemeBuilder {
    fn new(scheme: MotionScheme) -> Self {
        Self { scheme }
    }

    pub fn role(mut self, role: MotionRole, spec: MotionSpec) -> Self {
        let index = match role {
            MotionRole::FastEffects => 0,
            MotionRole::DefaultEffects => 1,
            MotionRole::SlowEffects => 2,
            MotionRole::FastSpatial => 3,
            MotionRole::DefaultSpatial => 4,
            MotionRole::SlowSpatial => 5,
        };
        self.scheme.specs[index] = spec;
        self
    }

    pub fn build(self) -> MotionScheme {
        self.scheme
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_roles_match_spec() {
        let scheme = MotionScheme::standard();
        let fast = scheme.spec(MotionRole::FastEffects);
        assert_eq!(fast.spring, SpringParameters::new(1.0, 3800.0));
        assert_eq!(fast.fallback_duration, dur::SHORT3);
        let slow_spatial = scheme.spec(MotionRole::SlowSpatial);
        assert_eq!(slow_spatial.spring.stiffness, 300.0);
        assert!((slow_spatial.spring.damping_ratio - 0.9).abs() < 0.000001);
        assert_eq!(slow_spatial.fallback_duration, Duration::from_millis(750));
    }

    #[test]
    fn builder_overrides_role() {
        let scheme = MotionScheme::builder()
            .role(
                MotionRole::FastEffects,
                MotionSpec::spring(
                    SpringParameters::new(0.5, 100.0),
                    dur::SHORT1,
                    easing::LINEAR,
                ),
            )
            .build();
        assert_eq!(scheme.spec(MotionRole::FastEffects).spring.stiffness, 100.0);
    }
}
