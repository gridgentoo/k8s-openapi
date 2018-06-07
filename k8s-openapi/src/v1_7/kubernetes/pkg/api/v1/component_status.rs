// Generated from definition io.k8s.kubernetes.pkg.api.v1.ComponentStatus

/// ComponentStatus (and ComponentStatusList) holds the cluster validation info.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ComponentStatus {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// List of component conditions observed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::v1_7::kubernetes::pkg::api::v1::ComponentCondition>>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::v1_7::apimachinery::pkg::apis::meta::v1::ObjectMeta>,
}
