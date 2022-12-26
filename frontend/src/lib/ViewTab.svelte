<script async lang="ts">
    import * as dialog from "@tauri-apps/api/dialog";
    import type {ReportEntry} from "../types/ReportEntry";
    import {readTextFile} from "@tauri-apps/api/fs";
    import {invoke} from "@tauri-apps/api";
    import {ViewOrder} from "../types/ViewOrder";

    let report_data: ReportEntry[] = [];
    let report_path: string = "";
    let filter: string = "";
    let file_size: number;
    let poller;
    let order = ViewOrder.FirstTsAsc;

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

    const sort_data = (report_data) => {
        switch (order) {
            case ViewOrder.FirstTsAsc:
                return report_data.sort((a, b) => a.first_ts - b.first_ts);
            case ViewOrder.FirstTsDesc:
                return report_data.sort((a, b) => b.first_ts - a.first_ts);
            case ViewOrder.LastTsAsc:
                return report_data.sort((a, b) => a.last_ts - b.last_ts);
            case ViewOrder.LastTsDesc:
                return report_data.sort((a, b) => b.last_ts - a.last_ts);
            case ViewOrder.PacketsAsc:
                return report_data.sort((a, b) => a.packets - b.packets);
            case ViewOrder.PacketsDesc:
                return report_data.sort((a, b) => b.packets - a.packets);
            case ViewOrder.BytesAsc:
                return report_data.sort((a, b) => a.bytes - b.bytes);
            case ViewOrder.BytesDesc:
                return report_data.sort((a, b) => b.bytes - a.bytes);
            case ViewOrder.SrcIpAsc:
                return report_data.sort((a, b) => a.source_ip.localeCompare(b.source_ip));
            case ViewOrder.SrcIpDesc:
                return report_data.sort((a, b) => b.source_ip.localeCompare(a.source_ip));
            case ViewOrder.DstIpAsc:
                return report_data.sort((a, b) => a.dest_ip.localeCompare(b.dest_ip));
            case ViewOrder.DstIpDesc:
                return report_data.sort((a, b) => b.dest_ip.localeCompare(a.dest_ip));
            case ViewOrder.SrcPortAsc:
                return report_data.sort((a, b) => a.source_port - b.source_port);
            case ViewOrder.SrcPortDesc:
                return report_data.sort((a, b) => b.source_port - a.source_port);
            case ViewOrder.DstPortAsc:
                return report_data.sort((a, b) => a.dest_port - b.dest_port);
            case ViewOrder.DstPortDesc:
                return report_data.sort((a, b) => b.dest_port - a.dest_port);
            case ViewOrder.ProtocolAsc:
                return report_data.sort((a, b) => a.protocols.localeCompare(b.protocols));
            case ViewOrder.ProtocolDesc:
                return report_data.sort((a, b) => b.protocols.localeCompare(a.protocols));
            default:
                return report_data;
        }
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
                report_data = sort_data(report_data);
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

    const set_order = (new_order: ViewOrder) => {
        order = new_order;
        report_data = sort_data(report_data);
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
                        <th class:order-asc="{order === ViewOrder.SrcIpAsc }" class:order-desc="{order === ViewOrder.SrcIpDesc }" on:click={() => order === ViewOrder.SrcIpAsc ? set_order(ViewOrder.SrcIpDesc) : set_order(ViewOrder.SrcIpAsc)}>Source IP</th>
                        <th class:order-asc="{order === ViewOrder.SrcPortAsc }" class:order-desc="{order === ViewOrder.SrcPortDesc }" on:click={() => order === ViewOrder.SrcPortAsc ? set_order(ViewOrder.SrcPortDesc) : set_order(ViewOrder.SrcPortAsc)}>Source port</th>
                        <th class:order-asc="{order === ViewOrder.DstIpAsc }" class:order-desc="{order === ViewOrder.DstIpDesc }" on:click={() => order === ViewOrder.DstIpAsc ? set_order(ViewOrder.DstIpDesc) : set_order(ViewOrder.DstIpAsc)}>Destination IP</th>
                        <th class:order-asc="{order === ViewOrder.DstPortAsc }" class:order-desc="{order === ViewOrder.DstPortDesc }" on:click={() => order === ViewOrder.DstPortAsc ? set_order(ViewOrder.DstPortDesc) : set_order(ViewOrder.DstPortAsc)}>Dest port</th>
                        <th class:order-asc="{order === ViewOrder.ProtocolAsc }" class:order-desc="{order === ViewOrder.ProtocolDesc }" on:click={() => order === ViewOrder.ProtocolAsc ? set_order(ViewOrder.ProtocolDesc) : set_order(ViewOrder.ProtocolAsc)}>Protocols</th>
                        <th class:order-asc="{order === ViewOrder.FirstTsAsc }" class:order-desc="{order === ViewOrder.FirstTsDesc }" on:click={() => order === ViewOrder.FirstTsAsc ? set_order(ViewOrder.FirstTsDesc) : set_order(ViewOrder.FirstTsAsc)}>First packet time</th>
                        <th class:order-asc="{order === ViewOrder.LastTsAsc }" class:order-desc="{order === ViewOrder.LastTsDesc }" on:click={() => order === ViewOrder.LastTsAsc ? set_order(ViewOrder.LastTsDesc) : set_order(ViewOrder.LastTsAsc)}>Last packet time</th>
                        <th class:order-asc="{order === ViewOrder.BytesAsc }" class:order-desc="{order === ViewOrder.BytesDesc }" on:click={() => order === ViewOrder.BytesAsc ? set_order(ViewOrder.BytesDesc) : set_order(ViewOrder.BytesAsc)}>Bytes</th>
                        <th class:order-asc="{order === ViewOrder.PacketsAsc }" class:order-desc="{order === ViewOrder.PacketsDesc }" on:click={() => order === ViewOrder.PacketsAsc ? set_order(ViewOrder.PacketsDesc) : set_order(ViewOrder.PacketsAsc)}>Packets</th>
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
                            <td>{formatBytes(entry.bytes, 2)}B</td>
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
      -webkit-user-select: none; /* Safari */
      -ms-user-select: none; /* IE 10 and IE 11 */
      user-select: none; /* Standard syntax */
      cursor: pointer;
      &::after {
        content: " ";
        margin-left: 5px;
        display: inline-block;
        width: 15px;
      }
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

    .order-desc::after{
      content: "▼";
    }

    .order-asc::after{
      content: "▲";
    }

    .order-asc::after, .order-desc::after{
      margin-left: 5px;
    }

    .order-asc, .order-desc{
      display: flex;
    }
</style>
