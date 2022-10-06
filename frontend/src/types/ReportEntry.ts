export type ReportEntry = {
    source_ip: string,
    source_port: number,
    dest_ip: string,
    dest_port: number,
    protocols: string[],
    first_ts: number,
    last_ts: number,
    bytes: number,
    packets: number,
};