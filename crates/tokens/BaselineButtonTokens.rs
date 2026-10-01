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
    TokenValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BaselineButtonTokens;
impl BaselineButtonTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
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
    pub const ICON_LABEL_SPACE: Dp = Dp(8.0);
    pub const ICON_SIZE: Dp = Dp(20.0);
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_SELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const LABEL_TEXT_UNSELECTED_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const LEADING_SPACE: Dp = Dp(24.0);
    pub const PRESSED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const PRESSED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_CONTAINER_SHAPE_ROUND: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTED_CONTAINER_SHAPE_SQUARE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const SELECTED_FOCUSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const TRAILING_SPACE: Dp = Dp(24.0);
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
impl BaselineButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(BaselineButtonTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(BaselineButtonTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "ContainerShapeRound",
            value: TokenValue::ShapeRole(BaselineButtonTokens::CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "ContainerShapeSquare",
            value: TokenValue::ShapeRole(BaselineButtonTokens::CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "DisabledContainerColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "DisabledContainerElevation",
            value: TokenValue::Dp(BaselineButtonTokens::DISABLED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "DisabledContainerOpacity",
            value: TokenValue::Float(BaselineButtonTokens::DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "DisabledIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "DisabledIconOpacity",
            value: TokenValue::Float(BaselineButtonTokens::DISABLED_ICON_OPACITY),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(BaselineButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "FocusedContainerElevation",
            value: TokenValue::Dp(BaselineButtonTokens::FOCUSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "FocusedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "FocusedLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "HoveredContainerElevation",
            value: TokenValue::Dp(BaselineButtonTokens::HOVERED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "HoveredIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "HoveredLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "IconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "IconLabelSpace",
            value: TokenValue::Dp(BaselineButtonTokens::ICON_LABEL_SPACE),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(BaselineButtonTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "LabelTextSelectedColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::LABEL_TEXT_SELECTED_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "LabelTextUnselectedColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::LABEL_TEXT_UNSELECTED_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "LeadingSpace",
            value: TokenValue::Dp(BaselineButtonTokens::LEADING_SPACE),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "PressedContainerElevation",
            value: TokenValue::Dp(BaselineButtonTokens::PRESSED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "PressedContainerShape",
            value: TokenValue::ShapeRole(BaselineButtonTokens::PRESSED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "PressedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "PressedLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedContainerShapeRound",
            value: TokenValue::ShapeRole(BaselineButtonTokens::SELECTED_CONTAINER_SHAPE_ROUND),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedContainerShapeSquare",
            value: TokenValue::ShapeRole(BaselineButtonTokens::SELECTED_CONTAINER_SHAPE_SQUARE),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedFocusedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedHoveredIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "TrailingSpace",
            value: TokenValue::Dp(BaselineButtonTokens::TRAILING_SPACE),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedContainerColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedFocusedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_FOCUSED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedHoveredIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_HOVERED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "BaselineButtonTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(BaselineButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
    ];
}
