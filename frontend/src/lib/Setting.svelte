<script async lang="ts">
  import {invoke} from "@tauri-apps/api";
  import * as dialog from "@tauri-apps/api/dialog";
  import {desktopDir} from "@tauri-apps/api/path";
  import type {NasooneSettings} from "src/types/NasooneSettings";

  export let settings: NasooneSettings;

  // on startup, set the default output folder to desktop
  (async () => {
    settings.folder = (await desktopDir()) as string;
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

  let filter_types = ["BPF", "Easy filtering"];

  let ip_filter = '';

  let mac_filter = '';

  let port_filter = '';

  let proto_filter = '';

  let filter = '';

  let html = 0;

  const set_filters = async () => {
    if (html === 2) {
      settings.filter = ip_filter.concat(mac_filter).concat(port_filter).concat(proto_filter);
    }
  }

  const handleFlag = async () => {
    if (filter === "BPF") {
      html = 1;
    }
    else if (filter === "Easy filtering") {
      html = 2;
    }
    else {
      html = 0;
    }
  }

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
  <div>
    <label for="commmon-filters">Choose one filtering option</label>
    <form on:change={handleFlag}>
      <select
        type="text"
        name="filter"
        id="filter"
        readonly
        class="cursor-pointer"
        bind:value={filter}>
          <option value={"No filters"}>No filters</option>
          {#each filter_types as type}
            <option value={type}>{type}</option>
          {/each}
      </select>
    </form>
  </div>
  <div>
    {#if html === 0}
      <label for="filter">No filtering actions</label>
    {/if}
    {#if html === 1}
      <label for="filter">Filter 
        <a href="https://biot.com/capstats/bpf.html" target="_blank">[BPF syntax]</a>
      </label>
      <textarea id="filter" bind:value={settings.filter}></textarea>
      <button id="set-filters" class="bg-accent-success" on:click={set_filters}>Set Filters</button>
    {/if}
    {#if html === 2}
      <label for="ip-filter">IP Address 
        <a href="https://github.com/davideaimar/nasoone" target="_blank">Use sintax: IPgeneric=[not]IP1,[not]IP2,...;IPsource=[not]IP1,[not]IP2...;IPdestination=[not]IP1,[not]IP2...;</a>
      </label>
      <textarea id="ip-filter" bind:value={ip_filter}></textarea>
      <label for="mac-filter">MAC Address
        <a href="https://github.com/davideaimar/nasoone" target="_blank">Use sintax: MACgeneric=[not]MAC1,[not]MAC2,...;MACsource=[not]MAC1,[not]MAC2,...;MACdestination=[not]MAC1,[not]MAC2,...;</a>
      </label>
      <textarea id="mac-filter" bind:value={mac_filter}></textarea>
      <label for="port-filter">Port 
        <a href="https://github.com/davideaimar/nasoone" target="_blank">Use sintax: Portgeneric=Port1,Port2,...;Portsource=Port1,Port2,...;Portdestination=Port1,Port2,...;</a>
      </label>
      <textarea id="port-filter" bind:value={port_filter}></textarea>
      <label for="proto-filter">Protocols 
        <a href="https://github.com/davideaimar/nasoone" target="_blank">Use sintax: Protocols=protocol1,protocol2,...</a>
      </label>
      <textarea id="prot-filter" bind:value={proto_filter}></textarea>
      <button id="set-filters" class="bg-accent-success" on:click={set_filters}>Set Filters</button>
    {/if}
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
