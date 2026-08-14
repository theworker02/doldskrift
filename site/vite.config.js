import { defineConfig } from "vite";
import { resolve } from "node:path";

/**
 * Static multi-page site for GitHub Pages.
 * Relative `base` so assets work under https://<user>.github.io/doldskrift/
 * (and locally via `vite preview`).
 */
export default defineConfig({
  base: "./",
  build: {
    outDir: "dist",
    emptyOutDir: true,
    rollupOptions: {
      input: {
        main: resolve(__dirname, "index.html"),
        product: resolve(__dirname, "product.html"),
        technology: resolve(__dirname, "technology.html"),
        demo: resolve(__dirname, "demo.html"),
        lens: resolve(__dirname, "lens.html"),
        visualRoundtrip: resolve(__dirname, "examples/visual-roundtrip.html"),
        blackPage: resolve(__dirname, "black-page.html"),
        partners: resolve(__dirname, "partners.html"),
        funding: resolve(__dirname, "funding.html"),
        docs: resolve(__dirname, "docs.html"),
        install: resolve(__dirname, "install.html"),
        spec: resolve(__dirname, "spec.html"),
        playground: resolve(__dirname, "playground.html"),
        font: resolve(__dirname, "font.html"),
        protocol: resolve(__dirname, "protocol.html"),
        benchmarks: resolve(__dirname, "benchmarks.html"),
        security: resolve(__dirname, "security.html"),
        glyphs: resolve(__dirname, "glyphs.html"),
        machineReader: resolve(__dirname, "machine-reader.html"),
        studio: resolve(__dirname, "studio.html"),
        radio: resolve(__dirname, "radio.html"),
        surfaces: resolve(__dirname, "surfaces.html"),
        neural: resolve(__dirname, "neural.html"),
        neuralLens: resolve(__dirname, "neural-lens.html"),
        unknownPage: resolve(__dirname, "unknown-page.html"),
        compatibility: resolve(__dirname, "compatibility.html"),
        notFound: resolve(__dirname, "404.html"),
      },
    },
  },
});
