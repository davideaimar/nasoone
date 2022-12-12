<script lang="ts">
    import {invoke} from "@tauri-apps/api";
    import Setting from "./Setting.svelte";
    import {State} from "../types/State";
    import type {NasooneSettings} from "../types/NasooneSettings";
    import {toast} from "@zerodevx/svelte-toast";

    export let state: State;

    let params: NasooneSettings = {
        device: "",
        folder: "",
        interval: 5,
        file_name: "nasoone_" + new Date().getTime(),
        filter: "",
    };

    const start = async () => {
        try {
            let bpf_filters = "";

            const result = await invoke("start", {
                device: params.device,
                outputFolder: params.folder,
                filename: params.file_name,
                interval: params.interval,
                filter: bpf_filters,
            });
            toast.push("Analysis started");
            state = State.Running;
        } catch (e) {
            alert(e);
        }
    };
</script>


<div>
    <h1>Start analyzing the network</h1>
    <p>Specify the parameters and start the capture.</p>
    <div>
        <Setting bind:settings={params} />
        <button id="start" class="bg-accent-secondary" on:click={start}>Start</button>
    </div>
    <!--
    <div>
        <p>Logging: </p>
        <code>
          <pre>settings: {JSON.stringify(params, null, 2)}
          </pre>
        </code>
    </div>
    -->
</div>