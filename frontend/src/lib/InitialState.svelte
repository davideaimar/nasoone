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
        BPF: false,
    };

    const start = async () => {
        try {
            console.log(params.filter);
            let filter = await invoke("new_filter", {});
            let bpf_filters = "";
            if (params.BPF === true) {
                bpf_filters = params.filter;
            }
            else {
                if (params.filter.length === 0) {
                    bpf_filters = "";
                }
                else {
                    let list = params.filter.split(";");
                    for (let l in list) {
                        let k = l.split("=")[0]; // Key
                        let v = l.split("=")[1]; // Value
                        switch(k) {
                            case "IPgeneric":
                                let list_ip = v.split(",");
                                for (let addr in list_ip) {
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("add_host", {fil: filter, host: v, not: not})
                                }
                                break;
                            case "IPsource":
                                let list_ips = v.split(",");
                                for (let addr in list_ips) {
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("add_src_host", {fil: filter, src_host: v, not: not})
                                }
                                break;
                            case "IPdestination":
                                let list_ipd = v.split(",");
                                for (let addr in list_ipd) {
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("add_dst_host", {fil: filter, dst_host: v, not: not})
                                }
                                break;
                            case "MACgeneric":
                                let list_mac = v.split(",");
                                for (let addr in list_mac) {
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("set_ether_host", {fil: filter, ether_host: v, not: not})
                                }
                                break;
                            case "MACsource":
                                let list_macs = v.split(",");
                                for (let addr in list_macs) {
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("set_ether_src_host", {fil: filter, ether_src_host: v, not: not})
                                }
                                break;
                            case "MACdestination":
                                let list_macd = v.split(",");
                                for (let addr in list_macd) {
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("set_ether_dst_host", {fil: filter, ether_dst_host: v, not: not});
                                }
                                break;
                            case "Portgeneric":
                                let list_port = v.split(",");
                                for (let addr in list_port) {
                                    try {
                                        filter = await invoke("set_port", {fil: filter, port: parseInt(v)});
                                    }
                                    catch(e) {
                                        alert(e);            
                                    }
                                }
                                break;
                            case "Portsource":
                                let list_ports = v.split(",");
                                for (let addr in list_ports) {
                                    filter = await invoke("set_src_port", {fil: filter, src_port: parseInt(v)});
                                }
                                break;
                            case "Portdestination":
                                let list_portd = v.split(",");
                                for (let addr in list_portd) {
                                    filter = await invoke("set_dst_port", {fil: filter, dst_port: parseInt(v)});
                                }
                                break;
                            case "Protocols":
                                let list_proto = v.split(",");
                                filter = await invoke("set_protocols", {fil: filter, proto: list_proto});
                                break;
                        }
                    }
                    bpf_filters = await invoke("get_BPF_filter", {fil: filter});
                }
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