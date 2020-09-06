use pcsc::*;
mod helpers;
mod insurance;
mod patient;
use flate2::read::GzDecoder;
use std::io::prelude::*;

const VERSION_1: &[u8] = b"\x00\xB2\x01\x84\x00";
const VERSION_2: &[u8] = b"\x00\xB2\x02\x84\x00";
const VERSION_3: &[u8] = b"\x00\xB2\x03\x84\x00";

#[derive(Debug)]
enum TerminalType {
    MOBILE,
    FIXED,
    UNKNOW,
}

#[derive(Debug)]
enum CardType {
    SYNC,
    ASYNC,
    NOTSAVED,
    UNKNOW,
}
///Kartenterminatype ermittel
/// 9000 - stationäres Kartenterminal
/// 9500 - mobiles Kartenterminal
const RESET_CT: &[u8] = b"\x00\xA4\x04\x0C\x07\xD2\x76\x00\x01\x44\x80\x00";

///Kartentype ermittel
/// 9000 - synchrone Karte
/// 9001 - asynchrone Karte
/// 6200 -  keine Karte (eGK oder KVK) gespeichert
const REQUEST_ICC1: &[u8] = b"\x20\x12\x01\x00\x01\x01";

const SELECT_MF: &[u8] = b"\x00\xA4\x04\x0C\x07\xD2\x76\x00\x01\x44\x80\x00";
const SELECT_HCA: &[u8] = b"\x00\xA4\x04\x0C\x06\xD2\x76\x00\x00\x01\x02";
const SELECT_FILE_PD: &[u8] = b"\x00\xB0\x81\x00\x02";
const SELECT_FILE_VD: &[u8] = b"\x00\xB0\x82\x00\x08";
const EJECT_CARD: &[u8] = b"\x20\x15\x01\x00\x01\x01";

fn main() {
    // Establish a PC/SC context.
    let ctx = match pcsc::Context::establish(Scope::User) {
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

    //TEST CODE
    get_card(&card);
}
pub fn get_card(card: &Card) -> (serde_json::Value, serde_json::Value) {
    send_apdu(&card, SELECT_MF);
    get_card_generation(&card);
    send_apdu(&card, SELECT_HCA);
    send_apdu(&card, SELECT_FILE_PD);

    let data = send_apdu(&card, &helpers::create_read_command(0x00, 0x02));
    let mut pd_length = ((data[0] as usize) << 8) + data[1] as usize;
    pd_length -= 0x02;

    send_apdu(&card, SELECT_MF);
    send_apdu(&card, SELECT_HCA);
    send_apdu(&card, SELECT_FILE_PD);
    let mut patient_data_compressed = read_file(&card, 0x02, pd_length);

    send_apdu(&card, SELECT_MF);
    send_apdu(&card, SELECT_HCA);
    send_apdu(&card, SELECT_FILE_VD);

    let data = send_apdu(&card, &helpers::create_read_command(0x00, 0x08));

    let vd_start = ((data[0] as usize) << 8) + data[1] as usize;
    let vd_end = ((data[2] as usize) << 8) + data[3] as usize;
    let vd_length = vd_end - (vd_start - 1);

    send_apdu(&card, SELECT_MF);
    send_apdu(&card, SELECT_HCA);
    send_apdu(&card, SELECT_FILE_VD);
    let mut insurance_data_compressed = read_file(&card, vd_start, vd_length);

    patient_data_compressed.extend(&vec![0x00; 16]);
    let mut decoder_patient_data = GzDecoder::new(&patient_data_compressed as &[u8]);
    let mut patient_data_xml = String::new();
    decoder_patient_data
        .read_to_string(&mut patient_data_xml)
        .unwrap();

    insurance_data_compressed.extend(&vec![0x00; 16]);
    let mut decoder_insurance_data = GzDecoder::new(&insurance_data_compressed as &[u8]);
    let mut insurance_data_xml = String::new();
    decoder_insurance_data
        .read_to_string(&mut insurance_data_xml)
        .unwrap();

    return (
        patient::parse_patient_to_json(&patient_data_xml),
        insurance::parse_insurance_to_json(&insurance_data_xml),
    );
}

pub fn read_file(card: &Card, offset: usize, length: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    let max_read: u8 = 0xFC;
    let mut pointer = offset;
    while result.len() < length {
        let bytes_left = length - result.len();
        let readlen: usize;
        if bytes_left < max_read as usize {
            readlen = bytes_left;
        } else {
            readlen = max_read as usize;
        }
        let data_chunk = send_apdu(
            &card,
            &helpers::create_read_command(pointer as i32, readlen as i32),
        );
        pointer += readlen;
        result.extend(&data_chunk[0..data_chunk.len() - 2]);
    }
    return result;
}

fn get_card_generation(card: &Card) -> String {
    let version1 = get_version(&card, VERSION_1);
    let version2 = get_version(&card, VERSION_2);
    let version3 = get_version(&card, VERSION_3);
    let mut generation = String::from("unknow");
    if version1 == "3.0.0" && version2 == "3.0.0" && version3 == "3.0.2" {
        generation = String::from("G1");
    } else if version1 == "3.0.0" && version2 == "3.0.1" && version3 == "3.0.3" {
        generation = String::from("G1 plus");
    } else if version1 == "4.0.0" && version2 == "4.0.0" && version3 == "4.0.0" {
        generation = String::from("G2");
    }
    return generation;
}

fn get_card_type(card: Card) -> CardType {
    let a = send_apdu(&card, REQUEST_ICC1);
    let type_number = helpers::decode_bcd(&a[0..2]);
    if type_number == 9000 {
        return CardType::SYNC;
    } else if type_number == 9001 {
        return CardType::ASYNC;
    } else if type_number == 6200 {
        return CardType::NOTSAVED;
    }
    return CardType::UNKNOW;
}

fn get_termial_type(card: Card) -> TerminalType {
    let a = send_apdu(&card, RESET_CT);
    let type_number = helpers::decode_bcd(&a[0..4]);
    if type_number == 9000 {
        return TerminalType::FIXED;
    } else if type_number == 9500 {
        return TerminalType::MOBILE;
    }
    return TerminalType::UNKNOW;
}

fn get_version(card: &Card, version_string: &[u8]) -> String {
    let a = helpers::unpack_bcd(&send_apdu(&card, version_string));
    return format!(
        "{}.{}.{}",
        helpers::decode_bcd(&a[0..3]),
        helpers::decode_bcd(&a[3..6]),
        helpers::decode_bcd(&a[6..10])
    );
}

fn send_apdu(card: &Card, cmd: &[u8]) -> Vec<u8> {
    let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
    let rapdu = match card.transmit(cmd, &mut rapdu_buf) {
        Ok(rapdu) => rapdu,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };
    return rapdu.iter().cloned().collect();
}
