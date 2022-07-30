<script lang="ts">
    import {invoke} from "@tauri-apps/api";
    import {State} from "../types/State";

    export let state: State;

    const resume = async () => {
        try {
            const result = await invoke("resume");
            alert(result);
            state = State.Running;
        } catch (e) {
            alert(e);
        }
    };

    const stop = async () => {
        try {
            const result = await invoke("stop");
            alert(result);
            state = State.Stopped;
        } catch (e) {
            alert(e);
        }
    };
</script>


<div>
    <button on:click={resume}>Resume</button>
    <button on:click={stop}>Stop</button>
</div>