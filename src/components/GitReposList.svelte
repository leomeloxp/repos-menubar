<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/shell";
  import { repos, type Repo } from "../lib/stores/repos";
  import { repoPath } from "../lib/stores/repoPath";
  import { onMount } from "svelte";
  import RepositoryBlock from "./RepositoryBlock.svelte";

  async function listRepos() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const listedRepos: Repo[] = (await invoke("list_repos", {
      repoPath: $repoPath,
    })) as Repo[];
    repos.set(listedRepos.sort((a, b) => a.path.localeCompare(b.path)));
  }

  function change() {
    if (document.visibilityState === "visible") {
      listRepos();
    }
  }

  onMount(() => {
    listRepos();
    document.addEventListener("visibilitychange", change);
    return () => document.removeEventListener("visibilitychange", change);
  });
</script>

<div>
  <div class="repo-list">
    {#each $repos as repo}
      <RepositoryBlock {repo} />
    {/each}
  </div>
</div>

<style>
  .repo-list {
    margin-top: 1rem;
  }
</style>
