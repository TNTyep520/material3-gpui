//! MD3 导航组件族（对应 material-web 的 navigation-bar / navigation-rail /
//! navigation-drawer 与 m3fx 的 `M3NavigationBar` / `M3NavigationRail` /
//! `M3NavigationDrawer` / `M3TopAppBar`）。
//!
//! 指示条滑动动画对齐 m3fx（Apache-2.0，© 2026 Glavo）：active indicator
//! 以 fastSpatial 弹簧在项间滑动。
//!
//! ```ignore
//! NavigationBar::new("nav")
//!     .item(NavigationItemSpec::new("Home", IconName::Home))
//!     .item(NavigationItemSpec::new("Search", IconName::Search))
//!     .selected(0)
//!     .on_change(|ix, _, _| {})
//!     .build(cx)   // -> Entity<NavigationBarState>
//! ```

use std::rc::Rc;
use std::time::Instant;

use gpui::{
    App, AppContext as _, Context, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::icon::{Icon, IconName};
use crate::motion::{Animatable, AnimatedComponent, AnimationDriver, MotionRole};
use crate::styles::navigation::{
    NavigationBarStyle, NavigationDrawerStyle, NavigationItemStyle, NavigationRailStyle,
};
use crate::theme::ActiveTheme;

type ChangeHandler = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// 可选中导航族(Bar / Rail / Drawer)的内部状态接口:
/// 点击项时由组件内部完成选中,状态真正变化才触发 on_change。
trait NavSelection: 'static {
    /// 当前选中下标。
    fn selected_index(&self) -> usize;
    /// 切换选中项(指示条动画由各实现自带)。
    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>)
    where
        Self: Sized;
}

impl NavSelection for NavigationBarState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

impl NavSelection for NavigationRailState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

impl NavSelection for NavigationDrawerState {
    fn selected_index(&self) -> usize {
        self.selected
    }

    fn select_index(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.select(index, window, cx)
    }
}

/// 导航项描述。
#[derive(Clone, Debug)]
pub struct NavigationItemSpec {
    /// 标签。
    pub label: SharedString,
    /// 图标。
    pub icon: Option<IconName>,
    /// 可选徽标文字（右上天角）。
    pub badge: Option<SharedString>,
}

impl NavigationItemSpec {
    /// 创建导航项。
    pub fn new(label: impl Into<SharedString>, icon: IconName) -> Self {
        Self {
            label: label.into(),
            icon: Some(icon),
            badge: None,
        }
    }

    /// 设置徽标文字。
    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }
}

