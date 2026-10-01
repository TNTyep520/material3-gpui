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
pub struct InputChipTokens;
impl InputChipTokens {
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DISABLED_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SELECTED_CONTAINER_OPACITY: f32 = 0.12;
    pub const DISABLED_UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_UNSELECTED_OUTLINE_OPACITY: f32 = 0.12;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const SELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const UNSELECTED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const AVATAR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const AVATAR_SIZE: Dp = Dp(24.0);
    pub const DISABLED_AVATAR_OPACITY: f32 = 0.38;
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const LEADING_ICON_SIZE: Dp = Dp(18.0);
    pub const SELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const SELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const TRAILING_ICON_SIZE: Dp = Dp(18.0);
    pub const UNSELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl InputChipTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "InputChipTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(InputChipTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(InputChipTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(InputChipTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(InputChipTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledSelectedContainerColor",
            value: TokenValue::ColorRole(InputChipTokens::DISABLED_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledSelectedContainerOpacity",
            value: TokenValue::Float(InputChipTokens::DISABLED_SELECTED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledUnselectedOutlineColor",
            value: TokenValue::ColorRole(InputChipTokens::DISABLED_UNSELECTED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledUnselectedOutlineOpacity",
            value: TokenValue::Float(InputChipTokens::DISABLED_UNSELECTED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(InputChipTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(InputChipTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(InputChipTokens::LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedDraggedLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedFocusLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedHoverLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedOutlineWidth",
            value: TokenValue::Dp(InputChipTokens::SELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedDraggedLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedFocusLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedFocusOutlineColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedHoverLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedOutlineColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedOutlineWidth",
            value: TokenValue::Dp(InputChipTokens::UNSELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "AvatarShape",
            value: TokenValue::ShapeRole(InputChipTokens::AVATAR_SHAPE),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "AvatarSize",
            value: TokenValue::Dp(InputChipTokens::AVATAR_SIZE),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledAvatarOpacity",
            value: TokenValue::Float(InputChipTokens::DISABLED_AVATAR_OPACITY),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledLeadingIconOpacity",
            value: TokenValue::Float(InputChipTokens::DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "LeadingIconSize",
            value: TokenValue::Dp(InputChipTokens::LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedDraggedLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_DRAGGED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedFocusLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedHoverLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedPressedLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_PRESSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedDraggedLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_DRAGGED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedFocusLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedHoverLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedPressedLeadingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_PRESSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "DisabledTrailingIconOpacity",
            value: TokenValue::Float(InputChipTokens::DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedDraggedTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_DRAGGED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedFocusTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedHoverTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedPressedTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_PRESSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "SelectedTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::SELECTED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "TrailingIconSize",
            value: TokenValue::Dp(InputChipTokens::TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedDraggedTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_DRAGGED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedFocusTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedHoverTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedPressedTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_PRESSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "InputChipTokens",
            name: "UnselectedTrailingIconColor",
            value: TokenValue::ColorRole(InputChipTokens::UNSELECTED_TRAILING_ICON_COLOR),
        },
    ];
}
