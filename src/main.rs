use std::io::Write;

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

    // 0x00 -> 0x04: magic
    let (head, rest) = data.split_at(std::mem::size_of::<u32>());
    let magic = u32::from_le_bytes(head.try_into().expect("failed to parse magic bytes"));
    if magic != FFXIV_MAGIC {
        panic!("bad magic (should be {:#0x}): {:#0x}", FFXIV_MAGIC, magic);
    }

    eprintln!("found magic: {:#0x?}", magic);

    std::io::stdout().write_all(&data).expect("failed to write");
}
