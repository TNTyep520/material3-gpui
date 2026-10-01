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
pub struct TimePickerTokens;
impl TimePickerTokens {
    pub const CLOCK_DIAL_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const CLOCK_DIAL_CONTAINER_SIZE: Dp = Dp(256.0);
    pub const CLOCK_DIAL_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const CLOCK_DIAL_SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SIZE: Dp = Dp(8.0);
    pub const CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SIZE: Dp = Dp(48.0);
    pub const CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_WIDTH: Dp = Dp(2.0);
    pub const CLOCK_DIAL_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const CLOCK_DIAL_UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADLINE_FONT: TypographyToken = TypographyKeyTokens::LABEL_MEDIUM;
    pub const PERIOD_SELECTOR_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const PERIOD_SELECTOR_HORIZONTAL_CONTAINER_HEIGHT: Dp = Dp(38.0);
    pub const PERIOD_SELECTOR_HORIZONTAL_CONTAINER_WIDTH: Dp = Dp(216.0);
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
    pub const PERIOD_SELECTOR_VERTICAL_CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const PERIOD_SELECTOR_VERTICAL_CONTAINER_WIDTH: Dp = Dp(52.0);
    pub const TIME_SELECTOR24_H_VERTICAL_CONTAINER_WIDTH: Dp = Dp(114.0);
    pub const TIME_SELECTOR_CONTAINER_HEIGHT: Dp = Dp(80.0);
    pub const TIME_SELECTOR_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_SMALL;
    pub const TIME_SELECTOR_CONTAINER_WIDTH: Dp = Dp(96.0);
    pub const TIME_SELECTOR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_LARGE;
    pub const TIME_SELECTOR_SELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const TIME_SELECTOR_SEPARATOR_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_SEPARATOR_FONT: TypographyToken = TypographyKeyTokens::DISPLAY_LARGE;
    pub const TIME_SELECTOR_UNSELECTED_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const TIME_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const TIME_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
}
impl TimePickerTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialColor",
            value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialContainerSize",
            value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_CONTAINER_SIZE),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialLabelTextFont",
            value: TokenValue::TypographyRole(TimePickerTokens::CLOCK_DIAL_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectedLabelTextColor",
            value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorCenterContainerColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorCenterContainerShape",
            value: TokenValue::ShapeRole(
                TimePickerTokens::CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SHAPE,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorCenterContainerSize",
            value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_SELECTOR_CENTER_CONTAINER_SIZE),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorHandleContainerColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorHandleContainerShape",
            value: TokenValue::ShapeRole(
                TimePickerTokens::CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SHAPE,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorHandleContainerSize",
            value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_SELECTOR_HANDLE_CONTAINER_SIZE),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorTrackContainerColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialSelectorTrackContainerWidth",
            value: TokenValue::Dp(TimePickerTokens::CLOCK_DIAL_SELECTOR_TRACK_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialShape",
            value: TokenValue::ShapeRole(TimePickerTokens::CLOCK_DIAL_SHAPE),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ClockDialUnselectedLabelTextColor",
            value: TokenValue::ColorRole(TimePickerTokens::CLOCK_DIAL_UNSELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(TimePickerTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(TimePickerTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(TimePickerTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "HeadlineColor",
            value: TokenValue::ColorRole(TimePickerTokens::HEADLINE_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "HeadlineFont",
            value: TokenValue::TypographyRole(TimePickerTokens::HEADLINE_FONT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorContainerShape",
            value: TokenValue::ShapeRole(TimePickerTokens::PERIOD_SELECTOR_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorHorizontalContainerHeight",
            value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_HORIZONTAL_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorHorizontalContainerWidth",
            value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_HORIZONTAL_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorLabelTextFont",
            value: TokenValue::TypographyRole(TimePickerTokens::PERIOD_SELECTOR_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorOutlineColor",
            value: TokenValue::ColorRole(TimePickerTokens::PERIOD_SELECTOR_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorOutlineWidth",
            value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorSelectedContainerColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_SELECTED_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorSelectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorSelectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorSelectedLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_SELECTED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorSelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorUnselectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorUnselectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorUnselectedLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorUnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::PERIOD_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorVerticalContainerHeight",
            value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_VERTICAL_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "PeriodSelectorVerticalContainerWidth",
            value: TokenValue::Dp(TimePickerTokens::PERIOD_SELECTOR_VERTICAL_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelector24HVerticalContainerWidth",
            value: TokenValue::Dp(TimePickerTokens::TIME_SELECTOR24_H_VERTICAL_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorContainerHeight",
            value: TokenValue::Dp(TimePickerTokens::TIME_SELECTOR_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorContainerShape",
            value: TokenValue::ShapeRole(TimePickerTokens::TIME_SELECTOR_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorContainerWidth",
            value: TokenValue::Dp(TimePickerTokens::TIME_SELECTOR_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorLabelTextFont",
            value: TokenValue::TypographyRole(TimePickerTokens::TIME_SELECTOR_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorSelectedContainerColor",
            value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorSelectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_SELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorSelectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_SELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorSelectedLabelTextColor",
            value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorSelectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_SELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorSeparatorColor",
            value: TokenValue::ColorRole(TimePickerTokens::TIME_SELECTOR_SEPARATOR_COLOR),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorSeparatorFont",
            value: TokenValue::TypographyRole(TimePickerTokens::TIME_SELECTOR_SEPARATOR_FONT),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorUnselectedContainerColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_UNSELECTED_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorUnselectedFocusLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_UNSELECTED_FOCUS_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorUnselectedHoverLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_UNSELECTED_HOVER_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorUnselectedLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_UNSELECTED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "TimePickerTokens",
            name: "TimeSelectorUnselectedPressedLabelTextColor",
            value: TokenValue::ColorRole(
                TimePickerTokens::TIME_SELECTOR_UNSELECTED_PRESSED_LABEL_TEXT_COLOR,
            ),
        },
    ];
}
