<script lang="ts">
  import { route } from "./lib/stores/route";
  import MainView from "./views/MainView.svelte";
  import SettingsView from "./views/SettingsView.svelte";
</script>

<div class="wrapper">
  <main class="container">
    {#if $route === "settings"}
      <SettingsView />
    {:else}
      <MainView />
    {/if}
  </main>
</div>

<style>
  .wrapper {
    position: relative;
  }

  .wrapper:before {
    content: " ";
    display: block;
    position: absolute;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    opacity: 0.2;
    background-image: url("/git-icon-logo.svg");
    background-repeat: no-repeat;
    background-position: bottom right;
    background-size: 50%;
  }

  .container {
    position: relative;
    padding: 32px 16px;
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
  }
</style>
