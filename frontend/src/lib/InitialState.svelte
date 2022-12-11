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
            if (params.filter.includes(";")) {
                const ip = params.filter.split(";")[0]
                const mac = params.filter.split(";")[1]
                const port = params.filter.split(";")[2]
                let bpf_ip = '';
                let bpf_mac = '';
                let bpf_port = '';

                // Set the BPF sintax for the elements
                if (ip !== "") {
                    bpf_ip = "host " + ip;
                }
                if (mac !== "") {
                    bpf_mac = "ether host " + mac; 
                }
                if (port !== "") {
                    bpf_port = "port " + port;
                }

                // BPF combinations
                if (bpf_ip !== '' && bpf_mac !== '' && bpf_port !== '') {
                    bpf_filters = bpf_ip + " AND " + bpf_mac + " AND " + bpf_port;
                }
                else if (bpf_ip !== '' && bpf_mac !== '') {
                    bpf_filters = bpf_ip + " AND " + bpf_mac;
                }
                else if (bpf_ip !== '' && bpf_port !== '') {
                    bpf_filters = bpf_ip + " AND " + bpf_port;
                }
                else if (bpf_mac !== '' && bpf_port !== '') {
                    bpf_filters = bpf_mac + " AND " + bpf_port;
                }
                else {
                    if (bpf_ip !== '') {
                        bpf_filters = bpf_ip;
                    }
                    else if (bpf_mac !== '') {
                        bpf_filters = bpf_mac;
                    }
                    else {
                        bpf_filters = bpf_port;
                    }
                }
            }
            else if (params.filter !== "") {
                bpf_filters = params.filter;
            }

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