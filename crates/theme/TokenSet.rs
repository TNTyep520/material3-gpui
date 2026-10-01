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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3TokenSet.java

use gpui::SharedString;

use super::ThemeMode;
use crate::motion::MotionScheme;

use super::color::ColorScheme;
use super::component_tokens::ComponentTokens;
use super::density::Density;
use super::elevation::ElevationTokens;
use super::profile::Profile;
use super::shape::Shapes;
use super::state::StateLayerTokens;
use super::typography::TypeScale;

#[derive(Clone, Debug)]
pub struct TokenSet {
    pub profile: Profile,

    pub density: Density,

    pub colors: ColorScheme,

    pub typography: TypeScale,

    pub shapes: Shapes,

    pub elevation: ElevationTokens,

    pub motion: MotionScheme,

    pub state_layer: StateLayerTokens,

    pub component: ComponentTokens,
}

impl TokenSet {
    pub fn androidx(mode: ThemeMode) -> Self {
        let colors = match mode {
            ThemeMode::Light => ColorScheme::androidx_light(),
            ThemeMode::Dark => ColorScheme::androidx_dark(),
        };
        Self::builder(Profile::Baseline2021, colors)
            .with_typography(TypeScale::androidx())
            .with_component(ComponentTokens::androidx())
            .build()
    }

    pub fn new(profile: Profile, colors: ColorScheme) -> Self {
        Self::builder(profile, colors).build()
    }

    pub fn builder(profile: Profile, colors: ColorScheme) -> TokenSetBuilder {
        TokenSetBuilder::from_profile(profile, colors)
    }

    pub fn rebuild(self) -> TokenSetBuilder {
        TokenSetBuilder::from_token_set(self)
    }
}

#[derive(Clone, Debug)]
pub struct TokenSetBuilder {
    profile: Profile,
    density: Density,
    colors: ColorScheme,
    typography: TypeScale,
    shapes: Shapes,
    elevation: ElevationTokens,
    motion: MotionScheme,
    state_layer: StateLayerTokens,
    component: ComponentTokens,
}

impl TokenSetBuilder {
    fn from_profile(profile: Profile, colors: ColorScheme) -> Self {
        Self {
            profile,
            density: Density::default(),
            colors,
            typography: TypeScale::baseline(),
            shapes: Shapes::baseline(),
            elevation: ElevationTokens::baseline(),
            motion: MotionScheme::standard(),
            state_layer: StateLayerTokens::baseline(),
            component: ComponentTokens::default(),
        }
    }

    fn from_token_set(tokens: TokenSet) -> Self {
        Self {
            profile: tokens.profile,
            density: tokens.density,
            colors: tokens.colors,
            typography: tokens.typography,
            shapes: tokens.shapes,
            elevation: tokens.elevation,
            motion: tokens.motion,
            state_layer: tokens.state_layer,
            component: tokens.component,
        }
    }

    pub fn with_density(mut self, density: Density) -> Self {
        self.density = density;
        self
    }

    pub fn with_colors(mut self, colors: ColorScheme) -> Self {
        self.colors = colors;
        self
    }

    pub fn with_typography(mut self, typography: TypeScale) -> Self {
        self.typography = typography;
        self
    }

    pub fn with_shapes(mut self, shapes: Shapes) -> Self {
        self.shapes = shapes;
        self
    }

    pub fn with_elevation(mut self, elevation: ElevationTokens) -> Self {
        self.elevation = elevation;
        self
    }

    pub fn with_motion(mut self, motion: MotionScheme) -> Self {
        self.motion = motion;
        self
    }

    pub fn with_state_layer(mut self, state_layer: StateLayerTokens) -> Self {
        self.state_layer = state_layer;
        self
    }

    pub fn with_component(mut self, component: ComponentTokens) -> Self {
        self.component = component;
        self
    }

    pub fn build(self) -> TokenSet {
        TokenSet {
            profile: self.profile,
            density: self.density,
            colors: self.colors,
            typography: self.typography,
            shapes: self.shapes,
            elevation: self.elevation,
            motion: self.motion,
            state_layer: self.state_layer,
            component: self.component,
        }
    }
}

pub const DEFAULT_FONT_FAMILY: &str = "Roboto";

pub fn font_family(name: &str) -> SharedString {
    SharedString::from(name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion::MotionRole;
    use crate::theme::color::ColorScheme;

    #[test]
    fn profile_selects_token_families() {
        let baseline = TokenSet::new(Profile::Baseline2021, ColorScheme::light());
        assert_eq!(baseline.profile, Profile::Baseline2021);
        assert_eq!(
            baseline
                .motion
                .spec(MotionRole::DefaultSpatial)
                .spring
                .stiffness,
            700.0
        );
        assert_eq!(baseline.shapes.medium, gpui::px(12.));
    }

    #[test]
    fn builder_overrides_groups() {
        let tokens = TokenSet::builder(Profile::Baseline2021, ColorScheme::dark())
            .with_motion(MotionScheme::standard())
            .build();

        assert_eq!(tokens.colors.surface, ColorScheme::dark().surface);
        assert_eq!(tokens.shapes.extra_small, gpui::px(4.));
    }
}
