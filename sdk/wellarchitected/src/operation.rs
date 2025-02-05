// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Associate a lens to a workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AssociateLenses {
    _private: (),
}
impl AssociateLenses {
    /// Creates a new builder-style object to manufacture [`AssociateLensesInput`](crate::input::AssociateLensesInput)
    pub fn builder() -> crate::input::associate_lenses_input::Builder {
        crate::input::associate_lenses_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for AssociateLenses {
    type Output = std::result::Result<
        crate::output::AssociateLensesOutput,
        crate::error::AssociateLensesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_associate_lenses_error(response)
        } else {
            crate::operation_deser::parse_associate_lenses_response(response)
        }
    }
}

/// <p>Create a milestone for an existing workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateMilestone {
    _private: (),
}
impl CreateMilestone {
    /// Creates a new builder-style object to manufacture [`CreateMilestoneInput`](crate::input::CreateMilestoneInput)
    pub fn builder() -> crate::input::create_milestone_input::Builder {
        crate::input::create_milestone_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateMilestone {
    type Output = std::result::Result<
        crate::output::CreateMilestoneOutput,
        crate::error::CreateMilestoneError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_milestone_error(response)
        } else {
            crate::operation_deser::parse_create_milestone_response(response)
        }
    }
}

/// <p>Create a new workload.</p>
/// <p>The owner of a workload can share the workload with other AWS accounts and IAM users
/// in the same AWS Region. Only the owner of a workload can delete it.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/wellarchitected/latest/userguide/define-workload.html">Defining a Workload</a> in the
/// <i>AWS Well-Architected Tool User Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateWorkload {
    _private: (),
}
impl CreateWorkload {
    /// Creates a new builder-style object to manufacture [`CreateWorkloadInput`](crate::input::CreateWorkloadInput)
    pub fn builder() -> crate::input::create_workload_input::Builder {
        crate::input::create_workload_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateWorkload {
    type Output =
        std::result::Result<crate::output::CreateWorkloadOutput, crate::error::CreateWorkloadError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_workload_error(response)
        } else {
            crate::operation_deser::parse_create_workload_response(response)
        }
    }
}

/// <p>Create a workload share.</p>
/// <p>The owner of a workload can share it with other AWS accounts and IAM users in the same
/// AWS Region. Shared access to a workload is not removed until the workload invitation is
/// deleted.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/wellarchitected/latest/userguide/workloads-sharing.html">Sharing a Workload</a> in the
/// <i>AWS Well-Architected Tool User Guide</i>.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateWorkloadShare {
    _private: (),
}
impl CreateWorkloadShare {
    /// Creates a new builder-style object to manufacture [`CreateWorkloadShareInput`](crate::input::CreateWorkloadShareInput)
    pub fn builder() -> crate::input::create_workload_share_input::Builder {
        crate::input::create_workload_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for CreateWorkloadShare {
    type Output = std::result::Result<
        crate::output::CreateWorkloadShareOutput,
        crate::error::CreateWorkloadShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_workload_share_error(response)
        } else {
            crate::operation_deser::parse_create_workload_share_response(response)
        }
    }
}

/// <p>Delete an existing workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteWorkload {
    _private: (),
}
impl DeleteWorkload {
    /// Creates a new builder-style object to manufacture [`DeleteWorkloadInput`](crate::input::DeleteWorkloadInput)
    pub fn builder() -> crate::input::delete_workload_input::Builder {
        crate::input::delete_workload_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteWorkload {
    type Output =
        std::result::Result<crate::output::DeleteWorkloadOutput, crate::error::DeleteWorkloadError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_workload_error(response)
        } else {
            crate::operation_deser::parse_delete_workload_response(response)
        }
    }
}

/// <p>Delete a workload share.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteWorkloadShare {
    _private: (),
}
impl DeleteWorkloadShare {
    /// Creates a new builder-style object to manufacture [`DeleteWorkloadShareInput`](crate::input::DeleteWorkloadShareInput)
    pub fn builder() -> crate::input::delete_workload_share_input::Builder {
        crate::input::delete_workload_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DeleteWorkloadShare {
    type Output = std::result::Result<
        crate::output::DeleteWorkloadShareOutput,
        crate::error::DeleteWorkloadShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_workload_share_error(response)
        } else {
            crate::operation_deser::parse_delete_workload_share_response(response)
        }
    }
}

