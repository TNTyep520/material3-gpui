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

use super::{ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, TokenEntry, TokenValue};

#[derive(Clone, Copy, Debug, Default)]
pub struct ElevatedButtonTokens;
impl ElevatedButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LABEL_TEXT_SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
}
impl ElevatedButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(ElevatedButtonTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "DisabledContainerElevation",
            value: TokenValue::Dp(ElevatedButtonTokens::DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(ElevatedButtonTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "DisabledIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "DisabledIconOpacity",
            value: TokenValue::Float(ElevatedButtonTokens::DISABLED_ICON_OPACITY),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(ElevatedButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "FocusedContainerElevation",
            value: TokenValue::Dp(ElevatedButtonTokens::FOCUSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "FocusedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "FocusedLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "HoveredContainerElevation",
            value: TokenValue::Dp(ElevatedButtonTokens::HOVERED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "HoveredIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "HoveredLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "LabelTextSelectedColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::LABEL_TEXT_SELECTED_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "LabelTextUnselectedColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::LABEL_TEXT_UNSELECTED_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(ElevatedButtonTokens::PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "PressedLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedFocusedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedHoveredIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedContainerColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedFocusedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedHoveredIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "ElevatedButtonTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(ElevatedButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
    ];
}
