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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3ElevationTokens.java

use crate::tokens::ElevationTokens as AndroidxElevationTokens;
use gpui::{BoxShadow, Hsla, Pixels, point, px};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElevationTokens {
    pub level0: Pixels,

    pub level1: Pixels,

    pub level2: Pixels,

    pub level3: Pixels,

    pub level4: Pixels,

    pub level5: Pixels,
}

impl Default for ElevationTokens {
    fn default() -> Self {
        Self::baseline()
    }
}

impl ElevationTokens {
    pub fn baseline() -> Self {
        Self {
            level0: AndroidxElevationTokens::LEVEL0.pixels(),
            level1: AndroidxElevationTokens::LEVEL1.pixels(),
            level2: AndroidxElevationTokens::LEVEL2.pixels(),
            level3: AndroidxElevationTokens::LEVEL3.pixels(),
            level4: AndroidxElevationTokens::LEVEL4.pixels(),
            level5: AndroidxElevationTokens::LEVEL5.pixels(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Elevation {
    Level0,

    Level1,

    Level2,

    Level3,

    Level4,

    Level5,
}

impl Elevation {
    pub fn shadows(self, shadow_color: Hsla) -> Vec<BoxShadow> {
        let key = |y: f32, blur: f32, spread: f32| BoxShadow {
            color: shadow_color.opacity(0.30),
            offset: point(px(0.), px(y)),
            blur_radius: px(blur),
            spread_radius: px(spread),
        };
        let ambient = |y: f32, blur: f32, spread: f32| BoxShadow {
            color: shadow_color.opacity(0.15),
            offset: point(px(0.), px(y)),
            blur_radius: px(blur),
            spread_radius: px(spread),
        };
        match self {
            Elevation::Level0 => vec![],
            Elevation::Level1 => vec![key(1., 2., 0.), ambient(1., 3., 1.)],
            Elevation::Level2 => vec![key(1., 2., 0.), ambient(2., 6., 2.)],
            Elevation::Level3 => vec![key(1., 3., 0.), ambient(4., 8., 3.)],
            Elevation::Level4 => vec![key(2., 3., 0.), ambient(6., 10., 4.)],
            Elevation::Level5 => vec![key(4., 4., 0.), ambient(8., 12., 6.)],
        }
    }

    pub fn dp(self) -> Pixels {
        match self {
            Elevation::Level0 => px(0.),
            Elevation::Level1 => px(1.),
            Elevation::Level2 => px(3.),
            Elevation::Level3 => px(6.),
            Elevation::Level4 => px(8.),
            Elevation::Level5 => px(12.),
        }
    }
}
