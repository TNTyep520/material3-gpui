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

use gpui::{App, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Styled, Window, px, svg};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IconName(SharedString);

impl IconName {
    pub fn new(name: impl Into<SharedString>) -> Self {
        Self(name.into())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn path(&self) -> SharedString {
        format!("md3-icons/{}.svg", self.0).into()
    }
}

impl From<&'static str> for IconName {
    fn from(name: &'static str) -> Self {
        Self(SharedString::from(name))
    }
}

impl From<String> for IconName {
    fn from(name: String) -> Self {
        Self(SharedString::from(name))
    }
}

impl From<SharedString> for IconName {
    fn from(name: SharedString) -> Self {
        Self(name)
    }
}

#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
    size: Pixels,
    color: Option<Hsla>,
}

impl Icon {
    pub fn new(name: impl Into<IconName>) -> Self {
        Self {
            name: name.into(),
            size: px(24.),
            color: None,
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        svg()
            .path(self.name.path())
            .size(self.size)
            .flex_none()
            .text_color(self.color.unwrap_or_else(|| window.text_style().color))
    }
}

#[cfg(test)]
mod tests {
    use super::IconName;

    #[test]
    fn names_map_to_asset_paths() {
        assert_eq!(IconName::new("bolt").name(), "bolt");
        assert_eq!(IconName::new("bolt").path(), "md3-icons/bolt.svg");
        assert_eq!(IconName::from("star"), IconName::new("star"));
        assert_eq!(IconName::from(String::from("star")), IconName::new("star"));
    }
}
