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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3ShapeTokens.java

use crate::tokens::ShapeTokens;
use gpui::{Pixels, px};

#[derive(Clone, Copy, Debug)]
pub struct Shapes {
    pub none: Pixels,

    pub extra_small: Pixels,

    pub small: Pixels,

    pub medium: Pixels,

    pub large: Pixels,

    pub large_increased: Pixels,

    pub extra_large: Pixels,

    pub extra_large_increased: Pixels,

    pub extra_extra_large: Pixels,

    pub full: Pixels,
}

impl Default for Shapes {
    fn default() -> Self {
        Self::baseline()
    }
}

impl Shapes {
    pub fn baseline() -> Self {
        Self {
            none: ShapeTokens::CORNER_VALUE_NONE.pixels(),
            extra_small: ShapeTokens::CORNER_VALUE_EXTRA_SMALL.pixels(),
            small: ShapeTokens::CORNER_VALUE_SMALL.pixels(),
            medium: ShapeTokens::CORNER_VALUE_MEDIUM.pixels(),
            large: ShapeTokens::CORNER_VALUE_LARGE.pixels(),
            large_increased: ShapeTokens::CORNER_VALUE_LARGE_INCREASED.pixels(),
            extra_large: ShapeTokens::CORNER_VALUE_EXTRA_LARGE.pixels(),
            extra_large_increased: ShapeTokens::CORNER_VALUE_EXTRA_LARGE_INCREASED.pixels(),
            extra_extra_large: ShapeTokens::CORNER_VALUE_EXTRA_EXTRA_LARGE.pixels(),
            full: px(999.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scales_match_m3fx() {
        let b = Shapes::baseline();
        assert_eq!(b.extra_small, px(4.));
        assert_eq!(b.large_increased, px(20.));
        assert_eq!(b.extra_extra_large, px(48.));
    }
}
