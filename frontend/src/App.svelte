<script lang="ts">
  import InitialState from "./lib/InitialState.svelte";
  import {State} from "./types/State";
  import RunningState from "./lib/RunningState.svelte";
  import PausedState from "./lib/PausedState.svelte";
  import StoppedState from "./lib/StoppedState.svelte";

  let state = State.Initial;

</script>

<main>
  <h1>Nasoone</h1>
  {#if (state === State.Initial)}
    <InitialState bind:state={state} />
  {:else if (state === State.Running)}
    <RunningState bind:state={state} />
  {:else if (state === State.Paused)}
    <PausedState bind:state={state} />
  {:else if (state === State.Stopped)}
    <StoppedState bind:state={state} />
  {/if}
</main>

<style>
</style>
