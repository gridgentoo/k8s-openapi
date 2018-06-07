// Generated from definition io.k8s.kubernetes.pkg.apis.autoscaling.v2alpha1.PodsMetricStatus

/// PodsMetricStatus indicates the current value of a metric describing each pod in the current scale target (for example, transactions-processed-per-second).
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PodsMetricStatus {
    /// currentAverageValue is the current value of the average of the metric across all relevant pods (as a quantity)
    #[serde(rename = "currentAverageValue")]
    pub current_average_value: ::v1_7::apimachinery::pkg::api::resource::Quantity,

    /// metricName is the name of the metric in question
    #[serde(rename = "metricName")]
    pub metric_name: String,
}
