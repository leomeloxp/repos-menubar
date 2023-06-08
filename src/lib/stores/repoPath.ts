import { writable } from "svelte/store";

export const repoPath = writable("/", (set) => {
  const storedRepoPath = localStorage.getItem("repoPath");
  if (storedRepoPath) {
    set(storedRepoPath);
  }
});

repoPath.subscribe((value) => {
  localStorage.setItem("repoPath", value);
});
