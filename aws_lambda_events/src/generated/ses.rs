use chrono::{DateTime, Utc};
use custom_serde::*;

/// `SimpleEmailEvent` is the outer structure of an event sent via SES.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SimpleEmailRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventVersion")]
    pub event_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSource")]
    pub event_source: Option<String>,
    pub ses: SimpleEmailService,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailService {
    pub mail: SimpleEmailMessage,
    pub receipt: SimpleEmailReceipt,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailMessage {
    #[serde(rename = "commonHeaders")]
    pub common_headers: SimpleEmailCommonHeaders,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub destination: Vec<String>,
    pub headers: Vec<SimpleEmailHeader>,
    #[serde(rename = "headersTruncated")]
    pub headers_truncated: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailReceipt {
    pub recipients: Vec<String>,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "spamVerdict")]
    pub spam_verdict: SimpleEmailVerdict,
    #[serde(rename = "dkimVerdict")]
    pub dkim_verdict: SimpleEmailVerdict,
    #[serde(rename = "dmarcVerdict")]
    pub dmarc_verdict: SimpleEmailVerdict,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "dmarcPolicy")]
    pub dmarc_policy: Option<String>,
    #[serde(rename = "spfVerdict")]
    pub spf_verdict: SimpleEmailVerdict,
    #[serde(rename = "virusVerdict")]
    pub virus_verdict: SimpleEmailVerdict,
    pub action: SimpleEmailReceiptAction,
    #[serde(rename = "processingTimeMillis")]
    pub processing_time_millis: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailHeader {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailCommonHeaders {
    pub from: Vec<String>,
    pub to: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "returnPath")]
    pub return_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub date: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub subject: Option<String>,
}

/// `SimpleEmailReceiptAction` is a logical union of fields present in all action
/// Types. For example, the FunctionARN and InvocationType fields are only
/// present for the Lambda Type, and the BucketName and ObjectKey fields are only
/// present for the S3 Type.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailReceiptAction {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "topicArn")]
    pub topic_arn: Option<String>,
    #[serde(rename = "bucketName")]
    pub bucket_name: Option<String>,
    #[serde(rename = "objectKey")]
    pub object_key: Option<String>,
    #[serde(rename = "smtpReplyCode")]
    pub smtp_reply_code: Option<String>,
    #[serde(rename = "statusCode")]
    pub status_code: Option<String>,
    pub message: Option<String>,
    pub sender: Option<String>,
    #[serde(rename = "invocationType")]
    pub invocation_type: Option<String>,
    #[serde(rename = "functionArn")]
    pub function_arn: Option<String>,
    #[serde(rename = "organizationArn")]
    pub organization_arn: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailVerdict {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub status: Option<String>,
}

pub type SimpleEmailDispositionValue = String;

/// `SimpleEmailDisposition` disposition return for SES to control rule functions
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailDisposition {
    pub disposition: SimpleEmailDispositionValue,
}
