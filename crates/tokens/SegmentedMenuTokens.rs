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
pub struct SegmentedMenuTokens;
impl SegmentedMenuTokens {
    pub const ACTIVE_CONTAINER_SHAPE: Dp = Dp(24.0);
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL2;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const GROUP_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_LOW;
    pub const GROUP_PADDING: Dp = Dp(4.0);
    pub const GROUP_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const HORIZONTAL_CONTAINER_BOTTOM_SPACE: Dp = Dp(8.0);
    pub const HORIZONTAL_CONTAINER_TOP_SPACE: Dp = Dp(8.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_BOTTOM_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_LEADING_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_SELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ICON_ONLY_ITEM_TOP_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_ITEM_TRAILING_SPACE: Dp = Dp(16.0);
    pub const HORIZONTAL_ICON_ONLY_SEGMENTED_GAP: Dp = Dp(4.0);
    pub const HORIZONTAL_ITEM_BETWEEN_SPACE: Dp = Dp(12.0);
    pub const HORIZONTAL_ITEM_BOTTOM_SPACE: Dp = Dp(6.0);
    pub const HORIZONTAL_ITEM_FOCUSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const HORIZONTAL_ITEM_HOVERED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const HORIZONTAL_ITEM_LEADING_SPACE: Dp = Dp(12.0);
    pub const HORIZONTAL_ITEM_PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const HORIZONTAL_ITEM_SELECTED_FOCUSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ITEM_SELECTED_HOVERED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ITEM_SELECTED_PRESSED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HORIZONTAL_ITEM_TOP_SPACE: Dp = Dp(6.0);
    pub const HORIZONTAL_ITEM_TRAILING_SPACE: Dp = Dp(12.0);
    pub const HORIZONTAL_SEGMENTED_GAP: Dp = Dp(2.0);
    pub const INACTIVE_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const ITEM: Dp = Dp(44.0);
    pub const ITEM_BETWEEN_SPACE: Dp = Dp(12.0);
    pub const ITEM_BOTTOM_SPACE: Dp = Dp(8.0);
    pub const ITEM_FIRST_CHILD_INNER_CORNER_CORNER_SIZE: ShapeToken =
        ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_FIRST_CHILD_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const ITEM_LAST_CHILD_INNER_CORNER_CORNER_SIZE: ShapeToken =
        ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_LAST_CHILD_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_LEADING_ICON_SIZE: Dp = Dp(20.0);
    pub const ITEM_LEADING_SPACE: Dp = Dp(16.0);
    pub const ITEM_SELECTED_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_MEDIUM;
    pub const ITEM_TOP_SPACE: Dp = Dp(8.0);
    pub const ITEM_TRAILING_ICON_SIZE: Dp = Dp(20.0);
    pub const ITEM_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ITEM_TRAILING_SUPPORTING_TEXT_FONT: TypographyToken =
        TypographyKeyTokens::LABEL_SMALL;
    pub const SEGMENTED_GAP: Dp = Dp(2.0);
}
impl SegmentedMenuTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ActiveContainerShape",
            value: TokenValue::Dp(SegmentedMenuTokens::ACTIVE_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(SegmentedMenuTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "GroupContainerColor",
            value: TokenValue::ColorRole(SegmentedMenuTokens::GROUP_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "GroupPadding",
            value: TokenValue::Dp(SegmentedMenuTokens::GROUP_PADDING),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "GroupShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::GROUP_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalContainerBottomSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_CONTAINER_BOTTOM_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalContainerTopSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_CONTAINER_TOP_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalIconOnlyItemBottomSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_BOTTOM_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalIconOnlyItemLeadingSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_LEADING_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalIconOnlyItemSelectedShape",
            value: TokenValue::ShapeRole(
                SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_SELECTED_SHAPE,
            ),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalIconOnlyItemTopSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_TOP_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalIconOnlyItemTrailingSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_ITEM_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalIconOnlySegmentedGap",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ICON_ONLY_SEGMENTED_GAP),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemBetweenSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_BETWEEN_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemBottomSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_BOTTOM_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemFocusedShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_FOCUSED_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemHoveredShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_HOVERED_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemLeadingSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_LEADING_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemPressedShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::HORIZONTAL_ITEM_PRESSED_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemSelectedFocusedShape",
            value: TokenValue::ShapeRole(
                SegmentedMenuTokens::HORIZONTAL_ITEM_SELECTED_FOCUSED_SHAPE,
            ),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemSelectedHoveredShape",
            value: TokenValue::ShapeRole(
                SegmentedMenuTokens::HORIZONTAL_ITEM_SELECTED_HOVERED_SHAPE,
            ),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemSelectedPressedShape",
            value: TokenValue::ShapeRole(
                SegmentedMenuTokens::HORIZONTAL_ITEM_SELECTED_PRESSED_SHAPE,
            ),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemTopSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_TOP_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalItemTrailingSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_ITEM_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "HorizontalSegmentedGap",
            value: TokenValue::Dp(SegmentedMenuTokens::HORIZONTAL_SEGMENTED_GAP),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "InactiveContainerShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::INACTIVE_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "Item",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemBetweenSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM_BETWEEN_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemBottomSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM_BOTTOM_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemFirstChildInnerCornerCornerSize",
            value: TokenValue::ShapeRole(
                SegmentedMenuTokens::ITEM_FIRST_CHILD_INNER_CORNER_CORNER_SIZE,
            ),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemFirstChildShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_FIRST_CHILD_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemFocusIndicatorColor",
            value: TokenValue::ColorRole(SegmentedMenuTokens::ITEM_FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemLabelTextFont",
            value: TokenValue::TypographyRole(SegmentedMenuTokens::ITEM_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemLastChildInnerCornerCornerSize",
            value: TokenValue::ShapeRole(
                SegmentedMenuTokens::ITEM_LAST_CHILD_INNER_CORNER_CORNER_SIZE,
            ),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemLastChildShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_LAST_CHILD_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemLeadingIconSize",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM_LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemLeadingSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM_LEADING_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemSelectedShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_SELECTED_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemShape",
            value: TokenValue::ShapeRole(SegmentedMenuTokens::ITEM_SHAPE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemSupportingTextFont",
            value: TokenValue::TypographyRole(SegmentedMenuTokens::ITEM_SUPPORTING_TEXT_FONT),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemTopSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM_TOP_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemTrailingIconSize",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM_TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemTrailingSpace",
            value: TokenValue::Dp(SegmentedMenuTokens::ITEM_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "ItemTrailingSupportingTextFont",
            value: TokenValue::TypographyRole(
                SegmentedMenuTokens::ITEM_TRAILING_SUPPORTING_TEXT_FONT,
            ),
        },
        TokenEntry {
            group: "SegmentedMenuTokens",
            name: "SegmentedGap",
            value: TokenValue::Dp(SegmentedMenuTokens::SEGMENTED_GAP),
        },
    ];
}
