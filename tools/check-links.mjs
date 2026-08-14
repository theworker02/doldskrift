#!/usr/bin/env node
/**
 * Internal link checker for README, docs, and site HTML.
 * Fails on missing relative file targets.
 */
import fs from "node:fs";
import path from "node:path";

const root = path.resolve(process.cwd());
const failures = [];

function walk(dir, pred, out = []) {
  if (!fs.existsSync(dir)) return out;
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    if (ent.name === "node_modules" || ent.name === "target" || ent.name === "dist") continue;
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) walk(p, pred, out);
    else if (pred(p)) out.push(p);
  }
  return out;
}

function checkMarkdown(file) {
  const text = fs.readFileSync(file, "utf8");
  const re = /\[[^\]]*\]\(([^)]+)\)/g;
  let m;
  while ((m = re.exec(text))) {
    let href = m[1].split("#")[0].split("?")[0].trim();
    if (!href || href.startsWith("http") || href.startsWith("mailto:") || href.startsWith("data:")) {
      continue;
    }
    const target = path.resolve(path.dirname(file), href);
    if (!fs.existsSync(target)) {
      failures.push(`${path.relative(root, file)} → missing ${href}`);
    }
  }
}

function checkHtml(file) {
  const text = fs.readFileSync(file, "utf8");
  const re = /(?:href|src)="([^"]+)"/g;
  let m;
  while ((m = re.exec(text))) {
    let href = m[1].split("#")[0].split("?")[0].trim();
    if (
      !href ||
      href.includes("${") ||
      href.startsWith("http") ||
      href.startsWith("mailto:") ||
      href.startsWith("data:") ||
      href.startsWith("/src/") ||
      href.startsWith("/logo") ||
      href.startsWith("/favicon") ||
      href.startsWith("/readme") ||
      href.startsWith("/specimen")
    ) {
      continue;
    }
    if (href.startsWith("/")) {
      // site public root — check site/public or site
      const pub = path.join(root, "site", "public", href.slice(1));
      const alt = path.join(root, "site", href.slice(1));
      if (!fs.existsSync(pub) && !fs.existsSync(alt)) {
        failures.push(`${path.relative(root, file)} → missing ${href}`);
      }
      continue;
    }
    const target = path.resolve(path.dirname(file), href);
    if (!fs.existsSync(target)) {
      failures.push(`${path.relative(root, file)} → missing ${href}`);
    }
  }
}

const md = walk(root, (p) => p.endsWith(".md") && !p.includes(`${path.sep}node_modules${path.sep}`));
const html = walk(path.join(root, "site"), (p) => p.endsWith(".html"));

for (const f of md) checkMarkdown(f);
for (const f of html) checkHtml(f);

if (failures.length) {
  console.error(`Link check failed (${failures.length}):`);
  for (const f of failures.slice(0, 80)) console.error("  " + f);
  if (failures.length > 80) console.error(`  … +${failures.length - 80} more`);
  process.exit(1);
}
console.log(`Link check OK (${md.length} markdown, ${html.length} HTML).`);
