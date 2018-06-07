// Generated from definition io.k8s.kubernetes.pkg.api.v1.LoadBalancerStatus

/// LoadBalancerStatus represents the status of a load-balancer.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LoadBalancerStatus {
    /// Ingress is a list containing ingress points for the load-balancer. Traffic intended for the service should be sent to these ingress points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingress: Option<Vec<::v1_7::kubernetes::pkg::api::v1::LoadBalancerIngress>>,
}
