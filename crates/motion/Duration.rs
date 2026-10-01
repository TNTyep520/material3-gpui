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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/animation/M3Motion.java

use std::time::Duration;

use crate::tokens::MotionTokens;

pub const fn ms(millis: u64) -> Duration {
    Duration::from_millis(millis)
}

pub const SHORT1: Duration = MotionTokens::SHORT1;

pub const SHORT2: Duration = MotionTokens::SHORT2;

pub const SHORT3: Duration = MotionTokens::SHORT3;

pub const SHORT4: Duration = MotionTokens::SHORT4;

pub const MEDIUM1: Duration = MotionTokens::MEDIUM1;

pub const MEDIUM2: Duration = MotionTokens::MEDIUM2;

pub const MEDIUM3: Duration = MotionTokens::MEDIUM3;

pub const MEDIUM4: Duration = MotionTokens::MEDIUM4;

pub const LONG1: Duration = MotionTokens::LONG1;

pub const LONG2: Duration = MotionTokens::LONG2;

pub const LONG3: Duration = MotionTokens::LONG3;

pub const LONG4: Duration = MotionTokens::LONG4;

pub const EXTRA_LONG1: Duration = MotionTokens::EXTRA_LONG1;

pub const EXTRA_LONG2: Duration = MotionTokens::EXTRA_LONG2;

pub const EXTRA_LONG3: Duration = MotionTokens::EXTRA_LONG3;

pub const EXTRA_LONG4: Duration = MotionTokens::EXTRA_LONG4;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_match_spec() {
        assert_eq!(SHORT3, Duration::from_millis(150));
        assert_eq!(LONG2, Duration::from_millis(500));
        assert_eq!(EXTRA_LONG4, Duration::from_millis(1000));
    }
}
