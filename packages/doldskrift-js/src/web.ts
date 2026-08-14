import { decode, encode } from "./codec.js";
import { Session } from "./session.js";

/**
 * Apply Doldskrift discovery attributes on a DOM element; encode when mode requires it.
 * Browser adapter helper — also available via thin `@doldskrift/web` re-export.
 */
export function renderDoldskrift(
  el: HTMLElement,
  text: string,
  options: {
    mode?: "visual" | "encoded" | "session";
    seed?: string;
  } = {}
): void {
  const mode = options.mode ?? "visual";
  el.classList.add("doldskrift", "agent-text");
  el.dataset.doldskrift = "1";
  el.dataset.doldskriftMode = mode;
  if (mode === "visual") {
    el.textContent = text;
    return;
  }
  el.setAttribute("aria-label", text);
  if (mode === "session") {
    const session = Session.builder().seed(options.seed ?? "").build();
    el.textContent = session.encode(text);
  } else {
    el.textContent = encode(text);
  }
}

/** Convenience alias for browser call sites. */
export function encodeDoldskrift(text: string): string {
  return encode(text);
}

/** Convenience alias for browser call sites. */
export function decodeDoldskrift(text: string): string {
  return decode(text);
}
