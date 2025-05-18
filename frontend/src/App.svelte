<script lang="ts">
  import { type View } from "./types";
  import { PlayerView } from "./views";

  let view = $state<View>("player");
</script>

<div class="app">
  {#if view === "player"}
    <PlayerView />
  {:else}
    Welcome to NFN
  {/if}
</div>
