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
    ColorSchemeKeyTokens, ColorToken, Dp, ShapeKeyTokens, ShapeToken, TokenEntry, TokenValue,
    TypographyKeyTokens, TypographyToken,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct OutlinedSegmentedButtonTokens;
impl OutlinedSegmentedButtonTokens {
    pub const CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const DISABLED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_LABEL_TEXT_OPACITY: f32 = 0.38;
    pub const DISABLED_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_OUTLINE_OPACITY: f32 = 0.12;
    pub const LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const UNSELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const ICON_SIZE: Dp = Dp(18.0);
}
impl OutlinedSegmentedButtonTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(OutlinedSegmentedButtonTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "DisabledIconColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::DISABLED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "DisabledIconOpacity",
            value: TokenValue::Float(OutlinedSegmentedButtonTokens::DISABLED_ICON_OPACITY),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "DisabledLabelTextColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::DISABLED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "DisabledLabelTextOpacity",
            value: TokenValue::Float(OutlinedSegmentedButtonTokens::DISABLED_LABEL_TEXT_OPACITY),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "DisabledOutlineColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::DISABLED_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "DisabledOutlineOpacity",
            value: TokenValue::Float(OutlinedSegmentedButtonTokens::DISABLED_OUTLINE_OPACITY),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "LabelTextFont",
            value: TokenValue::TypographyRole(OutlinedSegmentedButtonTokens::LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "OutlineColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::OUTLINE_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "OutlineWidth",
            value: TokenValue::Dp(OutlinedSegmentedButtonTokens::OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedContainerColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedFocusIconColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::SELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedHoverIconColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::SELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedLabelTextColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::SELECTED_PRESSED_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::SELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "Shape",
            value: TokenValue::ShapeRole(OutlinedSegmentedButtonTokens::SHAPE),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedFocusIconColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::UNSELECTED_FOCUS_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedHoverIconColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::UNSELECTED_HOVER_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::UNSELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::UNSELECTED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::UNSELECTED_PRESSED_ICON_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                OutlinedSegmentedButtonTokens::UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(OutlinedSegmentedButtonTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "OutlinedSegmentedButtonTokens",
            name: "IconSize",
            value: TokenValue::Dp(OutlinedSegmentedButtonTokens::ICON_SIZE),
        },
    ];
}
