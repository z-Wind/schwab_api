use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceError {
    pub message: String,
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/ServiceError.json"
        ));

        let val = serde_json::from_str::<ServiceError>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}