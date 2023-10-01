<script lang="ts">
  import { open } from "@tauri-apps/api/dialog";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import { listRepos } from "../lib/stores/repos";

  async function pickDir() {
    // Open a selection dialog for directories
    const selected = await open({
      directory: true,
    });

    // Multiple is not currently possible but we have it here for TS and future proofing purposes
    if (Array.isArray(selected) || selected == null) {
      return;
    }

    const result: true | string = await invoke("add_new_repo", {
      repoPath: selected,
    });

    if (result !== true) {
      console.error(result);
      return;
    }
    await listRepos();
  }

  onMount(() => {
    listRepos();
  });
</script>

<div class="path-picker">
  <button on:click={pickDir}>add new repo</button>
</div>

<style>
  .path-picker {
    display: flex;
    flex-direction: row;
    justify-content: end;
    align-items: center;
    gap: 32px;
    border-bottom: 1px solid var(--color-primary);
  }
  button {
    flex-shrink: 0;
  }
</style>
