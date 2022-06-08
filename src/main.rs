use windows::core::HSTRING;
use windows::Foundation::Metadata::ApiInformation;

fn main() {
    let contract_name = HSTRING::from("Windows.Foundation.UniversalApiContract");
    // Version 10.0.18362.0
    let supported = ApiInformation::IsApiContractPresentByMajor(contract_name, 8);
    println!("Supported: {:?}", supported);
}
