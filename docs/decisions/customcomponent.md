# Decision: CustomComponent (R4)

**Date:** 2026-06-24
**Status:** Decided — Option A (Drop / defer)
**Decider:** Builder agent (Rust lane)

---

## Context

The `CustomComponent` feature is a relation-driven custom embed: a text block
whose content matches a pattern like `/custom_component:jupyter:(file:cell)/`
signals that the following `File` block should be rendered as a custom widget
rather than a raw file.

The legacy TS tool had `resolveRelationCustomComponent` (commented out in
`page.ts`) and a `"CustomComponent"` `componentType` token, but the feature
was never fully wired up. The blog today has an empty `CustomComponent`
rendering branch — it renders nothing.

The Rust tool emits `Unknown { original_type: "relation" }` for relation blocks
(see `docs/data_model.md` §6).

### Options evaluated

**Option A — Drop (defer).** Treat relation-driven embeds as intentionally
unsupported for now. The blog's empty `CustomComponent` branch stays a no-op.
The `Unknown` fallback in the Rust tool is already the correct on-disk shape.
G1 (retiring the TS tool) can proceed knowing this gap is intentionally not
ported.

**Option B — Implement.** Add a `CustomComponent` `ComponentAttr` variant plus
a resolver in `tools/convert_blog_post/src/export_model/content_block/`, then
update `docs/data_model.md`. A follow-up blog task would wire up the rendering.

---

## Decision: Option A — Drop / Defer

### Rationale

1. **No active consumer.** The blog's `CustomComponent` branch is empty today;
   adding an emitter with no renderer has zero user-visible impact.

2. **Scope and risk.** Implementing Option B requires defining a new
   `ComponentAttr` variant, a resolver that detects the pattern across adjacent
   blocks (stateful traversal of the content tree), and new tests — a non-trivial
   change with a risk of regressions to existing block ordering.

3. **TS precedent.** The feature was commented out in the TS tool too, implying
   it was never production-ready. Porting a half-finished feature adds
   maintenance burden without clear benefit.

4. **Wave-1 priority.** R1 (coverImage), R2 (description), R3 (dual-asset copy)
   directly unblock the blog's OG/cover rendering and asset serving (higher
   severity). CustomComponent is Medium severity and has no dependents in wave 1.

### Consequence

- The Rust tool continues to emit `Unknown { original_type: "relation" }` for
  relation blocks. The blog ignores them (renders nothing), as today.
- G1 (retire TS tool) may proceed; this Part-1 gap is **intentionally not ported**.
- A follow-up task should be created when the blog is ready to render custom
  embeds: at that point both the Rust emitter (Option B above) and the Svelte
  renderer need to be implemented together.

### Follow-up note

When CustomComponent is eventually implemented, the Rust change should:
- Add `CustomComponent { relation_key: String, file_url: Option<String> }` to
  `ComponentAttr` (following the internally-tagged `componentType` union pattern).
- Detect the pattern: a `Text { style: Code }` block whose text matches
  `/custom_component:<type>:(...)//` immediately followed by a `File` block.
- Consume the `File` block (remove it from `contents`) and emit a single
  `CustomComponent` block in its place.
- Update `docs/data_model.md` §6 to document the new variant.
- Add tests in `export_model/content_block/` covering both detection and the
  `Unknown` fallback for non-matching relation blocks.
