use byteorder::{ByteOrder, LittleEndian};
use pcsc::*;
use std::str;
use std::u32;
fn main() {
    // Establish a PC/SC context.
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

    // List available readers.
    let mut readers_buf = [0; 2048];
    let mut readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            eprintln!("Failed to list readers: {}", err);
            std::process::exit(1);
        }
    };

    // Use the first reader.
    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            println!("No readers are connected.");
            return;
        }
    };
    println!("Using reader: {:?}", reader);

    // Connect to the card.
    let card = match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => card,
        Err(Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            return;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    };

    let version = b"\x00\xB2\x03\x84\x00";
    println!("Sending APDU: {:?}", version);
    let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
    let rapdu = match card.transmit(version, &mut rapdu_buf) {
        Ok(rapdu) => rapdu,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };

    let mut a = unpack_bcd(rapdu);
    ///   println!("{}.{}.{}", )

    println!("> {}", decode_bcd(&a[0..3]));
    println!("APDU response: {:?}", rapdu);
}

fn unpack_bcd(byte_array: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    for byte in byte_array {
        result.push(byte >> 4 & 0x0F);
        result.push(byte & 0x0F);
    }
    return result;
}

/*
def decode_bcd(half_byte_array):
    num = ''
    for byte in half_byte_array:
        byte &= 0b00001111
        assert byte < 10
        num += str(byte)
    return int(num)
*/
fn decode_bcd(byte_array: &[u8]) -> u32 {
    let mut result: Vec<u8> = vec![];
    let mut number: &str;
    for byte in byte_array {
        let mut a;
        let x: u8 = 10;
        a = byte & &0b00001111;
        assert!(a < x);
        result.push(a);
        //concat!(number, &str::from_utf8(&[a]).unwrap())
        //number + str::from_utf8(&[a]).unwrap()
    }
    println!("{:?} ", &result);

    println!(">> KK {}", str::from_utf8(&result).unwrap());

    print!("AAAAA  ::: {}", LittleEndian::read_u32(&result));

    return 3;
}

fn to_u32(slice: &[u8]) -> u32 {
    slice
        .iter()
        .fold((0, 1), |(acc, mul), &bit| {
            (acc + (mul * (1 & bit as u32)), mul.wrapping_add(mul))
        })
        .0
}
