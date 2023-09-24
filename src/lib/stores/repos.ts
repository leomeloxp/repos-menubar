import { writable } from "svelte/store";

export type Repo = { path: string; full_path: string; branch_name: string };

export const repos = writable<Repo[]>([]);
