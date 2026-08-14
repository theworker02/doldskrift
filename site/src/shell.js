/** Shared chrome for the static GitHub Pages site. */

const PRIMARY = [
  { href: "./index.html", id: "home", label: "Home" },
  { href: "./product.html", id: "product", label: "Product" },
  { href: "./demo.html", id: "demo", label: "Demo" },
  { href: "./technology.html", id: "tech", label: "Technology" },
  { href: "./neural.html", id: "neural", label: "Neural" },
  { href: "./benchmarks.html", id: "bench", label: "Research" },
  { href: "./docs.html", id: "docs", label: "Docs" },
];

const MORE = [
  { href: "./lens.html", id: "lens-direct", label: "Lens" },
  { href: "./surfaces.html", id: "surfaces", label: "Surfaces" },
  { href: "./neural-lens.html", id: "neural-lens", label: "Neural Lens" },
  { href: "./install.html", id: "install", label: "Install" },
  { href: "./unknown-page.html", id: "unknown", label: "Unknown Page" },
  { href: "./compatibility.html", id: "compat", label: "Compatibility" },
  { href: "./examples/visual-roundtrip.html", id: "examples", label: "Examples" },
  { href: "./black-page.html", id: "black", label: "Black Page" },
  { href: "./studio.html", id: "studio", label: "Studio" },
  { href: "./radio.html", id: "radio", label: "Radio" },
  { href: "./machine-reader.html", id: "reader", label: "Reader (experimental)" },
  { href: "./playground.html", id: "play", label: "Playground" },
  { href: "./glyphs.html", id: "glyphs", label: "Glyphs" },
  { href: "./font.html", id: "font", label: "Font" },
  { href: "./protocol.html", id: "protocol", label: "Protocol" },
  { href: "./spec.html", id: "spec", label: "Spec" },
  { href: "./security.html", id: "security", label: "Security" },
  { href: "./partners.html", id: "partners", label: "Partners" },
  { href: "./funding.html", id: "funding", label: "Support" },
  { href: "https://github.com/doldskrift/doldskrift", id: "github", label: "GitHub" },
];

export function mountShell({ active = "home", base = "./" } = {}) {
  const b = base.endsWith("/") ? base : `${base}/`;
  const href = (rel) => `${b}${rel.replace(/^\.\//, "")}`;
  const asset = (name) => `${b}${name.replace(/^\.\//, "")}`;
  ensureHeadMeta(asset);
  const wrap = document.querySelector(".wrap") || document.body;
  document.querySelectorAll("nav.nav, footer:not(.site-footer)").forEach((el) => el.remove());
  if (!document.querySelector(".site-nav")) {
    const nav = document.createElement("nav");
    nav.className = "site-nav";
    const link = (p) =>
      `<a href="${href(p.href)}" class="${p.id === active ? "active" : ""}" data-nav="${p.id}">${p.label}</a>`;
    nav.innerHTML = `
      <a class="brand" href="${href("./index.html")}" aria-label="Doldskrift home">
        <img class="brand-mark" src="${asset("logo-mark.svg")}" width="36" height="36" alt="" />
        <span class="brand-word">
          <span class="brand-name">Doldskrift <span class="brand-flag" title="Swedish-rooted name" aria-label="Sweden">🇸🇪</span></span>
          <span class="brand-tag">dold + skrift · concealed script</span>
        </span>
      </a>
      <button class="nav-toggle" type="button" aria-label="Open menu" aria-expanded="false">☰</button>
      <div class="nav-links">
        ${PRIMARY.map(link).join("")}
        <details class="nav-more">
          <summary>More</summary>
          <div class="nav-more-panel">${MORE.map(link).join("")}</div>
        </details>
      </div>
    `;
    wrap.prepend(nav);
    nav.querySelector(".nav-toggle")?.addEventListener("click", () => {
      const open = nav.classList.toggle("open");
      nav.querySelector(".nav-toggle").setAttribute("aria-expanded", String(open));
    });
  }
  if (!document.querySelector(".site-footer")) {
    const footer = document.createElement("footer");
    footer.className = "site-footer";
    footer.innerHTML = `
      <div class="footer-brand">
        <img src="${asset("logo-mark.svg")}" width="28" height="28" alt="" />
        <div>
          <strong>Doldskrift <span aria-hidden="true">🇸🇪</span></strong>
          <span>dold + skrift → concealed script · machine-native visual information · Apache-2.0 OR MIT</span>
        </div>
      </div>
      <div class="footer-links">
        <a href="${href("./product.html")}">Product</a>
        <a href="${href("./demo.html")}">Demo</a>
        <a href="${href("./technology.html")}">Technology</a>
        <a href="${href("./neural.html")}">Neural</a>
        <a href="${href("./lens.html")}">Lens</a>
        <a href="${href("./surfaces.html")}">Surfaces</a>
        <a href="${href("./install.html")}">Install</a>
        <a href="${href("./docs.html")}">Docs</a>
        <a href="${href("./security.html")}">Not encryption</a>
        <a href="${href("./partners.html")}">Partners</a>
        <a href="${href("./funding.html")}">Sponsor</a>
        <a href="https://thanks.dev/u/gh/theworker02" rel="noopener">thanks.dev</a>
        <a href="https://github.com/doldskrift/doldskrift">GitHub</a>
        <a href="https://github.com/doldskrift/doldskrift/blob/main/SUPPORT.md">Support</a>
      </div>
      <p class="footer-note">Maskinskriven betydelse — inte hemlighet. Designed for agents. Static GitHub Pages. Hidden ≠ encrypted (Open / Neural).</p>
    `;
    wrap.append(footer);
  }
}

function ensureHeadMeta(asset) {
  const head = document.head;
  const faviconHref = asset("favicon.svg");
  if (!head.querySelector(`link[rel="icon"][href="${faviconHref}"]`) && !head.querySelector('link[rel="icon"]')) {
    const icon = document.createElement("link");
    icon.rel = "icon";
    icon.href = faviconHref;
    icon.type = "image/svg+xml";
    head.append(icon);
  }
  if (!head.querySelector('meta[name="theme-color"]')) {
    const tc = document.createElement("meta");
    tc.name = "theme-color";
    tc.content = "#F7F4EF";
    head.append(tc);
  }
  const ogImage = asset("social-card.svg");
  if (!head.querySelector('meta[property="og:image"]')) {
    const og = document.createElement("meta");
    og.setAttribute("property", "og:image");
    og.content = ogImage;
    head.append(og);
  }
}
