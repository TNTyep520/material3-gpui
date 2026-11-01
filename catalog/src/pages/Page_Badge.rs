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

use gpui::{
    AnyElement, App, AppContext as _, Entity, IntoElement, ParentElement as _, Styled, div, px,
};
use material3_gpui::prelude::*;

use super::{ComponentPage, ExampleMeta, example_content};

pub struct BadgePage {
    nav_bar: Entity<NavigationBarState>,
}

impl BadgePage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        let nav_bar = NavigationBar::new("badge-nav-bar")
            .item(NavigationItemSpec::new("Home", IconName::new("home")).badge("24"))
            .item(NavigationItemSpec::new("Search", IconName::new("search")))
            .item(NavigationItemSpec::new("Profile", IconName::new("person")))
            .selected(0)
            .build(cx);
        cx.new(|_| Self { nav_bar })
    }
}

impl ComponentPage for BadgePage {
    fn description(&self) -> &'static str {
        "A badge can contain dynamic information, such as the presence of a new \
         notification or a number of pending requests. Badges can be icon only or contain \
         a short text."
    }

    fn examples(&self) -> &'static [ExampleMeta] {
        &[ExampleMeta {
            name: "NavigationBarItemWithBadge",
            description: "Badge examples",
        }]
    }

    fn example(&self, index: usize, _cx: &App) -> AnyElement {
        match index {
            0 => example_content([div()
                .w(px(420.))
                .max_w_full()
                .child(self.nav_bar.clone())
                .into_any_element()]),
            _ => example_content(Vec::<AnyElement>::new()),
        }
    }
}
