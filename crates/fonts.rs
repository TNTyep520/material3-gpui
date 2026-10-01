//! 内嵌 Roboto 正文字体注册；图标通过 SVG 资源渲染。

use std::borrow::Cow;

use gpui::App;

/// Roboto 正文字体族名（主题 `font_family` 默认值）。
pub const TEXT_FONT_FAMILY: &str = "Roboto";

/// 注册内嵌正文字体（Roboto Regular/Medium）。
///
/// 幂等：重复调用只会重复注册（平台层通常去重）。
/// 失败不 panic，返回错误交由调用方决定（[`crate::init`] 会记录并忽略）。
pub fn install(cx: &mut App) -> anyhow::Result<()> {
    const ROBOTO_REGULAR: &[u8] = include_bytes!("fonts/Roboto-Regular.ttf");
    const ROBOTO_MEDIUM: &[u8] = include_bytes!("fonts/Roboto-Medium.ttf");

    for (name, data) in [
        ("Roboto Regular", ROBOTO_REGULAR),
        ("Roboto Medium", ROBOTO_MEDIUM),
    ] {
        // 逐字体注册：单个失败不影响其余字体
        cx.text_system()
            .add_fonts(vec![Cow::Borrowed(data)])
            .map_err(|err| anyhow::anyhow!("{name}: {err}"))?;
    }
    Ok(())
}
