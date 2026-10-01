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

use std::borrow::Cow;

use gpui::App;

pub const TEXT_FONT_FAMILY: &str = "Roboto";

pub fn install(cx: &mut App) -> anyhow::Result<()> {
    const ROBOTO_REGULAR: &[u8] = include_bytes!("fonts/Roboto-Regular.ttf");
    const ROBOTO_MEDIUM: &[u8] = include_bytes!("fonts/Roboto-Medium.ttf");

    for (name, data) in [
        ("Roboto Regular", ROBOTO_REGULAR),
        ("Roboto Medium", ROBOTO_MEDIUM),
    ] {
        cx.text_system()
            .add_fonts(vec![Cow::Borrowed(data)])
            .map_err(|err| anyhow::anyhow!("{name}: {err}"))?;
    }
    Ok(())
}
