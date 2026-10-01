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
// 参考 https://github.com/androidx/androidx/blob/androidx-main/compose/material3/material3/src/commonMain/kotlin/androidx/compose/material3/SegmentedButton.kt

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, Corners, Div, ElementId, Entity, FocusHandle,
    InteractiveElement as _, IntoElement, ParentElement as _, Render, SharedString, Stateful,
    StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::icon::{Icon, IconName};
use crate::interaction::{InteractiveSurface, wire_events};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole, lerp_color};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(&[usize], &mut Window, &mut App)>;
type StyleOverride = Rc<dyn Fn(&mut SegmentedButtonStyle)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SegmentedButtonSelectionMode {
    #[default]
    Single,
    Multiple,
}

#[derive(Clone)]
pub struct SegmentedButton {
    label: SharedString,
    icon: Option<IconName>,
    selected: bool,
    disabled: bool,
}

impl SegmentedButton {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            selected: false,
            disabled: false,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }
}

pub struct SegmentedButtonRow {
    id: ElementId,
    buttons: Vec<SegmentedButton>,
    selection_mode: SegmentedButtonSelectionMode,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    style_override: Option<StyleOverride>,
}

pub struct SegmentedButtonRowState {
    row: SegmentedButtonRow,
    surfaces: Vec<InteractiveSurface>,
    selected_progress: Vec<Animatable>,
    focus_handles: Vec<FocusHandle>,
    driver: AnimationDriver,
}

impl SegmentedButtonRow {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            buttons: Vec::new(),
            selection_mode: SegmentedButtonSelectionMode::Single,
            disabled: false,
            on_change: None,
            style_override: None,
        }
    }

    pub fn buttons(mut self, buttons: impl IntoIterator<Item = SegmentedButton>) -> Self {
        self.buttons.extend(buttons);
        self
    }

    pub fn selection_mode(mut self, mode: SegmentedButtonSelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn enabled(self, enabled: bool) -> Self {
        self.disabled(!enabled)
    }

    pub fn on_change(
        mut self,
        handler: impl Fn(&[usize], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn style(mut self, style: impl Fn(&mut SegmentedButtonStyle) + 'static) -> Self {
        self.style_override = Some(Rc::new(style));
        self
    }

    pub fn build(mut self, cx: &mut App) -> Entity<SegmentedButtonRowState> {
        self.normalize_selection();
        let selected_flags: Vec<bool> = self.buttons.iter().map(|button| button.selected).collect();
        cx.new(|cx| SegmentedButtonRowState {
            surfaces: self
                .buttons
                .iter()
                .map(|_| InteractiveSurface::new())
                .collect(),
            selected_progress: selected_flags
                .iter()
                .map(|selected| Animatable::new(if *selected { 1.0 } else { 0.0 }, 1.0e-3))
                .collect(),
            focus_handles: self.buttons.iter().map(|_| cx.focus_handle()).collect(),
            row: self,
            driver: AnimationDriver::default(),
        })
    }

    fn normalize_selection(&mut self) {
        if self.selection_mode == SegmentedButtonSelectionMode::Single {
            let mut found = false;
            for button in &mut self.buttons {
                let selected = button.selected;
                button.selected &= !found;
                found |= selected;
            }
        }
    }

    fn selected_indices(&self) -> Vec<usize> {
        self.buttons
            .iter()
            .enumerate()
            .filter_map(|(index, button)| button.selected.then_some(index))
            .collect()
    }

    fn set_selected(&mut self, index: usize, selected: bool) -> bool {
        let Some(button) = self.buttons.get(index) else {
            return false;
        };
        if button.selected == selected {
            return false;
        }
        if selected && self.selection_mode == SegmentedButtonSelectionMode::Single {
            for (button_index, button) in self.buttons.iter_mut().enumerate() {
                button.selected = button_index == index;
            }
        } else if let Some(button) = self.buttons.get_mut(index) {
            button.selected = selected;
        }
        true
    }

    fn activate(&mut self, index: usize) -> bool {
        let Some(button) = self.buttons.get(index) else {
            return false;
        };
        if self.disabled || button.disabled {
            return false;
        }
        let selected =
            self.selection_mode == SegmentedButtonSelectionMode::Single || !button.selected;
        self.set_selected(index, selected)
    }
}

macro_rules! segmented_button_row_variant {
    ($name:ident, $mode:ident) => {
        #[doc = concat!("AndroidX ", stringify!($name), " 对应的分段按钮行。")]
        pub struct $name(SegmentedButtonRow);

        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self(
                    SegmentedButtonRow::new(id).selection_mode(SegmentedButtonSelectionMode::$mode),
                )
            }

            pub fn buttons(mut self, buttons: impl IntoIterator<Item = SegmentedButton>) -> Self {
                self.0 = self.0.buttons(buttons);
                self
            }

            pub fn enabled(mut self, enabled: bool) -> Self {
                self.0 = self.0.enabled(enabled);
                self
            }

            pub fn on_change(
                mut self,
                handler: impl Fn(&[usize], &mut Window, &mut App) + 'static,
            ) -> Self {
                self.0 = self.0.on_change(handler);
                self
            }

            pub fn build(self, cx: &mut App) -> Entity<SegmentedButtonRowState> {
                self.0.build(cx)
            }
        }
    };
}

