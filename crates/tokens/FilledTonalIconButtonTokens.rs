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
pub struct FilledTonalIconButtonTokens;
impl FilledTonalIconButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OPACITY: f32 = 0.38;
    pub const FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const SELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const UNSELECTED_FOCUSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_HOVERED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_PRESSED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
impl FilledTonalIconButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(FilledTonalIconButtonTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "DisabledColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::DISABLED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "DisabledOpacity",
            value: TokenValue::Float(FilledTonalIconButtonTokens::DISABLED_OPACITY),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "FocusedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::FOCUSED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "HoveredColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::HOVERED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "Color",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "PressedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::PRESSED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "SelectedFocusedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "SelectedHoveredColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "SelectedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "SelectedPressedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::SELECTED_PRESSED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "UnselectedContainerColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "UnselectedFocusedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_FOCUSED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "UnselectedHoveredColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_HOVERED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "UnselectedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_COLOR),
        },
        TokenEntry {
            group: "FilledTonalIconButtonTokens",
            name: "UnselectedPressedColor",
            value: TokenValue::ColorRole(FilledTonalIconButtonTokens::UNSELECTED_PRESSED_COLOR),
        },
    ];
}
