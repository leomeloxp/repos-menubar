<script lang="ts">
  import { open } from "@tauri-apps/api/shell";
  import { listRepos, type Repo } from "../lib/stores/repos";
  import { settings } from "../lib/stores/settings";
  import { invoke } from "@tauri-apps/api/tauri";
  export let repo: Repo;

  async function handleRemoveRepo() {
    const result: true | string = await invoke("remove_repo", {
      repoPath: repo.full_path,
    });

    if (result !== true) {
      console.error(result);
      return;
    }
    await listRepos();
  }
</script>

<div class="repository-block">
  <div class="info">
    <pre><strong>{repo.name}</strong>: {repo.branch_name}</pre>
  </div>
  <div class="actions">
    <button
      class="intent--danger"
      on:click={() => {
        handleRemoveRepo();
      }}
      title={`Remove "${repo.name}" from Repos Menubar`}
    >
      remove
    </button>
    <button
      on:click={() => {
        open(`${$settings.editor}://file/${repo.full_path}/`);
      }}
      title={`Open "${repo.name}"`}
    >
      open
    </button>
  </div>
</div>

<style>
  .repository-block {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid var(--color-primary);
  }
  .info {
    margin: 0.5rem 0;
    font-family: var(--font-mono);
    overflow-x: scroll;
  }
  .actions {
    flex-shrink: 0;
    padding: 0 0.5rem;
  }
  .actions button {
    cursor: pointer;
    margin: 0px;
    padding: 0.5rem 1rem;
  }
  .actions button.intent--danger {
    background-color: var(--colour-alert);
  }
</style>
