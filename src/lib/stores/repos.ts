import { writable } from "svelte/store";

export type Repo = { name: string; full_path: string; branch_name: string };

export const repos = writable<Repo[]>([]);
