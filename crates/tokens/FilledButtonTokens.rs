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
pub struct FilledButtonTokens;
impl FilledButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl FilledButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FilledButtonTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(FilledButtonTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(FilledButtonTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(FilledButtonTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "DisabledContainerElevation",
            value: TokenValue::Dp(FilledButtonTokens::DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(FilledButtonTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "DisabledIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "DisabledIconOpacity",
            value: TokenValue::Float(FilledButtonTokens::DISABLED_ICON_OPACITY),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(FilledButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "FocusedContainerElevation",
            value: TokenValue::Dp(FilledButtonTokens::FOCUSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "FocusedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "FocusedLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "HoveredContainerElevation",
            value: TokenValue::Dp(FilledButtonTokens::HOVERED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "HoveredIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "HoveredLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "LabelTextSelectedColor",
            value: TokenValue::ColorRole(FilledButtonTokens::LABEL_TEXT_SELECTED_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "LabelTextUnselectedColor",
            value: TokenValue::ColorRole(FilledButtonTokens::LABEL_TEXT_UNSELECTED_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(FilledButtonTokens::PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "PressedLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedFocusedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedHoveredIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedContainerColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedFocusedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedHoveredIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "FilledButtonTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(FilledButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
    ];
}
