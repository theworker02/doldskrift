import {
  encode,
  decode,
  Session,
  type Mode,
} from "@doldskrift/core";
import {
  createElement,
  useMemo,
  type ElementType,
  type HTMLAttributes,
  type ReactNode,
} from "react";

export type DoldskriftMode = Mode;

export interface DoldskriftOptions {
  mode?: DoldskriftMode;
  seed?: string | Uint8Array;
  sessionId?: string;
}

function childrenToText(children: ReactNode): string {
  if (children == null || typeof children === "boolean") return "";
  if (typeof children === "string" || typeof children === "number") {
    return String(children);
  }
  if (Array.isArray(children)) {
    return children.map(childrenToText).join("");
  }
  return "";
}

function encodeForMode(text: string, options: DoldskriftOptions = {}): string {
  const mode = options.mode ?? "encoded";
  if (mode === "visual") return text;
  if (mode === "session") {
    const session = Session.builder()
      .seed(options.seed ?? "")
      .sessionId(options.sessionId ?? "react")
      .build();
    return session.encode(text);
  }
  return encode(text);
}

function decodeForMode(encoded: string, options: DoldskriftOptions = {}): string {
  const mode = options.mode ?? "encoded";
  if (mode === "visual") return encoded;
  if (mode === "session") {
    const session = Session.builder()
      .seed(options.seed ?? "")
      .sessionId(options.sessionId ?? "react")
      .build();
    return session.decode(encoded);
  }
  return decode(encoded);
}

/** Encode `text` for display according to mode. */
export function useDoldskrift(
  text: string,
  options: DoldskriftOptions = {},
): { original: string; display: string; mode: DoldskriftMode } {
  const mode = options.mode ?? "encoded";
  const seedKey =
    typeof options.seed === "string"
      ? options.seed
      : options.seed
        ? Array.from(options.seed).join(",")
        : "";
  return useMemo(() => {
    return {
      original: text,
      display: encodeForMode(text, options),
      mode,
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [text, mode, seedKey, options.sessionId]);
}

/** Decode encoded text back to semantic UTF-8. */
export function useDoldskriftDecoder(
  encoded: string,
  options: DoldskriftOptions = {},
): { encoded: string; text: string; mode: DoldskriftMode } {
  const mode = options.mode ?? "encoded";
  const seedKey =
    typeof options.seed === "string"
      ? options.seed
      : options.seed
        ? Array.from(options.seed).join(",")
        : "";
  return useMemo(() => {
    return {
      encoded,
      text: decodeForMode(encoded, options),
      mode,
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [encoded, mode, seedKey, options.sessionId]);
}

export interface DoldskriftProps extends HTMLAttributes<HTMLElement> {
  mode?: DoldskriftMode;
  seed?: string | Uint8Array;
  sessionId?: string;
  as?: ElementType;
  className?: string;
  children?: ReactNode;
}

/**
 * Thin React wrapper for DOLDSKRIFT/1.
 *
 * - visual: unchanged text with `doldskrift agent-text` classes
 * - encoded/session: displays encoded PUA text; `aria-label` carries semantic text
 */
export function Doldskrift({
  mode = "encoded",
  seed,
  sessionId,
  as = "span",
  className,
  children,
  ...rest
}: DoldskriftProps) {
  const original = childrenToText(children);
  const display = encodeForMode(original, { mode, seed, sessionId });

  const classes = ["doldskrift", "agent-text", className].filter(Boolean).join(" ");

  const props: Record<string, unknown> = {
    ...rest,
    className: classes,
    "data-doldskrift": "1",
    "data-doldskrift-mode": mode,
    "data-doldskrift-version": "1",
  };

  if (mode === "visual") {
    props.children = children;
  } else {
    props.children = display;
    if (original) {
      props["aria-label"] = original;
    }
  }

  return createElement(as, props);
}

export {
  encode,
  decode,
  Session,
  BRAND_TOKENS,
  THANKS_DEV_URL,
  FUNDING_PAGE_URL,
  fundingInfo,
  DSK_PROJECT_MARK,
} from "@doldskrift/core";
export type { Mode };
