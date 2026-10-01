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
    ColorSchemeKeyTokens, ColorToken, Dp, ShapeKeyTokens, ShapeToken, ShapeValue, TokenEntry,
    TokenValue,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct CheckboxTokens;
impl CheckboxTokens {
    pub const CONTAINER_SHAPE: ShapeValue = ShapeValue::rounded(2.0);
    pub const CONTAINER_SIZE: Dp = Dp(18.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ICON_SIZE: Dp = Dp(18.0);
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const SELECTED_DISABLED_CONTAINER_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const SELECTED_ERROR_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_FOCUS_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_ERROR_HOVER_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_ERROR_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_ERROR_PRESSED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const SELECTED_ERROR_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const SELECTED_FOCUS_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_FOCUS_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_HOVER_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HOVER_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const SELECTED_PRESSED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_PRESSED_OUTLINE_WIDTH: Dp = Dp(0.0);
    pub const STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STATE_LAYER_SIZE: Dp = Dp(40.0);
    pub const UNSELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const UNSELECTED_DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_DISABLED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_ERROR_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_ERROR_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_ERROR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_ERROR_PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const UNSELECTED_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_HOVER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const UNSELECTED_PRESSED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_PRESSED_OUTLINE_WIDTH: Dp = Dp(2.0);
}
impl CheckboxTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "CheckboxTokens",
            name: "ContainerShape",
            value: TokenValue::Shape(CheckboxTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "ContainerSize",
            value: TokenValue::Dp(CheckboxTokens::CONTAINER_SIZE),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(CheckboxTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "IconSize",
            value: TokenValue::Dp(CheckboxTokens::ICON_SIZE),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedDisabledContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedDisabledContainerOpacity",
            value: TokenValue::Float(CheckboxTokens::SELECTED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedDisabledContainerOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::SELECTED_DISABLED_CONTAINER_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedDisabledIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorFocusContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_FOCUS_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorFocusIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorHoverContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_HOVER_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorHoverIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorPressedContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_PRESSED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedErrorPressedIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ERROR_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedFocusContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_FOCUS_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedFocusIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedFocusOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::SELECTED_FOCUS_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedHoverContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_HOVER_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedHoverIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedHoverOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::SELECTED_HOVER_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::SELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedPressedContainerColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_PRESSED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(CheckboxTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "SelectedPressedOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::SELECTED_PRESSED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "StateLayerShape",
            value: TokenValue::ShapeRole(CheckboxTokens::STATE_LAYER_SHAPE),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "StateLayerSize",
            value: TokenValue::Dp(CheckboxTokens::STATE_LAYER_SIZE),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedDisabledContainerOpacity",
            value: TokenValue::Float(CheckboxTokens::UNSELECTED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedDisabledOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedDisabledOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::UNSELECTED_DISABLED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedErrorFocusOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedErrorHoverOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_HOVER_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedErrorOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedErrorPressedOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_ERROR_PRESSED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedFocusOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedFocusOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::UNSELECTED_FOCUS_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedHoverOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_HOVER_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedHoverOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::UNSELECTED_HOVER_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::UNSELECTED_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedPressedOutlineColor",
            value: TokenValue::ColorRole(CheckboxTokens::UNSELECTED_PRESSED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "CheckboxTokens",
            name: "UnselectedPressedOutlineWidth",
            value: TokenValue::Dp(CheckboxTokens::UNSELECTED_PRESSED_OUTLINE_WIDTH),
        },
    ];
}
