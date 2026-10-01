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

use gpui::{AssetSource, SharedString};
use material3_gpui::assets::Md3Assets;
use std::borrow::Cow;

mod registry {
    include!("icons_registry.rs");
}

pub struct CatalogAssets;

impl AssetSource for CatalogAssets {
    fn load(&self, path: &str) -> gpui::Result<Option<Cow<'static, [u8]>>> {
        if let Some(name) = path
            .strip_prefix("md3-icons/")
            .and_then(|rest| rest.strip_suffix(".svg"))
            && let Some(bytes) = registry::lookup(name)
        {
            return Ok(Some(Cow::Borrowed(bytes)));
        }
        Md3Assets::new().load(path)
    }

    fn list(&self, path: &str) -> gpui::Result<Vec<SharedString>> {
        let mut out: Vec<SharedString> = registry::ICON_SVGS
            .iter()
            .map(|(name, _)| SharedString::from(format!("md3-icons/{name}.svg")))
            .filter(|candidate| candidate.starts_with(path))
            .collect();
        out.extend(Md3Assets::new().list(path)?);
        Ok(out)
    }
}
