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
pub struct FilledIconButtonTokens;
impl FilledIconButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl FilledIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(FilledIconButtonTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "DisabledColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::DISABLED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "DisabledOpacity",
            value: TokenValue::Float(FilledIconButtonTokens::DISABLED_OPACITY),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "FocusedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::FOCUSED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "HoveredColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::HOVERED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "Color",
            value: TokenValue::ColorRole(FilledIconButtonTokens::COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "PressedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::PRESSED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "SelectedFocusedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "SelectedHoveredColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "SelectedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "SelectedPressedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::SELECTED_PRESSED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "UnselectedContainerColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "UnselectedFocusedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "UnselectedHoveredColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "UnselectedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_COLOR),
        },
        TokenEntry {
            group: "FilledIconButtonTokens",
            name: "UnselectedPressedColor",
            value: TokenValue::ColorRole(FilledIconButtonTokens::UNSELECTED_PRESSED_COLOR),
        },
    ];
}
