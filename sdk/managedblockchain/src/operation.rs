// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Creates a member within a Managed Blockchain network.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateMember {
    _private: (),
}
impl CreateMember {
    /// Creates a new builder-style object to manufacture [`CreateMemberInput`](crate::input::CreateMemberInput)
    pub fn builder() -> crate::input::create_member_input::Builder {
        crate::input::create_member_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateMember {
    type Output =
        std::result::Result<crate::output::CreateMemberOutput, crate::error::CreateMemberError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_member_error(response)
        } else {
            crate::operation_deser::parse_create_member_response(response)
        }
    }
}

/// <p>Creates a new blockchain network using Amazon Managed Blockchain.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateNetwork {
    _private: (),
}
impl CreateNetwork {
    /// Creates a new builder-style object to manufacture [`CreateNetworkInput`](crate::input::CreateNetworkInput)
    pub fn builder() -> crate::input::create_network_input::Builder {
        crate::input::create_network_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateNetwork {
    type Output =
        std::result::Result<crate::output::CreateNetworkOutput, crate::error::CreateNetworkError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_network_error(response)
        } else {
            crate::operation_deser::parse_create_network_response(response)
        }
    }
}

/// <p>Creates a node on the specified blockchain network.</p>
/// <p>Applies to Hyperledger Fabric and Ethereum.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateNode {
    _private: (),
}
impl CreateNode {
    /// Creates a new builder-style object to manufacture [`CreateNodeInput`](crate::input::CreateNodeInput)
    pub fn builder() -> crate::input::create_node_input::Builder {
        crate::input::create_node_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateNode {
    type Output =
        std::result::Result<crate::output::CreateNodeOutput, crate::error::CreateNodeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_node_error(response)
        } else {
            crate::operation_deser::parse_create_node_response(response)
        }
    }
}

/// <p>Creates a proposal for a change to the network that other members of the network can vote on, for example, a proposal to add a new member to the network. Any member can create a proposal.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateProposal {
    _private: (),
}
impl CreateProposal {
    /// Creates a new builder-style object to manufacture [`CreateProposalInput`](crate::input::CreateProposalInput)
    pub fn builder() -> crate::input::create_proposal_input::Builder {
        crate::input::create_proposal_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateProposal {
    type Output =
        std::result::Result<crate::output::CreateProposalOutput, crate::error::CreateProposalError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_proposal_error(response)
        } else {
            crate::operation_deser::parse_create_proposal_response(response)
        }
    }
}

/// <p>Deletes a member. Deleting a member removes the member and all associated resources from the network. <code>DeleteMember</code> can only be called for a specified <code>MemberId</code> if the principal performing the action is associated with the AWS account that owns the member. In all other cases, the <code>DeleteMember</code> action is carried out as the result of an approved proposal to remove a member. If <code>MemberId</code> is the last member in a network specified by the last AWS account, the network is deleted also.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteMember {
    _private: (),
}
impl DeleteMember {
    /// Creates a new builder-style object to manufacture [`DeleteMemberInput`](crate::input::DeleteMemberInput)
    pub fn builder() -> crate::input::delete_member_input::Builder {
        crate::input::delete_member_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteMember {
    type Output =
        std::result::Result<crate::output::DeleteMemberOutput, crate::error::DeleteMemberError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_member_error(response)
        } else {
            crate::operation_deser::parse_delete_member_response(response)
        }
    }
}

/// <p>Deletes a node that your AWS account owns. All data on the node is lost and cannot be recovered.</p>
/// <p>Applies to Hyperledger Fabric and Ethereum.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteNode {
    _private: (),
}
impl DeleteNode {
    /// Creates a new builder-style object to manufacture [`DeleteNodeInput`](crate::input::DeleteNodeInput)
    pub fn builder() -> crate::input::delete_node_input::Builder {
        crate::input::delete_node_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteNode {
    type Output =
        std::result::Result<crate::output::DeleteNodeOutput, crate::error::DeleteNodeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_node_error(response)
        } else {
            crate::operation_deser::parse_delete_node_response(response)
        }
    }
}

/// <p>Returns detailed information about a member.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMember {
    _private: (),
}
impl GetMember {
    /// Creates a new builder-style object to manufacture [`GetMemberInput`](crate::input::GetMemberInput)
    pub fn builder() -> crate::input::get_member_input::Builder {
        crate::input::get_member_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetMember {
    type Output = std::result::Result<crate::output::GetMemberOutput, crate::error::GetMemberError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_member_error(response)
        } else {
            crate::operation_deser::parse_get_member_response(response)
        }
    }
}

