<script async lang="ts">
  import { invoke } from "@tauri-apps/api";
  import * as dialog from "@tauri-apps/api/dialog";
  import { desktopDir } from "@tauri-apps/api/path";
  import type { NasooneSettings } from "src/types/NasooneSettings";

  export let settings: NasooneSettings;

  // on startup, set the default output folder to desktop
  (async () => {
    const desktop_dir = (await desktopDir()) as string;
    settings.folder = desktop_dir;
  })();

  // on startup, load devices and set the default device to the first one in the list
  const load_devices = (async () => {
    const devices = (await invoke("get_devices")) as string;
    const device_list = JSON.parse(devices);
    settings.device = device_list[0];
    return device_list;
  })();

  // used to show the file dialog and choose the output folder
  const open_folder = async () => {
    const folder = await dialog.open({
      directory: true,
      multiple: false,
    });

    if (folder === null) {
      settings.folder = "";
    } else if (typeof folder === "string") {
      settings.folder = folder;
    } else {
      settings.folder = folder[0];
    }
  };
</script>

<div class="settings-wrapper">
  <div>
    {#await load_devices}
      <p>...waiting</p>
    {:then devices}
      <label for="device">Network interface:</label>
      <select name="device" id="device" bind:value={settings.device}>
        <option value="">Select an interface</option>
        {#each devices as device}
          <option value={device}>{device}</option>
        {/each}
      </select>
    {:catch _error}
      <p>An error occurred!</p>
    {/await}
  </div>
  <div>
    <label for="folder">Output folder:</label>
    <input
      type="text"
      name="folder"
      id="folder"
      readonly
      class="cursor-pointer"
      bind:value={settings.folder}
      on:click={open_folder}
    />
  </div>
  <div>
    <label for="filename">File name</label>
    <input
      type="text"
      id="filename"
      name="filename"
      bind:value={settings.file_name}
    />
  </div>
  <div>
    <label for="interval">Interval [s]</label>
    <input
      type="number"
      id="interval"
      min="1"
      max="1000"
      step="1"
      bind:value={settings.interval}
    />
  </div>
</div>

<style type="scss">
  .settings-wrapper{
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    margin: 0 auto;
    width: 100%;
    & div {
      width: 100%;
      margin: 0px 0px 15px 0px;
    }
  }
  label {
    display: block;
    margin: 0px 0px 5px 0px;
  }
</style>
