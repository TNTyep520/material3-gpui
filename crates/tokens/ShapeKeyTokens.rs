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

use super::{Dp, ShapeValue, TokenEntry, TokenValue};
use crate::theme::TokenSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ShapeToken {
    CornerExtraExtraLarge,
    CornerExtraLarge,
    CornerExtraLargeIncreased,
    CornerExtraLargeTop,
    CornerExtraSmall,
    CornerExtraSmallTop,
    CornerFull,
    CornerLarge,
    CornerLargeEnd,
    CornerLargeIncreased,
    CornerLargeStart,
    CornerLargeTop,
    CornerMedium,
    CornerNone,
    CornerSmall,
}
impl ShapeToken {
    pub const ALL: &'static [Self] = &[
        Self::CornerExtraExtraLarge,
        Self::CornerExtraLarge,
        Self::CornerExtraLargeIncreased,
        Self::CornerExtraLargeTop,
        Self::CornerExtraSmall,
        Self::CornerExtraSmallTop,
        Self::CornerFull,
        Self::CornerLarge,
        Self::CornerLargeEnd,
        Self::CornerLargeIncreased,
        Self::CornerLargeStart,
        Self::CornerLargeTop,
        Self::CornerMedium,
        Self::CornerNone,
        Self::CornerSmall,
    ];
    pub const fn id(self) -> u16 {
        match self {
            Self::CornerExtraExtraLarge => 0,
            Self::CornerExtraLarge => 1,
            Self::CornerExtraLargeIncreased => 2,
            Self::CornerExtraLargeTop => 3,
            Self::CornerExtraSmall => 4,
            Self::CornerExtraSmallTop => 5,
            Self::CornerFull => 6,
            Self::CornerLarge => 7,
            Self::CornerLargeEnd => 8,
            Self::CornerLargeIncreased => 9,
            Self::CornerLargeStart => 10,
            Self::CornerLargeTop => 11,
            Self::CornerMedium => 12,
            Self::CornerNone => 13,
            Self::CornerSmall => 14,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeKeyTokens;
impl ShapeKeyTokens {
    pub const CORNER_EXTRA_EXTRA_LARGE: ShapeToken = ShapeToken::CornerExtraExtraLarge;
    pub const CORNER_EXTRA_LARGE: ShapeToken = ShapeToken::CornerExtraLarge;
    pub const CORNER_EXTRA_LARGE_INCREASED: ShapeToken = ShapeToken::CornerExtraLargeIncreased;
    pub const CORNER_EXTRA_LARGE_TOP: ShapeToken = ShapeToken::CornerExtraLargeTop;
    pub const CORNER_EXTRA_SMALL: ShapeToken = ShapeToken::CornerExtraSmall;
    pub const CORNER_EXTRA_SMALL_TOP: ShapeToken = ShapeToken::CornerExtraSmallTop;
    pub const CORNER_FULL: ShapeToken = ShapeToken::CornerFull;
    pub const CORNER_LARGE: ShapeToken = ShapeToken::CornerLarge;
    pub const CORNER_LARGE_END: ShapeToken = ShapeToken::CornerLargeEnd;
    pub const CORNER_LARGE_INCREASED: ShapeToken = ShapeToken::CornerLargeIncreased;
    pub const CORNER_LARGE_START: ShapeToken = ShapeToken::CornerLargeStart;
    pub const CORNER_LARGE_TOP: ShapeToken = ShapeToken::CornerLargeTop;
    pub const CORNER_MEDIUM: ShapeToken = ShapeToken::CornerMedium;
    pub const CORNER_NONE: ShapeToken = ShapeToken::CornerNone;
    pub const CORNER_SMALL: ShapeToken = ShapeToken::CornerSmall;
}
impl ShapeToken {
    pub fn resolve(self, tokens: &TokenSet) -> ShapeValue {
        match self {
            Self::CornerExtraExtraLarge => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_extra_large)),
                top_end: Dp(f32::from(tokens.shapes.extra_extra_large)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_extra_large)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_extra_large)),
            },
            Self::CornerExtraLarge => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_large)),
                top_end: Dp(f32::from(tokens.shapes.extra_large)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_large)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_large)),
            },
            Self::CornerExtraLargeIncreased => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_large_increased)),
                top_end: Dp(f32::from(tokens.shapes.extra_large_increased)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_large_increased)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_large_increased)),
            },
            Self::CornerExtraLargeTop => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_large)),
                top_end: Dp(f32::from(tokens.shapes.extra_large)),
                bottom_end: Dp(0.0),
                bottom_start: Dp(0.0),
            },
            Self::CornerExtraSmall => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_small)),
                top_end: Dp(f32::from(tokens.shapes.extra_small)),
                bottom_end: Dp(f32::from(tokens.shapes.extra_small)),
                bottom_start: Dp(f32::from(tokens.shapes.extra_small)),
            },
            Self::CornerExtraSmallTop => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.extra_small)),
                top_end: Dp(f32::from(tokens.shapes.extra_small)),
                bottom_end: Dp(0.0),
                bottom_start: Dp(0.0),
            },
            Self::CornerFull => ShapeValue::Full,
            Self::CornerLarge => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large)),
                top_end: Dp(f32::from(tokens.shapes.large)),
                bottom_end: Dp(f32::from(tokens.shapes.large)),
                bottom_start: Dp(f32::from(tokens.shapes.large)),
            },
            Self::CornerLargeEnd => ShapeValue::Rounded {
                top_start: Dp(0.0),
                top_end: Dp(f32::from(tokens.shapes.large)),
                bottom_end: Dp(f32::from(tokens.shapes.large)),
                bottom_start: Dp(0.0),
            },
            Self::CornerLargeIncreased => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large_increased)),
                top_end: Dp(f32::from(tokens.shapes.large_increased)),
                bottom_end: Dp(f32::from(tokens.shapes.large_increased)),
                bottom_start: Dp(f32::from(tokens.shapes.large_increased)),
            },
            Self::CornerLargeStart => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large)),
                top_end: Dp(0.0),
                bottom_end: Dp(0.0),
                bottom_start: Dp(f32::from(tokens.shapes.large)),
            },
            Self::CornerLargeTop => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.large)),
                top_end: Dp(f32::from(tokens.shapes.large)),
                bottom_end: Dp(0.0),
                bottom_start: Dp(0.0),
            },
            Self::CornerMedium => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.medium)),
                top_end: Dp(f32::from(tokens.shapes.medium)),
                bottom_end: Dp(f32::from(tokens.shapes.medium)),
                bottom_start: Dp(f32::from(tokens.shapes.medium)),
            },
            Self::CornerNone => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.none)),
                top_end: Dp(f32::from(tokens.shapes.none)),
                bottom_end: Dp(f32::from(tokens.shapes.none)),
                bottom_start: Dp(f32::from(tokens.shapes.none)),
            },
            Self::CornerSmall => ShapeValue::Rounded {
                top_start: Dp(f32::from(tokens.shapes.small)),
                top_end: Dp(f32::from(tokens.shapes.small)),
                bottom_end: Dp(f32::from(tokens.shapes.small)),
                bottom_start: Dp(f32::from(tokens.shapes.small)),
            },
        }
    }
}
impl ShapeKeyTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerExtraExtraLarge",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_EXTRA_LARGE),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerExtraLarge",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_LARGE),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerExtraLargeIncreased",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_LARGE_INCREASED),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerExtraLargeTop",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_LARGE_TOP),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerExtraSmall",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_SMALL),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerExtraSmallTop",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_EXTRA_SMALL_TOP),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerFull",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_FULL),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerLarge",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerLargeEnd",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_END),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerLargeIncreased",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_INCREASED),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerLargeStart",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_START),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerLargeTop",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_LARGE_TOP),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerMedium",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_MEDIUM),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerNone",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_NONE),
        },
        TokenEntry {
            group: "ShapeKeyTokens",
            name: "CornerSmall",
            value: TokenValue::ShapeRole(ShapeKeyTokens::CORNER_SMALL),
        },
    ];
}