/// 导航项内容渲染（Bar / Rail / Drawer 共用）。
#[allow(clippy::too_many_arguments)]
fn navigation_item<T: NavSelection>(
    id: impl Into<ElementId>,
    spec: &NavigationItemSpec,
    selected: bool,
    horizontal: bool,
    indicator_offset: Option<f32>,
    item: &NavigationItemStyle,
    entity: &Entity<T>,
    on_change: Option<&ChangeHandler>,
    ix: usize,
) -> gpui::Stateful<gpui::Div> {
    let icon_color = if selected {
        item.selected_icon_color
    } else {
        item.unselected_icon_color
    };
    let label_color = if selected {
        item.selected_label_color
    } else {
        item.unselected_label_color
    };
    let (indicator_w, indicator_h) = item.indicator_size;
    let hover = item.hover_opacity;
    let pressed = item.pressed_opacity;
    let indicator_color = item.indicator_color;
    let click_entity = entity.clone();
    let on_change = on_change.cloned();

    // 指示条胶囊：仅选中项渲染，锚定整项宽度的中心（relative 0.5），
    // 按 (弹簧位置 − 项下标) 的偏移跨项滑动；越出部分由容器裁剪
    let pill = indicator_offset.map(|offset| {
        div()
            .absolute()
            .left(relative(0.5 + offset))
            .ml(-indicator_w / 2.0)
            .top(px(0.))
            .w(indicator_w)
            .h(indicator_h)
            .flex_none()
            .rounded(item.indicator_radius)
            .bg(indicator_color)
    });

    let base = div()
        .id(id)
        .relative()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(item_gap_from(horizontal)))
        .cursor_pointer()
        .text_color(label_color)
        .hover(move |s| s.bg(icon_color.opacity(hover)))
        .active(move |s| s.bg(icon_color.opacity(pressed)))
        // 指示条胶囊在下、图标在上;图标行铺满整项宽度,
        // 使胶囊的 relative 定位以整项为基准
        .child(
            div()
                .relative()
                .w_full()
                .h(indicator_h)
                .flex()
                .items_center()
                .justify_center()
                .when_some(pill, |el, pill| el.child(pill))
                .when_some(spec.icon, |el, icon| {
                    el.child(Icon::new(icon).size(item.icon_size).color(icon_color))
                }),
        )
        // 点击:组件内部先完成选中(指示条滑动),
        // 状态真正变化才触发一次 on_change(未设置回调时仅内部切换)
        .on_click(move |_, window, cx| {
            click_entity.update(cx, |state, cx| {
                let changed = ix != state.selected_index();
                state.select_index(ix, window, cx);
                if changed && let Some(handler) = on_change.clone() {
                    handler(ix, window, cx);
                }
            })
        });

    let base = item.label.apply(base);

    let base = item.label.apply(base);
    base.child(spec.label.clone())
}

fn item_gap_from(horizontal: bool) -> f32 {
    if horizontal {
        16. // rail: 图标区与标签间距
    } else {
        4. // bar: 紧凑
    }
}

/// MD3 底部导航栏。
pub struct NavigationBar {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
}

/// 底部导航栏的有状态部分。
pub struct NavigationBarState {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    indicator: Animatable,
    driver: AnimationDriver,
}

impl NavigationBar {
    /// 创建导航栏构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_change: None,
        }
    }

    /// 追加导航项。
    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.items.push(item);
        self
    }

    /// 批量追加。
    pub fn items(mut self, items: impl IntoIterator<Item = NavigationItemSpec>) -> Self {
        self.items.extend(items);
        self
    }

    /// 初始选中下标。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// 选中变化回调。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建实体。
    pub fn build(self, cx: &mut App) -> Entity<NavigationBarState> {
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        cx.new(|_| NavigationBarState {
            id: self.id,
            items: self.items,
            selected,
            on_change: self.on_change,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

impl NavigationBarState {
    /// 当前选中下标。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换选中项（指示条弹簧滑动）。
    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.items.len() || index == self.selected {
            return;
        }
        self.selected = index;
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.indicator
            .animate_to(index as f64, &spec, Instant::now());
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for NavigationBarState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for NavigationBarState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let bar = NavigationBarStyle::resolve(theme.token_set());
        let item = NavigationItemStyle::resolve(theme.token_set());
        let indicator_pos = self.indicator.value() as f32;
        let selected = self.selected;
        let entity = cx.entity();

        div()
            .id(self.id.clone())
            .w_full()
            .h(bar.height)
            .flex()
            .flex_none()
            .overflow_hidden()
            .bg(bar.container_color)
            .children(self.items.iter().enumerate().map(|(ix, spec)| {
                let is_selected = ix == selected;
                // 指示条画在选中项内部，弹簧进度做相对偏移
                let offset = if is_selected {
                    Some(indicator_pos - ix as f32)
                } else {
                    None
                };
                let item_el = navigation_item(
                    (SharedString::from(format!("{}-item", self.id)), ix),
                    spec,
                    is_selected,
                    false,
                    offset,
                    &item,
                    &entity,
                    self.on_change.as_ref(),
                    ix,
                );
                item_el
                    .flex_1()
                    .h_full()
                    .justify_center()
                    .px(bar.item_padding)
            }))
    }
}

/// MD3 导航侧栏（Rail）。
pub struct NavigationRail {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    header: Option<Entity<FabState>>,
}

use crate::components::fab::FabState;

/// 导航侧栏的有状态部分。
pub struct NavigationRailState {
    id: ElementId,
    items: Vec<NavigationItemSpec>,
    selected: usize,
    on_change: Option<ChangeHandler>,
    header: Option<Entity<FabState>>,
    indicator: Animatable,
    driver: AnimationDriver,
}

impl NavigationRail {
    /// 创建导航侧栏构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            selected: 0,
            on_change: None,
            header: None,
        }
    }

    /// 追加导航项。
    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.items.push(item);
        self
    }

    /// 批量追加。
    pub fn items(mut self, items: impl IntoIterator<Item = NavigationItemSpec>) -> Self {
        self.items.extend(items);
        self
    }

    /// 初始选中下标。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// 顶部槽位内容（通常为 FAB）。
    pub fn header(mut self, header: Entity<FabState>) -> Self {
        self.header = Some(header);
        self
    }

    /// 选中变化回调。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建实体。
    pub fn build(self, cx: &mut App) -> Entity<NavigationRailState> {
        let selected = self.selected.min(self.items.len().saturating_sub(1));
        cx.new(|_| NavigationRailState {
            id: self.id,
            items: self.items,
            selected,
            on_change: self.on_change,
            header: self.header,
            indicator: Animatable::new(selected as f64, 1.0e-3),
            driver: AnimationDriver::default(),
        })
    }
}

