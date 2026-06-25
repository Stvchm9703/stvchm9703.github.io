/**
 * Blog post type definitions — barrel export.
 *
 * These types match the JSON produced by `tools/convert_blog_post` (Rust).
 * See `docs/data_model.md`.
 *
 * To make the blog consume these (instead of the legacy `tools/convert_blog_post_ts`
 * types), repoint the `$generateor` alias in `svelte.config.js` to `src/types`:
 *
 *   $generateor: "src/types",
 *   "$generateor/*": "src/types/*",
 *
 * The existing imports `$generateor/page` and `$generateor/content_block` then
 * resolve to `src/types/page.ts` and `src/types/content_block.ts`.
 */

export * from "./content_block";
export * from "./page";
