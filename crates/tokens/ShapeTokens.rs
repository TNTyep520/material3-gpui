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

#[derive(Clone, Copy, Debug, Default)]
pub struct ShapeTokens;
impl ShapeTokens {
    pub const CORNER_EXTRA_EXTRA_LARGE: ShapeValue = ShapeValue::rounded(48.0);
    pub const CORNER_EXTRA_LARGE: ShapeValue = ShapeValue::rounded(28.0);
    pub const CORNER_EXTRA_LARGE_INCREASED: ShapeValue = ShapeValue::rounded(32.0);
    pub const CORNER_EXTRA_LARGE_TOP: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(28.0),
        top_end: Dp(28.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_EXTRA_SMALL: ShapeValue = ShapeValue::rounded(4.0);
    pub const CORNER_EXTRA_SMALL_TOP: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(4.0),
        top_end: Dp(4.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_FULL: ShapeValue = ShapeValue::Full;
    pub const CORNER_LARGE: ShapeValue = ShapeValue::rounded(16.0);
    pub const CORNER_LARGE_END: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(0.0),
        top_end: Dp(16.0),
        bottom_end: Dp(16.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_LARGE_INCREASED: ShapeValue = ShapeValue::rounded(20.0);
    pub const CORNER_LARGE_START: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(16.0),
        top_end: Dp(0.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(16.0),
    };
    pub const CORNER_LARGE_TOP: ShapeValue = ShapeValue::Rounded {
        top_start: Dp(16.0),
        top_end: Dp(16.0),
        bottom_end: Dp(0.0),
        bottom_start: Dp(0.0),
    };
    pub const CORNER_MEDIUM: ShapeValue = ShapeValue::rounded(12.0);
    pub const CORNER_NONE: ShapeValue = ShapeValue::rounded(0.0);
    pub const CORNER_SMALL: ShapeValue = ShapeValue::rounded(8.0);
    pub const CORNER_VALUE_EXTRA_EXTRA_LARGE: Dp = Dp(48.0);
    pub const CORNER_VALUE_EXTRA_LARGE: Dp = Dp(28.0);
    pub const CORNER_VALUE_EXTRA_LARGE_INCREASED: Dp = Dp(32.0);
    pub const CORNER_VALUE_EXTRA_SMALL: Dp = Dp(4.0);
    pub const CORNER_VALUE_LARGE: Dp = Dp(16.0);
    pub const CORNER_VALUE_LARGE_INCREASED: Dp = Dp(20.0);
    pub const CORNER_VALUE_MEDIUM: Dp = Dp(12.0);
    pub const CORNER_VALUE_NONE: Dp = Dp(0.0);
    pub const CORNER_VALUE_SMALL: Dp = Dp(8.0);
}
impl ShapeTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerExtraExtraLarge",
            value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_EXTRA_LARGE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerExtraLarge",
            value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_LARGE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerExtraLargeIncreased",
            value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_LARGE_INCREASED),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerExtraLargeTop",
            value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_LARGE_TOP),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerExtraSmall",
            value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_SMALL),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerExtraSmallTop",
            value: TokenValue::Shape(ShapeTokens::CORNER_EXTRA_SMALL_TOP),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerFull",
            value: TokenValue::Shape(ShapeTokens::CORNER_FULL),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerLarge",
            value: TokenValue::Shape(ShapeTokens::CORNER_LARGE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerLargeEnd",
            value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_END),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerLargeIncreased",
            value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_INCREASED),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerLargeStart",
            value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_START),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerLargeTop",
            value: TokenValue::Shape(ShapeTokens::CORNER_LARGE_TOP),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerMedium",
            value: TokenValue::Shape(ShapeTokens::CORNER_MEDIUM),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerNone",
            value: TokenValue::Shape(ShapeTokens::CORNER_NONE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerSmall",
            value: TokenValue::Shape(ShapeTokens::CORNER_SMALL),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueExtraExtraLarge",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_EXTRA_LARGE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueExtraLarge",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_LARGE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueExtraLargeIncreased",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_LARGE_INCREASED),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueExtraSmall",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_EXTRA_SMALL),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueLarge",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_LARGE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueLargeIncreased",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_LARGE_INCREASED),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueMedium",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_MEDIUM),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueNone",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_NONE),
        },
        TokenEntry {
            group: "ShapeTokens",
            name: "CornerValueSmall",
            value: TokenValue::Dp(ShapeTokens::CORNER_VALUE_SMALL),
        },
    ];
}
