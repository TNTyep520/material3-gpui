//! 内嵌资源源。
//!
//! 图标已改为字体字形渲染（见 [`crate::icon`]），不再内嵌图标 SVG；
//! 仅保留 [`CircularProgress` 旋转弧](crate::components::CircularProgress)
//! 所需的一个内部 SVG。[`Md3Assets`] 与 [`CombinedAssets`] 作为
//! 资源源工具保留，供用户组合自己的 `AssetSource`。
//!
//! ```ignore
//! gpui_platform::application().with_assets(Md3Assets).run(|cx| { ... })
//! ```

use anyhow::Result;
use gpui::{AssetSource, SharedString};
use std::borrow::Cow;

/// `CircularProgress` 旋转弧 SVG 的资源路径。
pub const PROGRESS_ARC_SVG_PATH: &str = "md3-icons/progress_arc.svg";

/// Material 站点图标(蓝色圆角方块 + 白圆盘 + M 徽标)的资源路径,
/// 供自定义标题栏/关于页等以 `img()` 渲染。
pub const MATERIAL3_FAVICON_SVG_PATH: &str = "md3-icons/material3-favicon.svg";

/// (asset_path, bytes)：仅保留内部需要的资源。
static RESOURCES: &[(&str, &[u8])] = &[
    (
        "md3-icons/progress_arc.svg",
        include_bytes!("assets/progress_arc.svg").as_slice(),
    ),
    (
        "md3-icons/material3-favicon.svg",
        include_bytes!("assets/material3-favicon.svg").as_slice(),
    ),
];

/// material3-gpui 的内嵌资源源
pub struct Md3Assets;

impl Md3Assets {
    /// 与另一个 AssetSource 组合：md3 内部资源优先，其余路径回退到 `fallback`。
    pub fn with_fallback(fallback: impl AssetSource) -> CombinedAssets {
        CombinedAssets {
            fallback: Box::new(fallback),
        }
    }

    fn find(path: &str) -> Option<&'static [u8]> {
        RESOURCES
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| *bytes)
    }
}

impl AssetSource for Md3Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(Md3Assets::find(path).map(Cow::Borrowed))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(RESOURCES
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}

/// [`Md3Assets`] 与用户资源源的组合体
pub struct CombinedAssets {
    fallback: Box<dyn AssetSource>,
}

impl AssetSource for CombinedAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = Md3Assets::find(path) {
            return Ok(Some(Cow::Borrowed(bytes)));
        }
        self.fallback.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut out: Vec<SharedString> = RESOURCES
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect();
        out.extend(self.fallback.list(path)?);
        Ok(out)
    }
}
