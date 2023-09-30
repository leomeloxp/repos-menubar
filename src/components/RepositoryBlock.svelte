<script lang="ts">
  import { open } from "@tauri-apps/api/shell";
  import type { Repo } from "src/lib/stores/repos";
  import { settings } from "../lib/stores/settings";
  export let repo: Repo;
</script>

<div class="repository-block">
  <div class="info">
    <pre><strong>{repo.name}</strong>: {repo.branch_name}</pre>
  </div>
  <div class="actions">
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
</style>
