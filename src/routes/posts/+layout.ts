// daisyUI assumes Tailwind CSS's Preflight
// import "@unocss/reset/tailwind.css";
// Import daisyUI **BEFORE** UnoCSS
// import "@kidonng/daisyui/index.css";
import "uno:theme.css";
import "uno:preflights.css";
import "uno:icons.css";
import "uno:typography.css";
import "uno:shortcuts.css";
import "uno:default.css";
import "uno:components.css";
import "uno.css";
import "uno:utilities.css";

export const prerender = true;

export const load = async () => {
  const today_date = new Date();
  return {
    today_date: today_date.toLocaleDateString(),
    today_year: today_date.getFullYear(),
  };
};
