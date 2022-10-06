<script lang="ts">
  import InitialState from "./lib/InitialState.svelte";
  import {State} from "./types/State";
  import RunningState from "./lib/RunningState.svelte";
  import {Tab} from "./types/Tab";
  import ViewTab from "./lib/ViewTab.svelte";
  import { SvelteToast } from '@zerodevx/svelte-toast'

  let state = State.Initial;
  let tab = Tab.Run;
</script>

<main>

  <SvelteToast/>

  <div class="tab-buttons-wrapper">
    <button class:selected="{tab === Tab.Run}" on:click={() => tab = Tab.Run}>Run</button>
    <button class:selected="{tab === Tab.View}" on:click={() => tab = Tab.View}>View</button>
  </div>

  <div style="display: {tab === Tab.View ? 'block' : 'none' }">
    <ViewTab />
  </div>

  <div style="display: {tab === Tab.Run ? 'block' : 'none' }">
    {#if (state === State.Initial)}
      <InitialState bind:state={state} />
    {:else if (state === State.Running) || (state === State.Paused)}
      <RunningState bind:state={state} />
    {/if}
  </div>

</main>

<style type="scss">
  .tab-buttons-wrapper{
    display: flex;
    flex-direction: row;
    justify-content: flex-start;
    margin-bottom: 1rem;
    & button{
      background-color: initial;
      border-radius: 0;
    }
    & button.selected{
      border-top: 3px solid var(--color-accent-secondary);
    }
  }
</style>
