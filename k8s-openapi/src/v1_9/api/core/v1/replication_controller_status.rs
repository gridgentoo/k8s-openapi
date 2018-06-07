// Generated from definition io.k8s.api.core.v1.ReplicationControllerStatus

/// ReplicationControllerStatus represents the current status of a replication controller.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ReplicationControllerStatus {
    /// The number of available replicas (ready for at least minReadySeconds) for this replication controller.
    #[serde(rename = "availableReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_replicas: Option<i32>,

    /// Represents the latest available observations of a replication controller's current state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<::v1_9::api::core::v1::ReplicationControllerCondition>>,

    /// The number of pods that have labels matching the labels of the pod template of the replication controller.
    #[serde(rename = "fullyLabeledReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fully_labeled_replicas: Option<i32>,

    /// ObservedGeneration reflects the generation of the most recently observed replication controller.
    #[serde(rename = "observedGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,

    /// The number of ready replicas for this replication controller.
    #[serde(rename = "readyReplicas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i32>,

    /// Replicas is the most recently oberved number of replicas. More info: https://kubernetes.io/docs/concepts/workloads/controllers/replicationcontroller#what-is-a-replicationcontroller
    pub replicas: i32,
}