/// <p>Returns detailed information about a network.</p>
/// <p>Applies to Hyperledger Fabric and Ethereum.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetNetwork {
    _private: (),
}
impl GetNetwork {
    /// Creates a new builder-style object to manufacture [`GetNetworkInput`](crate::input::GetNetworkInput)
    pub fn builder() -> crate::input::get_network_input::Builder {
        crate::input::get_network_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetNetwork {
    type Output =
        std::result::Result<crate::output::GetNetworkOutput, crate::error::GetNetworkError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_network_error(response)
        } else {
            crate::operation_deser::parse_get_network_response(response)
        }
    }
}

/// <p>Returns detailed information about a node.</p>
/// <p>Applies to Hyperledger Fabric and Ethereum.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetNode {
    _private: (),
}
impl GetNode {
    /// Creates a new builder-style object to manufacture [`GetNodeInput`](crate::input::GetNodeInput)
    pub fn builder() -> crate::input::get_node_input::Builder {
        crate::input::get_node_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetNode {
    type Output = std::result::Result<crate::output::GetNodeOutput, crate::error::GetNodeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_node_error(response)
        } else {
            crate::operation_deser::parse_get_node_response(response)
        }
    }
}

/// <p>Returns detailed information about a proposal.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetProposal {
    _private: (),
}
impl GetProposal {
    /// Creates a new builder-style object to manufacture [`GetProposalInput`](crate::input::GetProposalInput)
    pub fn builder() -> crate::input::get_proposal_input::Builder {
        crate::input::get_proposal_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetProposal {
    type Output =
        std::result::Result<crate::output::GetProposalOutput, crate::error::GetProposalError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_proposal_error(response)
        } else {
            crate::operation_deser::parse_get_proposal_response(response)
        }
    }
}

/// <p>Returns a list of all invitations for the current AWS account.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListInvitations {
    _private: (),
}
impl ListInvitations {
    /// Creates a new builder-style object to manufacture [`ListInvitationsInput`](crate::input::ListInvitationsInput)
    pub fn builder() -> crate::input::list_invitations_input::Builder {
        crate::input::list_invitations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListInvitations {
    type Output = std::result::Result<
        crate::output::ListInvitationsOutput,
        crate::error::ListInvitationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_invitations_error(response)
        } else {
            crate::operation_deser::parse_list_invitations_response(response)
        }
    }
}

/// <p>Returns a list of the members in a network and properties of their configurations.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListMembers {
    _private: (),
}
impl ListMembers {
    /// Creates a new builder-style object to manufacture [`ListMembersInput`](crate::input::ListMembersInput)
    pub fn builder() -> crate::input::list_members_input::Builder {
        crate::input::list_members_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListMembers {
    type Output =
        std::result::Result<crate::output::ListMembersOutput, crate::error::ListMembersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_members_error(response)
        } else {
            crate::operation_deser::parse_list_members_response(response)
        }
    }
}

/// <p>Returns information about the networks in which the current AWS account participates.</p>
/// <p>Applies to Hyperledger Fabric and Ethereum.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListNetworks {
    _private: (),
}
impl ListNetworks {
    /// Creates a new builder-style object to manufacture [`ListNetworksInput`](crate::input::ListNetworksInput)
    pub fn builder() -> crate::input::list_networks_input::Builder {
        crate::input::list_networks_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListNetworks {
    type Output =
        std::result::Result<crate::output::ListNetworksOutput, crate::error::ListNetworksError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_networks_error(response)
        } else {
            crate::operation_deser::parse_list_networks_response(response)
        }
    }
}

/// <p>Returns information about the nodes within a network.</p>
/// <p>Applies to Hyperledger Fabric and Ethereum.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListNodes {
    _private: (),
}
impl ListNodes {
    /// Creates a new builder-style object to manufacture [`ListNodesInput`](crate::input::ListNodesInput)
    pub fn builder() -> crate::input::list_nodes_input::Builder {
        crate::input::list_nodes_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListNodes {
    type Output = std::result::Result<crate::output::ListNodesOutput, crate::error::ListNodesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_nodes_error(response)
        } else {
            crate::operation_deser::parse_list_nodes_response(response)
        }
    }
}

/// <p>Returns a list of proposals for the network.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListProposals {
    _private: (),
}
impl ListProposals {
    /// Creates a new builder-style object to manufacture [`ListProposalsInput`](crate::input::ListProposalsInput)
    pub fn builder() -> crate::input::list_proposals_input::Builder {
        crate::input::list_proposals_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListProposals {
    type Output =
        std::result::Result<crate::output::ListProposalsOutput, crate::error::ListProposalsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_proposals_error(response)
        } else {
            crate::operation_deser::parse_list_proposals_response(response)
        }
    }
}

