# Dependency Graph & Parallel-Execution Wave Plan

Generated 2026-06-24 from `docs/features.md` (Parts 1 & 3) and
`tools/ipynb_data_visualization/docs/implementation_plan.md` (Phases 0–4), with
schema context from `docs/data_model.md`.

> Status: **planned, not executed.** A human reviews and approves the waves below
> before any builder agent runs. The gated task (G1) needs explicit confirmation.

## Task index

| id | subsystem | depends_on | blocks | wave | gated |
|----|-----------|-----------|--------|------|-------|
| B1 | blog | — | B3 | 1 | no |
| B2 | blog | — | — | 1 | no |
| B4 | blog/config | — | G1 | 1 | no |
| R1 | rust | — | X1, G1 | 1 | no |
| R2 | rust | — | — | 1 | no |
| R3 | rust | — | X2, G1 | 1 | no |
| R4 | rust | — | G1 | 1 | no |
| R5 | rust | — | — | 1 | no |
| P0 | python | — | P1 | 1 | no |
| D1 | rust+blog (decision) | — | — | 1 | no |
| B3 | blog | B1 | — | 2 | no |
| X1 | blog | R1 | — | 2 | no |
| X2 | blog | R3 | — | 2 | no |
| P1 | python | P0 | P2, P3 | 2 | no |
| P2 | python+blog | P1 | P4 | 3 | no |
| P3 | python+rust+blog | P1 | P4 | 3 | no |
| P4 | python | P2, P3 | — | 4 | no |
| G1 | repo (destructive) | B4, R1, R3, R4 | — | gated | **YES** |

## Dependency graph (edges: A → B means B depends on A)

```
B1 ──> B3
R1 ──> X1
R1 ──> G1*
R3 ──> X2
R3 ──> G1*
B4 ──> G1*
R4 ──> G1*
P0 ──> P1 ──> P2 ──> P4
              P1 ──> P3 ──> P4
(B2, R2, R5, D1 : no dependents)

* G1 is GATED — not auto-scheduled into a wave; runs only after human confirmation.
```

Acyclicity: every edge points from an earlier wave to a later wave (or into the
gated node). No back-edges ⇒ no cycles. Every `depends_on` references a real task id.

## Parallel-execution plan

- **Wave 1 (10 tasks):** `B1, B2, B4, P0, D1` (parallel) + the **Rust lane**
  `R1 → R2 → R3 → R4 → R5` (see decision below).
  - **DECIDED:** the whole Rust lane (`R1, R2, R3, R4, R5`) is assigned to a
    **single builder, executed sequentially** (R1 first). They share
    `page/mod.rs` and related files, so one owner avoids line-level conflicts.
    The 5 blog/python/decision tasks (`B1, B2, B4, P0, D1`) run in parallel
    alongside that single Rust builder.
- **Wave 2 (4 tasks, parallel):** `B3, X1, X2, P1`
  - `B3` needs `B1` (same file). `X1` needs `R1`. `X2` needs `R3`. `P1` needs `P0`.
- **Wave 3 (2 tasks, parallel):** `P2, P3`
  - Both need `P1`. `P3` also reuses `P2`'s blog renderer for display; if run truly
    concurrently, `P3` should render defensively / coordinate on the component.
- **Wave 4 (1 task):** `P4` — needs `P2` and `P3`.
- **Gated (separate, manual):** `G1` — after `B4, R1, R3, R4` are Done AND a human
  confirms. Not part of any automatic wave.

## Notes, ambiguities resolved

1. **Verify command for the blog.** Repo has BOTH `bun.lock` (2026-01-19) and the
   newer `pnpm-lock.yaml` (2026-03-14); `package.json` has no `packageManager`
   field and its scripts mix `bun run`/`bunx`. I chose **`pnpm`** as the primary
   verify runner (newer lockfile) with **bun** as the documented fallback. The
   check script itself is `svelte-kit sync && svelte-check --tsconfig ./tsconfig.json`.
   → **DECIDED (2026-06-24): `pnpm` is canonical.** `pnpm check` is the verify
   command; the `bunx` lines in task docs are a local-only fallback.
2. **`page/mod.rs` write contention — DECIDED (2026-06-24).** R1–R5 share
   `page/mod.rs` and related files. The **entire Rust lane (`R1 → R2 → R3 → R4 →
   R5`) is assigned to one builder and executed sequentially** (R1 first). This is
   one serial lane within wave 1; the other wave-1 tasks (`B1, B2, B4, P0, D1`)
   run in parallel beside it.
3. **R4 (CustomComponent) and R5 (user/workspace) are decision-bearing/optional.**
   R4 must produce a written decision (drop vs implement) because G1 depends on it
   to know whether a Part-1 gap is intentionally dropped. R5 may legitimately end as
   a documented deferral. Severity per `features.md`: R4 Medium, R5/R2 Low.
3a. **G1 hard blockers.** I gated G1 on `B4, R1, R3, R4` (the Part-1 gaps the report
    flags as genuine: #1 coverImage, #7 assets, #5 CustomComponent, plus the alias).
    R2/R5 are Low/optional and are intentionally NOT hard blockers — but their
    status should be noted at the gate.
4. **P3 vs P2 ordering.** Per the plan, P3 depends only on P1 (different carrier),
   but it reuses P2's blog renderer to display `data_tables`. I kept P2 and P3 in
   the same wave (both gated on P1) and noted that P3 should coordinate on / render
   defensively against the P2 component if they run concurrently. If you prefer a
   hard serialization, make P3 depend on P2 (moving P3 to wave 4 and P4 to wave 5).
5. **Drawio/Mermaid (`Latex` processor)** — features.md Part 3 #5 is now task
   **D1** (wave 1, decision/spike): `docs/tasks/D1_drawio_mermaid_latex_delivery.md`.
   The normalized-table shape (Part 1 #3/#4) remains **not** tasked — marked
   "Info / no action needed (parity acceptable)".
