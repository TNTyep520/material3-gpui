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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3TypographyTokens.java
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3TextStyle.java

use crate::tokens::TypographyTokens;
use gpui::{FontWeight, Pixels, Styled, px};

#[derive(Clone, Copy, Debug)]
pub struct TypeStyle {
    pub size: Pixels,

    pub line_height: Pixels,

    pub weight: FontWeight,

    pub tracking: f32,
}

impl TypeStyle {
    pub const fn new(size: f32, line_height: f32, weight: FontWeight) -> Self {
        Self {
            size: px(size),
            line_height: px(line_height),
            weight,
            tracking: 0.0,
        }
    }

    pub const fn with_tracking(
        size: f32,
        line_height: f32,
        weight: FontWeight,
        tracking: f32,
    ) -> Self {
        Self {
            size: px(size),
            line_height: px(line_height),
            weight,
            tracking,
        }
    }

    pub fn apply<E: Styled>(&self, el: E) -> E {
        el.text_size(self.size)
            .line_height(self.line_height)
            .font_weight(self.weight)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TypeScale {
    pub body_large_emphasized: TypeStyle,
    pub body_medium_emphasized: TypeStyle,
    pub body_small_emphasized: TypeStyle,
    pub display_large_emphasized: TypeStyle,
    pub display_medium_emphasized: TypeStyle,
    pub display_small_emphasized: TypeStyle,
    pub headline_large_emphasized: TypeStyle,
    pub headline_medium_emphasized: TypeStyle,
    pub headline_small_emphasized: TypeStyle,
    pub label_large_emphasized: TypeStyle,
    pub label_medium_emphasized: TypeStyle,
    pub label_small_emphasized: TypeStyle,
    pub title_large_emphasized: TypeStyle,
    pub title_medium_emphasized: TypeStyle,
    pub title_small_emphasized: TypeStyle,

    pub display_large: TypeStyle,

    pub display_medium: TypeStyle,

    pub display_small: TypeStyle,

    pub headline_large: TypeStyle,

    pub headline_medium: TypeStyle,

    pub headline_small: TypeStyle,

    pub title_large: TypeStyle,

    pub title_medium: TypeStyle,

    pub title_small: TypeStyle,

    pub body_large: TypeStyle,

    pub body_medium: TypeStyle,

    pub body_small: TypeStyle,

    pub label_large: TypeStyle,

    pub label_medium: TypeStyle,

    pub label_small: TypeStyle,
}

impl Default for TypeScale {
    fn default() -> Self {
        Self::baseline()
    }
}

impl TypeScale {
    pub fn expressive() -> Self {
        let emphasized = |style: TypeStyle| TypeStyle {
            weight: match style.weight {
                FontWeight::NORMAL => FontWeight::SEMIBOLD,
                FontWeight::MEDIUM => FontWeight::BOLD,
                other => other,
            },
            ..style
        };
        Self {
            body_large_emphasized: emphasized(TypeStyle::with_tracking(
                17.,
                26.,
                FontWeight::NORMAL,
                0.50,
            )),
            body_medium_emphasized: emphasized(TypeStyle::with_tracking(
                15.,
                22.,
                FontWeight::NORMAL,
                0.25,
            )),
            body_small_emphasized: emphasized(TypeStyle::with_tracking(
                13.,
                18.,
                FontWeight::NORMAL,
                0.40,
            )),
            display_large_emphasized: emphasized(TypeStyle::with_tracking(
                64.,
                72.,
                FontWeight::MEDIUM,
                -0.25,
            )),
            display_medium_emphasized: emphasized(TypeStyle::new(52., 60., FontWeight::MEDIUM)),
            display_small_emphasized: emphasized(TypeStyle::new(44., 52., FontWeight::MEDIUM)),
            headline_large_emphasized: emphasized(TypeStyle::new(36., 44., FontWeight::MEDIUM)),
            headline_medium_emphasized: emphasized(TypeStyle::new(32., 40., FontWeight::MEDIUM)),
            headline_small_emphasized: emphasized(TypeStyle::new(28., 36., FontWeight::MEDIUM)),
            label_large_emphasized: emphasized(TypeStyle::with_tracking(
                14.,
                20.,
                FontWeight::SEMIBOLD,
                0.10,
            )),
            label_medium_emphasized: emphasized(TypeStyle::with_tracking(
                13.,
                18.,
                FontWeight::SEMIBOLD,
                0.50,
            )),
            label_small_emphasized: emphasized(TypeStyle::with_tracking(
                12.,
                16.,
                FontWeight::SEMIBOLD,
                0.50,
            )),
            title_large_emphasized: emphasized(TypeStyle::new(24., 32., FontWeight::MEDIUM)),
            title_medium_emphasized: emphasized(TypeStyle::with_tracking(
                18.,
                26.,
                FontWeight::SEMIBOLD,
                0.15,
            )),
            title_small_emphasized: emphasized(TypeStyle::with_tracking(
                15.,
                22.,
                FontWeight::SEMIBOLD,
                0.10,
            )),
            display_large: TypeStyle::with_tracking(64., 72., FontWeight::MEDIUM, -0.25),
            display_medium: TypeStyle::new(52., 60., FontWeight::MEDIUM),
            display_small: TypeStyle::new(44., 52., FontWeight::MEDIUM),
            headline_large: TypeStyle::new(36., 44., FontWeight::MEDIUM),
            headline_medium: TypeStyle::new(32., 40., FontWeight::MEDIUM),
            headline_small: TypeStyle::new(28., 36., FontWeight::MEDIUM),
            title_large: TypeStyle::new(24., 32., FontWeight::MEDIUM),
            title_medium: TypeStyle::with_tracking(18., 26., FontWeight::SEMIBOLD, 0.15),
            title_small: TypeStyle::with_tracking(15., 22., FontWeight::SEMIBOLD, 0.10),
            body_large: TypeStyle::with_tracking(17., 26., FontWeight::NORMAL, 0.50),
            body_medium: TypeStyle::with_tracking(15., 22., FontWeight::NORMAL, 0.25),
            body_small: TypeStyle::with_tracking(13., 18., FontWeight::NORMAL, 0.40),
            label_large: TypeStyle::with_tracking(14., 20., FontWeight::SEMIBOLD, 0.10),
            label_medium: TypeStyle::with_tracking(13., 18., FontWeight::SEMIBOLD, 0.50),
            label_small: TypeStyle::with_tracking(12., 16., FontWeight::SEMIBOLD, 0.50),
        }
    }

    pub fn baseline() -> Self {
        Self {
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
            display_large: TypeStyle::with_tracking(57., 64., FontWeight::NORMAL, -0.25),
            display_medium: TypeStyle::new(45., 52., FontWeight::NORMAL),
            display_small: TypeStyle::new(36., 44., FontWeight::NORMAL),
            headline_large: TypeStyle::new(32., 40., FontWeight::NORMAL),
            headline_medium: TypeStyle::new(28., 36., FontWeight::NORMAL),
            headline_small: TypeStyle::new(24., 32., FontWeight::NORMAL),
            title_large: TypeStyle::new(22., 28., FontWeight::NORMAL),
            title_medium: TypeStyle::with_tracking(16., 24., FontWeight::MEDIUM, 0.15),
            title_small: TypeStyle::with_tracking(14., 20., FontWeight::MEDIUM, 0.10),
            body_large: TypeStyle::with_tracking(16., 24., FontWeight::NORMAL, 0.50),
            body_medium: TypeStyle::with_tracking(14., 20., FontWeight::NORMAL, 0.25),
            body_small: TypeStyle::with_tracking(12., 16., FontWeight::NORMAL, 0.40),
            label_large: TypeStyle::with_tracking(14., 20., FontWeight::MEDIUM, 0.10),
            label_medium: TypeStyle::with_tracking(12., 16., FontWeight::MEDIUM, 0.50),
            label_small: TypeStyle::with_tracking(11., 16., FontWeight::MEDIUM, 0.50),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn baseline_matches_material_web() {
        let t = TypeScale::baseline();
        assert_eq!(t.display_large.size, px(57.));
        assert_eq!(t.label_large.size, px(14.));
        assert_eq!(t.label_large.weight, FontWeight::MEDIUM);

        assert_eq!(t.title_small.size, t.label_large.size);
        assert_eq!(t.title_small.line_height, t.label_large.line_height);
    }

    #[test]
    fn expressive_matches_m3_expressive_type_scale() {
        let t = TypeScale::expressive();
        assert_eq!(t.display_large.size, px(64.));
        assert_eq!(t.display_large.weight, FontWeight::MEDIUM);
        assert_eq!(t.title_medium.size, px(18.));
        assert_eq!(t.title_medium.weight, FontWeight::SEMIBOLD);
        assert_eq!(t.body_large.weight, FontWeight::NORMAL);
        assert_eq!(
            t.label_large_emphasized.weight,
            FontWeight::BOLD,
            "emphasized bumps semibold labels to bold"
        );
        assert_eq!(t.headline_small.size, px(28.));
    }
}
