<script async lang="ts">
  import { invoke } from '@tauri-apps/api';

  
  const load_devices = (async () => {
		const devices = await invoke('get_devices') as string;
    // console.log(JSON.parse(devices));
    return JSON.parse(devices);
	})()
</script>

<main>
  <h1>Nasoone</h1>
  <p>Specify the parameters and start the capture.</p>
  <div>
    <div>
        {#await load_devices}
          <p>...waiting</p>
        {:then devices}
          <select name="device" id="device">
            {#each devices as device}
              <option value="{device}">{device}</option>
            {/each}
          </select>
        {:catch error}
          <p>An error occurred!</p>
        {/await}
    </div>
    <div>
      <label for="target">Dest path</label>
      <input type="file" id="target" name="target" />
    </div>
    <div>
      <label for="interval">Interval</label>
      <input type="number" id="interval" name="interval" value="1000" />
    </div>  
    <div>
      <label for="filename">File name</label>
      <input type="text" id="filename" name="filename" value="nasoone" />
    </div>
    <button id="start">Start</button>
  </div>
</main>

<style>
</style>