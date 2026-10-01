const fs = require("fs");
const path = require("path");

const iconsDir = path.join("catalog", "src", "symbols_icons");
const registryPath = path.join("catalog", "src", "icons_registry.rs");

function collectUsedNames() {
  const used = new Set();
  const scan = (file) => {
    const src = fs.readFileSync(file, "utf8");
    const re = /IconName::new\("([a-z0-9_]+)"\)/g;
    let match;
    while ((match = re.exec(src)) !== null) {
      used.add(match[1]);
    }
  };
  for (const base of ["catalog/src", "crates"]) {
    if (!fs.existsSync(base)) continue;
    const walk = (dir) => {
      for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
        const p = path.join(dir, entry.name);
        if (entry.isDirectory()) walk(p);
        else if (entry.name.endsWith(".rs")) scan(p);
      }
    };
    if (fs.statSync(base).isFile()) scan(base);
    else walk(base);
  }
  return [...used].sort();
}

function main() {
  const used = collectUsedNames();
  console.log("used icons:", used.length, "->", used.join(" "));

  const missing = used.filter((name) => !fs.existsSync(path.join(iconsDir, name + ".svg")));
  if (missing.length) {
    console.error("MISSING icon files:", missing.join(" "));
    process.exit(1);
  }

  const keep = new Set(used.map((name) => name + ".svg"));
  let removed = 0;
  for (const name of fs.readdirSync(iconsDir)) {
    if (!keep.has(name)) {
      fs.rmSync(path.join(iconsDir, name));
      removed += 1;
    }
  }
  console.log("removed", removed, "unused icons");

  const entries = used.map(
    (name) => `    ("${name}", include_bytes!(concat!("symbols_icons/", "${name}", ".svg"))),`
  );
  const content = [
    "// 由 script/generate-catalog-icons.js 生成;仅内嵌 catalog 实际使用的图标。",
    "pub static ICON_SVGS: &[(&str, &[u8])] = &[",
    entries.join("\n"),
    "];",
    "",
    "pub fn lookup(name: &str) -> Option<&'static [u8]> {",
    "    ICON_SVGS.iter().find(|(key, _)| *key == name).map(|(_, bytes)| *bytes)",
    "}",
    "",
  ];
  fs.writeFileSync(registryPath, content.join("\n"));
  console.log("registry written:", registryPath);
}

main();
