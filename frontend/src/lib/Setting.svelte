<script async lang="ts">
  import {invoke} from "@tauri-apps/api";
  import * as dialog from "@tauri-apps/api/dialog";
  import {desktopDir} from "@tauri-apps/api/path";
  import type {NasooneSettings} from "src/types/NasooneSettings";
  import type {IpFilter} from "../types/IpFilter";
  import {FilterDir} from "../types/FilterDir";
  import type {PortFilter} from "../types/PortFilter";

  export let settings: NasooneSettings;

  // on startup, set the default output folder to desktop
  (async () => {
    settings.folder = (await desktopDir()) as string;
  })();

  // on startup, load devices and set the default device to the first one in the list
  const load_devices = (async () => {
    const devices = (await invoke("get_devices")) as string;
    const device_list = JSON.parse(devices);
    settings.device = device_list[0][0];
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

  const ip_validator = /((^\s*((([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5])\.){3}([0-9]|[1-9][0-9]|1[0-9]{2}|2[0-4][0-9]|25[0-5]))\s*$)|(^\s*((([0-9A-Fa-f]{1,4}:){7}([0-9A-Fa-f]{1,4}|:))|(([0-9A-Fa-f]{1,4}:){6}(:[0-9A-Fa-f]{1,4}|((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){5}(((:[0-9A-Fa-f]{1,4}){1,2})|:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3})|:))|(([0-9A-Fa-f]{1,4}:){4}(((:[0-9A-Fa-f]{1,4}){1,3})|((:[0-9A-Fa-f]{1,4})?:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){3}(((:[0-9A-Fa-f]{1,4}){1,4})|((:[0-9A-Fa-f]{1,4}){0,2}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){2}(((:[0-9A-Fa-f]{1,4}){1,5})|((:[0-9A-Fa-f]{1,4}){0,3}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(([0-9A-Fa-f]{1,4}:){1}(((:[0-9A-Fa-f]{1,4}){1,6})|((:[0-9A-Fa-f]{1,4}){0,4}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:))|(:(((:[0-9A-Fa-f]{1,4}){1,7})|((:[0-9A-Fa-f]{1,4}){0,5}:((25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)(\.(25[0-5]|2[0-4]\d|1\d\d|[1-9]?\d)){3}))|:)))(%.+)?\s*$))/;


  let filter_types = ["BPF", "Easy filtering"];
  let selected_filter = 0;
  let filter = '';

  let form_ip_filter = {
    ip: "",
    dir: FilterDir.Both,
  };
  let form_port_filter = {
    port: 0,
    dir: FilterDir.Both,
  };
  let form_protocol_filter = 'No filter';

  let ips_filter: IpFilter[] = [];
  let ports_filter: PortFilter[] = [];
  let protocol_filters = ['No filter', 'tcp only', 'udp only'];

  const add_ip_filter = () => {
    if (ip_validator.test(form_ip_filter.ip)) {
      ips_filter.push({
        ip: form_ip_filter.ip,
        dir: form_ip_filter.dir,
      });
      form_ip_filter = {
        ip: "",
        dir: FilterDir.Both,
      };
      ips_filter = ips_filter; // to trigger the reactivity
      update_stored_filter();
    } else {
        alert("Invalid IP");
    }
  };

  const add_port_filter = () => {
    if (form_port_filter.port > 0 && form_port_filter.port < 65536) {
      ports_filter.push({
        port: form_port_filter.port,
        dir: form_port_filter.dir,
      });
      form_port_filter = {
        port: 0,
        dir: FilterDir.Both,
      };
      ports_filter = ports_filter; // to trigger the reactivity
      update_stored_filter();
    } else {
        alert("Invalid Port");
    }
  };

  const update_stored_filter = async () => {
    try {
      settings.filter = await invoke("get_bpf_filter_from_easy_filter", {
        srcIps: [ ... new Set(ips_filter.filter((ip) => ip.dir === FilterDir.Src || ip.dir == FilterDir.Both).map((ip) => ip.ip)) ],
        dstIps: [ ... new Set(ips_filter.filter((ip) => ip.dir === FilterDir.Dst || ip.dir == FilterDir.Both).map((ip) => ip.ip)) ],
        srcPorts: [ ... new Set(ports_filter.filter((port) => port.dir === FilterDir.Src || port.dir == FilterDir.Both).map((port) => port.port)) ],
        dstPorts: [ ... new Set(ports_filter.filter((port) => port.dir === FilterDir.Dst || port.dir == FilterDir.Both).map((port) => port.port)) ],
        tcp: form_protocol_filter === 'tcp only' || form_protocol_filter === 'No filter',
        udp: form_protocol_filter === 'udp only' || form_protocol_filter === 'No filter',
      });
      console.log("New filter: " + settings.filter);
    } catch (e) {
      alert("Error while generating the filter: " + e);
      console.error(e);
    }
  }

  const remove_ip_filter = (ip_filter) => {
    ips_filter.splice(ips_filter.indexOf(ip_filter), 1);
    ips_filter = ips_filter; // to trigger the reactivity
    update_stored_filter();
  };

  const remove_port_filter = (port_filter) => {
    ports_filter.splice(ports_filter.indexOf(port_filter), 1);
    ports_filter = ports_filter; // to trigger the reactivity
    update_stored_filter();
  };

  const get_dir_as_text = (dir: FilterDir) => {
    switch (dir) {
      case FilterDir.Src:
        return "Src";
      case FilterDir.Dst:
        return "Dst";
      case FilterDir.Both:
        return "";
    }
  }

  const handleFlag = async () => {
    if (filter === "BPF") {
      selected_filter = 1;
    }
    else if (filter === "Easy filtering") {
      selected_filter = 2;
    }
    else {
      selected_filter = 0;
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
          <option value={device[0]}>{device[1]}</option>
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
    <label for="interval">File update interval [s]</label>
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
    <label for="filter-select">Choose one filtering option</label>
    <form on:change={handleFlag}>
      <select
        name="filter"
        id="filter-select"
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
    {#if selected_filter === 0}
      <label for="filter">No filtering actions</label>
    {/if}
    {#if selected_filter === 1}
      <label for="filter">Filter 
        <a href="https://biot.com/capstats/bpf.html" target="_blank">[BPF syntax]</a>
      </label>
      <textarea id="filter" bind:value={settings.filter}></textarea>
    {/if}
    {#if selected_filter === 2}
      <label>IP Addresses</label>
      <div class="inserted_ips_wrapper">
        {#each ips_filter as ip_filter}
          <span>
            {ip_filter.ip} {get_dir_as_text(ip_filter.dir)}
            <button on:click={() => remove_ip_filter(ip_filter)} aria-label="click to remove">X</button>
          </span>
        {/each}
      </div>
      <div>
        <form id="ip_filter_form">
          <input type="text" bind:value={form_ip_filter.ip} placeholder="Ip address" />
          <select bind:value={form_ip_filter.dir}>
            <option value={FilterDir.Both}>both</option>
            <option value={FilterDir.Src}>src</option>
            <option value={FilterDir.Dst}>dst</option>
          </select>
          <button type="button" on:click={add_ip_filter}>Add</button>
        </form>
      </div>
      <label>Ports</label>
      <div class="inserted_ports_wrapper">
        {#each ports_filter as port_filter}
          <span>
            {port_filter.port} {get_dir_as_text(port_filter.dir)}
            <button on:click={() => remove_port_filter(port_filter)} aria-label="click to remove">X</button>
          </span>
        {/each}
      </div>
      <div>
        <form id="port_filter_form">
          <input type="number" bind:value={form_port_filter.port} placeholder="Ip address" />
          <select bind:value={form_port_filter.dir}>
            <option value={FilterDir.Both}>both</option>
            <option value={FilterDir.Src}>src</option>
            <option value={FilterDir.Dst}>dst</option>
          </select>
          <button type="button" on:click={add_port_filter}>Add</button>
        </form>
      </div>
      <label for="protocol-filter">Protocols</label>
      <select id="protocol-filter" bind:value={form_protocol_filter} on:change={update_stored_filter}>
        {#each protocol_filters as protocol}
          <option value={protocol}>{protocol}</option>
        {/each}
      </select>
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
      margin: 0 0 15px 0;
    }
  }
  label {
    display: block;
    margin: 0 0 5px 0;
  }
  .inserted_ips_wrapper, .inserted_ports_wrapper {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    margin-top: 15px !important;

    & span {
      background-color: var(--color-accent-secondary);
      border-radius: 18px;
      padding: 5px 15px;
      margin-right: 5px;
      margin-bottom: 4px;

      & button {
        background-color: transparent;
        border: none;
        cursor: pointer;
        padding: 0;
        margin-left: 5px;
      }
    }
  }

  #ip_filter_form, #port_filter_form {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: flex-start;
    margin: 0 auto;
    width: 100%;
  }

  #ip_filter_form input, #port_filter_form input {
    max-width: 200px;
    margin-right: 10px;
  }

  #ip_filter_form select, #port_filter_form select {
    max-width: 100px;
    margin-right: 10px;
  }
</style>
