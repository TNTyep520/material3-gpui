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
pub struct DatePickerModalTokens;
impl DatePickerModalTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGH;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_HEIGHT: Dp = Dp(568.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const CONTAINER_WIDTH: Dp = Dp(360.0);
    pub const DATE_CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const DATE_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const DATE_CONTAINER_WIDTH: Dp = Dp(40.0);
    pub const DATE_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const DATE_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DATE_SELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const DATE_STATE_LAYER_HEIGHT: Dp = Dp(40.0);
    pub const DATE_STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const DATE_STATE_LAYER_WIDTH: Dp = Dp(40.0);
    pub const DATE_TODAY_CONTAINER_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DATE_TODAY_CONTAINER_OUTLINE_WIDTH: Dp = Dp(1.0);
    pub const DATE_TODAY_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const DATE_UNSELECTED_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const HEADER_CONTAINER_HEIGHT: Dp = Dp(120.0);
    pub const HEADER_CONTAINER_WIDTH: Dp = Dp(360.0);
    pub const HEADER_HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_HEADLINE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_LARGE;
    pub const HEADER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
    pub const RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_COLOR: ColorToken =
        ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_HEIGHT: Dp = Dp(40.0);
    pub const RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_SHAPE: ShapeToken =
        ShapeKeyTokens::CORNER_FULL;
    pub const RANGE_SELECTION_CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL0;
    pub const RANGE_SELECTION_CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_NONE;
    pub const SELECTION_DATE_IN_RANGE_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SECONDARY_CONTAINER;
    pub const RANGE_SELECTION_HEADER_CONTAINER_HEIGHT: Dp = Dp(128.0);
    pub const RANGE_SELECTION_HEADER_HEADLINE_FONT: TypographyToken =
        TypographyKeyTokens::TITLE_LARGE;
    pub const RANGE_SELECTION_MONTH_SUBHEAD_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const RANGE_SELECTION_MONTH_SUBHEAD_FONT: TypographyToken =
        TypographyKeyTokens::TITLE_SMALL;
    pub const WEEKDAYS_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const WEEKDAYS_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const SELECTION_YEAR_CONTAINER_HEIGHT: Dp = Dp(36.0);
    pub const SELECTION_YEAR_CONTAINER_WIDTH: Dp = Dp(72.0);
    pub const SELECTION_YEAR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::BODY_LARGE;
    pub const SELECTION_YEAR_SELECTED_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTION_YEAR_SELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTION_YEAR_STATE_LAYER_HEIGHT: Dp = Dp(36.0);
    pub const SELECTION_YEAR_STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SELECTION_YEAR_STATE_LAYER_WIDTH: Dp = Dp(72.0);
    pub const SELECTION_YEAR_UNSELECTED_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
}
impl DatePickerModalTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(DatePickerModalTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(DatePickerModalTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(DatePickerModalTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "ContainerWidth",
            value: TokenValue::Dp(DatePickerModalTokens::CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateContainerHeight",
            value: TokenValue::Dp(DatePickerModalTokens::DATE_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateContainerShape",
            value: TokenValue::ShapeRole(DatePickerModalTokens::DATE_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateContainerWidth",
            value: TokenValue::Dp(DatePickerModalTokens::DATE_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateLabelTextFont",
            value: TokenValue::TypographyRole(DatePickerModalTokens::DATE_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateSelectedContainerColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::DATE_SELECTED_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateSelectedLabelTextColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::DATE_SELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateStateLayerHeight",
            value: TokenValue::Dp(DatePickerModalTokens::DATE_STATE_LAYER_HEIGHT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateStateLayerShape",
            value: TokenValue::ShapeRole(DatePickerModalTokens::DATE_STATE_LAYER_SHAPE),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateStateLayerWidth",
            value: TokenValue::Dp(DatePickerModalTokens::DATE_STATE_LAYER_WIDTH),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateTodayContainerOutlineColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::DATE_TODAY_CONTAINER_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateTodayContainerOutlineWidth",
            value: TokenValue::Dp(DatePickerModalTokens::DATE_TODAY_CONTAINER_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateTodayLabelTextColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::DATE_TODAY_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "DateUnselectedLabelTextColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::DATE_UNSELECTED_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "HeaderContainerHeight",
            value: TokenValue::Dp(DatePickerModalTokens::HEADER_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "HeaderContainerWidth",
            value: TokenValue::Dp(DatePickerModalTokens::HEADER_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "HeaderHeadlineColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::HEADER_HEADLINE_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "HeaderHeadlineFont",
            value: TokenValue::TypographyRole(DatePickerModalTokens::HEADER_HEADLINE_FONT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "HeaderSupportingTextColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::HEADER_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "HeaderSupportingTextFont",
            value: TokenValue::TypographyRole(DatePickerModalTokens::HEADER_SUPPORTING_TEXT_FONT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionActiveIndicatorContainerColor",
            value: TokenValue::ColorRole(
                DatePickerModalTokens::RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionActiveIndicatorContainerHeight",
            value: TokenValue::Dp(
                DatePickerModalTokens::RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_HEIGHT,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionActiveIndicatorContainerShape",
            value: TokenValue::ShapeRole(
                DatePickerModalTokens::RANGE_SELECTION_ACTIVE_INDICATOR_CONTAINER_SHAPE,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionContainerElevation",
            value: TokenValue::Dp(DatePickerModalTokens::RANGE_SELECTION_CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionContainerShape",
            value: TokenValue::ShapeRole(DatePickerModalTokens::RANGE_SELECTION_CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionDateInRangeLabelTextColor",
            value: TokenValue::ColorRole(
                DatePickerModalTokens::SELECTION_DATE_IN_RANGE_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionHeaderContainerHeight",
            value: TokenValue::Dp(DatePickerModalTokens::RANGE_SELECTION_HEADER_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionHeaderHeadlineFont",
            value: TokenValue::TypographyRole(
                DatePickerModalTokens::RANGE_SELECTION_HEADER_HEADLINE_FONT,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionMonthSubheadColor",
            value: TokenValue::ColorRole(
                DatePickerModalTokens::RANGE_SELECTION_MONTH_SUBHEAD_COLOR,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "RangeSelectionMonthSubheadFont",
            value: TokenValue::TypographyRole(
                DatePickerModalTokens::RANGE_SELECTION_MONTH_SUBHEAD_FONT,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "WeekdaysLabelTextColor",
            value: TokenValue::ColorRole(DatePickerModalTokens::WEEKDAYS_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "WeekdaysLabelTextFont",
            value: TokenValue::TypographyRole(DatePickerModalTokens::WEEKDAYS_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearContainerHeight",
            value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearContainerWidth",
            value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearLabelTextFont",
            value: TokenValue::TypographyRole(
                DatePickerModalTokens::SELECTION_YEAR_LABEL_TEXT_FONT,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearSelectedContainerColor",
            value: TokenValue::ColorRole(
                DatePickerModalTokens::SELECTION_YEAR_SELECTED_CONTAINER_COLOR,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearSelectedLabelTextColor",
            value: TokenValue::ColorRole(
                DatePickerModalTokens::SELECTION_YEAR_SELECTED_LABEL_TEXT_COLOR,
            ),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearStateLayerHeight",
            value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_STATE_LAYER_HEIGHT),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearStateLayerShape",
            value: TokenValue::ShapeRole(DatePickerModalTokens::SELECTION_YEAR_STATE_LAYER_SHAPE),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearStateLayerWidth",
            value: TokenValue::Dp(DatePickerModalTokens::SELECTION_YEAR_STATE_LAYER_WIDTH),
        },
        TokenEntry {
            group: "DatePickerModalTokens",
            name: "SelectionYearUnselectedLabelTextColor",
            value: TokenValue::ColorRole(
                DatePickerModalTokens::SELECTION_YEAR_UNSELECTED_LABEL_TEXT_COLOR,
            ),
        },
    ];
}
