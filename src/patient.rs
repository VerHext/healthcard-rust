use serde_json::json;
use xpath_reader::{Context, Reader};

pub fn parse_patient_to_json(xml: &str) -> serde_json::Value {
    let mut context = Context::new();
    context.set_namespace("vsd", "http://ws.gematik.de/fa/vsdm/vsd/v5.2");

    let reader = Reader::from_str(&xml, Some(&context)).unwrap();

    let cmd_version: String = reader.read("//@CDM_VERSION").unwrap_or(String::new());
    let insurant_id: String = reader
        .read("//vsd:Versicherten_ID//text()")
        .unwrap_or(String::new());
    let birthdate: String = reader
        .read("//vsd:Geburtsdatum//text()")
        .unwrap_or(String::new());
    let first_name: String = reader
        .read("//vsd:Vorname//text()")
        .unwrap_or(String::new());
    let last_name: String = reader
        .read("//vsd:Nachname//text()")
        .unwrap_or(String::new());
    let gender: String = reader
        .read("//vsd:Geschlecht//text()")
        .unwrap_or(String::new());
    let prefix: String = reader
        .read("//vsd:Vorsatzwort//text()")
        .unwrap_or(String::new());
    let name_addition: String = reader
        .read("//vsd:Namenszusatz//text()")
        .unwrap_or(String::new());
    let title: String = reader.read("//vsd:Titel//text()").unwrap_or(String::new());
    let zip_code: String = reader
        .read("//vsd:StrassenAdresse//vsd:Postleitzahl//text()")
        .unwrap_or(String::new());
    let city: String = reader
        .read("//vsd:StrassenAdresse//vsd:Ort//text()")
        .unwrap_or(String::new());
    let country: String = reader
        .read("//vsd:StrassenAdresse//vsd:Land//vsd:Wohnsitzlaendercode//text()")
        .unwrap_or(String::new());
    let street: String = reader
        .read("//vsd:StrassenAdresse//vsd:Strasse//text()")
        .unwrap_or(String::new());
    let house_number: String = reader
        .read("//vsd:StrassenAdresse//vsd:Hausnummer//text()")
        .unwrap_or(String::new());

    let pa_city: String = reader
        .read("//vsd:PostfachAdresse//vsd:Ort//text()")
        .unwrap_or(String::new());
    let pa_zip_code: String = reader
        .read("//vsd:PostfachAdresse//vsd:Postleitzahl//text()")
        .unwrap_or(String::new());
    let pa_mailbox: String = reader
        .read("//vsd:PostfachAdresse//vsd:Postfach//text()")
        .unwrap_or(String::new());

    return json!({
        "cmdVersion": cmd_version,
        "insurantId": insurant_id,
        "birthdate": birthdate,
        "firstName": first_name,
        "lastName": last_name,
        "gender": gender,
        "prefix": prefix,
        "nameAddition": name_addition,
        "title": title,
        "residenceAddress":{
            "street": street,
            "zipCode": zip_code,
            "city": city,
            "country": country,
            "houseNumber": house_number
        },
        "postalAddress":{
            "city": pa_city,
            "zipCode": pa_zip_code,
            "mailbox": pa_mailbox
        }
    });
}
