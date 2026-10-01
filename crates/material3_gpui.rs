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

#[path = "assets.rs"]
pub mod assets;
pub mod components;
#[path = "fonts.rs"]
pub mod fonts;
pub use components::icon;
#[path = "interaction.rs"]
pub mod interaction;
pub mod motion;
pub use components::overlay;
pub mod styles;
pub mod theme;
pub mod tokens;

pub use assets::Md3Assets;
pub use components::*;
pub use icon::{ALL_ICONS, ICON_COUNT, Icon, IconName, materialsymbolsrounded};
pub use theme::{ActiveTheme, Theme, ThemeMode};

use gpui::App;

pub fn init(cx: &mut App) {
    if let Err(err) = fonts::install(cx) {
        eprintln!("material3-gpui: failed to register embedded fonts: {err}");
    }
    if !cx.has_global::<Theme>() {
        cx.set_global(Theme::light());
    }
}

pub mod prelude {
    pub use crate::assets::Md3Assets;
    pub use crate::components::*;
    pub use crate::fonts::TEXT_FONT_FAMILY;
    pub use crate::icon::{ALL_ICONS, ICON_COUNT, Icon, IconName, materialsymbolsrounded};
    pub use crate::motion::{
        Animatable, AnimatedComponent, AnimationDriver, Easing, MotionRole, MotionScheme,
        MotionSpec, SpringParameters,
    };
    pub use crate::overlay::{
        MenuItem, MenuState, OverlayHostState, OverlayRegistry, Snackbar, close_menu,
        close_tooltip, host, show_menu, show_snackbar, show_tooltip,
    };
    #[cfg(feature = "dynamic-color")]
    pub use crate::theme::color_scheme_from_seed;
    pub use crate::theme::{
        ActiveTheme, ColorScheme, ComponentTokens, Density, Elevation, ElevationTokens, Profile,
        Shapes, StateLayerTokens, Theme, ThemeMode, TokenSet, TokenSetBuilder, TypeScale,
        TypeStyle,
    };
}
