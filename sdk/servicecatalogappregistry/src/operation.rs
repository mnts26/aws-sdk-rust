// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Associates an attribute group with an application to augment the application's metadata
/// with the group's attributes. This feature enables applications to be described with
/// user-defined details that are machine-readable, such as third-party integrations.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateAttributeGroup {
    _private: (),
}
impl AssociateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`AssociateAttributeGroupInput`](crate::input::AssociateAttributeGroupInput)
    pub fn builder() -> crate::input::associate_attribute_group_input::Builder {
        crate::input::associate_attribute_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AssociateAttributeGroup {
    type Output = std::result::Result<
        crate::output::AssociateAttributeGroupOutput,
        crate::error::AssociateAttributeGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_attribute_group_error(response)
        } else {
            crate::operation_deser::parse_associate_attribute_group_response(response)
        }
    }
}

/// <p>Associates a resource with an application. Both the resource and the application can be specified either by ID or name.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateResource {
    _private: (),
}
impl AssociateResource {
    /// Creates a new builder-style object to manufacture [`AssociateResourceInput`](crate::input::AssociateResourceInput)
    pub fn builder() -> crate::input::associate_resource_input::Builder {
        crate::input::associate_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AssociateResource {
    type Output = std::result::Result<
        crate::output::AssociateResourceOutput,
        crate::error::AssociateResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_resource_error(response)
        } else {
            crate::operation_deser::parse_associate_resource_response(response)
        }
    }
}

/// <p>Creates a new application that is the top-level node in a hierarchy of related cloud resource abstractions.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateApplication {
    _private: (),
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput)
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateApplication {
    type Output = std::result::Result<
        crate::output::CreateApplicationOutput,
        crate::error::CreateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_application_error(response)
        } else {
            crate::operation_deser::parse_create_application_response(response)
        }
    }
}

/// <p>Creates a new attribute group as a container for user-defined attributes. This feature
/// enables users to have full control over their cloud application's metadata in a rich
/// machine-readable format to facilitate integration with automated workflows and third-party
/// tools.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateAttributeGroup {
    _private: (),
}
impl CreateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`CreateAttributeGroupInput`](crate::input::CreateAttributeGroupInput)
    pub fn builder() -> crate::input::create_attribute_group_input::Builder {
        crate::input::create_attribute_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateAttributeGroup {
    type Output = std::result::Result<
        crate::output::CreateAttributeGroupOutput,
        crate::error::CreateAttributeGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_attribute_group_error(response)
        } else {
            crate::operation_deser::parse_create_attribute_group_response(response)
        }
    }
}

/// <p>Deletes an application that is specified either by its application ID or name. All associated attribute groups and resources must be disassociated from it before deleting an application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteApplication {
    _private: (),
}
impl DeleteApplication {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::input::DeleteApplicationInput)
    pub fn builder() -> crate::input::delete_application_input::Builder {
        crate::input::delete_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteApplication {
    type Output = std::result::Result<
        crate::output::DeleteApplicationOutput,
        crate::error::DeleteApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_application_error(response)
        } else {
            crate::operation_deser::parse_delete_application_response(response)
        }
    }
}

/// <p>Deletes an attribute group, specified either by its attribute group ID or name.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAttributeGroup {
    _private: (),
}
impl DeleteAttributeGroup {
    /// Creates a new builder-style object to manufacture [`DeleteAttributeGroupInput`](crate::input::DeleteAttributeGroupInput)
    pub fn builder() -> crate::input::delete_attribute_group_input::Builder {
        crate::input::delete_attribute_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteAttributeGroup {
    type Output = std::result::Result<
        crate::output::DeleteAttributeGroupOutput,
        crate::error::DeleteAttributeGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_attribute_group_error(response)
        } else {
            crate::operation_deser::parse_delete_attribute_group_response(response)
        }
    }
}

