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

use super::{ColorSchemeKeyTokens, ColorToken, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct StandardIconButtonTokens;
impl StandardIconButtonTokens {
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl StandardIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "DisabledColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::DISABLED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "DisabledOpacity",
            value: TokenValue::Float(StandardIconButtonTokens::DISABLED_OPACITY),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "FocusedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::FOCUSED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "HoveredColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::HOVERED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "Color",
            value: TokenValue::ColorRole(StandardIconButtonTokens::COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "PressedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::PRESSED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "SelectedFocusedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "SelectedHoveredColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "SelectedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "SelectedPressedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::SELECTED_PRESSED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "UnselectedFocusedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "UnselectedHoveredColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "UnselectedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_COLOR),
        },
        TokenEntry {
            group: "StandardIconButtonTokens",
            name: "UnselectedPressedColor",
            value: TokenValue::ColorRole(StandardIconButtonTokens::UNSELECTED_PRESSED_COLOR),
        },
    ];
}
