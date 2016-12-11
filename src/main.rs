use std::io;
use std::io::prelude::*;
use std::fs::File;

struct MP3Header
{
    // 11 bits
    frame_sync: u16,

    // 2 bits
    audio_version: u8,

    // 2 bits
    layer_description: u8,

    // 1 bit
    protection_bit: bool,

    // 4 bits
    bitrate_index: u8,

    // Two bits
    sampling_rate: u8,

    // 1 bit
    padding_bit: bool,

    // 1 bit
    private_bit: bool,

    // 2 bits
    channel_mode: u8,

    // 2 bits
    mode_extension: u8,

    // 1 bit
    copyright: bool,

    // 1 bit
    original: bool,

    // 2 bit
    emphasis: u8
}

fn output_header(header: MP3Header) -> ()
{
    println!("Frame sync: {:b}", header.frame_sync);
    println!("Protection bit: {}", header.protection_bit);
    println!("Layer description: {:b}", header.layer_description);
    println!("MPEG Audio Version ID: {:b}", header.audio_version);
    println!("Private bit: {}", header.private_bit);
    println!("Padding bit: {}", header.padding_bit);
    println!("Sampling rate: {:b}", header.sampling_rate);
    println!("Bitrate index: {:b}", header.bitrate_index);
    println!("Channel Mode: {:b}", header.channel_mode);
    println!("Mode Extension: {:b}", header.mode_extension);
    println!("Copyright: {}", header.copyright);
    println!("Original: {}", header.original);
    println!("Emphasis: {}", header.emphasis);
}

fn parse_header(buffer: Vec<u8>) -> MP3Header
{
    let mut i: i32 = 0;
    let mut frame_sync: u32 = 0;
    let mut synced: bool = false;
    let mut header = MP3Header {
        frame_sync:0,
        audio_version:0,
        layer_description:0,
        protection_bit:false,
        bitrate_index:0,
        sampling_rate:0,
        padding_bit:false,
        private_bit:false,
        channel_mode:0,
        mode_extension:0,
        copyright:false,
        original:false,
        emphasis:0};

    for byte in buffer
    {
        if byte == 0b11111111 {
            synced = true;
        }
        if synced {
            if i == 0
            {
                // Because fuck you're alignment. Stupid 11 bits.
                frame_sync |= byte as u32;
                frame_sync = frame_sync << 3;
            }
            if i == 1
            {
                // Steal the last part of the frame sync
                frame_sync |= byte as u32 >> 5;

                header.frame_sync = frame_sync as u16;
                header.layer_description = (byte >> 1) & 0b11;
                header.audio_version = (byte >> 3) & 0b11;
                header.protection_bit = if byte & 0b1 == 0b1 { false } else { true };
            }
            if i == 2
            {
                header.private_bit = if byte & 0b1 == 0b1 { true } else { false };
                header.padding_bit = if (byte >> 1) & 0b1 == 0b1 { true } else { false };
                header.sampling_rate = (byte >> 2) & 0b11;
                header.bitrate_index = (byte >> 4) & 0b1111;
            }
            if i == 3
            {
                header.emphasis = byte & 0b11;
                header.original = if (byte >> 2) & 0b1 == 0b1 { true } else { false };
                header.copyright = if (byte >> 3) & 0b1 == 0b1 { true } else { false };
                header.mode_extension = (byte >> 4) & 0b11;
                header.channel_mode = (byte >> 6) & 0b11;
                break;
            }
            i += 1;
        }
    }
    header
}

fn parse_file(filename: String) -> io::Result<()>
{
    let mut f = try!(File::open(filename));

    // The header is 32 bits, or 4 bytes long
    let mut buffer = vec![0; 20000];
    try!(f.read_exact(&mut buffer));
    output_header(parse_header(buffer));
    Ok(())
}

fn main()
{
    match parse_file("MoonlightSonata.mp3".to_string())
    {
        Ok(_) => println!("File parsed successfully."),
        Err(_) => println!("File was not parsed successfully.")
    }
}
