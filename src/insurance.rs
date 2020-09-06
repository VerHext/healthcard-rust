use serde_json::json;
use xpath_reader::{Context, Reader};

pub fn parse_insurance_to_json(xml: &str) -> serde_json::Value {
    let mut context = Context::new();
    context.set_namespace("vsd", "http://ws.gematik.de/fa/vsdm/vsd/v5.2");
    let reader = Reader::from_str(&xml, Some(&context)).unwrap();

    let cmd_version: String = reader.read("//@CDM_VERSION").unwrap_or(String::new());
    let insurance_start_date: String = reader
        .read("//vsd:Versicherungsschutz//vsd:Beginn//text()")
        .unwrap_or(String::new());
    let cost_accounting: String = reader
        .read("//vsd:Versicherungsschutz//vsd:Kostentraegerkennung//text()")
        .unwrap_or(String::new());
    let cost_country: String = reader
        .read("//vsd:Versicherungsschutz//vsd:Kostentraegerlaendercode//text()")
        .unwrap_or(String::new());
    let cost_name: String = reader
        .read("//vsd:Versicherungsschutz//vsd:Name//text()")
        .unwrap_or(String::new());
    let cost_carrier_id: String = reader
        .read("//vsd:Versicherungsschutz//vsd:AbrechnenderKostentraeger//vsd:Kostentraegerkennung//text()")
        .unwrap_or(String::new());
    let cost_carrier_country: String = reader
        .read("//vsd:Versicherungsschutz//vsd:AbrechnenderKostentraeger//vsd:Kostentraegerlaendercode//text()")
        .unwrap_or(String::new());
    let cost_carrier_name: String = reader
        .read("//vsd:Versicherungsschutz//vsd:AbrechnenderKostentraeger//vsd:Name//text()")
        .unwrap_or(String::new());

    let type_of_insurance: String = reader
        .read("//vsd:Zusatzinfos//vsd:ZusatzinfosGKV//vsd:Versichertenart//text()")
        .unwrap_or(String::new());
    let additional_info_billing_gkv: String = reader
        .read("//vsd:Zusatzinfos//vsd:ZusatzinfosGKV//vsd:Zusatzinfos_Abrechnung_GKV//vsd:WOP//text()")
        .unwrap_or(String::new());

    println!("{}", insurance_start_date);
    return json!({
        "cmdVersion": cmd_version,
        "costAccounting": cost_accounting,
        "costCountry": cost_country,
        "costName": cost_name,
        "costCarrierId": cost_carrier_id,
        "costCarrierCountry": cost_carrier_country,
        "costCarrierName": cost_carrier_name,
        "typeOfInsurance": type_of_insurance,
        "additionalInfoBillingGkv": additional_info_billing_gkv
    });
}
