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
pub struct DateInputModalTokens;
impl DateInputModalTokens {
    pub const CONTAINER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE;
    pub const CONTAINER_ELEVATION: Dp = ElevationTokens::LEVEL3;
    pub const CONTAINER_HEIGHT: Dp = Dp(512.0);
    pub const CONTAINER_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_EXTRA_LARGE;
    pub const CONTAINER_SURFACE_TINT_LAYER_COLOR: ColorToken = ColorSchemeKeyTokens::SURFACE_TINT;
    pub const CONTAINER_WIDTH: Dp = Dp(328.0);
    pub const HEADER_CONTAINER_HEIGHT: Dp = Dp(120.0);
    pub const HEADER_CONTAINER_WIDTH: Dp = Dp(328.0);
    pub const HEADER_HEADLINE_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_HEADLINE_FONT: TypographyToken = TypographyKeyTokens::HEADLINE_LARGE;
    pub const HEADER_SUPPORTING_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_SURFACE_VARIANT;
    pub const HEADER_SUPPORTING_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_LARGE;
}
impl DateInputModalTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "DateInputModalTokens",
            name: "ContainerColor",
            value: TokenValue::ColorRole(DateInputModalTokens::CONTAINER_COLOR),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "ContainerElevation",
            value: TokenValue::Dp(DateInputModalTokens::CONTAINER_ELEVATION),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "ContainerHeight",
            value: TokenValue::Dp(DateInputModalTokens::CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "ContainerShape",
            value: TokenValue::ShapeRole(DateInputModalTokens::CONTAINER_SHAPE),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "ContainerSurfaceTintLayerColor",
            value: TokenValue::ColorRole(DateInputModalTokens::CONTAINER_SURFACE_TINT_LAYER_COLOR),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "ContainerWidth",
            value: TokenValue::Dp(DateInputModalTokens::CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "HeaderContainerHeight",
            value: TokenValue::Dp(DateInputModalTokens::HEADER_CONTAINER_HEIGHT),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "HeaderContainerWidth",
            value: TokenValue::Dp(DateInputModalTokens::HEADER_CONTAINER_WIDTH),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "HeaderHeadlineColor",
            value: TokenValue::ColorRole(DateInputModalTokens::HEADER_HEADLINE_COLOR),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "HeaderHeadlineFont",
            value: TokenValue::TypographyRole(DateInputModalTokens::HEADER_HEADLINE_FONT),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "HeaderSupportingTextColor",
            value: TokenValue::ColorRole(DateInputModalTokens::HEADER_SUPPORTING_TEXT_COLOR),
        },
        TokenEntry {
            group: "DateInputModalTokens",
            name: "HeaderSupportingTextFont",
            value: TokenValue::TypographyRole(DateInputModalTokens::HEADER_SUPPORTING_TEXT_FONT),
        },
    ];
}
