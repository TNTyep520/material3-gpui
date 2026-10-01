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

use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, IntoElement, Render, WeakEntity, Window, div,
    prelude::*, px, relative,
};
use material3_gpui::prelude::*;

use super::{LogErr as _, gallery, showcase_group};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SchemeKind {
    Standard,
    Expressive,
}

pub struct MotionPage {
    weak: WeakEntity<Self>,
    position: Animatable,
    driver: AnimationDriver,
    role: MotionRole,
    scheme: SchemeKind,
    run_button: Entity<ButtonState>,
}

impl MotionPage {
    pub fn new(cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            let weak = cx.entity().downgrade();
            let run_button = {
                let weak = weak.clone();
                Button::new("motion-run", "Run")
                    .filled()
                    .on_click(move |_, window, cx| {
                        weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                            page.run(window, cx);
                        })
                        .log_err();
                    })
                    .build(cx)
            };
            Self {
                weak,
                position: Animatable::new(0.0, 1.0e-3),
                driver: AnimationDriver::default(),
                role: MotionRole::DefaultSpatial,
                scheme: SchemeKind::Standard,
                run_button,
            }
        })
    }

    fn run(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let target = if self.position.value() < 0.5 {
            1.0
        } else {
            0.0
        };
        let spec = *cx.theme().motion().spec(self.role);
        self.position.animate_to(target, &spec, Instant::now());
        if self.position.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }

    fn set_scheme(&mut self, scheme: SchemeKind, cx: &mut Context<Self>) {
        if self.scheme == scheme {
            return;
        }
        self.scheme = scheme;
        let motion = match scheme {
            SchemeKind::Standard => MotionScheme::standard(),
            SchemeKind::Expressive => MotionScheme::expressive(),
        };
        let colors = *cx.theme().colors();
        let tokens = TokenSet::builder(Profile::Baseline2021, colors)
            .with_motion(motion)
            .build();
        let mut theme = cx.theme().clone();
        theme.set_token_set(tokens);
        Theme::set(cx, theme);
        cx.refresh_windows();
    }
}

impl AnimatedComponent for MotionPage {
    fn step(&mut self, now: Instant) -> bool {
        self.position.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

fn pill(
    cx: &App,
    id: impl Into<ElementId>,
    label: &'static str,
    selected: bool,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let theme = cx.theme();
    let colors = *theme.colors();
    div()
        .id(id)
        .h(px(40.))
        .px(px(16.))
        .flex_none()
        .flex()
        .items_center()
        .cursor_pointer()
        .rounded_full()
        .when(selected, |el| {
            el.bg(colors.secondary_container)
                .text_color(colors.on_secondary_container)
        })
        .when(!selected, |el| {
            el.border_1()
                .border_color(colors.outline)
                .text_color(colors.on_surface_variant)
        })
        .child(theme.typography().label_large.apply(div()).child(label))
        .on_click(move |_, window, cx| on_click(window, cx))
}

impl Render for MotionPage {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.position.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let colors = *theme.colors();
        let typography = *theme.typography();
        let weak = self.weak.clone();
        let position = self.position.value() as f32;

        let roles = [
            (MotionRole::FastEffects, "Fast effects"),
            (MotionRole::DefaultEffects, "Default effects"),
            (MotionRole::SlowEffects, "Slow effects"),
            (MotionRole::FastSpatial, "Fast spatial"),
            (MotionRole::DefaultSpatial, "Default spatial"),
            (MotionRole::SlowSpatial, "Slow spatial"),
        ];
        let role = self.role;
        let scheme = self.scheme;

        gallery([
            showcase_group(
                cx,
                "Motion scheme",
                [
                    pill(
                        cx,
                        "scheme-standard",
                        "Standard",
                        scheme == SchemeKind::Standard,
                        {
                            let weak = weak.clone();
                            move |_, cx| {
                                weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                    page.set_scheme(SchemeKind::Standard, cx);
                                })
                                .log_err();
                            }
                        },
                    )
                    .into_any_element(),
                    pill(
                        cx,
                        "scheme-expressive",
                        "Expressive",
                        scheme == SchemeKind::Expressive,
                        {
                            let weak = weak.clone();
                            move |_, cx| {
                                weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                    page.set_scheme(SchemeKind::Expressive, cx);
                                })
                                .log_err();
                            }
                        },
                    )
                    .into_any_element(),
                ],
            )
            .into_any_element(),
            showcase_group(
                cx,
                "Motion role",
                roles
                    .into_iter()
                    .enumerate()
                    .map(|(index, (candidate, label))| {
                        pill(cx, ("motion-role", index), label, role == candidate, {
                            let weak = weak.clone();
                            move |_, cx| {
                                weak.update(cx, |page: &mut Self, cx: &mut Context<Self>| {
                                    page.role = candidate;
                                    cx.notify();
                                })
                                .log_err();
                            }
                        })
                        .into_any_element()
                    })
                    .collect::<Vec<_>>(),
            )
            .into_any_element(),
            showcase_group(
                cx,
                "Spring playground",
                [div()
                    .w_full()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(px(16.))
                    .child(div().w_full().child(self.run_button.clone()))
                    .child(
                        div()
                            .w_full()
                            .h(px(64.))
                            .rounded(px(12.))
                            .bg(colors.surface_container_high)
                            .relative()
                            .overflow_hidden()
                            .child(
                                div()
                                    .absolute()
                                    .size(px(40.))
                                    .top(px(12.))
                                    .left(relative(position.clamp(0., 1.)))
                                    .rounded(px(8.))
                                    .bg(colors.primary),
                            ),
                    )
                    .child(
                        typography
                            .body_medium
                            .apply(div())
                            .text_color(colors.on_surface_variant)
                            .child(
                                "Run toggles the box between both ends using the \
                                 selected role's spring.",
                            ),
                    )
                    .into_any_element()],
            )
            .into_any_element(),
        ])
    }
}
