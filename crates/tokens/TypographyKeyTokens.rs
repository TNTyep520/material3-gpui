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

use super::{TokenEntry, TokenValue};
use crate::theme::{TokenSet, TypeStyle};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypographyToken {
    BodyLarge,
    BodyMedium,
    BodySmall,
    DisplayLarge,
    DisplayMedium,
    DisplaySmall,
    HeadlineLarge,
    HeadlineMedium,
    HeadlineSmall,
    LabelLarge,
    LabelMedium,
    LabelSmall,
    TitleLarge,
    TitleMedium,
    TitleSmall,
    BodyLargeEmphasized,
    BodyMediumEmphasized,
    BodySmallEmphasized,
    DisplayLargeEmphasized,
    DisplayMediumEmphasized,
    DisplaySmallEmphasized,
    HeadlineLargeEmphasized,
    HeadlineMediumEmphasized,
    HeadlineSmallEmphasized,
    LabelLargeEmphasized,
    LabelMediumEmphasized,
    LabelSmallEmphasized,
    TitleLargeEmphasized,
    TitleMediumEmphasized,
    TitleSmallEmphasized,
}
impl TypographyToken {
    pub const ALL: &'static [Self] = &[
        Self::BodyLarge,
        Self::BodyMedium,
        Self::BodySmall,
        Self::DisplayLarge,
        Self::DisplayMedium,
        Self::DisplaySmall,
        Self::HeadlineLarge,
        Self::HeadlineMedium,
        Self::HeadlineSmall,
        Self::LabelLarge,
        Self::LabelMedium,
        Self::LabelSmall,
        Self::TitleLarge,
        Self::TitleMedium,
        Self::TitleSmall,
        Self::BodyLargeEmphasized,
        Self::BodyMediumEmphasized,
        Self::BodySmallEmphasized,
        Self::DisplayLargeEmphasized,
        Self::DisplayMediumEmphasized,
        Self::DisplaySmallEmphasized,
        Self::HeadlineLargeEmphasized,
        Self::HeadlineMediumEmphasized,
        Self::HeadlineSmallEmphasized,
        Self::LabelLargeEmphasized,
        Self::LabelMediumEmphasized,
        Self::LabelSmallEmphasized,
        Self::TitleLargeEmphasized,
        Self::TitleMediumEmphasized,
        Self::TitleSmallEmphasized,
    ];
    pub const fn id(self) -> u16 {
        match self {
            Self::BodyLarge => 0,
            Self::BodyMedium => 1,
            Self::BodySmall => 2,
            Self::DisplayLarge => 3,
            Self::DisplayMedium => 4,
            Self::DisplaySmall => 5,
            Self::HeadlineLarge => 6,
            Self::HeadlineMedium => 7,
            Self::HeadlineSmall => 8,
            Self::LabelLarge => 9,
            Self::LabelMedium => 10,
            Self::LabelSmall => 11,
            Self::TitleLarge => 12,
            Self::TitleMedium => 13,
            Self::TitleSmall => 14,
            Self::BodyLargeEmphasized => 15,
            Self::BodyMediumEmphasized => 16,
            Self::BodySmallEmphasized => 17,
            Self::DisplayLargeEmphasized => 18,
            Self::DisplayMediumEmphasized => 19,
            Self::DisplaySmallEmphasized => 20,
            Self::HeadlineLargeEmphasized => 21,
            Self::HeadlineMediumEmphasized => 22,
            Self::HeadlineSmallEmphasized => 23,
            Self::LabelLargeEmphasized => 24,
            Self::LabelMediumEmphasized => 25,
            Self::LabelSmallEmphasized => 26,
            Self::TitleLargeEmphasized => 27,
            Self::TitleMediumEmphasized => 28,
            Self::TitleSmallEmphasized => 29,
        }
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct TypographyKeyTokens;
impl TypographyKeyTokens {
    pub const BODY_LARGE: TypographyToken = TypographyToken::BodyLarge;
    pub const BODY_MEDIUM: TypographyToken = TypographyToken::BodyMedium;
    pub const BODY_SMALL: TypographyToken = TypographyToken::BodySmall;
    pub const DISPLAY_LARGE: TypographyToken = TypographyToken::DisplayLarge;
    pub const DISPLAY_MEDIUM: TypographyToken = TypographyToken::DisplayMedium;
    pub const DISPLAY_SMALL: TypographyToken = TypographyToken::DisplaySmall;
    pub const HEADLINE_LARGE: TypographyToken = TypographyToken::HeadlineLarge;
    pub const HEADLINE_MEDIUM: TypographyToken = TypographyToken::HeadlineMedium;
    pub const HEADLINE_SMALL: TypographyToken = TypographyToken::HeadlineSmall;
    pub const LABEL_LARGE: TypographyToken = TypographyToken::LabelLarge;
    pub const LABEL_MEDIUM: TypographyToken = TypographyToken::LabelMedium;
    pub const LABEL_SMALL: TypographyToken = TypographyToken::LabelSmall;
    pub const TITLE_LARGE: TypographyToken = TypographyToken::TitleLarge;
    pub const TITLE_MEDIUM: TypographyToken = TypographyToken::TitleMedium;
    pub const TITLE_SMALL: TypographyToken = TypographyToken::TitleSmall;
    pub const BODY_LARGE_EMPHASIZED: TypographyToken = TypographyToken::BodyLargeEmphasized;
    pub const BODY_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::BodyMediumEmphasized;
    pub const BODY_SMALL_EMPHASIZED: TypographyToken = TypographyToken::BodySmallEmphasized;
    pub const DISPLAY_LARGE_EMPHASIZED: TypographyToken = TypographyToken::DisplayLargeEmphasized;
    pub const DISPLAY_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::DisplayMediumEmphasized;
    pub const DISPLAY_SMALL_EMPHASIZED: TypographyToken = TypographyToken::DisplaySmallEmphasized;
    pub const HEADLINE_LARGE_EMPHASIZED: TypographyToken = TypographyToken::HeadlineLargeEmphasized;
    pub const HEADLINE_MEDIUM_EMPHASIZED: TypographyToken =
        TypographyToken::HeadlineMediumEmphasized;
    pub const HEADLINE_SMALL_EMPHASIZED: TypographyToken = TypographyToken::HeadlineSmallEmphasized;
    pub const LABEL_LARGE_EMPHASIZED: TypographyToken = TypographyToken::LabelLargeEmphasized;
    pub const LABEL_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::LabelMediumEmphasized;
    pub const LABEL_SMALL_EMPHASIZED: TypographyToken = TypographyToken::LabelSmallEmphasized;
    pub const TITLE_LARGE_EMPHASIZED: TypographyToken = TypographyToken::TitleLargeEmphasized;
    pub const TITLE_MEDIUM_EMPHASIZED: TypographyToken = TypographyToken::TitleMediumEmphasized;
    pub const TITLE_SMALL_EMPHASIZED: TypographyToken = TypographyToken::TitleSmallEmphasized;
}
impl TypographyToken {
    pub fn resolve(self, tokens: &TokenSet) -> TypeStyle {
        match self {
            Self::BodyLarge => tokens.typography.body_large,
            Self::BodyMedium => tokens.typography.body_medium,
            Self::BodySmall => tokens.typography.body_small,
            Self::DisplayLarge => tokens.typography.display_large,
            Self::DisplayMedium => tokens.typography.display_medium,
            Self::DisplaySmall => tokens.typography.display_small,
            Self::HeadlineLarge => tokens.typography.headline_large,
            Self::HeadlineMedium => tokens.typography.headline_medium,
            Self::HeadlineSmall => tokens.typography.headline_small,
            Self::LabelLarge => tokens.typography.label_large,
            Self::LabelMedium => tokens.typography.label_medium,
            Self::LabelSmall => tokens.typography.label_small,
            Self::TitleLarge => tokens.typography.title_large,
            Self::TitleMedium => tokens.typography.title_medium,
            Self::TitleSmall => tokens.typography.title_small,
            Self::BodyLargeEmphasized => tokens.typography.body_large_emphasized,
            Self::BodyMediumEmphasized => tokens.typography.body_medium_emphasized,
            Self::BodySmallEmphasized => tokens.typography.body_small_emphasized,
            Self::DisplayLargeEmphasized => tokens.typography.display_large_emphasized,
            Self::DisplayMediumEmphasized => tokens.typography.display_medium_emphasized,
            Self::DisplaySmallEmphasized => tokens.typography.display_small_emphasized,
            Self::HeadlineLargeEmphasized => tokens.typography.headline_large_emphasized,
            Self::HeadlineMediumEmphasized => tokens.typography.headline_medium_emphasized,
            Self::HeadlineSmallEmphasized => tokens.typography.headline_small_emphasized,
            Self::LabelLargeEmphasized => tokens.typography.label_large_emphasized,
            Self::LabelMediumEmphasized => tokens.typography.label_medium_emphasized,
            Self::LabelSmallEmphasized => tokens.typography.label_small_emphasized,
            Self::TitleLargeEmphasized => tokens.typography.title_large_emphasized,
            Self::TitleMediumEmphasized => tokens.typography.title_medium_emphasized,
            Self::TitleSmallEmphasized => tokens.typography.title_small_emphasized,
        }
    }
}
impl TypographyKeyTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "BodyLarge",
            value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_LARGE),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "BodyMedium",
            value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_MEDIUM),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "BodySmall",
            value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_SMALL),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "DisplayLarge",
            value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_LARGE),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "DisplayMedium",
            value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_MEDIUM),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "DisplaySmall",
            value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_SMALL),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "HeadlineLarge",
            value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_LARGE),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "HeadlineMedium",
            value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_MEDIUM),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "HeadlineSmall",
            value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_SMALL),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "LabelLarge",
            value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_LARGE),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "LabelMedium",
            value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_MEDIUM),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "LabelSmall",
            value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_SMALL),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "TitleLarge",
            value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_LARGE),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "TitleMedium",
            value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_MEDIUM),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "TitleSmall",
            value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_SMALL),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "BodyLargeEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "BodyMediumEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "BodySmallEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::BODY_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "DisplayLargeEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "DisplayMediumEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "DisplaySmallEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::DISPLAY_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "HeadlineLargeEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "HeadlineMediumEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "HeadlineSmallEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::HEADLINE_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "LabelLargeEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "LabelMediumEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "LabelSmallEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::LABEL_SMALL_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "TitleLargeEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_LARGE_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "TitleMediumEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_MEDIUM_EMPHASIZED),
        },
        TokenEntry {
            group: "TypographyKeyTokens",
            name: "TitleSmallEmphasized",
            value: TokenValue::TypographyRole(TypographyKeyTokens::TITLE_SMALL_EMPHASIZED),
        },
    ];
}
