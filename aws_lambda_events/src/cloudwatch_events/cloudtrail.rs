use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AWSAPICall<I = Value, O = Value> {
    pub event_version: String,
    pub user_identity: UserIdentity,
    pub event_time: String,
    pub event_source: String,
    pub event_name: String,
    pub aws_region: String,
    #[serde(rename = "sourceIPAddress")]
    pub source_ipaddress: String,
    pub user_agent: String,
    pub request_parameters: I,
    pub response_elements: O,
    #[serde(rename = "requestID")]
    pub request_id: String,
    #[serde(rename = "eventID")]
    pub event_id: String,
    pub event_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserIdentity {
    pub r#type: String,
    pub principal_id: String,
    pub arn: String,
    pub account_id: String,
    pub session_context: SessionContext,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionContext {
    pub attributes: Attributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub mfa_authenticated: String,
    pub creation_date: String,
}
