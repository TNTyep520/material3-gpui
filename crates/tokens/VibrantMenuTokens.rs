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
pub struct VibrantMenuTokens;
impl VibrantMenuTokens {
    pub const BUTTON_DISABLED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const BUTTON_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const BUTTON_SELECTED_DISABLED_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const BUTTON_SELECTED_ICON_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ICON_BUTTON_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ICON_BUTTON_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_FOCUSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_FOCUSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_FOCUSED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_HOVERED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_HOVERED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_HOVERED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_PRESSED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_PRESSED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_PRESSED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::TERTIARY;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY;
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
}
impl VibrantMenuTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ButtonDisabledIconIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::BUTTON_DISABLED_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ButtonIconIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::BUTTON_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ButtonSelectedDisabledIconIconColor",
            value: TokenValue::ColorRole(
                VibrantMenuTokens::BUTTON_SELECTED_DISABLED_ICON_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ButtonSelectedIconIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::BUTTON_SELECTED_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "IconButtonContainerColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ICON_BUTTON_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "IconButtonSelectedContainerColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ICON_BUTTON_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledLabelTextOpacity",
            value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledLeadingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledLeadingIconOpacity",
            value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledSupportingTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledSupportingTextOpacity",
            value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_SUPPORTING_TEXT_OPACITY),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledTrailingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledTrailingIconOpacity",
            value: TokenValue::Float(VibrantMenuTokens::ITEM_DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                VibrantMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemDisabledTrailingSupportingTextOpacity",
            value: TokenValue::Float(
                VibrantMenuTokens::ITEM_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemFocusedLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemFocusedLeadingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemFocusedSupportingTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemFocusedTrailingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_FOCUSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemFocusedTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                VibrantMenuTokens::ITEM_FOCUSED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemHoveredLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemHoveredLeadingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemHoveredSupportingTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemHoveredTrailingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_HOVERED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemHoveredTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                VibrantMenuTokens::ITEM_HOVERED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemLeadingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemPressedLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemPressedLeadingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemPressedSupportingTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemPressedTrailingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_PRESSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemPressedTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                VibrantMenuTokens::ITEM_PRESSED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedContainerColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedDisabledLabelTextOpacity",
            value: TokenValue::Float(VibrantMenuTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedDisabledLeadingIconOpacity",
            value: TokenValue::Float(
                VibrantMenuTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedDisabledSupportingTextOpacity",
            value: TokenValue::Float(
                VibrantMenuTokens::ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedDisabledTrailingIconOpacity",
            value: TokenValue::Float(
                VibrantMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedDisabledTrailingSupportingTextOpacity",
            value: TokenValue::Float(
                VibrantMenuTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedFocusedLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_FOCUSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedHoveredLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_HOVERED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedLeadingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedSupportingTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedTrailingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SELECTED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSelectedTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                VibrantMenuTokens::ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemSupportingTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemTrailingIconColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "VibrantMenuTokens",
            name: "ItemTrailingSupportingTextColor",
            value: TokenValue::ColorRole(VibrantMenuTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
        },
    ];
}
