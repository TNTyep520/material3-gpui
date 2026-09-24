//! Icon 元素：以字体字形渲染 Material Symbols 图标。
//!
//! 实现方式对齐 [m3fx](https://github.com/Glavo/m3fx) 的 `M3Icon`
//! （Apache-2.0，© 2026 Glavo）：图标由内嵌的
//! 符号字体（见 [`crate::fonts`]）以字形连字（ligature）渲染，
//! 不再使用 SVG 资源——任意图标无需手动导入，直接写 ligature 名即可。
//!
//! ```ignore
//! use material3_gpui::prelude::*;
//!
//! Icon::new(IconName::Favorite).size(px(24.)).color(theme.colors().primary)
//! Icon::new(IconName::Custom("bolt")).size(px(24.))   // 任意图标
//! ```
//! 不设置 `color` 时继承父元素文字颜色。

use gpui::{
    App, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Styled, Window, div, prelude::*, px,
};

pub use crate::fonts::ICON_FONT_FAMILY;

/// 图标名。
///
/// 每个 variant 对应一个 Material Symbols ligature 名；
/// [`IconName::Custom`] 接受任意图标的 ligature 名（如 `"bolt"`、
/// `"settings_backup_restore"`），只要该图标存在于内嵌符号字体中。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconName {
    /// add
    Add,
    /// arrow_back
    ArrowBack,
    /// check
    Check,
    /// chevron_right
    ChevronRight,
    /// close
    Close,
    /// delete
    Delete,
    /// edit
    Edit,
    /// favorite
    Favorite,
    /// home
    Home,
    /// info
    Info,
    /// menu
    Menu,
    /// more_vert
    MoreVert,
    /// person
    Person,
    /// 内部使用：环形进度指示（`progress_activity`）
    ProgressActivity,
    /// search
    Search,
    /// settings
    Settings,
    /// star
    Star,
    /// 任意图标的 ligature 名（`&'static str`，通常为字面量）
    Custom(&'static str),
}

impl IconName {
    /// Material Symbols ligature 名（字体渲染用）。
    pub fn ligature(&self) -> &'static str {
        match self {
            IconName::Add => "add",
            IconName::ArrowBack => "arrow_back",
            IconName::Check => "check",
            IconName::ChevronRight => "chevron_right",
            IconName::Close => "close",
            IconName::Delete => "delete",
            IconName::Edit => "edit",
            IconName::Favorite => "favorite",
            IconName::Home => "home",
            IconName::Info => "info",
            IconName::Menu => "menu",
            IconName::MoreVert => "more_vert",
            IconName::Person => "person",
            IconName::ProgressActivity => "progress_activity",
            IconName::Search => "search",
            IconName::Settings => "settings",
            IconName::Star => "star",
            IconName::Custom(name) => name,
        }
    }

    /// 图标的 PUA 码点（官方 codepoints 表）。
    ///
    /// 内置图标全部有码点：渲染直接用码点字符映射字形，
    /// 不依赖连字（ligature）查找的触发行为。
    /// [`IconName::Custom`] 无固定码点，返回 `None`（走 ligature 名渲染）。
    pub fn codepoint(self) -> Option<char> {
        let cp = match self {
            IconName::Add => 0xe145,
            IconName::ArrowBack => 0xe5c4,
            IconName::Check => 0xe668,
            IconName::ChevronRight => 0xe5cc,
            IconName::Close => 0xe5cd,
            IconName::Delete => 0xe92e,
            IconName::Edit => 0xf097,
            IconName::Favorite => 0xe87e,
            IconName::Home => 0xe9b2,
            IconName::Info => 0xe88e,
            IconName::Menu => 0xe5d2,
            IconName::MoreVert => 0xe5d4,
            IconName::Person => 0xf0d3,
            IconName::ProgressActivity => 0xe9d0,
            IconName::Search => 0xef7a,
            IconName::Settings => 0xe8b8,
            IconName::Star => 0xf09a,
            IconName::Custom(_) => return None,
        };
        char::from_u32(cp)
    }
}

/// 图标元素（字体字形渲染）
#[derive(IntoElement)]
pub struct Icon {
    name: IconName,
    size: Pixels,
    color: Option<Hsla>,
}

impl Icon {
    /// 创建图标。
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: px(24.),
            color: None,
        }
    }

    /// 从 ligature 名创建任意图标（等价于 `Icon::new(IconName::Custom(name))`）。
    pub fn ligature(name: &'static str) -> Self {
        Self::new(IconName::Custom(name))
    }

    /// 图标尺寸（默认 24dp）。
    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    /// 图标颜色；不设置则继承文字颜色。
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // 行高锁定为图标尺寸：gpui 会把字体的自然行（ascent+descent，
        // Material Symbols 为 1.2×字号）在行盒内居中放置，实测字形墨心
        // 恰好落在行盒中心，无需额外补偿（实测 ascent 26.4/descent 2.4 @24px）。
        //
        // 内置图标经 ligature 名触发字形。
        div()
            .font_family(ICON_FONT_FAMILY)
            .text_size(self.size)
            .line_height(self.size)
            .flex_none()
            .when_some(self.color, |el, color| el.text_color(color))
            .child(SharedString::from(self.name.ligature()))
    }
}