/// <p>Disassociates an attribute group from an application to remove the extra attributes contained in the attribute group from the application's metadata. This operation reverts <code>AssociateAttributeGroup</code>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateAttributeGroup {
    _private: (),
}
impl DisassociateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`DisassociateAttributeGroupInput`](crate::input::DisassociateAttributeGroupInput)
    pub fn builder() -> crate::input::disassociate_attribute_group_input::Builder {
        crate::input::disassociate_attribute_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisassociateAttributeGroup {
    type Output = std::result::Result<
        crate::output::DisassociateAttributeGroupOutput,
        crate::error::DisassociateAttributeGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_attribute_group_error(response)
        } else {
            crate::operation_deser::parse_disassociate_attribute_group_response(response)
        }
    }
}

/// <p>Disassociates a resource from application. Both the resource and the application can be specified either by ID or name.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateResource {
    _private: (),
}
impl DisassociateResource {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceInput`](crate::input::DisassociateResourceInput)
    pub fn builder() -> crate::input::disassociate_resource_input::Builder {
        crate::input::disassociate_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisassociateResource {
    type Output = std::result::Result<
        crate::output::DisassociateResourceOutput,
        crate::error::DisassociateResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_resource_error(response)
        } else {
            crate::operation_deser::parse_disassociate_resource_response(response)
        }
    }
}

/// <p>Retrieves metadata information about one of your applications. The application can be specified either by its unique ID or by its name (which is unique within one account in one region at a given point in time). Specify by ID in automated workflows if you want to make sure that the exact same application is returned or a <code>ResourceNotFoundException</code> is thrown, avoiding the ABA addressing problem.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetApplication {
    _private: (),
}
impl GetApplication {
    /// Creates a new builder-style object to manufacture [`GetApplicationInput`](crate::input::GetApplicationInput)
    pub fn builder() -> crate::input::get_application_input::Builder {
        crate::input::get_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetApplication {
    type Output =
        std::result::Result<crate::output::GetApplicationOutput, crate::error::GetApplicationError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_application_error(response)
        } else {
            crate::operation_deser::parse_get_application_response(response)
        }
    }
}

/// <p>Gets the resource associated with the application.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAssociatedResource {
    _private: (),
}
impl GetAssociatedResource {
    /// Creates a new builder-style object to manufacture [`GetAssociatedResourceInput`](crate::input::GetAssociatedResourceInput)
    pub fn builder() -> crate::input::get_associated_resource_input::Builder {
        crate::input::get_associated_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetAssociatedResource {
    type Output = std::result::Result<
        crate::output::GetAssociatedResourceOutput,
        crate::error::GetAssociatedResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_associated_resource_error(response)
        } else {
            crate::operation_deser::parse_get_associated_resource_response(response)
        }
    }
}

/// <p>Retrieves an attribute group, either by its name or its ID. The attribute group can be specified either by its unique ID or by its name.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAttributeGroup {
    _private: (),
}
impl GetAttributeGroup {
    /// Creates a new builder-style object to manufacture [`GetAttributeGroupInput`](crate::input::GetAttributeGroupInput)
    pub fn builder() -> crate::input::get_attribute_group_input::Builder {
        crate::input::get_attribute_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetAttributeGroup {
    type Output = std::result::Result<
        crate::output::GetAttributeGroupOutput,
        crate::error::GetAttributeGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_attribute_group_error(response)
        } else {
            crate::operation_deser::parse_get_attribute_group_response(response)
        }
    }
}

/// <p>Retrieves a list of all of your applications. Results are paginated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListApplications {
    _private: (),
}
impl ListApplications {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::input::ListApplicationsInput)
    pub fn builder() -> crate::input::list_applications_input::Builder {
        crate::input::list_applications_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListApplications {
    type Output = std::result::Result<
        crate::output::ListApplicationsOutput,
        crate::error::ListApplicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_applications_error(response)
        } else {
            crate::operation_deser::parse_list_applications_response(response)
        }
    }
}

/// <p>Lists all attribute groups that are associated with specified application.  Results are paginated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAssociatedAttributeGroups {
    _private: (),
}
impl ListAssociatedAttributeGroups {
    /// Creates a new builder-style object to manufacture [`ListAssociatedAttributeGroupsInput`](crate::input::ListAssociatedAttributeGroupsInput)
    pub fn builder() -> crate::input::list_associated_attribute_groups_input::Builder {
        crate::input::list_associated_attribute_groups_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListAssociatedAttributeGroups {
    type Output = std::result::Result<
        crate::output::ListAssociatedAttributeGroupsOutput,
        crate::error::ListAssociatedAttributeGroupsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_associated_attribute_groups_error(response)
        } else {
            crate::operation_deser::parse_list_associated_attribute_groups_response(response)
        }
    }
}

/// <p>Lists all resources that are associated with specified application. Results are paginated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAssociatedResources {
    _private: (),
}
impl ListAssociatedResources {
    /// Creates a new builder-style object to manufacture [`ListAssociatedResourcesInput`](crate::input::ListAssociatedResourcesInput)
    pub fn builder() -> crate::input::list_associated_resources_input::Builder {
        crate::input::list_associated_resources_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListAssociatedResources {
    type Output = std::result::Result<
        crate::output::ListAssociatedResourcesOutput,
        crate::error::ListAssociatedResourcesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_associated_resources_error(response)
        } else {
            crate::operation_deser::parse_list_associated_resources_response(response)
        }
    }
}

/// <p>Lists all attribute groups which you have access to. Results are paginated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAttributeGroups {
    _private: (),
}
impl ListAttributeGroups {
    /// Creates a new builder-style object to manufacture [`ListAttributeGroupsInput`](crate::input::ListAttributeGroupsInput)
    pub fn builder() -> crate::input::list_attribute_groups_input::Builder {
        crate::input::list_attribute_groups_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListAttributeGroups {
    type Output = std::result::Result<
        crate::output::ListAttributeGroupsOutput,
        crate::error::ListAttributeGroupsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_attribute_groups_error(response)
        } else {
            crate::operation_deser::parse_list_attribute_groups_response(response)
        }
    }
}

/// <p>Lists all of the tags on the resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// <p>Syncs the resource with current AppRegistry records.</p>
/// <p>Specifically, the resource’s AppRegistry system tags sync with its associated application. We remove the resource's AppRegistry system tags if it does not associate with the application. The caller must have permissions to read and update the resource.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SyncResource {
    _private: (),
}
impl SyncResource {
    /// Creates a new builder-style object to manufacture [`SyncResourceInput`](crate::input::SyncResourceInput)
    pub fn builder() -> crate::input::sync_resource_input::Builder {
        crate::input::sync_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for SyncResource {
    type Output =
        std::result::Result<crate::output::SyncResourceOutput, crate::error::SyncResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_sync_resource_error(response)
        } else {
            crate::operation_deser::parse_sync_resource_response(response)
        }
    }
}

/// <p>Assigns one or more tags (key-value pairs) to the specified resource.</p>
/// <p>Each tag consists of a key and an optional value. If a tag with the same key is already associated with the resource, this action updates its value.</p>
/// <p>This operation returns an empty response if the call was successful.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// <p>Removes tags from a resource.</p>
/// <p>This operation returns an empty response if the call was successful.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// <p>Updates an existing application with new attributes.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateApplication {
    _private: (),
}
impl UpdateApplication {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::input::UpdateApplicationInput)
    pub fn builder() -> crate::input::update_application_input::Builder {
        crate::input::update_application_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateApplication {
    type Output = std::result::Result<
        crate::output::UpdateApplicationOutput,
        crate::error::UpdateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_application_error(response)
        } else {
            crate::operation_deser::parse_update_application_response(response)
        }
    }
}

/// <p>Updates an existing attribute group with new details. </p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAttributeGroup {
    _private: (),
}
impl UpdateAttributeGroup {
    /// Creates a new builder-style object to manufacture [`UpdateAttributeGroupInput`](crate::input::UpdateAttributeGroupInput)
    pub fn builder() -> crate::input::update_attribute_group_input::Builder {
        crate::input::update_attribute_group_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateAttributeGroup {
    type Output = std::result::Result<
        crate::output::UpdateAttributeGroupOutput,
        crate::error::UpdateAttributeGroupError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_attribute_group_error(response)
        } else {
            crate::operation_deser::parse_update_attribute_group_response(response)
        }
    }
}
