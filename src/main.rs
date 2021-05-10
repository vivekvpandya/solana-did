use crate::state::{DecentralizedIdentifier, SolData, VerificationMethod};
use clap::{App, Arg};
use qrcode::render::unicode;
use qrcode::QrCode;
use solana_program::pubkey::Pubkey;
mod error;
mod state;

fn main() {
    let app = App::new("solana-did")
                .version("0.1.0")
                .about("Reads solana DID with --in argument, specify --json to export as JSON, specify --qr to export as QRCode image.")
                .author("Vivek Pandya")
                .arg(Arg::with_name("input")
                            .short("i")
                            .long("in")
                            .value_name("in_str")
                            .required(true)
                            .help("DID with following format did:sol:devnet:8khtkDMiZAXChwZNbKu4ugry1mdJ3d8RW6P62VD9UbA1")
                            .takes_value(true))
                .arg(Arg::with_name("json")
                            .short("j")
                            .long("json")
                            .required(false)
							.takes_value(false)
                            .help("Export DID document as JSON"))
				.arg(Arg::with_name("qrcode")
                            .short("q")
                            .long("qr")
                            .required(false)
							.takes_value(false)
                            .help("Export DID document as QRCode")
                            );

    let matches = app.get_matches();
    let input_graph = matches.value_of("input").unwrap();
    let as_json = matches.is_present("json");
    let as_qr = matches.is_present("qrcode");

    // TODO: code to connect to Solana

    /// Pubkey for tests
    pub const TEST_PUBKEY: Pubkey = Pubkey::new_from_array([100; 32]);
    println!("Pubkey : {}", TEST_PUBKEY);
    /// DID for tests
    pub const TEST_KEY_ID: &str = "did:sol:FcFhBFRf6smQ48p7jFcE35uNuE9ScuUu6R2rdFtWjWhP#key1";
    /// Controller for tests
    pub const TEST_DID: &str = "did:sol:FcFhBFRf6smQ48p7jFcE35uNuE9ScuUu6R2rdFtWjWhP";

    pub fn test_did(sol_data: &'static SolData) -> DecentralizedIdentifier {
        DecentralizedIdentifier::new(sol_data)
    }

    pub fn test_verification_method() -> VerificationMethod {
        VerificationMethod {
            id: VerificationMethod::DEFAULT_KEY_ID.to_string(),
            verification_type: VerificationMethod::DEFAULT_TYPE.to_string(),
            pubkey: TEST_PUBKEY,
        }
    }

    pub fn test_sol_data() -> SolData {
        SolData {
            authority: TEST_PUBKEY,
            version: SolData::DEFAULT_VERSION.to_string(),
            verification_method: vec![test_verification_method()],
            authentication: vec![VerificationMethod::DEFAULT_KEY_ID.to_string()],
            capability_invocation: vec![VerificationMethod::DEFAULT_KEY_ID.to_string()],
            capability_delegation: vec![],
            key_agreement: vec![],
            assertion_method: vec![],
            service: vec![],
        }
    }
    let data = test_sol_data();
    let serialized = serde_json::to_string_pretty(&data).unwrap();
    if as_json {
        println!("{}", serialized);
    }
    if as_qr {
        let code = QrCode::new(serialized).unwrap();
        let image = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{}", image);
    }
}
