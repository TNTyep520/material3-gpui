//! 内嵌字体注册。
//!
//! - **Roboto**（Apache-2.0）：MD3 默认正文字体，Regular / Medium 两个字重。
//! - **Material Symbols Rounded**（Apache-2.0，Regular 实例）：
//!   [`crate::icon::Icon`] 以字体字形渲染图标（对齐 m3fx `M3Icon` 的实现方式）。
//!
//! 字体均为 TTF 直接内嵌、经 [`install`] 注册，[`crate::init`] 默认调用。
//!
//! **图标字体按官方 Material Symbols 自托管指引生成**（Google Fonts css2：
//! 可变轴实例化 `opsz,wght,FILL,GRAD@24,400,0,0`，全字形 TTF）。

use std::borrow::Cow;

use gpui::App;

/// Material Symbols Rounded 图标字体族名（ligature 渲染时引用）。
pub const ICON_FONT_FAMILY: &str = "Material Symbols Rounded";

/// Roboto 正文字体族名（主题 `font_family` 默认值）。
pub const TEXT_FONT_FAMILY: &str = "Roboto";

/// 注册内嵌字体（Roboto Regular/Medium + Material Symbols Rounded）。
///
/// 幂等：重复调用只会重复注册（平台层通常去重）。
/// 失败不 panic，返回错误交由调用方决定（[`crate::init`] 会记录并忽略）。
pub fn install(cx: &mut App) -> anyhow::Result<()> {
    const ROBOTO_REGULAR: &[u8] = include_bytes!("fonts/Roboto-Regular.ttf");
    const ROBOTO_MEDIUM: &[u8] = include_bytes!("fonts/Roboto-Medium.ttf");
    const MATERIAL_SYMBOLS_ROUNDED: &[u8] =
        include_bytes!("fonts/MaterialSymbolsRounded-Regular.ttf");

    for (name, data) in [
        ("Roboto Regular", ROBOTO_REGULAR),
        ("Roboto Medium", ROBOTO_MEDIUM),
        ("Material Symbols Rounded", MATERIAL_SYMBOLS_ROUNDED),
    ] {
        // 逐字体注册：单个失败不影响其余字体
        cx.text_system()
            .add_fonts(vec![Cow::Borrowed(data)])
            .map_err(|err| anyhow::anyhow!("{name}: {err}"))?;
    }
    Ok(())
}
