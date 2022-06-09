use windows::core::{Interface, HSTRING};
use windows::Foundation::Metadata::ApiInformation;
use windows::Win32::System::WinRT::Graphics::Capture::IGraphicsCaptureItemInterop;

fn main() {
    let contract_name = HSTRING::from("Windows.Foundation.UniversalApiContract");
    // Version 10.0.18362.0
    let supported = ApiInformation::IsApiContractPresentByMajor(contract_name, 8);
    println!("Supported: {:?}", supported);

    //   let iid = GUID::from_u128(0x3628e81b_3cac_4c60_b7f4_23ce0e0c3356);
    let iid = IGraphicsCaptureItemInterop::IID;
    println!("IID: {:?}", iid);
}
