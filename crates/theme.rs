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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/theme/M3Theme.java

#[path = "theme/Color.rs"]
mod color;
#[path = "theme/ComponentTokens.rs"]
mod component_tokens;
#[path = "theme/Density.rs"]
mod density;
#[cfg(feature = "dynamic-color")]
#[path = "theme/DynamicColor.rs"]
mod dynamic_color;
#[path = "theme/Elevation.rs"]
mod elevation;
#[path = "theme/Profile.rs"]
mod profile;
#[path = "theme/Shape.rs"]
mod shape;
#[path = "theme/State.rs"]
mod state;
#[path = "theme/TokenSet.rs"]
mod token_set;
#[path = "theme/Typography.rs"]
mod typography;

pub use color::{ColorScheme, hex};
pub use component_tokens::{
    ButtonTokens, ComponentTokens, MenuTokens, SliderTokens, SnackbarTokens, SwitchTokens,
    TextFieldTokens, TooltipTokens,
};
pub use density::Density;
#[cfg(feature = "dynamic-color")]
pub use dynamic_color::color_scheme_from_seed;
pub use elevation::{Elevation, ElevationTokens};
pub use profile::Profile;
pub use shape::Shapes;
pub use state::*;
pub use token_set::{DEFAULT_FONT_FAMILY, TokenSet, TokenSetBuilder, font_family};
pub use typography::{TypeScale, TypeStyle};

use gpui::{App, Global, SharedString};

use crate::motion::MotionScheme;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ThemeMode {
    #[default]
    Light,

    Dark,
}

#[derive(Clone, Debug)]
pub struct Theme {
    mode: ThemeMode,
    tokens: TokenSet,
    font_family: SharedString,
}

impl Global for Theme {}

impl Theme {
    pub fn androidx(mode: ThemeMode) -> Self {
        Self::from_token_set(TokenSet::androidx(mode), mode, DEFAULT_FONT_FAMILY)
    }

    pub fn light() -> Self {
        Self::from_seed(0x6750A4, ThemeMode::Light, Profile::Baseline2021)
    }

    pub fn dark() -> Self {
        Self::from_seed(0x6750A4, ThemeMode::Dark, Profile::Baseline2021)
    }

    pub fn from_seed(seed: u32, mode: ThemeMode, profile: Profile) -> Self {
        let is_dark = mode == ThemeMode::Dark;

        #[cfg(feature = "dynamic-color")]
        let colors = if seed == 0x6750A4 && profile == Profile::Baseline2021 {
            if is_dark {
                ColorScheme::dark()
            } else {
                ColorScheme::light()
            }
        } else {
            color_scheme_from_seed(seed, is_dark, profile)
        };

        #[cfg(not(feature = "dynamic-color"))]
        let colors = if is_dark {
            ColorScheme::dark()
        } else {
            ColorScheme::light()
        };
        let _ = (seed, profile);

        Self::from_token_set(TokenSet::new(profile, colors), mode, DEFAULT_FONT_FAMILY)
    }

    pub fn from_token_set(
        tokens: TokenSet,
        mode: ThemeMode,
        font_family: impl Into<SharedString>,
    ) -> Self {
        Self {
            mode,
            tokens,
            font_family: font_family.into(),
        }
    }

    pub fn mode(&self) -> ThemeMode {
        self.mode
    }

    pub fn is_dark(&self) -> bool {
        self.mode == ThemeMode::Dark
    }

    pub fn font_family(&self) -> &SharedString {
        &self.font_family
    }

    pub fn token_set(&self) -> &TokenSet {
        &self.tokens
    }

    pub fn set_token_set(&mut self, tokens: TokenSet) {
        self.tokens = tokens;
    }

    pub fn profile(&self) -> Profile {
        self.tokens.profile
    }

    pub fn density(&self) -> Density {
        self.tokens.density
    }

    pub fn colors(&self) -> &ColorScheme {
        &self.tokens.colors
    }

    pub fn typography(&self) -> &TypeScale {
        &self.tokens.typography
    }

    pub fn shapes(&self) -> &Shapes {
        &self.tokens.shapes
    }

    pub fn elevation_tokens(&self) -> &ElevationTokens {
        &self.tokens.elevation
    }

    pub fn motion(&self) -> &MotionScheme {
        &self.tokens.motion
    }

    pub fn state_layer(&self) -> &StateLayerTokens {
        &self.tokens.state_layer
    }

    pub fn component(&self) -> &ComponentTokens {
        &self.tokens.component
    }

    pub fn global(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    pub fn set(cx: &mut App, theme: Theme) {
        cx.set_global(theme);
    }
}

pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        Theme::global(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seed_matches_baseline_defaults() {
        let theme = Theme::light();
        assert_eq!(theme.mode(), ThemeMode::Light);
        assert_eq!(theme.profile(), Profile::Baseline2021);
    }
}
