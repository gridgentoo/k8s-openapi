// Generated from definition io.k8s.kubernetes.pkg.apis.extensions.v1beta1.NetworkPolicyPort

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct NetworkPolicyPort {
    /// If specified, the port on the given protocol.  This can either be a numerical or named port on a pod.  If this field is not provided, this matches all port names and numbers. If present, only traffic on the specified protocol AND port will be matched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<::v1_7::apimachinery::pkg::util::intstr::IntOrString>,

    /// Optional.  The protocol (TCP or UDP) which traffic must match. If not specified, this field defaults to TCP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
}
