# healthcard-rust

With this rust library you can simply read data as json from German public health insurance cards. (Elektronische Gesundheitskarte eGK)

It is based off of this repo:
https://github.com/Blueshoe/python-healthcard

## Usage

A easy example to get insurance data as json object.

```rust
use healthcard_rust::*;

fn main() {
    println!("Read data from German public health insurance cards (eGK)");
    let card = get_card();
    println!("Generation {:?}", healthcard_rust::get_card_generation(&card));
    println!("{:?}", healthcard_rust::get_card_data(&card).to_string())
}

```

The lib is automatically looking for a card reader. If there are multiple card readers
available it uses the first one be default.

## Example response

```json
{
  "insurance": {
    "additionalInfoBillingGkv": "number",
    "cmdVersion": "5.2.0",
    "costAccounting": "number",
    "costCarrierCountry": "string?",
    "costCarrierId": "number",
    "costCarrierName": "string?",
    "costCountry": "string?",
    "costName": "string?",
    "typeOfInsurance": "number?"
  },
  "patient": {
    "birthdate": "number",
    "cmdVersion": "5.2.0",
    "firstName": "string",
    "gender": "string?",
    "insurantId": "number",
    "lastName": "string",
    "nameAddition": "string?",
    "postalAddress": {
      "city": "string?",
      "mailbox": "string?",
      "zipCode": "string?"
    },
    "prefix": "string?",
    "residenceAddress": {
      "city": "string?",
      "country": "string?",
      "houseNumber": "number?",
      "street": "string?",
      "zipCode": "string?"
    },
    "title": ""
  }
}
```

You can read more about the implementaion of the eGk in German: https://fachportal.gematik.de/fileadmin/user_upload/fachportal/files/Spezifikationen/Basis-Rollout/Elektronische_Gesundheitskarte/gemLF_Impl_eGK_V160.pdf

## License

Licensed under the MIT license.
