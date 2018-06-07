// Generated from definition io.k8s.api.rbac.v1.ClusterRole

/// ClusterRole is a cluster level, logical grouping of PolicyRules that can be referenced as a unit by a RoleBinding or ClusterRoleBinding.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ClusterRole {
    /// APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#resources
    #[serde(rename = "apiVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,

    /// Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/api-conventions.md#types-kinds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    /// Standard object's metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<::v1_8::apimachinery::pkg::apis::meta::v1::ObjectMeta>,

    /// Rules holds all the PolicyRules for this ClusterRole
    pub rules: Vec<::v1_8::api::rbac::v1::PolicyRule>,
}
