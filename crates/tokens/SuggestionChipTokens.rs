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

use super::{
    ColorSchemeKeyTokens, ColorToken, Dp, ElevationTokens, ShapeKeyTokens, ShapeToken, TokenEntry,
    TokenValue, TypographyKeyTokens, TypographyToken,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct SuggestionChipTokens;
impl SuggestionChipTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const DRAGGED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ELEVATED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ELEVATED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ELEVATED_DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const ELEVATED_DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const ELEVATED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const ELEVATED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FLAT_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const FLAT_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FLAT_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FLAT_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const DRAGGED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LEADING_ICON_SIZE: Dp = Dp(18.0);
    pub const PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
}
impl SuggestionChipTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(SuggestionChipTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(SuggestionChipTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(SuggestionChipTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(SuggestionChipTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "DraggedLabelTextColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedContainerColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::ELEVATED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedContainerElevation",
            value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedDisabledContainerColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::ELEVATED_DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedDisabledContainerElevation",
            value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedDisabledContainerOpacity",
            value: TokenValue::Float(SuggestionChipTokens::ELEVATED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedFocusContainerElevation",
            value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedHoverContainerElevation",
            value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "ElevatedPressedContainerElevation",
            value: TokenValue::Dp(SuggestionChipTokens::ELEVATED_PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FlatContainerElevation",
            value: TokenValue::Dp(SuggestionChipTokens::FLAT_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FlatDisabledOutlineColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::FLAT_DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FlatDisabledOutlineOpacity",
            value: TokenValue::Float(SuggestionChipTokens::FLAT_DISABLED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FlatFocusOutlineColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::FLAT_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FlatOutlineColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::FLAT_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FlatOutlineWidth",
            value: TokenValue::Dp(SuggestionChipTokens::FLAT_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FocusLabelTextColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "HoverLabelTextColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(SuggestionChipTokens::LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "PressedLabelTextColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "DisabledLeadingIconColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "DisabledLeadingIconOpacity",
            value: TokenValue::Float(SuggestionChipTokens::DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "DraggedLeadingIconColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::DRAGGED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "FocusLeadingIconColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "HoverLeadingIconColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "LeadingIconColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "LeadingIconSize",
            value: TokenValue::Dp(SuggestionChipTokens::LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "SuggestionChipTokens",
            name: "PressedLeadingIconColor",
            value: TokenValue::ColorRole(SuggestionChipTokens::PRESSED_LEADING_ICON_COLOR),
        },
    ];
}
