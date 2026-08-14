# DEP-0001: Doldskrift Enhancement Proposal Process

- **Status:** Accepted
- **Created:** 2026-08-12
- **Authors:** Doldskrift maintainers

## Abstract

Defines how design decisions for Doldskrift (protocol, glyph engine, tooling, security wording) are proposed, reviewed, and recorded.

## Motivation

DSK/1, MGE/2, and the multi-language SDKs must stay interoperable. Ad-hoc chat decisions do not survive across releases. DEPs create a durable, citable record tied to code and golden vectors.

## Process

### 1. Draft

1. Copy the structure of this file into `deps/DEP-XXXX-short-title.md` (next free number).
2. Set **Status** to `Draft`.
3. Fill Abstract, Motivation, Specification, Rationale, Compatibility, Security/Accessibility, Open questions.
4. Open a PR titled `docs(dep): DEP-XXXX …`.

### 2. Review

Maintainers and interested implementers review for:

- Normative clarity (MUST / SHOULD / MAY per RFC 2119)
- Impact on Rust and JavaScript (and vectors)
- Security honesty (no encryption claims)
- Accessibility impact
- Forward compatibility with DSK/1 parsers

### 3. Accept or reject

- **Accepted** — merge; implement in a follow-up PR if not already done.
- **Rejected** — merge with Status `Rejected` and a short reason, or close without merge if abandoned.
- **Superseded** — point to the replacement DEP.

### 4. Implementation

Accepted DEPs that change wire or mapping behavior MUST:

1. Update `spec/DOLDSKRIFT-1.md` (or a linked normative doc).
2. Add or update `spec/vectors/` when encode/decode outcomes change.
3. Keep Rust and JS in lockstep before release.

## Document structure (required sections)

```markdown
# DEP-XXXX: Title

- Status: Draft | Accepted | Rejected | Superseded
- Created: YYYY-MM-DD
- Authors: …

## Abstract
## Motivation
## Specification
## Rationale
## Compatibility
## Security and accessibility
## Open questions / future work
```

## What needs a DEP?

| Needs a DEP | Usually does not |
|-------------|------------------|
| Mapping algorithm changes | Typo fixes in docs |
| Container field layout | Pure refactors |
| New protocol mode or version | CI config |
| MGE major behavior / fingerprint format | Sample site polish |
| Security model wording that products rely on | Dependency bumps |

## Compatibility

This process itself is non-normative for wire parsers. It governs how humans change the normative corpus.

## Security and accessibility

DEPs that touch encoding, discovery, or UI MUST include an explicit security and accessibility section. “N/A” is only allowed for pure process docs (this one).

## Open questions

- Whether experimental vision/ECC land as DSK/1 extensions or wait for DSK/2 (tracked in research docs).
