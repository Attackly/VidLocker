import { writable } from "svelte/store";

const isBrowser = typeof window !== "undefined";

const initialTheme = isBrowser
  ? localStorage.getItem("theme") || "bird"
  : "bird";

export const theme = writable(initialTheme);

theme.subscribe((value) => {
  if (isBrowser) {
    localStorage.setItem("theme", value);
    document.documentElement.setAttribute("data-theme", value);
  }
});