impl NavigationRailState {
    /// 当前选中下标。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换选中项（指示条弹簧滑动）。
    pub fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if index >= self.items.len() || index == self.selected {
            return;
        }
        self.selected = index;
        let spec = *cx.theme().motion().spec(MotionRole::FastSpatial);
        self.indicator
            .animate_to(index as f64, &spec, Instant::now());
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        cx.notify();
    }
}

impl AnimatedComponent for NavigationRailState {
    fn step(&mut self, now: Instant) -> bool {
        self.indicator.tick(now)
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl Render for NavigationRailState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.indicator.is_running() {
            self.schedule_next(window, cx);
        }
        let theme = cx.theme();
        let rail = NavigationRailStyle::resolve(theme.token_set());
        let item = NavigationItemStyle::resolve(theme.token_set());
        let selected = self.selected;
        let header = self.header.clone();
        let entity = cx.entity();

        div()
            .id(self.id.clone())
            .w(rail.width)
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .items_center()
            .pt(rail.top_padding)
            .gap(rail.item_gap)
            .bg(rail.container_color)
            .children(header.map(|h| div().pb(px(16.)).child(h)))
            .children(self.items.iter().enumerate().map(|(ix, spec)| {
                let is_selected = ix == selected;
                // Rail 为纵向列表：指示条在选中项内居中（无跨项滑动）
                let offset = if is_selected { Some(0.0) } else { None };
                navigation_item(
                    (SharedString::from(format!("{}-item", self.id)), ix),
                    spec,
                    is_selected,
                    true,
                    offset,
                    &item,
                    &entity,
                    self.on_change.as_ref(),
                    ix,
                )
                .w_full()
                .py(rail.item_gap)
            }))
    }
}

/// 导航抽屉内容项（分组内条目或分组标题）。
pub enum DrawerEntry {
    /// 分组标题。
    Section(SharedString),
    /// 导航项。
    Item(NavigationItemSpec),
}

/// MD3 导航抽屉。
pub struct NavigationDrawer {
    id: ElementId,
    entries: Vec<DrawerEntry>,
    selected: usize,
    modal: bool,
    on_change: Option<ChangeHandler>,
}

/// 导航抽屉的有状态部分。
pub struct NavigationDrawerState {
    id: ElementId,
    entries: Vec<DrawerEntry>,
    selected: usize,
    modal: bool,
    on_change: Option<ChangeHandler>,
    driver: AnimationDriver,
}

