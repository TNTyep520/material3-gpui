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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3StateLayerTokens.java

use crate::tokens::StateTokens;
use gpui::{Hsla, Pixels, Rgba, px};

pub const HOVER_OPACITY: f32 = StateTokens::HOVER_STATE_LAYER_OPACITY;

pub const FOCUS_OPACITY: f32 = StateTokens::FOCUS_STATE_LAYER_OPACITY;

pub const PRESSED_OPACITY: f32 = StateTokens::PRESSED_STATE_LAYER_OPACITY;

pub const DRAGGED_OPACITY: f32 = StateTokens::DRAGGED_STATE_LAYER_OPACITY;

pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;

pub const DISABLED_CONTENT_OPACITY: f32 = 0.38;

#[derive(Clone, Copy, Debug)]
pub struct StateLayerTokens {
    pub hover: f32,

    pub focus: f32,

    pub pressed: f32,

    pub dragged: f32,

    pub disabled_container: f32,

    pub disabled_content: f32,

    pub focus_indicator_thickness: Pixels,

    pub focus_indicator_outer_offset: Pixels,

    pub focus_indicator_inner_offset: Pixels,
}

impl Default for StateLayerTokens {
    fn default() -> Self {
        Self::baseline()
    }
}

impl StateLayerTokens {
    pub fn baseline() -> Self {
        Self {
            hover: HOVER_OPACITY,
            focus: FOCUS_OPACITY,
            pressed: PRESSED_OPACITY,
            dragged: DRAGGED_OPACITY,
            disabled_container: DISABLED_CONTAINER_OPACITY,
            disabled_content: DISABLED_CONTENT_OPACITY,
            focus_indicator_thickness: px(3.0),
            focus_indicator_outer_offset: px(2.0),
            focus_indicator_inner_offset: px(-2.0),
        }
    }
}

pub fn blend(base: Hsla, overlay: Hsla, opacity: f32) -> Hsla {
    let b: Rgba = base.into();
    let o: Rgba = overlay.into();
    Rgba {
        r: b.r + (o.r - b.r) * opacity,
        g: b.g + (o.g - b.g) * opacity,
        b: b.b + (o.b - b.b) * opacity,
        a: b.a,
    }
    .into()
}

pub fn hover_layer(base: Hsla, content: Hsla) -> Hsla {
    blend(base, content, HOVER_OPACITY)
}

pub fn pressed_layer(base: Hsla, content: Hsla) -> Hsla {
    blend(base, content, PRESSED_OPACITY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::rgb;

    #[test]
    fn blend_full_opacity_returns_overlay() {
        let base = rgb(0x000000).into();
        let overlay = rgb(0xffffff).into();
        let mixed = blend(base, overlay, 1.0);
        let rgba: Rgba = mixed.into();
        assert!(rgba.r > 0.99);
    }

    #[test]
    fn baseline_matches_m3fx() {
        let t = StateLayerTokens::baseline();
        assert_eq!(t.hover, 0.08);
        assert_eq!(t.focus, 0.10);
        assert_eq!(t.pressed, 0.10);
        assert_eq!(t.dragged, 0.16);
    }
}
