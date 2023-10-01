import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";

export type Repo = { name: string; full_path: string; branch_name: string };

export const repos = writable<Repo[]>([]);

export async function listRepos() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const listedRepos: Repo[] = (await invoke("list_repos")) as Repo[];
  repos.set(listedRepos.sort((a, b) => a.name.localeCompare(b.name)));
}
