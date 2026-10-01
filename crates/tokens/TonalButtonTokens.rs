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
pub struct TonalButtonTokens;
impl TonalButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const FOCUSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY;
    pub const UNSELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
}
impl TonalButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "TonalButtonTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(TonalButtonTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(TonalButtonTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(TonalButtonTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "DisabledContainerElevation",
            value: TokenValue::Dp(TonalButtonTokens::DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(TonalButtonTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "DisabledIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "DisabledIconOpacity",
            value: TokenValue::Float(TonalButtonTokens::DISABLED_ICON_OPACITY),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(TonalButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "FocusedContainerElevation",
            value: TokenValue::Dp(TonalButtonTokens::FOCUSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "FocusedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "FocusedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "HoveredContainerElevation",
            value: TokenValue::Dp(TonalButtonTokens::HOVERED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "HoveredIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "HoveredLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(TonalButtonTokens::PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "PressedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedFocusedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedHoveredIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedContainerColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedFocusedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedHoveredIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "TonalButtonTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(TonalButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
    ];
}
