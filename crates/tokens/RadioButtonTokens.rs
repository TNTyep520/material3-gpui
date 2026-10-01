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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/Tokens.kt

use super::{ColorSchemeKeyTokens, ColorToken, Dp, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct RadioButtonTokens;
impl RadioButtonTokens {
    pub const DISABLED_SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SELECTED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_UNSELECTED_ICON_OPACITY: f32 = 0.38;
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STATE_LAYER_SIZE: Dp = Dp(40.0);
    pub const UNSELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
}
impl RadioButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "RadioButtonTokens",
            name: "DisabledSelectedIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::DISABLED_SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "DisabledSelectedIconOpacity",
            value: TokenValue::Float(RadioButtonTokens::DISABLED_SELECTED_ICON_OPACITY),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "DisabledUnselectedIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::DISABLED_UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "DisabledUnselectedIconOpacity",
            value: TokenValue::Float(RadioButtonTokens::DISABLED_UNSELECTED_ICON_OPACITY),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(RadioButtonTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "SelectedFocusIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "SelectedHoverIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "StateLayerSize",
            value: TokenValue::Dp(RadioButtonTokens::STATE_LAYER_SIZE),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "UnselectedFocusIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "UnselectedHoverIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "RadioButtonTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(RadioButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
        },
    ];
}
