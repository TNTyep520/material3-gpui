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
pub struct AssistChipTokens;
impl AssistChipTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const DRAGGED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
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
    pub const FLAT_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FLAT_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DRAGGED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ICON_SIZE: Dp = Dp(18.0);
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
}
impl AssistChipTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "AssistChipTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(AssistChipTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(AssistChipTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(AssistChipTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(AssistChipTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(AssistChipTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "DraggedLabelTextColor",
            value: TokenValue::ColorRole(AssistChipTokens::DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedContainerColor",
            value: TokenValue::ColorRole(AssistChipTokens::ELEVATED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedContainerElevation",
            value: TokenValue::Dp(AssistChipTokens::ELEVATED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedDisabledContainerColor",
            value: TokenValue::ColorRole(AssistChipTokens::ELEVATED_DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedDisabledContainerElevation",
            value: TokenValue::Dp(AssistChipTokens::ELEVATED_DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedDisabledContainerOpacity",
            value: TokenValue::Float(AssistChipTokens::ELEVATED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedFocusContainerElevation",
            value: TokenValue::Dp(AssistChipTokens::ELEVATED_FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedHoverContainerElevation",
            value: TokenValue::Dp(AssistChipTokens::ELEVATED_HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "ElevatedPressedContainerElevation",
            value: TokenValue::Dp(AssistChipTokens::ELEVATED_PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FlatContainerElevation",
            value: TokenValue::Dp(AssistChipTokens::FLAT_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FlatDisabledOutlineColor",
            value: TokenValue::ColorRole(AssistChipTokens::FLAT_DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FlatDisabledOutlineOpacity",
            value: TokenValue::Float(AssistChipTokens::FLAT_DISABLED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FlatFocusOutlineColor",
            value: TokenValue::ColorRole(AssistChipTokens::FLAT_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FlatOutlineColor",
            value: TokenValue::ColorRole(AssistChipTokens::FLAT_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FlatOutlineWidth",
            value: TokenValue::Dp(AssistChipTokens::FLAT_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(AssistChipTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FocusLabelTextColor",
            value: TokenValue::ColorRole(AssistChipTokens::FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "HoverLabelTextColor",
            value: TokenValue::ColorRole(AssistChipTokens::HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(AssistChipTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(AssistChipTokens::LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "PressedLabelTextColor",
            value: TokenValue::ColorRole(AssistChipTokens::PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "DisabledIconColor",
            value: TokenValue::ColorRole(AssistChipTokens::DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "DisabledIconOpacity",
            value: TokenValue::Float(AssistChipTokens::DISABLED_ICON_OPACITY),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "DraggedIconColor",
            value: TokenValue::ColorRole(AssistChipTokens::DRAGGED_ICON_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "FocusIconColor",
            value: TokenValue::ColorRole(AssistChipTokens::FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "HoverIconColor",
            value: TokenValue::ColorRole(AssistChipTokens::HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(AssistChipTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "IconSize",
            value: TokenValue::Dp(AssistChipTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "AssistChipTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(AssistChipTokens::PRESSED_ICON_COLOR),
        },
    ];
}
