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
// 参考 https://github.com/Glavo/m3fx/blob/main/src/main/java/org/glavo/m3fx/tokens/M3Profile.java

#[cfg(feature = "dynamic-color")]
use mcu_dynamiccolor::{SpecVersion, Variant};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Profile {
    #[default]
    Baseline2021,
}

impl Profile {
    #[cfg(feature = "dynamic-color")]
    pub fn color_spec_version(self) -> SpecVersion {
        match self {
            Profile::Baseline2021 => SpecVersion::Spec2021,
        }
    }

    #[cfg(feature = "dynamic-color")]
    pub fn color_style(self) -> Variant {
        match self {
            Profile::Baseline2021 => Variant::TonalSpot,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn profile_is_baseline_2021() {
        assert!(matches!(
            super::Profile::Baseline2021,
            super::Profile::Baseline2021
        ));
    }
}
