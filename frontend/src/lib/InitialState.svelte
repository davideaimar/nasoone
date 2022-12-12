<script lang="ts">
    import {invoke} from "@tauri-apps/api";
    import Setting from "./Setting.svelte";
    import {State} from "../types/State";
    import type {NasooneSettings} from "../types/NasooneSettings";
    import {toast} from "@zerodevx/svelte-toast";

    export let state: State;

    // const possible_keys = ["IPs", "IPd", "MACs", "MACd", "Ports", "Portd", "Protocol"]

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
            if(params.filter.includes(";")) {
                let dict = new Map()
                let list = params.filter.split(";");

                for (let i = 0; i<list.length; i++) {
                    let kv = list[i].split("=");
                    dict.set(kv[0], kv[1])
                }
                
                for (let k in dict.keys()) {
                    switch(k){
                        case "IPs":
                            bpf_filters += " src host " + dict.get(k);
                            break;
                        case "IPd":
                            bpf_filters += " dst host " + dict.get(k);
                            break;
                        case "MACs":
                            bpf_filters += " ether src host " + dict.get(k);
                            break;
                        case "MACd":
                            bpf_filters += " ether dst host " + dict.get(k);
                            break;
                        case "Ports":
                            bpf_filters += " src port " + dict.get(k);
                            break;
                        case "Portd":
                            bpf_filters += " dst port " + dict.get(k);
                            break;
                        case "Protocol":
                            bpf_filters += dict.get(k);
                            break;
                    }
                }

                bpf_filters.trimStart();
                bpf_filters.trimEnd();

            }
            else {
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