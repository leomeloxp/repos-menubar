<script lang="ts">
  export let title: string;
  import { appWindow } from "@tauri-apps/api/window";

  async function closeWindow() {
    await appWindow.hide();
  }
</script>

<div>
  <h1>{title}</h1>
  <div class="action">
    <slot />
    <button on:click={closeWindow}>X</button>
  </div>
</div>

<style>
  div {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 32px;
  }
  h1 {
    text-align: center;
    width: 100%;
  }
</style>
