#[tokio::test]
async fn create() {
	use k8s_openapi::api::core::v1 as api;
	use k8s_openapi::api::batch::v1 as batch;
	use k8s_openapi::apimachinery::pkg::apis::meta::v1 as meta;

	let mut client = crate::Client::new("job-create");

	let job_spec = batch::JobSpec {
		backoff_limit: Some(0),
		template: api::PodTemplateSpec {
			spec: Some(api::PodSpec {
				containers: vec![
					api::Container {
						name: "k8s-openapi-tests-create-job".to_string(),
						image: "alpine".to_string().into(),
						command: Some(vec![
							"sh".to_string(),
							"-c".to_string(),
							"exit $TEST_ARG".to_string(),
						]),
						env: Some(vec![
							api::EnvVar {
								name: "TEST_ARG".to_string(),
								value: Some("5".to_string()),
								..Default::default()
							},
						]),
						..Default::default()
					},
				],
				restart_policy: Some("Never".to_string()),
				..Default::default()
			}),
			..Default::default()
		},
		..Default::default()
	};

	let job = batch::Job {
		metadata: meta::ObjectMeta {
			name: Some("k8s-openapi-tests-create-job".to_string()),
			..Default::default()
		},
		spec: Some(job_spec),
		..Default::default()
	};

	let (request, response_body) =
		batch::Job::create("default", &job, Default::default())
		.expect("couldn't create job");
	let job: batch::Job = match client.get_single_value(request, response_body).await {
		(k8s_openapi::CreateResponse::Created(job), _) => job,
		(other, status_code) => panic!("{other:?} {status_code}"),
	};

	let job_image =
		job
		.spec.expect("couldn't get job spec")
		.template
		.spec.expect("couldn't get job spec template spec")
		.containers.into_iter().next().expect("couldn't get job container spec")
		.image.expect("couldn't get job container image");
	assert_eq!(job_image, "alpine");

	let (job_name, job_uid) = {
		let metadata = job.metadata;
		(metadata.name.expect("couldn't get job name"), metadata.uid.expect("couldn't get job uid"))
	};

	// Wait for job to fail
	loop {
		let (request, response_body) = batch::Job::read(&job_name, "default").expect("couldn't get job");
		let job: batch::Job = match client.get_single_value(request, response_body).await {
			(batch::ReadJobResponse::Ok(job), _) => job,
			(other, status_code) => panic!("{other:?} {status_code}"),
		};

		let job_status =
			job
			.status.expect("couldn't get job status");

		if job_status.failed == Some(1) {
			break;
		}

		tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	}

	// Find a pod of the failed job using owner reference
	let job_pod_status = loop {
		let (request, response_body) = api::Pod::list("default", Default::default()).expect("couldn't list pods");
		let pod_list = match client.get_single_value(request, response_body).await {
			(k8s_openapi::ListResponse::Ok(pod_list), _) => pod_list,
			(other, status_code) => panic!("{other:?} {status_code}"),
		};

		let job_pod_status =
			pod_list
			.items.into_iter()
			.find(|pod|
				pod.metadata.owner_references.as_ref()
				.and_then(|owner_references| owner_references.first())
				.map(|owner_reference| owner_reference.uid.as_ref()) == Some(&*job_uid))
			.and_then(|job_pod| job_pod.status);

		if let Some(job_pod_status) = job_pod_status {
			if job_pod_status.phase == Some("Failed".to_string()) {
				break job_pod_status;
			}
		}

		tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	};

	let job_pod_container_state_terminated =
		job_pod_status
		.container_statuses.expect("couldn't get job pod container statuses")
		.into_iter().next().expect("couldn't get job pod container status")
		.state.expect("couldn't get job pod container state")
		.terminated.expect("couldn't get job pod container termination info");
	assert_eq!(job_pod_container_state_terminated.exit_code, 5);

	let (request, response_body) = batch::Job::delete(&job_name, "default", Default::default()).expect("couldn't delete job");
	match client.get_single_value(request, response_body).await {
		(k8s_openapi::DeleteResponse::OkStatus(_) | k8s_openapi::DeleteResponse::OkValue(_), _) => (),
		(other, status_code) => panic!("{other:?} {status_code}"),
	}

	// Delete all pods of the job using label selector
	let (request, response_body) =
		api::Pod::delete_collection(
			"default",
			Default::default(),
			k8s_openapi::ListOptional {
				label_selector: Some("job-name=k8s-openapi-tests-create-job"),
				..Default::default()
			},
		)
		.expect("couldn't delete pods collection");
	match client.get_single_value(request, response_body).await {
		(k8s_openapi::DeleteResponse::OkStatus(_) | k8s_openapi::DeleteResponse::OkValue(_), _) => (),
		(other, status_code) => panic!("{other:?} {status_code}"),
	}
}
