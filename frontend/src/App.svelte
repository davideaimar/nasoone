<script lang="ts">
  import Setting from "./lib/Setting.svelte";
  import type { NasooneSettings } from "./types/NasooneSettings";
  import { invoke } from "@tauri-apps/api";

  let params: NasooneSettings = {
    device: "",
    folder: "",
    interval: 5,
    file_name: "nasoone_" + new Date().getTime(),
  };

  const start = async () => {
    try {
      const result = await invoke("start", {
        device: params.device, 
        outputFolder: params.folder, 
        filename: params.file_name,
        interval: params.interval
      });
      alert(result);
    } catch (e) {
      alert(e);
    }
  };

</script>

<main>
  <h1>Nasoone</h1>
  <p>Specify the parameters and start the capture.</p>
  <div>
    <Setting bind:settings={params} />
    <button id="start" class="bg-accent-secondary" on:click={start}>Start</button>
  </div>
  <div>
    <p>Logging: </p>
    <code>
      <pre>settings: {JSON.stringify(params, null, 2)}
      </pre>
    </code>
  </div>
</main>

<style>
</style>
