<script lang="ts">
  import { route } from "./lib/stores/route";
  import MainView from "./views/MainView.svelte";
  import SettingsView from "./views/SettingsView.svelte";
</script>

<main class="container">
  {#if $route === "settings"}
    <SettingsView />
  {:else}
    <MainView />
  {/if}
</main>
