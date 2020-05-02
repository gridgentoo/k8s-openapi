initSidebarItems({"enum":[["CreateCustomResourceDefinitionResponse","Use `<CreateCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::create_custom_resource_definition`]"],["DeleteCollectionCustomResourceDefinitionResponse","Use `<DeleteCollectionCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::delete_collection_custom_resource_definition`]"],["DeleteCustomResourceDefinitionResponse","Use `<DeleteCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::delete_custom_resource_definition`]"],["JSONSchemaPropsOrArray","JSONSchemaPropsOrArray represents a value that can either be a JSONSchemaProps or an array of JSONSchemaProps. Mainly here for serialization purposes."],["JSONSchemaPropsOrBool","JSONSchemaPropsOrBool represents JSONSchemaProps or a boolean value. Defaults to true for the boolean property."],["JSONSchemaPropsOrStringArray","JSONSchemaPropsOrStringArray represents a JSONSchemaProps or a string array."],["ListCustomResourceDefinitionResponse","Use `<ListCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::list_custom_resource_definition`]"],["PatchCustomResourceDefinitionResponse","Use `<PatchCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::patch_custom_resource_definition`]"],["PatchCustomResourceDefinitionStatusResponse","Use `<PatchCustomResourceDefinitionStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::patch_custom_resource_definition_status`]"],["ReadCustomResourceDefinitionResponse","Use `<ReadCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::read_custom_resource_definition`]"],["ReadCustomResourceDefinitionStatusResponse","Use `<ReadCustomResourceDefinitionStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::read_custom_resource_definition_status`]"],["ReplaceCustomResourceDefinitionResponse","Use `<ReplaceCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::replace_custom_resource_definition`]"],["ReplaceCustomResourceDefinitionStatusResponse","Use `<ReplaceCustomResourceDefinitionStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::replace_custom_resource_definition_status`]"],["WatchCustomResourceDefinitionResponse","Use `<WatchCustomResourceDefinitionResponse as Response>::try_from_parts` to parse the HTTP response body of [`CustomResourceDefinition::watch_custom_resource_definition`]"]],"struct":[["CreateCustomResourceDefinitionOptional","Optional parameters of [`CustomResourceDefinition::create_custom_resource_definition`]"],["CustomResourceColumnDefinition","CustomResourceColumnDefinition specifies a column for server side printing."],["CustomResourceConversion","CustomResourceConversion describes how to convert different versions of a CR."],["CustomResourceDefinition","CustomResourceDefinition represents a resource that should be exposed on the API server.  Its name MUST be in the format <.spec.name>.<.spec.group>. Deprecated in v1.16, planned for removal in v1.19. Use apiextensions.k8s.io/v1 CustomResourceDefinition instead."],["CustomResourceDefinitionCondition","CustomResourceDefinitionCondition contains details for the current condition of this pod."],["CustomResourceDefinitionList","CustomResourceDefinitionList is a list of CustomResourceDefinition objects."],["CustomResourceDefinitionNames","CustomResourceDefinitionNames indicates the names to serve this CustomResourceDefinition"],["CustomResourceDefinitionSpec","CustomResourceDefinitionSpec describes how a user wants their resource to appear"],["CustomResourceDefinitionStatus","CustomResourceDefinitionStatus indicates the state of the CustomResourceDefinition"],["CustomResourceDefinitionVersion","CustomResourceDefinitionVersion describes a version for CRD."],["CustomResourceSubresourceScale","CustomResourceSubresourceScale defines how to serve the scale subresource for CustomResources."],["CustomResourceSubresourceStatus","CustomResourceSubresourceStatus defines how to serve the status subresource for CustomResources. Status is represented by the `.status` JSON path inside of a CustomResource. When set, * exposes a /status subresource for the custom resource * PUT requests to the /status subresource take a custom resource object, and ignore changes to anything except the status stanza * PUT/POST/PATCH requests to the custom resource ignore changes to the status stanza"],["CustomResourceSubresources","CustomResourceSubresources defines the status and scale subresources for CustomResources."],["CustomResourceValidation","CustomResourceValidation is a list of validation methods for CustomResources."],["ExternalDocumentation","ExternalDocumentation allows referencing an external resource for extended documentation."],["JSON","JSON represents any valid JSON value. These types are supported: bool, int64, float64, string, []interface{}, map[string]interface{} and nil."],["JSONSchemaProps","JSONSchemaProps is a JSON-Schema following Specification Draft 4 (http://json-schema.org/)."],["ReadCustomResourceDefinitionOptional","Optional parameters of [`CustomResourceDefinition::read_custom_resource_definition`]"],["ReadCustomResourceDefinitionStatusOptional","Optional parameters of [`CustomResourceDefinition::read_custom_resource_definition_status`]"],["ReplaceCustomResourceDefinitionOptional","Optional parameters of [`CustomResourceDefinition::replace_custom_resource_definition`]"],["ReplaceCustomResourceDefinitionStatusOptional","Optional parameters of [`CustomResourceDefinition::replace_custom_resource_definition_status`]"],["ServiceReference","ServiceReference holds a reference to Service.legacy.k8s.io"],["WebhookClientConfig","WebhookClientConfig contains the information to make a TLS connection with the webhook."]]});