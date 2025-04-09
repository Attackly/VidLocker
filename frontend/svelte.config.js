import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";
import { build } from "vite";

const config = {
  kit: {
    adapter: adapter({
      fallback: "index.html", // ✅ Enables SPA mode
    }),
    prerender: {
      entries: [], // ✅ Stops Svelte from forcing full prerendering
    },
  },
};
export default config;
