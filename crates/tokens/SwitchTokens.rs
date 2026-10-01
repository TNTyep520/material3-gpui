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
};

#[derive(Clone, Copy, Debug, Default)]
pub struct SwitchTokens;
impl SwitchTokens {
    pub const DISABLED_SELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const DISABLED_SELECTED_HANDLE_OPACITY: f32 = 1.0;
    pub const DISABLED_SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_SELECTED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_SELECTED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_TRACK_OPACITY: f32 = 0.12;
    pub const DISABLED_UNSELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE;
    pub const DISABLED_UNSELECTED_HANDLE_OPACITY: f32 = 0.38;
    pub const DISABLED_UNSELECTED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const DISABLED_UNSELECTED_ICON_OPACITY: f32 = 0.38;
    pub const DISABLED_UNSELECTED_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const DISABLED_UNSELECTED_TRACK_OUTLINE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE;
    pub const FOCUS_INDICATOR_COLOR: ColorToken = ColorSchemeKeyTokens::SECONDARY;
    pub const HANDLE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const PRESSED_HANDLE_HEIGHT: Dp = Dp(28.0);
    pub const PRESSED_HANDLE_WIDTH: Dp = Dp(28.0);
    pub const SELECTED_FOCUS_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const SELECTED_FOCUS_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_FOCUS_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY;
    pub const SELECTED_HANDLE_HEIGHT: Dp = Dp(24.0);
    pub const SELECTED_HANDLE_WIDTH: Dp = Dp(24.0);
    pub const SELECTED_HOVER_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const SELECTED_HOVER_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_HOVER_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_ICON_SIZE: Dp = Dp(16.0);
    pub const SELECTED_PRESSED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY_CONTAINER;
    pub const SELECTED_PRESSED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::ON_PRIMARY_CONTAINER;
    pub const SELECTED_PRESSED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const SELECTED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::PRIMARY;
    pub const STATE_LAYER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const STATE_LAYER_SIZE: Dp = Dp(40.0);
    pub const TRACK_HEIGHT: Dp = Dp(32.0);
    pub const TRACK_OUTLINE_WIDTH: Dp = Dp(2.0);
    pub const TRACK_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const TRACK_WIDTH: Dp = Dp(52.0);
    pub const UNSELECTED_FOCUS_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_FOCUS_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_FOCUS_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_FOCUS_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_HANDLE_HEIGHT: Dp = Dp(16.0);
    pub const UNSELECTED_HANDLE_WIDTH: Dp = Dp(16.0);
    pub const UNSELECTED_HOVER_HANDLE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_HOVER_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_HOVER_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_HOVER_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_ICON_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_ICON_SIZE: Dp = Dp(16.0);
    pub const UNSELECTED_PRESSED_HANDLE_COLOR: ColorToken =
        ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const UNSELECTED_PRESSED_ICON_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_PRESSED_TRACK_COLOR: ColorToken =
        ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_PRESSED_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const UNSELECTED_TRACK_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_CONTAINER_HIGHEST;
    pub const UNSELECTED_TRACK_OUTLINE_COLOR: ColorToken = ColorSchemeKeyTokens::OUTLINE;
    pub const ICON_HANDLE_HEIGHT: Dp = Dp(24.0);
    pub const ICON_HANDLE_WIDTH: Dp = Dp(24.0);
}
impl SwitchTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledSelectedHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::DISABLED_SELECTED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledSelectedHandleOpacity",
            value: TokenValue::Float(SwitchTokens::DISABLED_SELECTED_HANDLE_OPACITY),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledSelectedIconColor",
            value: TokenValue::ColorRole(SwitchTokens::DISABLED_SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledSelectedIconOpacity",
            value: TokenValue::Float(SwitchTokens::DISABLED_SELECTED_ICON_OPACITY),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledSelectedTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::DISABLED_SELECTED_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledTrackOpacity",
            value: TokenValue::Float(SwitchTokens::DISABLED_TRACK_OPACITY),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledUnselectedHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledUnselectedHandleOpacity",
            value: TokenValue::Float(SwitchTokens::DISABLED_UNSELECTED_HANDLE_OPACITY),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledUnselectedIconColor",
            value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledUnselectedIconOpacity",
            value: TokenValue::Float(SwitchTokens::DISABLED_UNSELECTED_ICON_OPACITY),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledUnselectedTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "DisabledUnselectedTrackOutlineColor",
            value: TokenValue::ColorRole(SwitchTokens::DISABLED_UNSELECTED_TRACK_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "FocusIndicatorColor",
            value: TokenValue::ColorRole(SwitchTokens::FOCUS_INDICATOR_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "HandleShape",
            value: TokenValue::ShapeRole(SwitchTokens::HANDLE_SHAPE),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "PressedHandleHeight",
            value: TokenValue::Dp(SwitchTokens::PRESSED_HANDLE_HEIGHT),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "PressedHandleWidth",
            value: TokenValue::Dp(SwitchTokens::PRESSED_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedFocusHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_FOCUS_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedFocusIconColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedFocusTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_FOCUS_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedHandleHeight",
            value: TokenValue::Dp(SwitchTokens::SELECTED_HANDLE_HEIGHT),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedHandleWidth",
            value: TokenValue::Dp(SwitchTokens::SELECTED_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedHoverHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_HOVER_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedHoverIconColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedHoverTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_HOVER_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedIconColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedIconSize",
            value: TokenValue::Dp(SwitchTokens::SELECTED_ICON_SIZE),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedPressedHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_PRESSED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedPressedIconColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedPressedTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_PRESSED_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "SelectedTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::SELECTED_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "StateLayerShape",
            value: TokenValue::ShapeRole(SwitchTokens::STATE_LAYER_SHAPE),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "StateLayerSize",
            value: TokenValue::Dp(SwitchTokens::STATE_LAYER_SIZE),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "TrackHeight",
            value: TokenValue::Dp(SwitchTokens::TRACK_HEIGHT),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "TrackOutlineWidth",
            value: TokenValue::Dp(SwitchTokens::TRACK_OUTLINE_WIDTH),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "TrackShape",
            value: TokenValue::ShapeRole(SwitchTokens::TRACK_SHAPE),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "TrackWidth",
            value: TokenValue::Dp(SwitchTokens::TRACK_WIDTH),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedFocusHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedFocusIconColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedFocusTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedFocusTrackOutlineColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_FOCUS_TRACK_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedHandleHeight",
            value: TokenValue::Dp(SwitchTokens::UNSELECTED_HANDLE_HEIGHT),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedHandleWidth",
            value: TokenValue::Dp(SwitchTokens::UNSELECTED_HANDLE_WIDTH),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedHoverHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedHoverIconColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedHoverTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedHoverTrackOutlineColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_HOVER_TRACK_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedIconColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedIconSize",
            value: TokenValue::Dp(SwitchTokens::UNSELECTED_ICON_SIZE),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedPressedHandleColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_HANDLE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedPressedIconColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_ICON_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedPressedTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedPressedTrackOutlineColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_PRESSED_TRACK_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedTrackColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_TRACK_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "UnselectedTrackOutlineColor",
            value: TokenValue::ColorRole(SwitchTokens::UNSELECTED_TRACK_OUTLINE_COLOR),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "IconHandleHeight",
            value: TokenValue::Dp(SwitchTokens::ICON_HANDLE_HEIGHT),
        },
        TokenEntry {
            group: "SwitchTokens",
            name: "IconHandleWidth",
            value: TokenValue::Dp(SwitchTokens::ICON_HANDLE_WIDTH),
        },
    ];
}
