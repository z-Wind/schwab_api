use serde::Deserialize;
use serde::Serialize;

pub type UserPreferences = Vec<UserPreference>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreference {
    pub accounts: Vec<UserPreferenceAccount>,
    pub streamer_info: Vec<StreamerInfo>,
    pub offers: Vec<Offer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferenceAccount {
    pub account_number: String,
    pub primary_account: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub nick_name: String,
    pub account_color: String,
    pub display_acct_id: String,
    pub auto_position_effect: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamerInfo {
    pub streamer_socket_url: String,
    pub schwab_client_customer_id: String,
    pub schwab_client_correl_id: String,
    pub schwab_client_channel: String,
    pub schwab_client_function_id: String,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offer {
    #[serde(rename = "level2Permissions")]
    pub level2permissions: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/UserPreferences.json"
        ));

        let val = serde_json::from_str::<UserPreferences>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}