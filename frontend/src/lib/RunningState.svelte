<script lang="ts">
    import {invoke} from "@tauri-apps/api";
    import {State} from "../types/State";

    export let state: State;

    const pause = async () => {
        try {
            const result = await invoke("pause");
            alert(result);
            state = State.Paused;
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
    <button on:click={pause}>Pause</button>
    <button on:click={stop}>Stop</button>
</div>