impl NavigationDrawer {
    /// 创建导航抽屉构建器。
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            entries: Vec::new(),
            selected: 0,
            modal: false,
            on_change: None,
        }
    }

    /// 追加条目。
    pub fn entry(mut self, entry: DrawerEntry) -> Self {
        self.entries.push(entry);
        self
    }

    /// 追加导航项。
    pub fn item(mut self, item: NavigationItemSpec) -> Self {
        self.entries.push(DrawerEntry::Item(item));
        self
    }

    /// 追加分组标题。
    pub fn section(mut self, title: impl Into<SharedString>) -> Self {
        self.entries.push(DrawerEntry::Section(title.into()));
        self
    }

    /// 初始选中下标（按条目中的 Item 序号）。
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index;
        self
    }

    /// modal 变体（带阴影，配 scrim 使用）。
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = modal;
        self
    }

    /// 选中变化回调。
    pub fn on_change(mut self, handler: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// 构建实体。
    pub fn build(self, cx: &mut App) -> Entity<NavigationDrawerState> {
        cx.new(|_| NavigationDrawerState {
            id: self.id,
            entries: self.entries,
            selected: self.selected,
            modal: self.modal,
            on_change: self.on_change,
            driver: AnimationDriver::default(),
        })
    }
}

impl AnimatedComponent for NavigationDrawerState {
    fn step(&mut self, _now: Instant) -> bool {
        false
    }

    fn driver_mut(&mut self) -> &mut AnimationDriver {
        &mut self.driver
    }
}

impl NavigationDrawerState {
    /// 当前选中下标（按条目中的 Item 序号）。
    pub fn selected(&self) -> usize {
        self.selected
    }

    /// 切换选中条目（按 Item 序号；选中高亮即时更新）。
    pub fn select(&mut self, index: usize, _window: &mut Window, cx: &mut Context<Self>) {
        let item_count = self
            .entries
            .iter()
            .filter(|entry| matches!(entry, DrawerEntry::Item(_)))
            .count();
        if index >= item_count || index == self.selected {
            return;
        }
        self.selected = index;
        cx.notify();
    }
}

impl Render for NavigationDrawerState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let drawer = NavigationDrawerStyle::resolve(theme.token_set(), self.modal);
        let item = NavigationItemStyle::resolve(theme.token_set());
        let width = drawer.drawer_width(self.modal);
        let entity = cx.entity();
        let mut item_ix = 0usize;

        let mut column = div()
            .id(self.id.clone())
            .w(width)
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .gap(px(4.))
            .p(drawer.padding)
            .bg(drawer.container_color)
            .when(self.modal, |el| {
                el.shadow(drawer.modal_elevation.shadows(drawer.shadow_color))
            });

        for entry in self.entries.iter() {
            match entry {
                DrawerEntry::Section(title) => {
                    let el = drawer
                        .section_header
                        .apply(div())
                        .px(drawer.item_horizontal_padding)
                        .pt(px(16.))
                        .text_color(drawer.section_header_color)
                        .child(title.clone());
                    column = column.child(el);
                }
                DrawerEntry::Item(spec) => {
                    let ix = item_ix;
                    let is_selected = ix == self.selected;
                    let on_change = self.on_change.clone();
                    let el = navigation_item(
                        (SharedString::from(format!("{}-item", self.id)), ix),
                        spec,
                        is_selected,
                        true,
                        None,
                        &item,
                        &entity,
                        on_change.as_ref(),
                        ix,
                    )
                    .w_full()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .h(px(56.))
                    .rounded(item.indicator_radius)
                    .px(drawer.item_horizontal_padding)
                    .when(is_selected, |el| el.bg(item.indicator_color));
                    column = column.child(el);
                    item_ix += 1;
                }
            }
        }
        column
    }
}
