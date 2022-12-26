<script async lang="ts">
    import * as dialog from "@tauri-apps/api/dialog";
    import type {ReportEntry} from "../types/ReportEntry";
    import {readTextFile} from "@tauri-apps/api/fs";
    import {invoke} from "@tauri-apps/api";

    let report_data: ReportEntry[] = [];
    let report_path: string = "";
    let filter: string = "";
    let file_size: number;
    let poller;

    const setupPoller = () => {
        if (poller) {
            clearInterval(poller)
        }
        poller = setInterval(parse_file, 1000);
    }

    function formatBytes(bytes, digits) {
        if (!+bytes) return '0'

        const k = 1024
        const dm = digits < 0 ? 0 : digits
        const sizes = ['', 'K', 'M', 'G']

        const i = Math.floor(Math.log(bytes) / Math.log(k))

        return `${parseFloat((bytes / Math.pow(k, i)).toFixed(dm))} ${sizes[i]}`
    }

    const n_formatter = (num, digits) => {
        const lookup = [
            { value: 1, symbol: "" },
            { value: 1e3, symbol: "K" },
            { value: 1e6, symbol: "M" }
        ];
        const rx = /\.0+$|(\.[0-9]*[1-9])0+$/;
        let item = lookup.slice().reverse().find(function(item) {
            return num >= item.value;
        });
        return item ? (num / item.value).toFixed(digits).replace(rx, "$1") + item.symbol : "0";
    }

    const parse_file = async () => {
        if (!report_path || report_path === "")
            return;
        try {
            const new_size: number = await invoke("get_file_size", {
                path: report_path
            });
            if (!file_size || new_size != file_size) {
                file_size = new_size;
                const result = await readTextFile(report_path);
                const headers = result.split("\n")[0];
                if (headers.replaceAll(" ", "") !== "sourceip;sourceport;destinationip;destinationport;protocols;first;last;bytes;packets") {
                    alert("Invalid file");
                    report_path = "";
                    return;
                }
                const entries = result.split("\n").slice(1)
                if (entries.length > 0)
                    report_data = [];
                for (let i = 0; i < entries.length; i++) {
                    const [src_ip, src_port, dst_ip, dst_port, protocols, first_ts, last_ts, bytes, packets] = entries[i].split(";");
                    if (src_ip && src_port && dst_ip && dst_port && protocols && first_ts && last_ts && bytes && packets)
                        report_data.push({
                            source_ip: src_ip,
                            source_port: src_port,
                            dest_ip: dst_ip,
                            dest_port: dst_port,
                            protocols: protocols,
                            first_ts: new Date(Number(first_ts)),
                            last_ts: new Date(Number(last_ts)),
                            bytes: Number(bytes),
                            packets: Number(packets)
                        } as ReportEntry);
                }
                report_data = report_data.sort((a, b) => a.first_ts - b.first_ts);
                console.log("Data loaded!");
            }
        } catch (e) {
            console.log(e);
        }
    }

    const format_date = (date) => {
        const now = new Date();
        const offsetMs = now.getTimezoneOffset() * 60 * 1000;
        const dateLocal = new Date(date.getTime() - offsetMs);
        return dateLocal.toISOString().replace(/-/g, "/").replace(/[T|Z]/g, " ");
    }

    const update_file = async () => {
        await parse_file();
    };

    // used to show the file dialog and choose the output folder
    const open_file = async () => {
        const file = await dialog.open({
            multiple: false,
        });

        try {
            if (typeof file === "string") {
                report_path = file;
                await update_file();
            } else {
                console.log("No file selected");
            }
        } catch (error) {
            console.log(error);
            return false;
        }
    };

    $: filtered_data = report_data.filter((entry) => {
        if (filter) {
            return entry.source_ip.includes(filter) || entry.dest_ip.includes(filter);
        } else {
            return true;
        }
    });

    $: setupPoller();

</script>

<div>
    <div class="file-button">
        <div>
            <input
                    type="text"
                    placeholder="Filter by IP"
                    bind:value={filter}
            />
        </div>
        <button on:click={open_file}>Open report</button>
    </div>
    {#if (filtered_data.length >= 0)}
        <div class="table-wrapper">
            <table>
                <thead class="tbl-header">
                    <tr>
                        <th>Source IP</th>
                        <th>Source port</th>
                        <th>Destination IP</th>
                        <th>Dest port</th>
                        <th>Protocols</th>
                        <th>First packet time</th>
                        <th>Last packet time</th>
                        <th>Bytes</th>
                        <th>Packets</th>
                    </tr>
                </thead>
                <tbody class="tbl-content">
                    {#each filtered_data as entry}
                        <tr>
                            <td>{entry.source_ip}</td>
                            <td>{entry.source_port}</td>
                            <td>{entry.dest_ip}</td>
                            <td>{entry.dest_port}</td>
                            <td>{entry.protocols}</td>
                            <td>{format_date(entry.first_ts)}</td>
                            <td>{format_date(entry.last_ts)}</td>
                            <td>{entry.bytes}</td>
                            <td>{entry.packets}</td>
                        </tr>
                    {/each}
                    <tr>
                        <td></td>
                        <td></td>
                        <td></td>
                        <td></td>
                        <td></td>
                        <td></td>
                        <td>TOT:</td>
                        <td>{formatBytes(filtered_data.reduce( (a, b) => a + Number(b.bytes), 0 ), 2) }B</td>
                        <td>{n_formatter(filtered_data.reduce( (a, b) => a + Number(b.packets), 0 ), 2) }</td>
                    </tr>
                <tbody>
            </table>
        </div>
    {:else}
        <div class="no-data">
            <p>No data to show</p>
        </div>
    {/if}
</div>

<style type="scss">
    .seconds-input{
        display: inline;
        width: 40px;
    }
    .file-button{
        display: flex;
        justify-content: space-between;
        margin-bottom: 20px;
    }
    .table-wrapper{
      display: block;
      table-layout: fixed;
      overflow-x: scroll;
      white-space: nowrap;
    }
    table{
      min-width: 1600px;
    }
    .tbl-header{
      background-color: rgba(255,255,255,0.3);
    }
    .tbl-content{
      height:300px;
      overflow-x:auto;
      margin-top: 0px;
      border: 1px solid rgba(255,255,255,0.3);
    }
    th{
      padding: 20px 15px;
      text-align: left;
      font-weight: 500;
      font-size: 12px;
      color: #fff;
      text-transform: uppercase;
    }
    td{
      padding: 15px;
      text-align: left;
      vertical-align:middle;
      font-weight: 300;
      font-size: 12px;
      color: #fff;
      border-bottom: solid 1px rgba(255,255,255,0.1);
    }
</style>
