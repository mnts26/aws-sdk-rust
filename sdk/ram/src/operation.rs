// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Accepts an invitation to a resource share from another Amazon Web Services account.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AcceptResourceShareInvitation {
    _private: (),
}
impl AcceptResourceShareInvitation {
    /// Creates a new builder-style object to manufacture [`AcceptResourceShareInvitationInput`](crate::input::AcceptResourceShareInvitationInput)
    pub fn builder() -> crate::input::accept_resource_share_invitation_input::Builder {
        crate::input::accept_resource_share_invitation_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AcceptResourceShareInvitation {
    type Output = std::result::Result<
        crate::output::AcceptResourceShareInvitationOutput,
        crate::error::AcceptResourceShareInvitationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_accept_resource_share_invitation_error(response)
        } else {
            crate::operation_deser::parse_accept_resource_share_invitation_response(response)
        }
    }
}

/// <p>Associates the specified resource share with the specified principals and resources.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateResourceShare {
    _private: (),
}
impl AssociateResourceShare {
    /// Creates a new builder-style object to manufacture [`AssociateResourceShareInput`](crate::input::AssociateResourceShareInput)
    pub fn builder() -> crate::input::associate_resource_share_input::Builder {
        crate::input::associate_resource_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AssociateResourceShare {
    type Output = std::result::Result<
        crate::output::AssociateResourceShareOutput,
        crate::error::AssociateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_resource_share_error(response)
        } else {
            crate::operation_deser::parse_associate_resource_share_response(response)
        }
    }
}

/// <p>Associates a permission with a resource share.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateResourceSharePermission {
    _private: (),
}
impl AssociateResourceSharePermission {
    /// Creates a new builder-style object to manufacture [`AssociateResourceSharePermissionInput`](crate::input::AssociateResourceSharePermissionInput)
    pub fn builder() -> crate::input::associate_resource_share_permission_input::Builder {
        crate::input::associate_resource_share_permission_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AssociateResourceSharePermission {
    type Output = std::result::Result<
        crate::output::AssociateResourceSharePermissionOutput,
        crate::error::AssociateResourceSharePermissionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_resource_share_permission_error(response)
        } else {
            crate::operation_deser::parse_associate_resource_share_permission_response(response)
        }
    }
}

/// <p>Creates a resource share. You must provide a list of the Amazon Resource Names (ARNs) for the
/// resources you want to share. You must also specify who you want to share the resources
/// with, and the permissions that you grant them.</p>
/// <note>
/// <p>Sharing a resource makes it available for use by principals outside of the
/// Amazon Web Services account that created the resource. Sharing doesn't change any permissions or
/// quotas that apply to the resource in the account that created it.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateResourceShare {
    _private: (),
}
impl CreateResourceShare {
    /// Creates a new builder-style object to manufacture [`CreateResourceShareInput`](crate::input::CreateResourceShareInput)
    pub fn builder() -> crate::input::create_resource_share_input::Builder {
        crate::input::create_resource_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateResourceShare {
    type Output = std::result::Result<
        crate::output::CreateResourceShareOutput,
        crate::error::CreateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_resource_share_error(response)
        } else {
            crate::operation_deser::parse_create_resource_share_response(response)
        }
    }
}

/// <p>Deletes the specified resource share.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteResourceShare {
    _private: (),
}
impl DeleteResourceShare {
    /// Creates a new builder-style object to manufacture [`DeleteResourceShareInput`](crate::input::DeleteResourceShareInput)
    pub fn builder() -> crate::input::delete_resource_share_input::Builder {
        crate::input::delete_resource_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteResourceShare {
    type Output = std::result::Result<
        crate::output::DeleteResourceShareOutput,
        crate::error::DeleteResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_resource_share_error(response)
        } else {
            crate::operation_deser::parse_delete_resource_share_response(response)
        }
    }
}

