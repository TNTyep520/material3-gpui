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
pub struct StandardMenuTokens;
impl StandardMenuTokens {
    pub const BUTTON_DISABLED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const BUTTON_SELECTED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const DISABLED_BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ICON_BUTTON_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ICON_BUTTON_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const ITEM_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_FOCUSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_FOCUSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVERED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_HOVERED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVERED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl StandardMenuTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ButtonDisabledIconIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::BUTTON_DISABLED_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ButtonIconIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::BUTTON_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ButtonSelectedIconIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::BUTTON_SELECTED_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(StandardMenuTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "DisabledButtonIconIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::DISABLED_BUTTON_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "IconButtonContainerColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ICON_BUTTON_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "IconButtonSelectedContainerColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ICON_BUTTON_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemContainerColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledLabelTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledLabelTextOpacity",
            value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledLeadingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledLeadingIconOpacity",
            value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledSupportingTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledSupportingTextOpacity",
            value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_OPACITY),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledTrailingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledTrailingIconOpacity",
            value: TokenValue::Float(StandardMenuTokens::ITEM_DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemDisabledTrailingSupportingTextOpacity",
            value: TokenValue::Float(
                StandardMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemFocusedLabelTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemFocusedLeadingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_FOCUSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemFocusedTrailingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_FOCUSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemHoveredLabelTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemHoveredLeadingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_HOVERED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemHoveredTrailingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_HOVERED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemLabelTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemLeadingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemPressedLabelTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemPressedLeadingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_PRESSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemPressedTrailingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_PRESSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedContainerColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledContainerColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_DISABLED_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledContainerOpacity",
            value: TokenValue::Float(StandardMenuTokens::ITEM_SELECTED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledLabelTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledLabelTextOpacity",
            value: TokenValue::Float(StandardMenuTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledLeadingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledLeadingIconOpacity",
            value: TokenValue::Float(
                StandardMenuTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledTrailingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledTrailingIconOpacity",
            value: TokenValue::Float(
                StandardMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedDisabledTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedFocusedLeadingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_FOCUSED_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedFocusedSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_FOCUSED_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedFocusedTrailingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_FOCUSED_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedFocusedTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedHoveredLeadingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_HOVERED_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedHoveredSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_HOVERED_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedHoveredTrailingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_HOVERED_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedHoveredTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedLabelTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedLeadingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedPressedLeadingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedPressedSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_PRESSED_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedPressedTrailingIconColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedPressedTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedSupportingTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedTrailingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SELECTED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSelectedTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                StandardMenuTokens::ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemSupportingTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemTrailingIconColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "StandardMenuTokens",
            name: "ItemTrailingSupportingTextColor",
            value: TokenValue::ColorRole(StandardMenuTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
        },
    ];
}
