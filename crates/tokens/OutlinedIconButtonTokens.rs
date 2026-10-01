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
pub struct OutlinedIconButtonTokens;
impl OutlinedIconButtonTokens {
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl OutlinedIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "DisabledColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::DISABLED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "DisabledOpacity",
            value: TokenValue::Float(OutlinedIconButtonTokens::DISABLED_OPACITY),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "DisabledOutlineColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "FocusedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::FOCUSED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "HoveredColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::HOVERED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "Color",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "OutlineColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "PressedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::PRESSED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "SelectedDisabledContainerColor",
            value: TokenValue::ColorRole(
                OutlinedIconButtonTokens::SELECTED_DISABLED_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "SelectedDisabledContainerOpacity",
            value: TokenValue::Float(OutlinedIconButtonTokens::SELECTED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "SelectedFocusedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "SelectedHoveredColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "SelectedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "SelectedPressedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::SELECTED_PRESSED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "UnselectedDisabledOutlineColor",
            value: TokenValue::ColorRole(
                OutlinedIconButtonTokens::UNSELECTED_DISABLED_OUTLINE_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "UnselectedFocusedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "UnselectedHoveredColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "UnselectedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "UnselectedOutlineColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedIconButtonTokens",
            name: "UnselectedPressedColor",
            value: TokenValue::ColorRole(OutlinedIconButtonTokens::UNSELECTED_PRESSED_COLOR),
        },
    ];
}