/// <p>Disassociates the specified principals or resources from the specified resource share.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateResourceShare {
    _private: (),
}
impl DisassociateResourceShare {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceShareInput`](crate::input::DisassociateResourceShareInput)
    pub fn builder() -> crate::input::disassociate_resource_share_input::Builder {
        crate::input::disassociate_resource_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisassociateResourceShare {
    type Output = std::result::Result<
        crate::output::DisassociateResourceShareOutput,
        crate::error::DisassociateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_resource_share_error(response)
        } else {
            crate::operation_deser::parse_disassociate_resource_share_response(response)
        }
    }
}

/// <p>Disassociates an RAM permission from a resource share.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateResourceSharePermission {
    _private: (),
}
impl DisassociateResourceSharePermission {
    /// Creates a new builder-style object to manufacture [`DisassociateResourceSharePermissionInput`](crate::input::DisassociateResourceSharePermissionInput)
    pub fn builder() -> crate::input::disassociate_resource_share_permission_input::Builder {
        crate::input::disassociate_resource_share_permission_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisassociateResourceSharePermission {
    type Output = std::result::Result<
        crate::output::DisassociateResourceSharePermissionOutput,
        crate::error::DisassociateResourceSharePermissionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_resource_share_permission_error(response)
        } else {
            crate::operation_deser::parse_disassociate_resource_share_permission_response(response)
        }
    }
}

/// <p>Enables resource sharing within your organization in Organizations.</p>
/// <p>The caller must be the master account for the organization.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct EnableSharingWithAwsOrganization {
    _private: (),
}
impl EnableSharingWithAwsOrganization {
    /// Creates a new builder-style object to manufacture [`EnableSharingWithAwsOrganizationInput`](crate::input::EnableSharingWithAwsOrganizationInput)
    pub fn builder() -> crate::input::enable_sharing_with_aws_organization_input::Builder {
        crate::input::enable_sharing_with_aws_organization_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for EnableSharingWithAwsOrganization {
    type Output = std::result::Result<
        crate::output::EnableSharingWithAwsOrganizationOutput,
        crate::error::EnableSharingWithAwsOrganizationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_enable_sharing_with_aws_organization_error(response)
        } else {
            crate::operation_deser::parse_enable_sharing_with_aws_organization_response(response)
        }
    }
}

/// <p>Gets the contents of an RAM permission in JSON format.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetPermission {
    _private: (),
}
impl GetPermission {
    /// Creates a new builder-style object to manufacture [`GetPermissionInput`](crate::input::GetPermissionInput)
    pub fn builder() -> crate::input::get_permission_input::Builder {
        crate::input::get_permission_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetPermission {
    type Output =
        std::result::Result<crate::output::GetPermissionOutput, crate::error::GetPermissionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_permission_error(response)
        } else {
            crate::operation_deser::parse_get_permission_response(response)
        }
    }
}

/// <p>Gets the policies for the specified resources that you own and have shared.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourcePolicies {
    _private: (),
}
impl GetResourcePolicies {
    /// Creates a new builder-style object to manufacture [`GetResourcePoliciesInput`](crate::input::GetResourcePoliciesInput)
    pub fn builder() -> crate::input::get_resource_policies_input::Builder {
        crate::input::get_resource_policies_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetResourcePolicies {
    type Output = std::result::Result<
        crate::output::GetResourcePoliciesOutput,
        crate::error::GetResourcePoliciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_policies_error(response)
        } else {
            crate::operation_deser::parse_get_resource_policies_response(response)
        }
    }
}

/// <p>Gets the resources or principals for the resource shares that you own.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceShareAssociations {
    _private: (),
}
impl GetResourceShareAssociations {
    /// Creates a new builder-style object to manufacture [`GetResourceShareAssociationsInput`](crate::input::GetResourceShareAssociationsInput)
    pub fn builder() -> crate::input::get_resource_share_associations_input::Builder {
        crate::input::get_resource_share_associations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetResourceShareAssociations {
    type Output = std::result::Result<
        crate::output::GetResourceShareAssociationsOutput,
        crate::error::GetResourceShareAssociationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_share_associations_error(response)
        } else {
            crate::operation_deser::parse_get_resource_share_associations_response(response)
        }
    }
}

/// <p>Gets the invitations that you have received for resource shares.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceShareInvitations {
    _private: (),
}
impl GetResourceShareInvitations {
    /// Creates a new builder-style object to manufacture [`GetResourceShareInvitationsInput`](crate::input::GetResourceShareInvitationsInput)
    pub fn builder() -> crate::input::get_resource_share_invitations_input::Builder {
        crate::input::get_resource_share_invitations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetResourceShareInvitations {
    type Output = std::result::Result<
        crate::output::GetResourceShareInvitationsOutput,
        crate::error::GetResourceShareInvitationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_share_invitations_error(response)
        } else {
            crate::operation_deser::parse_get_resource_share_invitations_response(response)
        }
    }
}

/// <p>Gets the resource shares that you own or the resource shares that are shared with you.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetResourceShares {
    _private: (),
}
impl GetResourceShares {
    /// Creates a new builder-style object to manufacture [`GetResourceSharesInput`](crate::input::GetResourceSharesInput)
    pub fn builder() -> crate::input::get_resource_shares_input::Builder {
        crate::input::get_resource_shares_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetResourceShares {
    type Output = std::result::Result<
        crate::output::GetResourceSharesOutput,
        crate::error::GetResourceSharesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_resource_shares_error(response)
        } else {
            crate::operation_deser::parse_get_resource_shares_response(response)
        }
    }
}

/// <p>Lists the resources in a resource share that is shared with you but that the invitation is still
/// pending for.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPendingInvitationResources {
    _private: (),
}
impl ListPendingInvitationResources {
    /// Creates a new builder-style object to manufacture [`ListPendingInvitationResourcesInput`](crate::input::ListPendingInvitationResourcesInput)
    pub fn builder() -> crate::input::list_pending_invitation_resources_input::Builder {
        crate::input::list_pending_invitation_resources_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPendingInvitationResources {
    type Output = std::result::Result<
        crate::output::ListPendingInvitationResourcesOutput,
        crate::error::ListPendingInvitationResourcesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_pending_invitation_resources_error(response)
        } else {
            crate::operation_deser::parse_list_pending_invitation_resources_response(response)
        }
    }
}

/// <p>Lists the RAM permissions.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPermissions {
    _private: (),
}
impl ListPermissions {
    /// Creates a new builder-style object to manufacture [`ListPermissionsInput`](crate::input::ListPermissionsInput)
    pub fn builder() -> crate::input::list_permissions_input::Builder {
        crate::input::list_permissions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPermissions {
    type Output = std::result::Result<
        crate::output::ListPermissionsOutput,
        crate::error::ListPermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_permissions_error(response)
        } else {
            crate::operation_deser::parse_list_permissions_response(response)
        }
    }
}

/// <p>Lists the principals that you have shared resources with or that have shared resources
/// with you.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPrincipals {
    _private: (),
}
impl ListPrincipals {
    /// Creates a new builder-style object to manufacture [`ListPrincipalsInput`](crate::input::ListPrincipalsInput)
    pub fn builder() -> crate::input::list_principals_input::Builder {
        crate::input::list_principals_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListPrincipals {
    type Output =
        std::result::Result<crate::output::ListPrincipalsOutput, crate::error::ListPrincipalsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_principals_error(response)
        } else {
            crate::operation_deser::parse_list_principals_response(response)
        }
    }
}

/// <p>Lists the resources that you added to a resource shares or the resources that are shared with
/// you.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResources {
    _private: (),
}
impl ListResources {
    /// Creates a new builder-style object to manufacture [`ListResourcesInput`](crate::input::ListResourcesInput)
    pub fn builder() -> crate::input::list_resources_input::Builder {
        crate::input::list_resources_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListResources {
    type Output =
        std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resources_error(response)
        } else {
            crate::operation_deser::parse_list_resources_response(response)
        }
    }
}

/// <p>Lists the RAM permissions that are associated with a resource share.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResourceSharePermissions {
    _private: (),
}
impl ListResourceSharePermissions {
    /// Creates a new builder-style object to manufacture [`ListResourceSharePermissionsInput`](crate::input::ListResourceSharePermissionsInput)
    pub fn builder() -> crate::input::list_resource_share_permissions_input::Builder {
        crate::input::list_resource_share_permissions_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListResourceSharePermissions {
    type Output = std::result::Result<
        crate::output::ListResourceSharePermissionsOutput,
        crate::error::ListResourceSharePermissionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resource_share_permissions_error(response)
        } else {
            crate::operation_deser::parse_list_resource_share_permissions_response(response)
        }
    }
}

/// <p>Lists the shareable resource types supported by RAM.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListResourceTypes {
    _private: (),
}
impl ListResourceTypes {
    /// Creates a new builder-style object to manufacture [`ListResourceTypesInput`](crate::input::ListResourceTypesInput)
    pub fn builder() -> crate::input::list_resource_types_input::Builder {
        crate::input::list_resource_types_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListResourceTypes {
    type Output = std::result::Result<
        crate::output::ListResourceTypesOutput,
        crate::error::ListResourceTypesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_resource_types_error(response)
        } else {
            crate::operation_deser::parse_list_resource_types_response(response)
        }
    }
}

/// <p>Resource shares that were created by attaching a policy to a resource are visible only
/// to the resource share owner, and the resource share cannot be modified in RAM.</p>
/// <p>Use this API action to promote the resource share. When you promote the resource
/// share, it becomes:</p>
/// <ul>
/// <li>
/// <p>Visible to all principals that it is shared with.</p>
/// </li>
/// <li>
/// <p>Modifiable in RAM.</p>
/// </li>
/// </ul>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PromoteResourceShareCreatedFromPolicy {
    _private: (),
}
impl PromoteResourceShareCreatedFromPolicy {
    /// Creates a new builder-style object to manufacture [`PromoteResourceShareCreatedFromPolicyInput`](crate::input::PromoteResourceShareCreatedFromPolicyInput)
    pub fn builder() -> crate::input::promote_resource_share_created_from_policy_input::Builder {
        crate::input::promote_resource_share_created_from_policy_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for PromoteResourceShareCreatedFromPolicy {
    type Output = std::result::Result<
        crate::output::PromoteResourceShareCreatedFromPolicyOutput,
        crate::error::PromoteResourceShareCreatedFromPolicyError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_promote_resource_share_created_from_policy_error(response)
        } else {
            crate::operation_deser::parse_promote_resource_share_created_from_policy_response(
                response,
            )
        }
    }
}

/// <p>Rejects an invitation to a resource share from another Amazon Web Services account.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RejectResourceShareInvitation {
    _private: (),
}
impl RejectResourceShareInvitation {
    /// Creates a new builder-style object to manufacture [`RejectResourceShareInvitationInput`](crate::input::RejectResourceShareInvitationInput)
    pub fn builder() -> crate::input::reject_resource_share_invitation_input::Builder {
        crate::input::reject_resource_share_invitation_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for RejectResourceShareInvitation {
    type Output = std::result::Result<
        crate::output::RejectResourceShareInvitationOutput,
        crate::error::RejectResourceShareInvitationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_reject_resource_share_invitation_error(response)
        } else {
            crate::operation_deser::parse_reject_resource_share_invitation_response(response)
        }
    }
}

/// <p>Adds the specified tags to the specified resource share that you own.</p>
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

/// <p>Removes the specified tags from the specified resource share that you own.</p>
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

/// <p>Updates the specified resource share that you own.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateResourceShare {
    _private: (),
}
impl UpdateResourceShare {
    /// Creates a new builder-style object to manufacture [`UpdateResourceShareInput`](crate::input::UpdateResourceShareInput)
    pub fn builder() -> crate::input::update_resource_share_input::Builder {
        crate::input::update_resource_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateResourceShare {
    type Output = std::result::Result<
        crate::output::UpdateResourceShareOutput,
        crate::error::UpdateResourceShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_resource_share_error(response)
        } else {
            crate::operation_deser::parse_update_resource_share_response(response)
        }
    }
}
