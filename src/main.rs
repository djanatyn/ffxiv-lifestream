use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process::Command;

const CAPTURE_FILTER: &str = "tcp port 55024 and ip src host 204.2.229.99";
const FFXIV_PACKET_DATA_OFFSET: usize = 0x44;
const FFXIV_MAGIC: u32 = 0x41a05252;

fn main() {
    let mut cap = pcap::Capture::from_device("any")
        .expect("failed to find device")
        .immediate_mode(true)
        .open()
        .expect("failed to open device");

    cap.filter(CAPTURE_FILTER, true)
        .expect("failed to apply filter");

    let packet = cap.next_packet().expect("failed to get packet");
    let data = packet
        .iter()
        .skip(FFXIV_PACKET_DATA_OFFSET)
        .cloned()
        .collect::<Vec<u8>>();

    let mut cursor = io::Cursor::new(&data);

    // 0x00 -> 0x04: magic
    let mut magic_bytes = [0; 0x4];
    cursor
        .read(&mut magic_bytes)
        .expect("failed to read magic bytes");
    let magic = u32::from_le_bytes(magic_bytes.try_into().expect("failed to parse magic bytes"));

    // panic if magic bytes not found
    if magic != FFXIV_MAGIC {
        panic!("bad magic (should be {:#0x}): {:#0x}", FFXIV_MAGIC, magic);
    } else {
        eprintln!("found magic: {:#0x?}", magic);
    }

    // 0x40 -> 0x44: timestamp
    let mut timestamp_bytes = [0; 0x4];
    cursor
        .seek(SeekFrom::Start(16 * 4))
        .expect("failed to seek");
    cursor
        .read(&mut timestamp_bytes)
        .expect("failed to read timestamp");
    let timestamp: u32 = u32::from_le_bytes(
        timestamp_bytes
            .try_into()
            .expect("failed to convert timestamp to u32"),
    );

    // run `date` to convert to human-readable
    let date = Command::new("date")
        .arg("--utc")
        .arg("--date")
        .arg(format!("@{timestamp}"))
        .output()
        .expect("failed to convert timestamp");
    let date_string = std::str::from_utf8(&date.stdout)
        .expect("failed to read stdout")
        .trim();
    eprintln!("found timestamp: {}", date_string);

    // 0x70 -> 0x72: channel
    // read LE word at 0x70 -> 0x74
    let mut channel_bytes = [0; 0x4];
    cursor
        .seek(SeekFrom::Start(28 * 4))
        .expect("failed to seek channel");
    cursor
        .read(&mut channel_bytes)
        .expect("failed to read channel bytes");
    let channel_word = u32::from_le_bytes(
        channel_bytes
            .try_into()
            .expect("failed to convert channel bytes to u16"),
    );

    // take first 2 bytes
    let channel = u16::from_be_bytes(channel_word.to_be_bytes()[0..2].try_into().expect("failed"));
    eprintln!("found channel: {:#0x?}", channel);

    // write packet data to stdout
    std::io::stdout().write_all(&data).expect("failed to write");
}