/// <p>Disassociate a lens from a workload.</p>
/// <note>
/// <p>The AWS Well-Architected Framework lens (<code>wellarchitected</code>) cannot be
/// removed from a workload.</p>
/// </note>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DisassociateLenses {
    _private: (),
}
impl DisassociateLenses {
    /// Creates a new builder-style object to manufacture [`DisassociateLensesInput`](crate::input::DisassociateLensesInput)
    pub fn builder() -> crate::input::disassociate_lenses_input::Builder {
        crate::input::disassociate_lenses_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for DisassociateLenses {
    type Output = std::result::Result<
        crate::output::DisassociateLensesOutput,
        crate::error::DisassociateLensesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_disassociate_lenses_error(response)
        } else {
            crate::operation_deser::parse_disassociate_lenses_response(response)
        }
    }
}

/// <p>Get the answer to a specific question in a workload review.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAnswer {
    _private: (),
}
impl GetAnswer {
    /// Creates a new builder-style object to manufacture [`GetAnswerInput`](crate::input::GetAnswerInput)
    pub fn builder() -> crate::input::get_answer_input::Builder {
        crate::input::get_answer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetAnswer {
    type Output = std::result::Result<crate::output::GetAnswerOutput, crate::error::GetAnswerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_answer_error(response)
        } else {
            crate::operation_deser::parse_get_answer_response(response)
        }
    }
}

/// <p>Get lens review.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetLensReview {
    _private: (),
}
impl GetLensReview {
    /// Creates a new builder-style object to manufacture [`GetLensReviewInput`](crate::input::GetLensReviewInput)
    pub fn builder() -> crate::input::get_lens_review_input::Builder {
        crate::input::get_lens_review_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetLensReview {
    type Output =
        std::result::Result<crate::output::GetLensReviewOutput, crate::error::GetLensReviewError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_lens_review_error(response)
        } else {
            crate::operation_deser::parse_get_lens_review_response(response)
        }
    }
}

/// <p>Get lens review report.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetLensReviewReport {
    _private: (),
}
impl GetLensReviewReport {
    /// Creates a new builder-style object to manufacture [`GetLensReviewReportInput`](crate::input::GetLensReviewReportInput)
    pub fn builder() -> crate::input::get_lens_review_report_input::Builder {
        crate::input::get_lens_review_report_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetLensReviewReport {
    type Output = std::result::Result<
        crate::output::GetLensReviewReportOutput,
        crate::error::GetLensReviewReportError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_lens_review_report_error(response)
        } else {
            crate::operation_deser::parse_get_lens_review_report_response(response)
        }
    }
}

/// <p>Get lens version differences.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetLensVersionDifference {
    _private: (),
}
impl GetLensVersionDifference {
    /// Creates a new builder-style object to manufacture [`GetLensVersionDifferenceInput`](crate::input::GetLensVersionDifferenceInput)
    pub fn builder() -> crate::input::get_lens_version_difference_input::Builder {
        crate::input::get_lens_version_difference_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetLensVersionDifference {
    type Output = std::result::Result<
        crate::output::GetLensVersionDifferenceOutput,
        crate::error::GetLensVersionDifferenceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_lens_version_difference_error(response)
        } else {
            crate::operation_deser::parse_get_lens_version_difference_response(response)
        }
    }
}

/// <p>Get a milestone for an existing workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetMilestone {
    _private: (),
}
impl GetMilestone {
    /// Creates a new builder-style object to manufacture [`GetMilestoneInput`](crate::input::GetMilestoneInput)
    pub fn builder() -> crate::input::get_milestone_input::Builder {
        crate::input::get_milestone_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetMilestone {
    type Output =
        std::result::Result<crate::output::GetMilestoneOutput, crate::error::GetMilestoneError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_milestone_error(response)
        } else {
            crate::operation_deser::parse_get_milestone_response(response)
        }
    }
}

/// <p>Get an existing workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetWorkload {
    _private: (),
}
impl GetWorkload {
    /// Creates a new builder-style object to manufacture [`GetWorkloadInput`](crate::input::GetWorkloadInput)
    pub fn builder() -> crate::input::get_workload_input::Builder {
        crate::input::get_workload_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for GetWorkload {
    type Output =
        std::result::Result<crate::output::GetWorkloadOutput, crate::error::GetWorkloadError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_workload_error(response)
        } else {
            crate::operation_deser::parse_get_workload_response(response)
        }
    }
}

/// <p>List of answers.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListAnswers {
    _private: (),
}
impl ListAnswers {
    /// Creates a new builder-style object to manufacture [`ListAnswersInput`](crate::input::ListAnswersInput)
    pub fn builder() -> crate::input::list_answers_input::Builder {
        crate::input::list_answers_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListAnswers {
    type Output =
        std::result::Result<crate::output::ListAnswersOutput, crate::error::ListAnswersError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_answers_error(response)
        } else {
            crate::operation_deser::parse_list_answers_response(response)
        }
    }
}

