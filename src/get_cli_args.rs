use docopt::Docopt;
use bitcoin::network::constants::Network as BtcNetwork;
use std::time::{
    SystemTime, 
    UNIX_EPOCH,
};
use crate::{
    errors::AppError,
    usage_info::USAGE_INFO,
    constants::BLOCK_EXPLORER_URL,
    types::{
        Result,
        BtcAddressesAndAmounts,
    },
};

#[allow(non_snake_case)]
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct CliArgs {
    pub flag_nonce: u64,
    pub flag_fee: usize,
    pub arg_data: String,
    pub cmd_getUtxos: bool,
    pub arg_to: Vec<String>, // FIXME How to check is valid address?
    pub flag_change: String, // FIXME How to check is valid address?
    pub flag_network: String,
    pub arg_amount: Vec<u64>, 
    pub flag_keyfile: String,
    pub flag_logLevel: String,
    pub arg_recipient: String,
    pub arg_btcAddress: String,
    pub cmd_makeOnlineTx: bool,
    pub arg_ethAddress: String,
    pub arg_tx_id: Vec<String>,
    pub cmd_makeOfflineTx: bool,
    pub arg_utxos: Option<String>,
    pub arg_utxo_indices: Vec<u32>,
    pub cmd_getUtxosForAddress: bool,
    pub flag_utxoFile: Option<String>, 
    pub cmd_makeOnlineOpReturnTx: bool,
    pub cmd_makeOfflineOpReturnTx: bool,
    pub cmd_getPBTCDepositAddress: bool,
    pub flag_outputPath: Option<String>,
}

pub fn get_addresses_and_amounts_from_cli_args(
    addresses: &Vec<String>,
    amounts: &Vec<u64>,
) -> BtcAddressesAndAmounts {
    addresses
        .iter()
        .enumerate()
        .map(|(i, address)| (address.clone(), amounts[i]))
        .collect::<BtcAddressesAndAmounts>()
}

pub fn get_network_from_cli_arg(network_cli_arg: &String) -> BtcNetwork {
    info!("✔ Getting network from cli-arg: '{}'", network_cli_arg);
    match &network_cli_arg[..] {
        "Testnet" => {
            info!("✔ Using network: 'Testnet'");
            BtcNetwork::Testnet
        }
        _ => {
            info!("✔ Using network: 'Bitcoin'");
            BtcNetwork::Bitcoin
        }
    }
}

pub fn get_nonce_from_cli_arg(nonce_cli_arg: &u64) -> Result<u64> {
    info!("✔ Getting nonce from cli-arg: `{}`", nonce_cli_arg);
    match nonce_cli_arg {
        0 => {
            info!("✔ Using timestamp as nonce!");
            Ok(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)?
                    .as_secs()
            )
        }
        _ => Ok(*nonce_cli_arg),
    }
}

pub fn get_api_endpoint_from_cli_args(network_cli_arg: &String) -> String {
    info!("✔ Getting API endpoint...");
    let api_url = match &network_cli_arg[..] {
        "Testnet" => format!("{}testnet/api/", BLOCK_EXPLORER_URL),
        _ => format!("{}/api/", BLOCK_EXPLORER_URL),
    };
    info!("✔ Using API endpoint: `{}`", api_url);
    api_url
}

pub fn get_cli_args() -> Result<CliArgs> {
    match Docopt::new(USAGE_INFO) 
        .and_then(|d| d.deserialize()) {
            Ok(cli_args) => Ok(cli_args),
            Err(e) => Err(AppError::Custom(
                format!("✘ Docopt error: {}", e)
            ))
        }
}