/// <p>Returns the list of votes for a specified proposal, including the value of each vote and the unique identifier of the member that cast the vote.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListProposalVotes {
    _private: (),
}
impl ListProposalVotes {
    /// Creates a new builder-style object to manufacture [`ListProposalVotesInput`](crate::input::ListProposalVotesInput)
    pub fn builder() -> crate::input::list_proposal_votes_input::Builder {
        crate::input::list_proposal_votes_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListProposalVotes {
    type Output = std::result::Result<
        crate::output::ListProposalVotesOutput,
        crate::error::ListProposalVotesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_proposal_votes_error(response)
        } else {
            crate::operation_deser::parse_list_proposal_votes_response(response)
        }
    }
}

/// <p>Returns a list of tags for the specified resource. Each tag consists of a key and optional value.</p>
/// <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p>
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

/// <p>Rejects an invitation to join a network. This action can be called by a principal in an AWS account that has received an invitation to create a member and join a network.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RejectInvitation {
    _private: (),
}
impl RejectInvitation {
    /// Creates a new builder-style object to manufacture [`RejectInvitationInput`](crate::input::RejectInvitationInput)
    pub fn builder() -> crate::input::reject_invitation_input::Builder {
        crate::input::reject_invitation_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for RejectInvitation {
    type Output = std::result::Result<
        crate::output::RejectInvitationOutput,
        crate::error::RejectInvitationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_reject_invitation_error(response)
        } else {
            crate::operation_deser::parse_reject_invitation_response(response)
        }
    }
}

/// <p>Adds or overwrites the specified tags for the specified Amazon Managed Blockchain resource. Each tag consists of a key and optional value.</p>
/// <p>When you specify a tag key that already exists, the tag value is overwritten with the new value. Use <code>UntagResource</code> to remove tag keys.</p>
/// <p>A resource can have up to 50 tags. If you try to create more than 50 tags for a resource, your request fails and returns an error.</p>
/// <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p>
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

/// <p>Removes the specified tags from the Amazon Managed Blockchain resource.</p>
/// <p>For more information about tags, see <a href="https://docs.aws.amazon.com/managed-blockchain/latest/ethereum-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Ethereum Developer Guide</i>, or <a href="https://docs.aws.amazon.com/managed-blockchain/latest/hyperledger-fabric-dev/tagging-resources.html">Tagging Resources</a> in the <i>Amazon Managed Blockchain Hyperledger Fabric Developer Guide</i>.</p>
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

/// <p>Updates a member configuration with new parameters.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateMember {
    _private: (),
}
impl UpdateMember {
    /// Creates a new builder-style object to manufacture [`UpdateMemberInput`](crate::input::UpdateMemberInput)
    pub fn builder() -> crate::input::update_member_input::Builder {
        crate::input::update_member_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateMember {
    type Output =
        std::result::Result<crate::output::UpdateMemberOutput, crate::error::UpdateMemberError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_member_error(response)
        } else {
            crate::operation_deser::parse_update_member_response(response)
        }
    }
}

/// <p>Updates a node configuration with new parameters.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateNode {
    _private: (),
}
impl UpdateNode {
    /// Creates a new builder-style object to manufacture [`UpdateNodeInput`](crate::input::UpdateNodeInput)
    pub fn builder() -> crate::input::update_node_input::Builder {
        crate::input::update_node_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateNode {
    type Output =
        std::result::Result<crate::output::UpdateNodeOutput, crate::error::UpdateNodeError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_node_error(response)
        } else {
            crate::operation_deser::parse_update_node_response(response)
        }
    }
}

/// <p>Casts a vote for a specified <code>ProposalId</code> on behalf of a member. The member to vote as, specified by <code>VoterMemberId</code>, must be in the same AWS account as the principal that calls the action.</p>
/// <p>Applies only to Hyperledger Fabric.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct VoteOnProposal {
    _private: (),
}
impl VoteOnProposal {
    /// Creates a new builder-style object to manufacture [`VoteOnProposalInput`](crate::input::VoteOnProposalInput)
    pub fn builder() -> crate::input::vote_on_proposal_input::Builder {
        crate::input::vote_on_proposal_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for VoteOnProposal {
    type Output =
        std::result::Result<crate::output::VoteOnProposalOutput, crate::error::VoteOnProposalError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_vote_on_proposal_error(response)
        } else {
            crate::operation_deser::parse_vote_on_proposal_response(response)
        }
    }
}