/// <p>List the available lenses.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLenses {
    _private: (),
}
impl ListLenses {
    /// Creates a new builder-style object to manufacture [`ListLensesInput`](crate::input::ListLensesInput)
    pub fn builder() -> crate::input::list_lenses_input::Builder {
        crate::input::list_lenses_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListLenses {
    type Output =
        std::result::Result<crate::output::ListLensesOutput, crate::error::ListLensesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_lenses_error(response)
        } else {
            crate::operation_deser::parse_list_lenses_response(response)
        }
    }
}

/// <p>List lens review improvements.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLensReviewImprovements {
    _private: (),
}
impl ListLensReviewImprovements {
    /// Creates a new builder-style object to manufacture [`ListLensReviewImprovementsInput`](crate::input::ListLensReviewImprovementsInput)
    pub fn builder() -> crate::input::list_lens_review_improvements_input::Builder {
        crate::input::list_lens_review_improvements_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListLensReviewImprovements {
    type Output = std::result::Result<
        crate::output::ListLensReviewImprovementsOutput,
        crate::error::ListLensReviewImprovementsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_lens_review_improvements_error(response)
        } else {
            crate::operation_deser::parse_list_lens_review_improvements_response(response)
        }
    }
}

/// <p>List lens reviews.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListLensReviews {
    _private: (),
}
impl ListLensReviews {
    /// Creates a new builder-style object to manufacture [`ListLensReviewsInput`](crate::input::ListLensReviewsInput)
    pub fn builder() -> crate::input::list_lens_reviews_input::Builder {
        crate::input::list_lens_reviews_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListLensReviews {
    type Output = std::result::Result<
        crate::output::ListLensReviewsOutput,
        crate::error::ListLensReviewsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_lens_reviews_error(response)
        } else {
            crate::operation_deser::parse_list_lens_reviews_response(response)
        }
    }
}

/// <p>List all milestones for an existing workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListMilestones {
    _private: (),
}
impl ListMilestones {
    /// Creates a new builder-style object to manufacture [`ListMilestonesInput`](crate::input::ListMilestonesInput)
    pub fn builder() -> crate::input::list_milestones_input::Builder {
        crate::input::list_milestones_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListMilestones {
    type Output =
        std::result::Result<crate::output::ListMilestonesOutput, crate::error::ListMilestonesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_milestones_error(response)
        } else {
            crate::operation_deser::parse_list_milestones_response(response)
        }
    }
}

/// <p>List lens notifications.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListNotifications {
    _private: (),
}
impl ListNotifications {
    /// Creates a new builder-style object to manufacture [`ListNotificationsInput`](crate::input::ListNotificationsInput)
    pub fn builder() -> crate::input::list_notifications_input::Builder {
        crate::input::list_notifications_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListNotifications {
    type Output = std::result::Result<
        crate::output::ListNotificationsOutput,
        crate::error::ListNotificationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_notifications_error(response)
        } else {
            crate::operation_deser::parse_list_notifications_response(response)
        }
    }
}

/// <p>List  the workload invitations.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListShareInvitations {
    _private: (),
}
impl ListShareInvitations {
    /// Creates a new builder-style object to manufacture [`ListShareInvitationsInput`](crate::input::ListShareInvitationsInput)
    pub fn builder() -> crate::input::list_share_invitations_input::Builder {
        crate::input::list_share_invitations_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListShareInvitations {
    type Output = std::result::Result<
        crate::output::ListShareInvitationsOutput,
        crate::error::ListShareInvitationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_share_invitations_error(response)
        } else {
            crate::operation_deser::parse_list_share_invitations_response(response)
        }
    }
}

/// <p>List the tags for a resource.</p>
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

/// <p>List workloads. Paginated.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListWorkloads {
    _private: (),
}
impl ListWorkloads {
    /// Creates a new builder-style object to manufacture [`ListWorkloadsInput`](crate::input::ListWorkloadsInput)
    pub fn builder() -> crate::input::list_workloads_input::Builder {
        crate::input::list_workloads_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListWorkloads {
    type Output =
        std::result::Result<crate::output::ListWorkloadsOutput, crate::error::ListWorkloadsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_workloads_error(response)
        } else {
            crate::operation_deser::parse_list_workloads_response(response)
        }
    }
}

