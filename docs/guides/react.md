# React guide

Package: **`@doldskrift/react`** (depends on `@doldskrift/core`).

## Install

```bash
npm install @doldskrift/react @doldskrift/core
```

## `<Doldskrift>` component

```tsx
import { Doldskrift } from "@doldskrift/react";

export function AgentLine() {
  return (
    <Doldskrift mode="encoded" className="agent-text">
      hello agent
    </Doldskrift>
  );
}
```

### Props

| Prop | Default | Meaning |
|------|---------|---------|
| `mode` | `"encoded"` | `visual` \| `encoded` \| `session` |
| `seed` | `""` | Session seed (string or `Uint8Array`) |
| `sessionId` | `"react"` | Session id |
| `as` | `"span"` | Element type |
| `className` | — | Appended to `doldskrift agent-text` |
| `children` | — | Semantic text (stringified) |

Behavior:

- Sets `data-doldskrift`, `data-doldskrift-mode`, `data-doldskrift-version`.
- **visual:** children unchanged.
- **encoded/session:** children replaced by encoded PUA; `aria-label` = semantic text.

## Hooks

```tsx
import { useDoldskrift, useDoldskriftDecoder } from "@doldskrift/react";

const { original, display, mode } = useDoldskrift("ping", { mode: "session", seed: "alpha" });
const { text } = useDoldskriftDecoder(display, { mode: "session", seed: "alpha" });
```

## Accessibility

Do not remove `aria-label` for encoded content meant for people. See [accessibility](../accessibility.md).

## Styling

Load `DoldskriftText` (or Mono/Micro) and target `.doldskrift.agent-text`. Prefer CSS variables from your app theme rather than hard-coding purple “AI” defaults.
