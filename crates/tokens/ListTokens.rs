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
pub struct ListTokens;
impl ListTokens {
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const DIVIDER_BOTTOM_SPACE: Dp = Dp(0.0);
    pub const DIVIDER_LEADING_SPACE: Dp = Dp(16.0);
    pub const DIVIDER_TOP_SPACE: Dp = Dp(0.0);
    pub const DIVIDER_TRAILING_SPACE: Dp = Dp(16.0);
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const ITEM_BETWEEN_SPACE: Dp = Dp(12.0);
    pub const ITEM_BOTTOM_SPACE: Dp = Dp(10.0);
    pub const ITEM_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const ITEM_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const ITEM_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const ITEM_DISABLED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_EXTRA_SMALL;
    pub const ITEM_DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_OVERLINE_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_STATE_LAYER_OPACITY: f32 = 0.1;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_DISABLED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_DRAGGED_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL4;
    pub const ITEM_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_DRAGGED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_DRAGGED_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_DRAGGED_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_FOCUS_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUS_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_HOVER_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVER_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_HOVERED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_MEDIUM;
    pub const ITEM_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const ITEM_LARGE_LEADING_VIDEO_HEIGHT: Dp = Dp(64.0);
    pub const ITEM_LARGE_LEADING_VIDEO_WIDTH: Dp = Dp(114.0);
    pub const ITEM_LEADING_AVATAR_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const ITEM_LEADING_AVATAR_LABEL_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const ITEM_LEADING_AVATAR_LABEL_FONT: TypographyToken = TypographyKeyTokens::TITLE_MEDIUM;
    pub const ITEM_LEADING_AVATAR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ITEM_LEADING_AVATAR_SIZE: Dp = Dp(40.0);
    pub const ITEM_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_LEADING_ICON_EXPRESSIVE_SIZE: Dp = Dp(20.0);
    pub const ITEM_LEADING_ICON_SIZE: Dp = Dp(24.0);
    pub const ITEM_LEADING_IMAGE_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const ITEM_LEADING_IMAGE_HEIGHT: Dp = Dp(56.0);
    pub const ITEM_LEADING_IMAGE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const ITEM_LEADING_IMAGE_WIDTH: Dp = Dp(56.0);
    pub const ITEM_LEADING_SPACE: Dp = Dp(16.0);
    pub const ITEM_LEADING_VIDEO_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const ITEM_LEADING_VIDEO_WIDTH: Dp = Dp(100.0);
    pub const ITEM_ONE_LINE_CONTAINER_HEIGHT: Dp = Dp(56.0);
    pub const ITEM_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_OVERLINE_FONT: TypographyToken = TypographyKeyTokens::LABEL_SMALL;
    pub const ITEM_PRESSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_PRESSED_LEADING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_PRESSED_TRAILING_ICON_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_SEGMENTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const ITEM_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_DISABLED_CONTAINER_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_OVERLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_OVERLINE_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_STATE_LAYER_OPACITY: f32 = 0.1;
    pub const ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY: f32 = 0.38;
    pub const ITEM_SELECTED_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_DRAGGED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_DRAGGED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_DRAGGED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_FOCUS_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_FOCUS_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_HOVER_LEADING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_HOVER_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_HOVERED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_OVERLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_CONTAINER_EXPRESSIVE_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_LARGE;
    pub const ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const ITEM_SELECTED_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const ITEM_SMALL_LEADING_VIDEO_HEIGHT: Dp = Dp(56.0);
    pub const ITEM_SMALL_LEADING_VIDEO_WIDTH: Dp = Dp(100.0);
    pub const ITEM_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_MEDIUM;
    pub const ITEM_THREE_LINE_CONTAINER_HEIGHT: Dp = Dp(88.0);
    pub const ITEM_TOP_SPACE: Dp = Dp(10.0);
    pub const ITEM_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_ICON_EXPRESSIVE_SIZE: Dp = Dp(20.0);
    pub const ITEM_TRAILING_ICON_SIZE: Dp = Dp(24.0);
    pub const ITEM_TRAILING_SPACE: Dp = Dp(16.0);
    pub const ITEM_TRAILING_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const ITEM_TRAILING_SUPPORTING_TEXT_FONT: TypographyToken =
        TypographyKeyTokens::LABEL_SMALL;
    pub const ITEM_TWO_LINE_CONTAINER_HEIGHT: Dp = Dp(72.0);
    pub const ITEM_UNSELECTED_TRAILING_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const SEGMENTED_GAP: Dp = Dp(2.0);
}
impl ListTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ListTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(ListTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "DividerBottomSpace",
            value: TokenValue::Dp(ListTokens::DIVIDER_BOTTOM_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "DividerLeadingSpace",
            value: TokenValue::Dp(ListTokens::DIVIDER_LEADING_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "DividerTopSpace",
            value: TokenValue::Dp(ListTokens::DIVIDER_TOP_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "DividerTrailingSpace",
            value: TokenValue::Dp(ListTokens::DIVIDER_TRAILING_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(ListTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemBetweenSpace",
            value: TokenValue::Dp(ListTokens::ITEM_BETWEEN_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemBottomSpace",
            value: TokenValue::Dp(ListTokens::ITEM_BOTTOM_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemContainerColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemContainerElevation",
            value: TokenValue::Dp(ListTokens::ITEM_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemContainerExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_CONTAINER_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemContainerShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledContainerExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_DISABLED_CONTAINER_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledLabelTextOpacity",
            value: TokenValue::Float(ListTokens::ITEM_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledLeadingIconOpacity",
            value: TokenValue::Float(ListTokens::ITEM_DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledOverlineColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_OVERLINE_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledOverlineOpacity",
            value: TokenValue::Float(ListTokens::ITEM_DISABLED_OVERLINE_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledStateLayerOpacity",
            value: TokenValue::Float(ListTokens::ITEM_DISABLED_STATE_LAYER_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledSupportingTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledSupportingTextOpacity",
            value: TokenValue::Float(ListTokens::ITEM_DISABLED_SUPPORTING_TEXT_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDisabledTrailingIconOpacity",
            value: TokenValue::Float(ListTokens::ITEM_DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDraggedContainerElevation",
            value: TokenValue::Dp(ListTokens::ITEM_DRAGGED_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDraggedContainerExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDraggedLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDraggedLeadingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DRAGGED_LEADING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemDraggedTrailingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_DRAGGED_TRAILING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemFocusLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemFocusLeadingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_FOCUS_LEADING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemFocusTrailingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_FOCUS_TRAILING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemFocusedContainerExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemHoverLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemHoverLeadingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_HOVER_LEADING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemHoverTrailingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_HOVER_TRAILING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemHoveredContainerExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_HOVERED_CONTAINER_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLabelTextFont",
            value: TokenValue::TypographyRole(ListTokens::ITEM_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLargeLeadingVideoHeight",
            value: TokenValue::Dp(ListTokens::ITEM_LARGE_LEADING_VIDEO_HEIGHT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLargeLeadingVideoWidth",
            value: TokenValue::Dp(ListTokens::ITEM_LARGE_LEADING_VIDEO_WIDTH),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingAvatarColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_LEADING_AVATAR_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingAvatarLabelColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_LEADING_AVATAR_LABEL_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingAvatarLabelFont",
            value: TokenValue::TypographyRole(ListTokens::ITEM_LEADING_AVATAR_LABEL_FONT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingAvatarShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_AVATAR_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingAvatarSize",
            value: TokenValue::Dp(ListTokens::ITEM_LEADING_AVATAR_SIZE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingIconExpressiveSize",
            value: TokenValue::Dp(ListTokens::ITEM_LEADING_ICON_EXPRESSIVE_SIZE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingIconSize",
            value: TokenValue::Dp(ListTokens::ITEM_LEADING_ICON_SIZE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingImageExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_IMAGE_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingImageHeight",
            value: TokenValue::Dp(ListTokens::ITEM_LEADING_IMAGE_HEIGHT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingImageShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_IMAGE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingImageWidth",
            value: TokenValue::Dp(ListTokens::ITEM_LEADING_IMAGE_WIDTH),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingSpace",
            value: TokenValue::Dp(ListTokens::ITEM_LEADING_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingVideoShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_LEADING_VIDEO_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemLeadingVideoWidth",
            value: TokenValue::Dp(ListTokens::ITEM_LEADING_VIDEO_WIDTH),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemOneLineContainerHeight",
            value: TokenValue::Dp(ListTokens::ITEM_ONE_LINE_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemOverlineColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_OVERLINE_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemOverlineFont",
            value: TokenValue::TypographyRole(ListTokens::ITEM_OVERLINE_FONT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemPressedContainerExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_PRESSED_CONTAINER_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemPressedLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemPressedLeadingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_PRESSED_LEADING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemPressedTrailingIconIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_PRESSED_TRAILING_ICON_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSegmentedContainerColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SEGMENTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedContainerColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedContainerExpressiveShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_CONTAINER_EXPRESSIVE_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedContainerShape",
            value: TokenValue::ShapeRole(ListTokens::ITEM_SELECTED_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledContainerColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledContainerExpressiveShape",
            value: TokenValue::ShapeRole(
                ListTokens::ITEM_SELECTED_DISABLED_CONTAINER_EXPRESSIVE_SHAPE,
            ),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledContainerOpacity",
            value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledLabelTextOpacity",
            value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledLeadingIconOpacity",
            value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_LEADING_ICON_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledOverlineColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_OVERLINE_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledOverlineOpacity",
            value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_OVERLINE_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledStateLayerOpacity",
            value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_STATE_LAYER_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledSupportingTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledSupportingTextOpacity",
            value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_SUPPORTING_TEXT_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledTrailingIconOpacity",
            value: TokenValue::Float(ListTokens::ITEM_SELECTED_DISABLED_TRAILING_ICON_OPACITY),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledTrailingSupportingTextColor",
            value: TokenValue::ColorRole(
                ListTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDisabledTrailingSupportingTextOpacity",
            value: TokenValue::Float(
                ListTokens::ITEM_SELECTED_DISABLED_TRAILING_SUPPORTING_TEXT_OPACITY,
            ),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDraggedContainerExpressiveShape",
            value: TokenValue::ShapeRole(
                ListTokens::ITEM_SELECTED_DRAGGED_CONTAINER_EXPRESSIVE_SHAPE,
            ),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDraggedLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DRAGGED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDraggedLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DRAGGED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedDraggedTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_DRAGGED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedFocusLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedFocusLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_FOCUS_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedFocusTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_FOCUS_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedFocusedContainerExpressiveShape",
            value: TokenValue::ShapeRole(
                ListTokens::ITEM_SELECTED_FOCUSED_CONTAINER_EXPRESSIVE_SHAPE,
            ),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedHoverLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedHoverLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_HOVER_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedHoverTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_HOVER_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedHoveredContainerExpressiveShape",
            value: TokenValue::ShapeRole(
                ListTokens::ITEM_SELECTED_HOVERED_CONTAINER_EXPRESSIVE_SHAPE,
            ),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedOverlineColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_OVERLINE_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedPressedContainerExpressiveShape",
            value: TokenValue::ShapeRole(
                ListTokens::ITEM_SELECTED_PRESSED_CONTAINER_EXPRESSIVE_SHAPE,
            ),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_PRESSED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedPressedLeadingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_PRESSED_LEADING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedPressedTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_PRESSED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedSupportingTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSelectedTrailingSupportingTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SELECTED_TRAILING_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSmallLeadingVideoHeight",
            value: TokenValue::Dp(ListTokens::ITEM_SMALL_LEADING_VIDEO_HEIGHT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSmallLeadingVideoWidth",
            value: TokenValue::Dp(ListTokens::ITEM_SMALL_LEADING_VIDEO_WIDTH),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSupportingTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemSupportingTextFont",
            value: TokenValue::TypographyRole(ListTokens::ITEM_SUPPORTING_TEXT_FONT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemThreeLineContainerHeight",
            value: TokenValue::Dp(ListTokens::ITEM_THREE_LINE_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTopSpace",
            value: TokenValue::Dp(ListTokens::ITEM_TOP_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTrailingIconExpressiveSize",
            value: TokenValue::Dp(ListTokens::ITEM_TRAILING_ICON_EXPRESSIVE_SIZE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTrailingIconSize",
            value: TokenValue::Dp(ListTokens::ITEM_TRAILING_ICON_SIZE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTrailingSpace",
            value: TokenValue::Dp(ListTokens::ITEM_TRAILING_SPACE),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTrailingSupportingTextColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_TRAILING_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTrailingSupportingTextFont",
            value: TokenValue::TypographyRole(ListTokens::ITEM_TRAILING_SUPPORTING_TEXT_FONT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemTwoLineContainerHeight",
            value: TokenValue::Dp(ListTokens::ITEM_TWO_LINE_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "ListTokens",
            name: "ItemUnselectedTrailingIconColor",
            value: TokenValue::ColorRole(ListTokens::ITEM_UNSELECTED_TRAILING_ICON_COLOR),
        },
        TokenEntry {
            group: "ListTokens",
            name: "SegmentedGap",
            value: TokenValue::Dp(ListTokens::SEGMENTED_GAP),
        },
    ];
}
