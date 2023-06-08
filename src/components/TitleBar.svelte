<script lang="ts">
  import { goToSettingsView } from "../lib/stores/route";
  export let title: string;
</script>

<div>
  <h1>{title}</h1>
  <div class="action">
    <slot />
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
