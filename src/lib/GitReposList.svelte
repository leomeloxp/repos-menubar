<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/shell";
  import { repoPath } from "./stores/repoPath";
  import { onMount } from "svelte";

  let repos: { path: string; full_path: string; branch_name: string }[] = [];

  async function listRepos() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    repos = await invoke("list_repos", {
      repoPath: $repoPath,
    });
  }

  onMount(() => {
    listRepos();
  });
</script>

<div>
  <label for="repoPath"
    ><span> Repositories Path </span>
    <input type="text" name="repoPath" id="repoPath" bind:value={$repoPath} />
  </label>
  <button type="button" on:click={listRepos}>List Repos</button>
  <div class="repo-list">
    {#each repos as repo}
      <pre class="repo"><strong>{repo.path}</strong>: {repo.branch_name}</pre>
      <button
        on:click={() => {
          open(`vscode-insiders://file/${repo.full_path}/`);
        }}
        >vci
      </button>
      <button
        on:click={() => {
          open(`vscode://file/${repo.full_path}/`);
        }}
        >vc
      </button>
    {/each}
  </div>
</div>

<style>
  .repo-list {
    margin-top: 1rem;
  }

  .repo {
    margin: 0.5rem 0;
  }
</style>
