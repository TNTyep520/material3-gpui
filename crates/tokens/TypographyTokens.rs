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

use super::{TextStyleToken, TokenEntry, TokenValue, TypeScaleTokens};
use crate::theme::TypeScale;

#[derive(Clone, Copy, Debug, Default)]
pub struct TypographyTokens;
impl TypographyTokens {
    pub const BODY_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_LARGE_FONT,
        weight: TypeScaleTokens::BODY_LARGE_WEIGHT,
        size: TypeScaleTokens::BODY_LARGE_SIZE,
        line_height: TypeScaleTokens::BODY_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_LARGE_TRACKING,
    };
    pub const BODY_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_MEDIUM_FONT,
        weight: TypeScaleTokens::BODY_MEDIUM_WEIGHT,
        size: TypeScaleTokens::BODY_MEDIUM_SIZE,
        line_height: TypeScaleTokens::BODY_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_MEDIUM_TRACKING,
    };
    pub const BODY_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_SMALL_FONT,
        weight: TypeScaleTokens::BODY_SMALL_WEIGHT,
        size: TypeScaleTokens::BODY_SMALL_SIZE,
        line_height: TypeScaleTokens::BODY_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_SMALL_TRACKING,
    };
    pub const DISPLAY_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_LARGE_FONT,
        weight: TypeScaleTokens::DISPLAY_LARGE_WEIGHT,
        size: TypeScaleTokens::DISPLAY_LARGE_SIZE,
        line_height: TypeScaleTokens::DISPLAY_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_LARGE_TRACKING,
    };
    pub const DISPLAY_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_MEDIUM_FONT,
        weight: TypeScaleTokens::DISPLAY_MEDIUM_WEIGHT,
        size: TypeScaleTokens::DISPLAY_MEDIUM_SIZE,
        line_height: TypeScaleTokens::DISPLAY_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_MEDIUM_TRACKING,
    };
    pub const DISPLAY_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_SMALL_FONT,
        weight: TypeScaleTokens::DISPLAY_SMALL_WEIGHT,
        size: TypeScaleTokens::DISPLAY_SMALL_SIZE,
        line_height: TypeScaleTokens::DISPLAY_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_SMALL_TRACKING,
    };
    pub const HEADLINE_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_LARGE_FONT,
        weight: TypeScaleTokens::HEADLINE_LARGE_WEIGHT,
        size: TypeScaleTokens::HEADLINE_LARGE_SIZE,
        line_height: TypeScaleTokens::HEADLINE_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_LARGE_TRACKING,
    };
    pub const HEADLINE_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_MEDIUM_FONT,
        weight: TypeScaleTokens::HEADLINE_MEDIUM_WEIGHT,
        size: TypeScaleTokens::HEADLINE_MEDIUM_SIZE,
        line_height: TypeScaleTokens::HEADLINE_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_MEDIUM_TRACKING,
    };
    pub const HEADLINE_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_SMALL_FONT,
        weight: TypeScaleTokens::HEADLINE_SMALL_WEIGHT,
        size: TypeScaleTokens::HEADLINE_SMALL_SIZE,
        line_height: TypeScaleTokens::HEADLINE_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_SMALL_TRACKING,
    };
    pub const LABEL_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_LARGE_FONT,
        weight: TypeScaleTokens::LABEL_LARGE_WEIGHT,
        size: TypeScaleTokens::LABEL_LARGE_SIZE,
        line_height: TypeScaleTokens::LABEL_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_LARGE_TRACKING,
    };
    pub const LABEL_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_MEDIUM_FONT,
        weight: TypeScaleTokens::LABEL_MEDIUM_WEIGHT,
        size: TypeScaleTokens::LABEL_MEDIUM_SIZE,
        line_height: TypeScaleTokens::LABEL_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_MEDIUM_TRACKING,
    };
    pub const LABEL_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_SMALL_FONT,
        weight: TypeScaleTokens::LABEL_SMALL_WEIGHT,
        size: TypeScaleTokens::LABEL_SMALL_SIZE,
        line_height: TypeScaleTokens::LABEL_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_SMALL_TRACKING,
    };
    pub const TITLE_LARGE: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_LARGE_FONT,
        weight: TypeScaleTokens::TITLE_LARGE_WEIGHT,
        size: TypeScaleTokens::TITLE_LARGE_SIZE,
        line_height: TypeScaleTokens::TITLE_LARGE_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_LARGE_TRACKING,
    };
    pub const TITLE_MEDIUM: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_MEDIUM_FONT,
        weight: TypeScaleTokens::TITLE_MEDIUM_WEIGHT,
        size: TypeScaleTokens::TITLE_MEDIUM_SIZE,
        line_height: TypeScaleTokens::TITLE_MEDIUM_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_MEDIUM_TRACKING,
    };
    pub const TITLE_SMALL: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_SMALL_FONT,
        weight: TypeScaleTokens::TITLE_SMALL_WEIGHT,
        size: TypeScaleTokens::TITLE_SMALL_SIZE,
        line_height: TypeScaleTokens::TITLE_SMALL_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_SMALL_TRACKING,
    };
    pub const BODY_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::BODY_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::BODY_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::BODY_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_LARGE_EMPHASIZED_TRACKING,
    };
    pub const BODY_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const BODY_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::BODY_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::BODY_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::BODY_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::BODY_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::BODY_SMALL_EMPHASIZED_TRACKING,
    };
    pub const DISPLAY_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_TRACKING,
    };
    pub const DISPLAY_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const DISPLAY_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_TRACKING,
    };
    pub const HEADLINE_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_TRACKING,
    };
    pub const HEADLINE_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const HEADLINE_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_TRACKING,
    };
    pub const LABEL_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_LARGE_EMPHASIZED_TRACKING,
    };
    pub const LABEL_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const LABEL_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::LABEL_SMALL_EMPHASIZED_TRACKING,
    };
    pub const TITLE_LARGE_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_FONT,
        weight: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_LARGE_EMPHASIZED_TRACKING,
    };
    pub const TITLE_MEDIUM_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_FONT,
        weight: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_TRACKING,
    };
    pub const TITLE_SMALL_EMPHASIZED: TextStyleToken = TextStyleToken {
        font_family: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_FONT,
        weight: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_WEIGHT,
        size: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_SIZE,
        line_height: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_LINE_HEIGHT,
        tracking: TypeScaleTokens::TITLE_SMALL_EMPHASIZED_TRACKING,
    };
}
impl TypeScale {
    pub fn androidx() -> Self {
        Self {
            body_large: TypographyTokens::BODY_LARGE.type_style(1.0),
            body_medium: TypographyTokens::BODY_MEDIUM.type_style(1.0),
            body_small: TypographyTokens::BODY_SMALL.type_style(1.0),
            display_large: TypographyTokens::DISPLAY_LARGE.type_style(1.0),
            display_medium: TypographyTokens::DISPLAY_MEDIUM.type_style(1.0),
            display_small: TypographyTokens::DISPLAY_SMALL.type_style(1.0),
            headline_large: TypographyTokens::HEADLINE_LARGE.type_style(1.0),
            headline_medium: TypographyTokens::HEADLINE_MEDIUM.type_style(1.0),
            headline_small: TypographyTokens::HEADLINE_SMALL.type_style(1.0),
            label_large: TypographyTokens::LABEL_LARGE.type_style(1.0),
            label_medium: TypographyTokens::LABEL_MEDIUM.type_style(1.0),
            label_small: TypographyTokens::LABEL_SMALL.type_style(1.0),
            title_large: TypographyTokens::TITLE_LARGE.type_style(1.0),
            title_medium: TypographyTokens::TITLE_MEDIUM.type_style(1.0),
            title_small: TypographyTokens::TITLE_SMALL.type_style(1.0),
            body_large_emphasized: TypographyTokens::BODY_LARGE_EMPHASIZED.type_style(1.0),
            body_medium_emphasized: TypographyTokens::BODY_MEDIUM_EMPHASIZED.type_style(1.0),
            body_small_emphasized: TypographyTokens::BODY_SMALL_EMPHASIZED.type_style(1.0),
            display_large_emphasized: TypographyTokens::DISPLAY_LARGE_EMPHASIZED.type_style(1.0),
            display_medium_emphasized: TypographyTokens::DISPLAY_MEDIUM_EMPHASIZED.type_style(1.0),
            display_small_emphasized: TypographyTokens::DISPLAY_SMALL_EMPHASIZED.type_style(1.0),
            headline_large_emphasized: TypographyTokens::HEADLINE_LARGE_EMPHASIZED.type_style(1.0),
            headline_medium_emphasized: TypographyTokens::HEADLINE_MEDIUM_EMPHASIZED
                .type_style(1.0),
            headline_small_emphasized: TypographyTokens::HEADLINE_SMALL_EMPHASIZED.type_style(1.0),
            label_large_emphasized: TypographyTokens::LABEL_LARGE_EMPHASIZED.type_style(1.0),
            label_medium_emphasized: TypographyTokens::LABEL_MEDIUM_EMPHASIZED.type_style(1.0),
            label_small_emphasized: TypographyTokens::LABEL_SMALL_EMPHASIZED.type_style(1.0),
            title_large_emphasized: TypographyTokens::TITLE_LARGE_EMPHASIZED.type_style(1.0),
            title_medium_emphasized: TypographyTokens::TITLE_MEDIUM_EMPHASIZED.type_style(1.0),
            title_small_emphasized: TypographyTokens::TITLE_SMALL_EMPHASIZED.type_style(1.0),
        }
    }
}
impl TypographyTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "TypographyTokens",
            name: "BodyLarge",
            value: TokenValue::TextStyle(TypographyTokens::BODY_LARGE),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "BodyMedium",
            value: TokenValue::TextStyle(TypographyTokens::BODY_MEDIUM),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "BodySmall",
            value: TokenValue::TextStyle(TypographyTokens::BODY_SMALL),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "DisplayLarge",
            value: TokenValue::TextStyle(TypographyTokens::DISPLAY_LARGE),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "DisplayMedium",
            value: TokenValue::TextStyle(TypographyTokens::DISPLAY_MEDIUM),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "DisplaySmall",
            value: TokenValue::TextStyle(TypographyTokens::DISPLAY_SMALL),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "HeadlineLarge",
            value: TokenValue::TextStyle(TypographyTokens::HEADLINE_LARGE),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "HeadlineMedium",
            value: TokenValue::TextStyle(TypographyTokens::HEADLINE_MEDIUM),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "HeadlineSmall",
            value: TokenValue::TextStyle(TypographyTokens::HEADLINE_SMALL),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "LabelLarge",
            value: TokenValue::TextStyle(TypographyTokens::LABEL_LARGE),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "LabelMedium",
            value: TokenValue::TextStyle(TypographyTokens::LABEL_MEDIUM),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "LabelSmall",
            value: TokenValue::TextStyle(TypographyTokens::LABEL_SMALL),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "TitleLarge",
            value: TokenValue::TextStyle(TypographyTokens::TITLE_LARGE),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "TitleMedium",
            value: TokenValue::TextStyle(TypographyTokens::TITLE_MEDIUM),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "TitleSmall",
            value: TokenValue::TextStyle(TypographyTokens::TITLE_SMALL),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "BodyLargeEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::BODY_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "BodyMediumEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::BODY_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "BodySmallEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::BODY_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "DisplayLargeEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::DISPLAY_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "DisplayMediumEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::DISPLAY_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "DisplaySmallEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::DISPLAY_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "HeadlineLargeEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::HEADLINE_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "HeadlineMediumEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::HEADLINE_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "HeadlineSmallEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::HEADLINE_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "LabelLargeEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::LABEL_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "LabelMediumEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::LABEL_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "LabelSmallEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::LABEL_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "TitleLargeEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::TITLE_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "TitleMediumEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::TITLE_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyTokens",
            name: "TitleSmallEmphasized",
            value: TokenValue::TextStyle(TypographyTokens::TITLE_SMALL_EMPHASIZED),
        },
    ];
}
