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
pub struct BadgeTokens;
impl BadgeTokens {
    pub const COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const LARGE_COLOR: ColorToken = ColorSchemeKeyTokens::ERROR;
    pub const LARGE_LABEL_TEXT_COLOR: ColorToken = ColorSchemeKeyTokens::ON_ERROR;
    pub const LARGE_LABEL_TEXT_FONT: TypographyToken = TypographyKeyTokens::LABEL_SMALL;
    pub const LARGE_SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const LARGE_SIZE: Dp = Dp(16.0);
    pub const SHAPE: ShapeToken = ShapeKeyTokens::CORNER_FULL;
    pub const SIZE: Dp = Dp(6.0);
}
impl BadgeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "BadgeTokens",
            name: "Color",
            value: TokenValue::ColorRole(BadgeTokens::COLOR),
        },
        TokenEntry {
            group: "BadgeTokens",
            name: "LargeColor",
            value: TokenValue::ColorRole(BadgeTokens::LARGE_COLOR),
        },
        TokenEntry {
            group: "BadgeTokens",
            name: "LargeLabelTextColor",
            value: TokenValue::ColorRole(BadgeTokens::LARGE_LABEL_TEXT_COLOR),
        },
        TokenEntry {
            group: "BadgeTokens",
            name: "LargeLabelTextFont",
            value: TokenValue::TypographyRole(BadgeTokens::LARGE_LABEL_TEXT_FONT),
        },
        TokenEntry {
            group: "BadgeTokens",
            name: "LargeShape",
            value: TokenValue::ShapeRole(BadgeTokens::LARGE_SHAPE),
        },
        TokenEntry {
            group: "BadgeTokens",
            name: "LargeSize",
            value: TokenValue::Dp(BadgeTokens::LARGE_SIZE),
        },
        TokenEntry {
            group: "BadgeTokens",
            name: "Shape",
            value: TokenValue::ShapeRole(BadgeTokens::SHAPE),
        },
        TokenEntry {
            group: "BadgeTokens",
            name: "Size",
            value: TokenValue::Dp(BadgeTokens::SIZE),
        },
    ];
}
