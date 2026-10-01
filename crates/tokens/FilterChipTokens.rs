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
pub struct FilterChipTokens;
impl FilterChipTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(32.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const ELEVATED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ELEVATED_DISABLED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const ELEVATED_DISABLED_CONTAINER_OPACITY: f32 = 0.12;
    pub const ELEVATED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const ELEVATED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const ELEVATED_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ELEVATED_UNSELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const FLAT_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_DISABLED_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_DISABLED_SELECTED_CONTAINER_OPACITY: f32 = 0.12;
    pub const FLAT_DISABLED_UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FLAT_DISABLED_UNSELECTED_OUTLINE_OPACITY: f32 = 0.12;
    pub const FLAT_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const FLAT_SELECTED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_SELECTED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL1;
    pub const FLAT_SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const FLAT_SELECTED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_UNSELECTED_FOCUS_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_UNSELECTED_FOCUS_OUTLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const FLAT_UNSELECTED_HOVER_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FLAT_UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE_VARIANT;
    pub const FLAT_UNSELECTED_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const FLAT_UNSELECTED_PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const SELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ICON_SIZE: Dp = Dp(18.0);
    pub const DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const SELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const UNSELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const SELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const UNSELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl FilterChipTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "FilterChipTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(FilterChipTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(FilterChipTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(FilterChipTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "DraggedContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::ELEVATED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedDisabledContainerColor",
            value: TokenValue::ColorRole(FilterChipTokens::ELEVATED_DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedDisabledContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::ELEVATED_DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedDisabledContainerOpacity",
            value: TokenValue::Float(FilterChipTokens::ELEVATED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedFocusContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::ELEVATED_FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedHoverContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::ELEVATED_HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedPressedContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::ELEVATED_PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedSelectedContainerColor",
            value: TokenValue::ColorRole(FilterChipTokens::ELEVATED_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "ElevatedUnselectedContainerColor",
            value: TokenValue::ColorRole(FilterChipTokens::ELEVATED_UNSELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::FLAT_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatDisabledSelectedContainerColor",
            value: TokenValue::ColorRole(FilterChipTokens::FLAT_DISABLED_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatDisabledSelectedContainerOpacity",
            value: TokenValue::Float(FilterChipTokens::FLAT_DISABLED_SELECTED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatDisabledUnselectedOutlineColor",
            value: TokenValue::ColorRole(FilterChipTokens::FLAT_DISABLED_UNSELECTED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatDisabledUnselectedOutlineOpacity",
            value: TokenValue::Float(FilterChipTokens::FLAT_DISABLED_UNSELECTED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatSelectedContainerColor",
            value: TokenValue::ColorRole(FilterChipTokens::FLAT_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatSelectedFocusContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatSelectedHoverContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatSelectedOutlineWidth",
            value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatSelectedPressedContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::FLAT_SELECTED_PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatUnselectedFocusContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_FOCUS_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatUnselectedFocusOutlineColor",
            value: TokenValue::ColorRole(FilterChipTokens::FLAT_UNSELECTED_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatUnselectedHoverContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_HOVER_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatUnselectedOutlineColor",
            value: TokenValue::ColorRole(FilterChipTokens::FLAT_UNSELECTED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatUnselectedOutlineWidth",
            value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FlatUnselectedPressedContainerElevation",
            value: TokenValue::Dp(FilterChipTokens::FLAT_UNSELECTED_PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(FilterChipTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(FilterChipTokens::LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedDraggedLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedFocusLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedHoverLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedDraggedLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedFocusLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedHoverLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "IconSize",
            value: TokenValue::Dp(FilterChipTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "DisabledLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "DisabledLeadingIconOpacity",
            value: TokenValue::Float(FilterChipTokens::DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedDraggedLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_DRAGGED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedFocusLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedHoverLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedPressedLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_PRESSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedDraggedLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_DRAGGED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedFocusLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedHoverLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedPressedLeadingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_PRESSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "DisabledTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "DisabledTrailingIconOpacity",
            value: TokenValue::Float(FilterChipTokens::DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedDraggedTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_DRAGGED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedFocusTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedHoverTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedPressedTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_PRESSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "SelectedTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::SELECTED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedDraggedTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_DRAGGED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedFocusTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedHoverTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedPressedTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_PRESSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "FilterChipTokens",
            name: "UnselectedTrailingIconColor",
            value: TokenValue::ColorRole(FilterChipTokens::UNSELECTED_TRAILING_ICON_COLOR),
        },
    ];
}
