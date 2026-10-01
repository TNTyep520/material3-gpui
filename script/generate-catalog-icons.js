const fs = require("fs");
const path = require("path");

const iconsDir = path.join("catalog", "src", "symbols_icons");
const registryPath = path.join("catalog", "src", "icons_registry.rs");
const poolDir =
  process.argv[2] || path.join("target", "icon_pool", "catalog", "symbols_icons");

function collectUsedNames(poolNames) {
  const used = new Set();
  const patterns = [
    /IconName::new\("([a-z0-9_]+)"\)/g,
    /Icon::new\("([a-z0-9_]+)"\)/g,
    /icon: "([a-z0-9_]+)"/g,
  ];
  const scan = (file, catalogOnly) => {
    const src = fs.readFileSync(file, "utf8");
    for (const re of patterns) {
      let match;
      while ((match = re.exec(src)) !== null) {
        used.add(match[1]);
      }
    }
    if (catalogOnly) {
      let match;
      const literal = /"([a-z0-9_]+)"/g;
      while ((match = literal.exec(src)) !== null) {
        if (poolNames.has(match[1])) {
          used.add(match[1]);
        }
      }
    }
  };
  const scanTree = (dir, catalogOnly) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const p = path.join(dir, entry.name);
      if (entry.isDirectory()) scanTree(p, catalogOnly);
      else if (entry.name.endsWith(".rs")) scan(p, catalogOnly);
    }
  };
  scanTree("catalog/src", true);
  scanTree("crates", false);
  return [...used].sort();
}

function main() {
  const poolNames = new Set(
    fs
      .readdirSync(poolDir)
      .filter((name) => name.endsWith(".svg"))
      .map((name) => name.slice(0, -4))
  );
  const used = collectUsedNames(poolNames);
  console.log("used icons:", used.length, "->", used.join(" "));

  const missing = used.filter((name) => !poolNames.has(name));
  if (missing.length) {
    console.error("MISSING in pool:", missing.join(" "));
    process.exit(1);
  }

  const keep = new Set(used.map((name) => name + ".svg"));
  for (const name of fs.readdirSync(iconsDir)) {
    if (!keep.has(name)) fs.rmSync(path.join(iconsDir, name));
  }
  for (const name of used) {
    fs.copyFileSync(
      path.join(poolDir, name + ".svg"),
      path.join(iconsDir, name + ".svg")
    );
  }

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
