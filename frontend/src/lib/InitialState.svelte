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

    const regex_ipv4 = new RegExp("(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])");
    const regex_ipv6 = new RegExp("((([0-9a-fA-F]){1,4})\\:){7}([0-9a-fA-F]){1,4}");
    const regex_mac = new RegExp("^([0-9A-F]{2}[:-]){5}([0-9A-F]{2})$");

    const start = async () => {
        try {
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
                                    if (regex_ipv4.test(addr) === false && regex_ipv6.test(addr) === false){
                                        throw new Error("Invalid IP address!");
                                    }
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("add_host", {fil: filter, host: addr, not: not})
                                }
                                break;
                            case "IPsource":
                                let list_ips = v.split(",");
                                for (let addr in list_ips) {
                                    if (regex_ipv4.test(addr) === false && regex_ipv6.test(addr) === false){
                                        throw new Error("Invalid IP address!");
                                    }
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("add_src_host", {fil: filter, src_host: addr, not: not})
                                }
                                break;
                            case "IPdestination":
                                let list_ipd = v.split(",");
                                for (let addr in list_ipd) {
                                    if (regex_ipv4.test(addr) === false && regex_ipv6.test(addr) === false){
                                        throw new Error("Invalid IP address!");
                                    }
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("add_dst_host", {fil: filter, dst_host: addr, not: not})
                                }
                                break;
                            case "MACgeneric":
                                let list_mac = v.split(",");
                                for (let addr in list_mac) {
                                    if (regex_mac.test(addr) === false) {
                                        throw new Error("Invalid MAC address!");
                                    }
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("set_ether_host", {fil: filter, ether_host: addr, not: not})
                                }
                                break;
                            case "MACsource":
                                let list_macs = v.split(",");
                                for (let addr in list_macs) {
                                    if (regex_mac.test(addr) === false) {
                                        throw new Error("Invalid MAC address!");
                                    }
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("set_ether_src_host", {fil: filter, ether_src_host: addr, not: not})
                                }
                                break;
                            case "MACdestination":
                                let list_macd = v.split(",");
                                for (let addr in list_macd) {
                                    if (regex_mac.test(addr) === false) {
                                        throw new Error("Invalid MAC address!");
                                    }
                                    let not = false;
                                    if (addr.includes("[not]") === true) {
                                        not = true;
                                        addr.replace("[not]", "");
                                    }
                                    filter = await invoke("set_ether_dst_host", {fil: filter, ether_dst_host: addr, not: not});
                                }
                                break;
                            case "Portgeneric":
                                let list_port = v.split(",");
                                for (let port in list_port) {
                                    if (typeof parseInt(port) !== "number"){
                                        throw new Error("Invalid Port!");
                                    }
                                    filter = await invoke("set_port", {fil: filter, port: parseInt(port)});
                                }
                                break;
                            case "Portsource":
                                let list_ports = v.split(",");
                                for (let port in list_ports) {
                                    if (typeof parseInt(port) !== "number"){
                                        throw new Error("Invalid Port!");
                                    }
                                    filter = await invoke("set_src_port", {fil: filter, src_port: parseInt(port)});
                                }
                                break;
                            case "Portdestination":
                                let list_portd = v.split(",");
                                for (let port in list_portd) {
                                    if (typeof parseInt(port) !== "number"){
                                        throw new Error("Invalid Port!");
                                    }
                                    filter = await invoke("set_dst_port", {fil: filter, dst_port: parseInt(port)});
                                }
                                break;
                            case "Protocols":
                                let list_proto = v.split(",");
                                try {
                                    filter = await invoke("set_protocols", {fil: filter, proto: list_proto});
                                }
                                catch {
                                    throw new Error("Protocol not found");
                                }
                                break;
                            default:
                                throw new Error("Incorrect field!");
                                
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