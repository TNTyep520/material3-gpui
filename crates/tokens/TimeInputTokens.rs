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
pub struct TimeInputTokens;
impl TimeInputTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADLINE_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
    pub const PERIOD_SELECTOR_CONTAINER_HEIGHT: Dp = Dp(72.0);
    pub const PERIOD_SELECTOR_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const PERIOD_SELECTOR_CONTAINER_WIDTH: Dp = Dp(52.0);
    pub const PERIOD_SELECTOR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::TITLE_MEDIUM;
    pub const PERIOD_SELECTOR_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const PERIOD_SELECTOR_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const PERIOD_SELECTOR_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_TERTIARY_CONTAINER;
    pub const PERIOD_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const PERIOD_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TIME_FIELD_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TIME_FIELD_CONTAINER_HEIGHT: Dp = Dp(72.0);
    pub const TIME_FIELD_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const TIME_FIELD_CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const TIME_FIELD_FOCUS_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const TIME_FIELD_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_FIELD_FOCUS_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const TIME_FIELD_FOCUS_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const TIME_FIELD_HOVER_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_FIELD_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_FIELD_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_MEDIUM;
    pub const TIME_FIELD_SEPARATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_FIELD_SEPARATOR_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_LARGE;
    pub const TIME_FIELD_SUPPORTING_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const TIME_FIELD_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_SMALL;
}
impl TimeInputTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "TimeInputTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(TimeInputTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(TimeInputTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(TimeInputTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(TimeInputTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "HeadlineColor",
            value: TokenValue::ColorRole(TimeInputTokens::HEADLINE_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "HeadlineFont",
            value: TokenValue::TypographyRole(TimeInputTokens::HEADLINE_FONT),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorContainerHeight",
            value: TokenValue::Dp(TimeInputTokens::PERIOD_SELECTOR_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorContainerShape",
            value: TokenValue::ShapeRole(TimeInputTokens::PERIOD_SELECTOR_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorContainerWidth",
            value: TokenValue::Dp(TimeInputTokens::PERIOD_SELECTOR_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorLabelTextFont",
            value: TokenValue::TypographyRole(TimeInputTokens::PERIOD_SELECTOR_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorOutlineColor",
            value: TokenValue::ColorRole(TimeInputTokens::PERIOD_SELECTOR_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorOutlineWidth",
            value: TokenValue::Dp(TimeInputTokens::PERIOD_SELECTOR_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorSelectedContainerColor",
            value: TokenValue::ColorRole(TimeInputTokens::PERIOD_SELECTOR_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorSelectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorSelectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorSelectedLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_SELECTED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorSelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorUnselectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorUnselectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorUnselectedLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "PeriodSelectorUnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                TimeInputTokens::PERIOD_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldContainerColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldContainerHeight",
            value: TokenValue::Dp(TimeInputTokens::TIME_FIELD_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldContainerShape",
            value: TokenValue::ShapeRole(TimeInputTokens::TIME_FIELD_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldContainerWidth",
            value: TokenValue::Dp(TimeInputTokens::TIME_FIELD_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldFocusContainerColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_FOCUS_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldFocusLabelTextColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_FOCUS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldFocusOutlineColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_FOCUS_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldFocusOutlineWidth",
            value: TokenValue::Dp(TimeInputTokens::TIME_FIELD_FOCUS_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldHoverLabelTextColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_HOVER_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldLabelTextColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldLabelTextFont",
            value: TokenValue::TypographyRole(TimeInputTokens::TIME_FIELD_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldSeparatorColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_SEPARATOR_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldSeparatorFont",
            value: TokenValue::TypographyRole(TimeInputTokens::TIME_FIELD_SEPARATOR_FONT),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldSupportingTextColor",
            value: TokenValue::ColorRole(TimeInputTokens::TIME_FIELD_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "TimeInputTokens",
            name: "TimeFieldSupportingTextFont",
            value: TokenValue::TypographyRole(TimeInputTokens::TIME_FIELD_SUPPORTING_TEXT_FONT),
        },
    ];
}
