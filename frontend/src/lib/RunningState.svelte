<script lang="ts">
    import {invoke} from "@tauri-apps/api";
    import {State} from "../types/State";
    import { toast } from '@zerodevx/svelte-toast'

    export let state: State;
    let tot_packets = 0;
    let poller

    const setupPoller = () => {
        if (poller) {
            clearInterval(poller)
        }
        poller = setInterval(update_packets, 1000)
    }

    const update_packets = async () => {
        tot_packets = await invoke("get_total_packets");
    };

    const resume = async () => {
        try {
            const result = await invoke("resume");
            toast.push("Resumed");
            state = State.Running;
        } catch (e) {
            alert(e);
        }
    };

    const pause = async () => {
        try {
            const result = await invoke("pause");
            toast.push("Paused");
            state = State.Paused;
        } catch (e) {
            alert(e);
        }
    };

    const stop = async () => {
        try {
            await invoke("stop");
            await invoke("reset");
            //alert(result);
            toast.push("Analysis stopped, " + tot_packets + " packets processed.");
            state = State.Initial;
        } catch (e) {
            alert(e);
        }
    };

    const n_formatter = (num, digits) => {
        const lookup = [
            { value: 1, symbol: "" },
            { value: 1e3, symbol: "k" },
            { value: 1e6, symbol: "M" }
        ];
        const rx = /\.0+$|(\.[0-9]*[1-9])0+$/;
        let item = lookup.slice().reverse().find(function(item) {
            return num >= item.value;
        });
        return item ? (num / item.value).toFixed(digits).replace(rx, "$1") + item.symbol : "0";
    }

    $: setupPoller();
</script>


<div class="running-wrapper">
    {#if state === State.Running}
        <h1>Nasoone is running</h1>
    {:else }
        <h1>Nasoone is paused</h1>
    {/if}

    <div class="action-wrapper">
        <button class="action-button" on:click={() => state === State.Running ? pause() : resume()}>
            {#if state === State.Running}
                <svg xmlns="http://www.w3.org/2000/svg" width="56" height="56" fill="currentColor" class="bi bi-pause-fill" viewBox="0 0 16 16">
                    <path d="M5.5 3.5A1.5 1.5 0 0 1 7 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5zm5 0A1.5 1.5 0 0 1 12 5v6a1.5 1.5 0 0 1-3 0V5a1.5 1.5 0 0 1 1.5-1.5z"/>
                </svg>
            {:else }
                <svg xmlns="http://www.w3.org/2000/svg" width="56" height="56" fill="currentColor" class="bi bi-play-fill" viewBox="0 0 16 16">
                    <path d="m11.596 8.697-6.363 3.692c-.54.313-1.233-.066-1.233-.697V4.308c0-.63.692-1.01 1.233-.696l6.363 3.692a.802.802 0 0 1 0 1.393z"/>
                </svg>
            {/if}
        </button>
        <p>{n_formatter(tot_packets, 1)} packets received</p>
    </div>

    <button class="stop-button" on:click={stop}>Stop</button>
</div>

<style type="scss">
    h1 {
        width: 100%;
    }
    .running-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: space-between;
        height: 100vh;
        width: 100vw;
    }

    .action-wrapper{
        display: flex;
        flex-direction: column;
        align-items: center;
    }

    .action-button{
        width: 80px;
        height: 80px;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        border-radius: 100%;
    }

    .action-button:hover{
        background-color: var(--color-accent-secondary);
    }

    .stop-button{
        background-color: var(--color-error);
    }
</style>