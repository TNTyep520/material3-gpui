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
pub struct ChipsTokens;
impl ChipsTokens {
    pub const AVATAR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const AVATAR_SIZE: Dp = Dp(24.0);
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const FOCUSED_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HEIGHT: Dp = Dp(32.0);
    pub const LABEL_TEXT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const LEADING_ICON_SIZE: Dp = Dp(18.0);
    pub const PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const TRAILING_ICON_SIZE: Dp = Dp(18.0);
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_OPACITY: f32 = 0.1;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const UNSELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl ChipsTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ChipsTokens",
            name: "AvatarShape",
            value: TokenValue::ShapeRole(ChipsTokens::AVATAR_SHAPE),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "AvatarSize",
            value: TokenValue::Dp(ChipsTokens::AVATAR_SIZE),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(ChipsTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(ChipsTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "DisabledLeadingIconColor",
            value: TokenValue::ColorRole(ChipsTokens::DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "DisabledTrailingIconColor",
            value: TokenValue::ColorRole(ChipsTokens::DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(ChipsTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "FocusedIndicatorColor",
            value: TokenValue::ColorRole(ChipsTokens::FOCUSED_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "Height",
            value: TokenValue::Dp(ChipsTokens::HEIGHT),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "LabelText",
            value: TokenValue::TypographyRole(ChipsTokens::LABEL_TEXT),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "LeadingIconSize",
            value: TokenValue::Dp(ChipsTokens::LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "PressedShape",
            value: TokenValue::ShapeRole(ChipsTokens::PRESSED_SHAPE),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(ChipsTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedDisabledContainerColor",
            value: TokenValue::ColorRole(ChipsTokens::SELECTED_DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedDisabledContainerOpacity",
            value: TokenValue::Float(ChipsTokens::SELECTED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedLabelTextColor",
            value: TokenValue::ColorRole(ChipsTokens::SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedLeadingIconColor",
            value: TokenValue::ColorRole(ChipsTokens::SELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedOutlineWidth",
            value: TokenValue::Dp(ChipsTokens::SELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedShape",
            value: TokenValue::ShapeRole(ChipsTokens::SELECTED_SHAPE),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "SelectedTrailingIconColor",
            value: TokenValue::ColorRole(ChipsTokens::SELECTED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(ChipsTokens::TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedDisabledOutlineColor",
            value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedDisabledOutlineOpacity",
            value: TokenValue::Float(ChipsTokens::UNSELECTED_DISABLED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedLabelTextColor",
            value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedLeadingIconColor",
            value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedOutlineColor",
            value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedOutlineWidth",
            value: TokenValue::Dp(ChipsTokens::UNSELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedShape",
            value: TokenValue::ShapeRole(ChipsTokens::UNSELECTED_SHAPE),
        },
        TokenEntry {
            group: "ChipsTokens",
            name: "UnselectedTrailingIconColor",
            value: TokenValue::ColorRole(ChipsTokens::UNSELECTED_TRAILING_ICON_COLOR),
        },
    ];
}
