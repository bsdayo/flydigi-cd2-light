use rusb::DeviceHandle;
use std::thread;
use std::time::Duration;

const PACKET_SIZE: usize = 64;
const ENDPOINT_OUT: u8 = 0x01;
const DATA_CHUNK: usize = 50;

/// Builds the initialization packet sent before LED data.
fn build_init_packet() -> [u8; PACKET_SIZE] {
    let mut packet = [0u8; PACKET_SIZE];
    packet[0..4].copy_from_slice(&[0x5a, 0xa5, 0x61, 0x07]);
    packet[4..6].copy_from_slice(&[0x0a, 0x00]);
    packet[6] = 0x00;
    packet[7..10].copy_from_slice(&[0x00, 0x0a, 0x7c]);
    packet
}

/// Builds a single data frame for the LED payload.
///
/// # Arguments
///
/// * `frame_idx` - The index of the frame.
/// * `data` - The payload chunk to embed in the frame.
/// * `is_last` - Whether this is the final frame in the sequence.
fn build_data_frame(frame_idx: u16, data: &[u8], is_last: bool) -> [u8; PACKET_SIZE] {
    let mut packet = [0u8; PACKET_SIZE];
    packet[0..2].copy_from_slice(&[0x5a, 0xa5]);
    packet[2] = 0x62;
    packet[3] = if is_last { 0x31 } else { 0x39 };
    packet[4..6].copy_from_slice(&[0x00, 0x0a]);
    packet[6..8].copy_from_slice(&frame_idx.to_be_bytes());
    packet[8] = data.len() as u8;
    packet[9..9 + data.len()].copy_from_slice(data);

    let checksum_end = 9 + data.len();
    let mut sum = 0u8;
    for i in 2..checksum_end {
        sum = sum.wrapping_add(packet[i]);
    }
    packet[checksum_end] = sum;
    packet
}

/// Sends RGB LED data to the device via USB interrupt transfers.
///
/// The data is prefixed with a header, split into chunks, and transmitted
/// as a series of framed packets.
///
/// # Arguments
///
/// * `handle` - An open USB device handle.
/// * `rgb_data` - Raw RGB values (3 bytes per LED).
pub fn send_led_data(
    handle: &DeviceHandle<rusb::GlobalContext>,
    rgb_data: &[u8],
) -> rusb::Result<()> {
    let init_packet = build_init_packet();
    handle.write_interrupt(ENDPOINT_OUT, &init_packet, Duration::from_secs(1))?;
    thread::sleep(Duration::from_millis(5));

    let first_frame_header = [0x01u8, 0x01, 0x64, 0x03, 0x00, 0x00];
    let mut all_data = Vec::with_capacity(first_frame_header.len() + rgb_data.len());
    all_data.extend_from_slice(&first_frame_header);
    all_data.extend_from_slice(rgb_data);

    let total_frames = (all_data.len() + DATA_CHUNK - 1) / DATA_CHUNK;

    for i in 0..total_frames {
        let start = i * DATA_CHUNK;
        let end = ((i + 1) * DATA_CHUNK).min(all_data.len());
        let chunk = &all_data[start..end];
        let is_last = i == total_frames - 1;
        let frame = build_data_frame(i as u16, chunk, is_last);
        handle.write_interrupt(ENDPOINT_OUT, &frame, Duration::from_secs(1))?;
        thread::sleep(Duration::from_millis(5));
    }

    Ok(())
}