segmented_button_row_variant!(SingleChoiceSegmentedButtonRow, Single);
segmented_button_row_variant!(MultiChoiceSegmentedButtonRow, Multiple);

impl SegmentedButtonRowState {
    pub fn selected(&self) -> Vec<usize> {
        self.row.selected_indices()
    }

    pub fn set_selected(
        &mut self,
        index: usize,
        selected: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.row.set_selected(index, selected) {
            self.animate_selection(window, cx);
            cx.notify();
        }
    }

    fn animate_selection(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let spec = *cx.theme().motion().spec(MotionRole::DefaultEffects);
        for (index, button) in self.row.buttons.iter().enumerate() {
            if let Some(progress) = self.selected_progress.get_mut(index) {
                progress.animate_to(
                    if button.selected { 1.0 } else { 0.0 },
                    &spec,
                    Instant::now(),
                );
            }
        }
        if self.selected_progress.iter().any(Animatable::is_running) {
            self.schedule_next(window, cx);
        }
    }

    fn activate(entity: &Entity<Self>, index: usize, window: &mut Window, cx: &mut App) {
        let change = entity.update(cx, |state, cx| {
            if !state.row.activate(index) {
                return None;
            }
            state.animate_selection(window, cx);
            cx.notify();
            state
                .row
                .on_change
                .clone()
                .map(|handler| (handler, state.selected()))
        });
        if let Some((handler, selected)) = change {
            handler(&selected, window, cx);
        }
    }
}

