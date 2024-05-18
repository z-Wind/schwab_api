use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceError {
    pub message: Option<String>,
    pub errors: Option<Vec<ErrorDetail>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetail {
    pub id: String,
    pub status: i64,
    pub title: String,
    pub detail: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_de() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/ServiceError_real.json"
        ));

        let val = serde_json::from_str::<ServiceError>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de2() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/ServiceError_real.json"
        ));

        let val = serde_json::from_str::<ServiceError>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }

    #[test]
    fn test_de3() {
        let json = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/model/Trader/ServiceError_real2.json"
        ));

        let val = serde_json::from_str::<ServiceError>(json);
        println!("{val:?}");
        assert!(val.is_ok());
    }
}