/// <p>List the workload shares associated with the workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListWorkloadShares {
    _private: (),
}
impl ListWorkloadShares {
    /// Creates a new builder-style object to manufacture [`ListWorkloadSharesInput`](crate::input::ListWorkloadSharesInput)
    pub fn builder() -> crate::input::list_workload_shares_input::Builder {
        crate::input::list_workload_shares_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for ListWorkloadShares {
    type Output = std::result::Result<
        crate::output::ListWorkloadSharesOutput,
        crate::error::ListWorkloadSharesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_workload_shares_error(response)
        } else {
            crate::operation_deser::parse_list_workload_shares_response(response)
        }
    }
}

/// <p>Adds one or more tags to the specified resource.</p>
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

/// <p>Deletes specified tags from a resource.</p>
/// <p>To specify multiple tags, use separate <b>tagKeys</b> parameters, for example:</p>
/// <p>
/// <code>DELETE /tags/WorkloadArn?tagKeys=key1&tagKeys=key2</code>
/// </p>
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

/// <p>Update the answer to a specific question in a workload review.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAnswer {
    _private: (),
}
impl UpdateAnswer {
    /// Creates a new builder-style object to manufacture [`UpdateAnswerInput`](crate::input::UpdateAnswerInput)
    pub fn builder() -> crate::input::update_answer_input::Builder {
        crate::input::update_answer_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateAnswer {
    type Output =
        std::result::Result<crate::output::UpdateAnswerOutput, crate::error::UpdateAnswerError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_answer_error(response)
        } else {
            crate::operation_deser::parse_update_answer_response(response)
        }
    }
}

/// <p>Update lens review.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateLensReview {
    _private: (),
}
impl UpdateLensReview {
    /// Creates a new builder-style object to manufacture [`UpdateLensReviewInput`](crate::input::UpdateLensReviewInput)
    pub fn builder() -> crate::input::update_lens_review_input::Builder {
        crate::input::update_lens_review_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateLensReview {
    type Output = std::result::Result<
        crate::output::UpdateLensReviewOutput,
        crate::error::UpdateLensReviewError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_lens_review_error(response)
        } else {
            crate::operation_deser::parse_update_lens_review_response(response)
        }
    }
}

/// <p>Update a workload invitation.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateShareInvitation {
    _private: (),
}
impl UpdateShareInvitation {
    /// Creates a new builder-style object to manufacture [`UpdateShareInvitationInput`](crate::input::UpdateShareInvitationInput)
    pub fn builder() -> crate::input::update_share_invitation_input::Builder {
        crate::input::update_share_invitation_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateShareInvitation {
    type Output = std::result::Result<
        crate::output::UpdateShareInvitationOutput,
        crate::error::UpdateShareInvitationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_share_invitation_error(response)
        } else {
            crate::operation_deser::parse_update_share_invitation_response(response)
        }
    }
}

/// <p>Update an existing workload.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateWorkload {
    _private: (),
}
impl UpdateWorkload {
    /// Creates a new builder-style object to manufacture [`UpdateWorkloadInput`](crate::input::UpdateWorkloadInput)
    pub fn builder() -> crate::input::update_workload_input::Builder {
        crate::input::update_workload_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateWorkload {
    type Output =
        std::result::Result<crate::output::UpdateWorkloadOutput, crate::error::UpdateWorkloadError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_workload_error(response)
        } else {
            crate::operation_deser::parse_update_workload_response(response)
        }
    }
}

/// <p>Update a workload share.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateWorkloadShare {
    _private: (),
}
impl UpdateWorkloadShare {
    /// Creates a new builder-style object to manufacture [`UpdateWorkloadShareInput`](crate::input::UpdateWorkloadShareInput)
    pub fn builder() -> crate::input::update_workload_share_input::Builder {
        crate::input::update_workload_share_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpdateWorkloadShare {
    type Output = std::result::Result<
        crate::output::UpdateWorkloadShareOutput,
        crate::error::UpdateWorkloadShareError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_workload_share_error(response)
        } else {
            crate::operation_deser::parse_update_workload_share_response(response)
        }
    }
}

/// <p>Upgrade lens review.</p>
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpgradeLensReview {
    _private: (),
}
impl UpgradeLensReview {
    /// Creates a new builder-style object to manufacture [`UpgradeLensReviewInput`](crate::input::UpgradeLensReviewInput)
    pub fn builder() -> crate::input::upgrade_lens_review_input::Builder {
        crate::input::upgrade_lens_review_input::Builder::default()
    }
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl smithy_http::response::ParseStrictResponse for UpgradeLensReview {
    type Output = std::result::Result<
        crate::output::UpgradeLensReviewOutput,
        crate::error::UpgradeLensReviewError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_upgrade_lens_review_error(response)
        } else {
            crate::operation_deser::parse_upgrade_lens_review_response(response)
        }
    }
}
