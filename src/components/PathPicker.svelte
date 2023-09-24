<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { repos, type Repo } from "../lib/stores/repos";
  import { repoPath } from "../lib/stores/repoPath";
  import { onMount } from "svelte";
  import { open } from "@tauri-apps/api/dialog";
  import { homeDir } from "@tauri-apps/api/path";

  async function listRepos() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const listedRepos: Repo[] = (await invoke("list_repos", {
      repoPath: $repoPath,
    })) as Repo[];
    repos.set(listedRepos.sort((a, b) => a.path.localeCompare(b.path)));
  }

  async function pickDir() {
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
    });
    const homeDirPath = await homeDir();

    // Multiple is not currently possible but we have it here for TS and future proofing purposes
    if (Array.isArray(selected) || selected == null) {
      return;
    }

    const homeReplacedPath = selected.replace(homeDirPath, "~/");
    repoPath.set(homeReplacedPath);
    const listedRepos: Repo[] = (await invoke("list_repos", {
      repoPath: $repoPath,
    })) as Repo[];
    repos.set(listedRepos.sort((a, b) => a.path.localeCompare(b.path)));
  }

  onMount(() => {
    listRepos();
  });
</script>

<div class="path-picker">
  <label for="repoPath"
    ><span> Path </span>
    <input
      type="text"
      name="repoPath"
      id="repoPath"
      disabled
      bind:value={$repoPath}
    />
  </label>
  <button on:click={pickDir}>Pick Directory</button>
</div>

<style>
  .path-picker {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    gap: 32px;
    border-bottom: 1px solid var(--color-primary);
  }
  label {
    flex-grow: 0.75;
    margin: 0.5rem 0;

    display: flex;
    align-items: center;
    flex-direction: row;
    justify-content: space-between;
    width: 100%;
    gap: 16px;
  }
  label span {
    flex-grow: 1;
    flex-shrink: 0;
  }
  input {
    border: none;
    /* background: none; */
    border-radius: 8px;
    font-family: var(--font-mono);
    font-size: 1rem;
    /* color: var(--color-primary); */
    padding: 0.5rem 1rem;
    margin: 0.5rem 0;
    width: 100%;
  }
  button {
    flex-shrink: 0;
  }
</style>
