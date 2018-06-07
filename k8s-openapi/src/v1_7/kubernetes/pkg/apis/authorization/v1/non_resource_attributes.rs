// Generated from definition io.k8s.kubernetes.pkg.apis.authorization.v1.NonResourceAttributes

/// NonResourceAttributes includes the authorization attributes available for non-resource requests to the Authorizer interface
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NonResourceAttributes {
    /// Path is the URL path of the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Verb is the standard HTTP verb
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verb: Option<String>,
}
