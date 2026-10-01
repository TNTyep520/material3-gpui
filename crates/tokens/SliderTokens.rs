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
pub struct SliderTokens;
impl SliderTokens {
    pub const ACTIVE_CONTAINER_OPACITY: f32 = 1.0;
    pub const ACTIVE_HANDLE_HEIGHT: Dp = Dp(44.0);
    pub const ACTIVE_HANDLE_LEADING_SPACE: Dp = Dp(6.0);
    pub const ACTIVE_HANDLE_PADDING: Dp = Dp(6.0);
    pub const ACTIVE_HANDLE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ACTIVE_HANDLE_TRAILING_SPACE: Dp = Dp(6.0);
    pub const ACTIVE_HANDLE_WIDTH: Dp = Dp(4.0);
    pub const ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const ACTIVE_TRACK_HEIGHT: Dp = Dp(16.0);
    pub const ACTIVE_TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const ACTIVE_TRACK_SHAPE_LEADING: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const DISABLED_ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_ACTIVE_TRACK_OPACITY: f32 = 0.38;
    pub const DISABLED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_HANDLE_OPACITY: f32 = 0.38;
    pub const DISABLED_HANDLE_WIDTH: Dp = Dp(4.0);
    pub const DISABLED_INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_INACTIVE_TRACK_OPACITY: f32 = 0.12;
    pub const DISABLED_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const FOCUS_HANDLE_WIDTH: Dp = Dp(2.0);
    pub const FOCUS_INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const FOCUS_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HANDLE_HEIGHT: Dp = Dp(44.0);
    pub const HANDLE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const HANDLE_WIDTH: Dp = Dp(4.0);
    pub const HOVER_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const HOVER_HANDLE_WIDTH: Dp = Dp(4.0);
    pub const HOVER_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const INACTIVE_CONTAINER_OPACITY: f32 = 1.0;
    pub const INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const INACTIVE_TRACK_HEIGHT: Dp = Dp(16.0);
    pub const INACTIVE_TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const LABEL_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const PRESSED_ACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const PRESSED_HANDLE_WIDTH: Dp = Dp(2.0);
    pub const PRESSED_INACTIVE_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const PRESSED_STOP_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SLIDER_ACTIVE_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STOP_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const STOP_INDICATOR_COLOR_SELECTED: ColorToken = ColorSchemeKeyTokens::SECONDARY_CONTAINER;
    pub const STOP_INDICATOR_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STOP_INDICATOR_SIZE: Dp = Dp(4.0);
    pub const STOP_INDICATOR_TRAILING_SPACE: Dp = Dp(6.0);
    pub const VALUE_INDICATOR_ACTIVE_BOTTOM_SPACE: Dp = Dp(12.0);
    pub const VALUE_INDICATOR_CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::INVERSE_SURFACE;
    pub const VALUE_INDICATOR_LABEL_TEXT_COLOR: ColorToken =
        ColorSchemeKeyTokens::INVERSE_ON_SURFACE;
    pub const VALUE_INDICATOR_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
}
impl SliderTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveContainerOpacity",
            value: TokenValue::Float(SliderTokens::ACTIVE_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveHandleHeight",
            value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_HEIGHT),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveHandleLeadingSpace",
            value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_LEADING_SPACE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveHandlePadding",
            value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_PADDING),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveHandleShape",
            value: TokenValue::ShapeRole(SliderTokens::ACTIVE_HANDLE_SHAPE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveHandleTrailingSpace",
            value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveHandleWidth",
            value: TokenValue::Dp(SliderTokens::ACTIVE_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::ACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveTrackHeight",
            value: TokenValue::Dp(SliderTokens::ACTIVE_TRACK_HEIGHT),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveTrackShape",
            value: TokenValue::ShapeRole(SliderTokens::ACTIVE_TRACK_SHAPE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ActiveTrackShapeLeading",
            value: TokenValue::ShapeRole(SliderTokens::ACTIVE_TRACK_SHAPE_LEADING),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledActiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::DISABLED_ACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledActiveTrackOpacity",
            value: TokenValue::Float(SliderTokens::DISABLED_ACTIVE_TRACK_OPACITY),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledHandleColor",
            value: TokenValue::ColorRole(SliderTokens::DISABLED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledHandleOpacity",
            value: TokenValue::Float(SliderTokens::DISABLED_HANDLE_OPACITY),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledHandleWidth",
            value: TokenValue::Dp(SliderTokens::DISABLED_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledInactiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::DISABLED_INACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledInactiveTrackOpacity",
            value: TokenValue::Float(SliderTokens::DISABLED_INACTIVE_TRACK_OPACITY),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "DisabledStopColor",
            value: TokenValue::ColorRole(SliderTokens::DISABLED_STOP_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "FocusActiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::FOCUS_ACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "FocusHandleWidth",
            value: TokenValue::Dp(SliderTokens::FOCUS_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "FocusInactiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::FOCUS_INACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "FocusStopColor",
            value: TokenValue::ColorRole(SliderTokens::FOCUS_STOP_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "HandleColor",
            value: TokenValue::ColorRole(SliderTokens::HANDLE_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "HandleHeight",
            value: TokenValue::Dp(SliderTokens::HANDLE_HEIGHT),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "HandleShape",
            value: TokenValue::ShapeRole(SliderTokens::HANDLE_SHAPE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "HandleWidth",
            value: TokenValue::Dp(SliderTokens::HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "HoverHandleColor",
            value: TokenValue::ColorRole(SliderTokens::HOVER_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "HoverHandleWidth",
            value: TokenValue::Dp(SliderTokens::HOVER_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "HoverStopColor",
            value: TokenValue::ColorRole(SliderTokens::HOVER_STOP_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "InactiveContainerOpacity",
            value: TokenValue::Float(SliderTokens::INACTIVE_CONTAINER_OPACITY),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "InactiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::INACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "InactiveTrackHeight",
            value: TokenValue::Dp(SliderTokens::INACTIVE_TRACK_HEIGHT),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "InactiveTrackShape",
            value: TokenValue::ShapeRole(SliderTokens::INACTIVE_TRACK_SHAPE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "LabelContainerColor",
            value: TokenValue::ColorRole(SliderTokens::LABEL_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "LabelTextColor",
            value: TokenValue::ColorRole(SliderTokens::LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "PressedActiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::PRESSED_ACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "PressedHandleColor",
            value: TokenValue::ColorRole(SliderTokens::PRESSED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "PressedHandleWidth",
            value: TokenValue::Dp(SliderTokens::PRESSED_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "PressedInactiveTrackColor",
            value: TokenValue::ColorRole(SliderTokens::PRESSED_INACTIVE_TRACK_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "PressedStopColor",
            value: TokenValue::ColorRole(SliderTokens::PRESSED_STOP_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "SliderActiveHandleColor",
            value: TokenValue::ColorRole(SliderTokens::SLIDER_ACTIVE_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "StopIndicatorColor",
            value: TokenValue::ColorRole(SliderTokens::STOP_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "StopIndicatorColorSelected",
            value: TokenValue::ColorRole(SliderTokens::STOP_INDICATOR_COLOR_SELECTED),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "StopIndicatorShape",
            value: TokenValue::ShapeRole(SliderTokens::STOP_INDICATOR_SHAPE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "StopIndicatorSize",
            value: TokenValue::Dp(SliderTokens::STOP_INDICATOR_SIZE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "StopIndicatorTrailingSpace",
            value: TokenValue::Dp(SliderTokens::STOP_INDICATOR_TRAILING_SPACE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ValueIndicatorActiveBottomSpace",
            value: TokenValue::Dp(SliderTokens::VALUE_INDICATOR_ACTIVE_BOTTOM_SPACE),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ValueIndicatorContainerColor",
            value: TokenValue::ColorRole(SliderTokens::VALUE_INDICATOR_CONTAINER_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ValueIndicatorLabelTextColor",
            value: TokenValue::ColorRole(SliderTokens::VALUE_INDICATOR_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "SliderTokens",
            name: "ValueIndicatorLabelTextFont",
            value: TokenValue::TypographyRole(SliderTokens::VALUE_INDICATOR_LABEL_TEXT_FONT),
        },
    ];
}
