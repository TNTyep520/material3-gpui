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
pub struct OutlinedButtonTokens;
impl OutlinedButtonTokens {
    pub const DISABLED_CONTAINER_OPACITY: f32 = 0.1;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FOCUSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVERED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVERED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
}
impl OutlinedButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(OutlinedButtonTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "DisabledIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "DisabledIconOpacity",
            value: TokenValue::Float(OutlinedButtonTokens::DISABLED_ICON_OPACITY),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(OutlinedButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "DisabledOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "FocusedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "FocusedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "FocusedOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::FOCUSED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "HoveredIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "HoveredLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "HoveredOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::HOVERED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "OutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "PressedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "PressedOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::PRESSED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedDisabledContainerColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedFocusedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedHoveredIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedDisabledOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedFocusedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedFocusedOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_FOCUSED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedHoveredIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedHoveredOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_HOVERED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedButtonTokens",
            name: "UnselectedPressedOutlineColor",
            value: TokenValue::ColorRole(OutlinedButtonTokens::UNSELECTED_PRESSED_OUTLINE_COLOR),
        },
    ];
}