impl AnimatedComponent for SegmentedButtonRowState {
    fn step(&mut self, now: Instant) -> bool {
        let mut running = false;
        for progress in &mut self.selected_progress {
            running |= progress.tick(now);
        }
        for surface in &mut self.surfaces {
            running |= surface.step(now);
        }
        running
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for SegmentedButtonRowState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.surfaces.iter().any(InteractiveSurface::is_animating)
            || self.selected_progress.iter().any(Animatable::is_running)
        {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let entity = cx.entity();
        let count = self.row.buttons.len();
        let mut row = div().id(self.row.id.clone()).flex().w_full();
        for (index, ((button, surface), focus)) in self
            .row
            .buttons
            .iter()
            .zip(&self.surfaces)
            .zip(&self.focus_handles)
            .enumerate()
        {
            let disabled = self.row.disabled || button.disabled;
            let p = self.selected_progress[index].value() as f32;
            let mut style = SegmentedButtonStyle::resolve(theme.token_set(), false, disabled);
            let checked_style = SegmentedButtonStyle::resolve(theme.token_set(), true, disabled);
            style.content_color = lerp_color(style.content_color, checked_style.content_color, p);
            style.container_color = match (style.container_color, checked_style.container_color) {
                (Some(rest), Some(checked)) => Some(lerp_color(rest, checked, p)),
                (rest, checked) => {
                    if p >= 0.5 {
                        checked
                    } else {
                        rest
                    }
                }
            };
            if let Some(customize) = &self.row.style_override {
                customize(&mut style);
            }
            let first = index == 0;
            let last = index + 1 == count;
            let shape = |element: Stateful<Div>| {
                element
                    .when(first, |element| {
                        element
                            .rounded_tl(style.corner_radius)
                            .rounded_bl(style.corner_radius)
                    })
                    .when(last, |element| {
                        element
                            .rounded_tr(style.corner_radius)
                            .rounded_br(style.corner_radius)
                    })
            };
            let base = div()
                .id(index)
                .relative()
                .flex_1()
                .min_w(style.min_width)
                .h(style.height)
                .border(style.outline_width)
                .border_color(style.outline_color)
                .when(!first, |element| element.ml(-style.outline_width))
                .when_some(style.container_color, |element, color| element.bg(color));
            let base = shape(base);
            let content = div()
                .id(("content", index))
                .relative()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .px(style.horizontal_padding)
                .gap(style.gap)
                .text_color(style.content_color)
                .overflow_hidden();
            let mut content = shape(content);
            if !disabled {
                content = wire_events(content, &entity, theme.motion(), move |state| {
                    &mut state.surfaces[index]
                });
                let corners = Corners {
                    top_left: if first { style.corner_radius } else { px(0.) },
                    bottom_left: if first { style.corner_radius } else { px(0.) },
                    top_right: if last { style.corner_radius } else { px(0.) },
                    bottom_right: if last { style.corner_radius } else { px(0.) },
                };
                content = surface
                    .overlay_with_corners(style.content_color, theme.state_layer().pressed, corners)
                    .apply(content)
                    .child(surface.bounds.capture_element());
                let click_entity = entity.clone();
                let key_entity = entity.clone();
                let click_focus = focus.clone();
                let focus_color = style.content_color.opacity(theme.state_layer().focus);
                content = content
                    .cursor_pointer()
                    .track_focus(focus)
                    .tab_stop(true)
                    .focus(move |style| style.bg(focus_color))
                    .on_click(move |_, window, cx| {
                        window.focus(&click_focus);
                        Self::activate(&click_entity, index, window, cx);
                    })
                    .on_key_down(move |event, window, cx| {
                        if !event.is_held
                            && matches!(event.keystroke.key.as_str(), "space" | "enter")
                        {
                            cx.stop_propagation();
                            Self::activate(&key_entity, index, window, cx);
                        }
                    });
            }
            let leading_icon = if p > 0.01 {
                Some(
                    Icon::new(IconName::new("check"))
                        .size(style.icon_size * (0.5 + 0.5 * p))
                        .color(style.content_color.opacity(p)),
                )
            } else {
                button.icon.clone().map(|icon| {
                    Icon::new(icon)
                        .size(style.icon_size)
                        .color(style.content_color)
                })
            };
            content = style
                .label
                .apply(content)
                .when_some(leading_icon, |element, icon| element.child(icon))
                .child(button.label.clone());
            row = row.child(base.child(content));
        }
        row
    }
}

pub use appearance::SegmentedButtonStyle;

mod appearance {
    use crate::theme::{TokenSet, TypeStyle};
    use crate::tokens::OutlinedSegmentedButtonTokens;
    use gpui::{Hsla, Pixels, px};
    #[derive(Clone, Copy, Debug)]
    pub struct SegmentedButtonStyle {
        pub container_color: Option<Hsla>,
        pub content_color: Hsla,
        pub outline_color: Hsla,
        pub outline_width: Pixels,
        pub height: Pixels,
        pub min_width: Pixels,
        pub corner_radius: Pixels,
        pub horizontal_padding: Pixels,
        pub icon_size: Pixels,
        pub gap: Pixels,
        pub label: TypeStyle,
    }
    impl SegmentedButtonStyle {
        pub fn resolve(tokens: &TokenSet, selected: bool, disabled: bool) -> Self {
            let colors = &tokens.colors;
            Self {
                container_color: selected.then_some(colors.secondary_container),
                content_color: if disabled {
                    colors.disabled_content(&tokens.state_layer)
                } else if selected {
                    colors.on_secondary_container
                } else {
                    colors.on_surface
                },
                outline_color: if disabled {
                    colors
                        .outline
                        .opacity(tokens.state_layer.disabled_container)
                } else {
                    colors.outline
                },
                outline_width: OutlinedSegmentedButtonTokens::OUTLINE_WIDTH.pixels(),
                height: OutlinedSegmentedButtonTokens::CONTAINER_HEIGHT.pixels(),
                min_width: px(64.),
                corner_radius: tokens.shapes.full,
                horizontal_padding: px(12.),
                icon_size: OutlinedSegmentedButtonTokens::ICON_SIZE.pixels(),
                gap: px(8.),
                label: OutlinedSegmentedButtonTokens::LABEL_TEXT_FONT.resolve(tokens),
            }
        }
    }
}
