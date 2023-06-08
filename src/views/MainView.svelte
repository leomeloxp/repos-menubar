<script>
  import PathPicker from "../components/PathPicker.svelte";
  import TitleBar from "../components/TitleBar.svelte";
  import GitReposList from "../components/GitReposList.svelte";
  import { goToSettingsView } from "../lib/stores/route";
</script>

<TitleBar title="Repos Menubar">
  <button on:click={goToSettingsView}> Settings </button>
</TitleBar>
<PathPicker />
<GitReposList />
