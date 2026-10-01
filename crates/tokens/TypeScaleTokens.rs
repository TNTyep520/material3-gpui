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

use super::{FontFamilyToken, Sp, TokenEntry, TokenValue, TypefaceTokens};
use gpui::FontWeight;

#[derive(Clone, Copy, Debug, Default)]
pub struct TypeScaleTokens;
impl TypeScaleTokens {
    pub const BODY_LARGE_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_LARGE_LINE_HEIGHT: Sp = Sp(24.0);
    pub const BODY_LARGE_SIZE: Sp = Sp(16.0);
    pub const BODY_LARGE_TRACKING: Sp = Sp(0.5);
    pub const BODY_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const BODY_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_MEDIUM_LINE_HEIGHT: Sp = Sp(20.0);
    pub const BODY_MEDIUM_SIZE: Sp = Sp(14.0);
    pub const BODY_MEDIUM_TRACKING: Sp = Sp(0.2);
    pub const BODY_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const BODY_SMALL_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_SMALL_LINE_HEIGHT: Sp = Sp(16.0);
    pub const BODY_SMALL_SIZE: Sp = Sp(12.0);
    pub const BODY_SMALL_TRACKING: Sp = Sp(0.4);
    pub const BODY_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const DISPLAY_LARGE_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_LARGE_LINE_HEIGHT: Sp = Sp(64.0);
    pub const DISPLAY_LARGE_SIZE: Sp = Sp(57.0);
    pub const DISPLAY_LARGE_TRACKING: Sp = Sp(-0.2);
    pub const DISPLAY_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const DISPLAY_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_MEDIUM_LINE_HEIGHT: Sp = Sp(52.0);
    pub const DISPLAY_MEDIUM_SIZE: Sp = Sp(45.0);
    pub const DISPLAY_MEDIUM_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const DISPLAY_SMALL_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_SMALL_LINE_HEIGHT: Sp = Sp(44.0);
    pub const DISPLAY_SMALL_SIZE: Sp = Sp(36.0);
    pub const DISPLAY_SMALL_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const HEADLINE_LARGE_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_LARGE_LINE_HEIGHT: Sp = Sp(40.0);
    pub const HEADLINE_LARGE_SIZE: Sp = Sp(32.0);
    pub const HEADLINE_LARGE_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const HEADLINE_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_MEDIUM_LINE_HEIGHT: Sp = Sp(36.0);
    pub const HEADLINE_MEDIUM_SIZE: Sp = Sp(28.0);
    pub const HEADLINE_MEDIUM_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const HEADLINE_SMALL_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_SMALL_LINE_HEIGHT: Sp = Sp(32.0);
    pub const HEADLINE_SMALL_SIZE: Sp = Sp(24.0);
    pub const HEADLINE_SMALL_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const LABEL_LARGE_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_LARGE_LINE_HEIGHT: Sp = Sp(20.0);
    pub const LABEL_LARGE_SIZE: Sp = Sp(14.0);
    pub const LABEL_LARGE_TRACKING: Sp = Sp(0.1);
    pub const LABEL_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const LABEL_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_MEDIUM_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_MEDIUM_SIZE: Sp = Sp(12.0);
    pub const LABEL_MEDIUM_TRACKING: Sp = Sp(0.5);
    pub const LABEL_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const LABEL_SMALL_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_SMALL_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_SMALL_SIZE: Sp = Sp(11.0);
    pub const LABEL_SMALL_TRACKING: Sp = Sp(0.5);
    pub const LABEL_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const TITLE_LARGE_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const TITLE_LARGE_LINE_HEIGHT: Sp = Sp(28.0);
    pub const TITLE_LARGE_SIZE: Sp = Sp(22.0);
    pub const TITLE_LARGE_TRACKING: Sp = Sp(0.0);
    pub const TITLE_LARGE_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_REGULAR;
    pub const TITLE_MEDIUM_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_MEDIUM_LINE_HEIGHT: Sp = Sp(24.0);
    pub const TITLE_MEDIUM_SIZE: Sp = Sp(16.0);
    pub const TITLE_MEDIUM_TRACKING: Sp = Sp(0.2);
    pub const TITLE_MEDIUM_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const TITLE_SMALL_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_SMALL_LINE_HEIGHT: Sp = Sp(20.0);
    pub const TITLE_SMALL_SIZE: Sp = Sp(14.0);
    pub const TITLE_SMALL_TRACKING: Sp = Sp(0.1);
    pub const TITLE_SMALL_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const BODY_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(24.0);
    pub const BODY_LARGE_EMPHASIZED_SIZE: Sp = Sp(16.0);
    pub const BODY_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.15);
    pub const BODY_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const BODY_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(20.0);
    pub const BODY_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(14.0);
    pub const BODY_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.25);
    pub const BODY_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const BODY_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const BODY_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(16.0);
    pub const BODY_SMALL_EMPHASIZED_SIZE: Sp = Sp(12.0);
    pub const BODY_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.4);
    pub const BODY_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const DISPLAY_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(64.0);
    pub const DISPLAY_LARGE_EMPHASIZED_SIZE: Sp = Sp(57.0);
    pub const DISPLAY_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const DISPLAY_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(52.0);
    pub const DISPLAY_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(45.0);
    pub const DISPLAY_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const DISPLAY_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const DISPLAY_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(44.0);
    pub const DISPLAY_SMALL_EMPHASIZED_SIZE: Sp = Sp(36.0);
    pub const DISPLAY_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const DISPLAY_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const HEADLINE_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(40.0);
    pub const HEADLINE_LARGE_EMPHASIZED_SIZE: Sp = Sp(32.0);
    pub const HEADLINE_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const HEADLINE_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(36.0);
    pub const HEADLINE_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(28.0);
    pub const HEADLINE_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const HEADLINE_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const HEADLINE_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(32.0);
    pub const HEADLINE_SMALL_EMPHASIZED_SIZE: Sp = Sp(24.0);
    pub const HEADLINE_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const HEADLINE_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const LABEL_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(20.0);
    pub const LABEL_LARGE_EMPHASIZED_SIZE: Sp = Sp(14.0);
    pub const LABEL_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.1);
    pub const LABEL_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const LABEL_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(12.0);
    pub const LABEL_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.5);
    pub const LABEL_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const LABEL_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const LABEL_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(16.0);
    pub const LABEL_SMALL_EMPHASIZED_SIZE: Sp = Sp(11.0);
    pub const LABEL_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.5);
    pub const LABEL_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const TITLE_LARGE_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::BRAND;
    pub const TITLE_LARGE_EMPHASIZED_LINE_HEIGHT: Sp = Sp(28.0);
    pub const TITLE_LARGE_EMPHASIZED_SIZE: Sp = Sp(22.0);
    pub const TITLE_LARGE_EMPHASIZED_TRACKING: Sp = Sp(0.0);
    pub const TITLE_LARGE_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_MEDIUM;
    pub const TITLE_MEDIUM_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_MEDIUM_EMPHASIZED_LINE_HEIGHT: Sp = Sp(24.0);
    pub const TITLE_MEDIUM_EMPHASIZED_SIZE: Sp = Sp(16.0);
    pub const TITLE_MEDIUM_EMPHASIZED_TRACKING: Sp = Sp(0.15);
    pub const TITLE_MEDIUM_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
    pub const TITLE_SMALL_EMPHASIZED_FONT: FontFamilyToken = TypefaceTokens::PLAIN;
    pub const TITLE_SMALL_EMPHASIZED_LINE_HEIGHT: Sp = Sp(20.0);
    pub const TITLE_SMALL_EMPHASIZED_SIZE: Sp = Sp(14.0);
    pub const TITLE_SMALL_EMPHASIZED_TRACKING: Sp = Sp(0.1);
    pub const TITLE_SMALL_EMPHASIZED_WEIGHT: FontWeight = TypefaceTokens::WEIGHT_BOLD;
}
impl TypeScaleTokens {
    pub(super) const ENTRIES: &'static [TokenEntry] = &[
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeFont",
            value: TokenValue::FontFamily(TypeScaleTokens::BODY_LARGE_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeSize",
            value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeTracking",
            value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::BODY_LARGE_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumFont",
            value: TokenValue::FontFamily(TypeScaleTokens::BODY_MEDIUM_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumSize",
            value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumTracking",
            value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::BODY_MEDIUM_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallFont",
            value: TokenValue::FontFamily(TypeScaleTokens::BODY_SMALL_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallSize",
            value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallTracking",
            value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::BODY_SMALL_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeFont",
            value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_LARGE_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeSize",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeTracking",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_LARGE_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumFont",
            value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_MEDIUM_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumSize",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumTracking",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_MEDIUM_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallFont",
            value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_SMALL_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallSize",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallTracking",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_SMALL_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeFont",
            value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_LARGE_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeSize",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeTracking",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_LARGE_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumFont",
            value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_MEDIUM_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumSize",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumTracking",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_MEDIUM_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallFont",
            value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_SMALL_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallSize",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallTracking",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_SMALL_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeFont",
            value: TokenValue::FontFamily(TypeScaleTokens::LABEL_LARGE_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeSize",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeTracking",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::LABEL_LARGE_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumFont",
            value: TokenValue::FontFamily(TypeScaleTokens::LABEL_MEDIUM_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumSize",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumTracking",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::LABEL_MEDIUM_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallFont",
            value: TokenValue::FontFamily(TypeScaleTokens::LABEL_SMALL_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallSize",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallTracking",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::LABEL_SMALL_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeFont",
            value: TokenValue::FontFamily(TypeScaleTokens::TITLE_LARGE_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeSize",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeTracking",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::TITLE_LARGE_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumFont",
            value: TokenValue::FontFamily(TypeScaleTokens::TITLE_MEDIUM_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumSize",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumTracking",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::TITLE_MEDIUM_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallFont",
            value: TokenValue::FontFamily(TypeScaleTokens::TITLE_SMALL_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallSize",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallTracking",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::TITLE_SMALL_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::BODY_LARGE_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::BODY_LARGE_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyLargeEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::BODY_LARGE_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodyMediumEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::BODY_MEDIUM_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::BODY_SMALL_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::BODY_SMALL_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "BodySmallEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::BODY_SMALL_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayLargeEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_LARGE_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplayMediumEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_MEDIUM_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "DisplaySmallEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::DISPLAY_SMALL_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineLargeEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_LARGE_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineMediumEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_MEDIUM_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "HeadlineSmallEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::HEADLINE_SMALL_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelLargeEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::LABEL_LARGE_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelMediumEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::LABEL_MEDIUM_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "LabelSmallEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::LABEL_SMALL_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleLargeEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::TITLE_LARGE_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleMediumEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::TITLE_MEDIUM_EMPHASIZED_WEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallEmphasizedFont",
            value: TokenValue::FontFamily(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_FONT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallEmphasizedLineHeight",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_LINE_HEIGHT),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallEmphasizedSize",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_SIZE),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallEmphasizedTracking",
            value: TokenValue::Sp(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_TRACKING),
        },
        TokenEntry {
            group: "TypeScaleTokens",
            name: "TitleSmallEmphasizedWeight",
            value: TokenValue::FontWeight(TypeScaleTokens::TITLE_SMALL_EMPHASIZED_WEIGHT),
        },
    ];
}
