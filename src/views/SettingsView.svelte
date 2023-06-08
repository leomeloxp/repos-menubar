<script>
  import TitleBar from "../components/TitleBar.svelte";
  import { goToMainView } from "../lib/stores/route";
  import { settings } from "../lib/stores/settings";
</script>

<div>
  <TitleBar title="Settings">
    <button on:click={goToMainView}>Close</button>
  </TitleBar>
  <div class="settings-list">
    <label for="editor">
      <span>Editor to use</span>
      <select name="editor" id="editor" bind:value={$settings.editor}>
        <option value="vscode">VSCode</option>
        <option value="vscode-insiders">VSCode Insiders</option>
      </select>
    </label>
  </div>
</div>

<style>
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
  select {
    border: none;
    /* background: none; */
    border-radius: 8px;
    font-size: 1rem;
    /* color: var(--color-primary); */
    width: 100%;
    padding: 0.5rem 1rem;
    margin: 0.5rem 0;
    height: 36px;
  }
</style>
