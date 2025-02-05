// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Details about a member account that was invited to contribute to a behavior
/// graph.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MemberDetail {
    /// <p>The AWS account identifier for the member account.</p>
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The AWS account root user email address for the member account.</p>
    pub email_address: std::option::Option<std::string::String>,
    /// <p>The ARN of the behavior graph that the member account was invited to.</p>
    pub graph_arn: std::option::Option<std::string::String>,
    /// <p>The AWS account identifier of the administrator account for the behavior graph.</p>
    pub master_id: std::option::Option<std::string::String>,
    /// <p>The AWS account identifier of the administrator account for the behavior graph.</p>
    pub administrator_id: std::option::Option<std::string::String>,
    /// <p>The current membership status of the member account. The status can have one of the
    /// following values:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>INVITED</code> - Indicates that the member was sent an invitation but has
    /// not yet responded.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>VERIFICATION_IN_PROGRESS</code> - Indicates that Detective is verifying that the
    /// account identifier and email address provided for the member account match. If they
    /// do match, then Detective sends the invitation. If the email address and account
    /// identifier don't match, then the member cannot be added to the behavior graph.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>VERIFICATION_FAILED</code> - Indicates that the account and email address
    /// provided for the member account do not match, and Detective did not send an invitation to
    /// the account.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ENABLED</code> - Indicates that the member account accepted the invitation
    /// to contribute to the behavior graph.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>ACCEPTED_BUT_DISABLED</code> - Indicates that the member account accepted
    /// the invitation but is prevented from contributing data to the behavior graph.
    /// <code>DisabledReason</code> provides the reason why the member account is not
    /// enabled.</p>
    /// </li>
    /// </ul>
    /// <p>Member accounts that declined an invitation or that were removed from the behavior graph
    /// are not included.</p>
    pub status: std::option::Option<crate::model::MemberStatus>,
    /// <p>For member accounts with a status of <code>ACCEPTED_BUT_DISABLED</code>, the reason that
    /// the member account is not enabled.</p>
    /// <p>The reason can have one of the following values:</p>
    /// <ul>
    /// <li>
    /// <p>
    /// <code>VOLUME_TOO_HIGH</code> - Indicates that adding the member account would
    /// cause the data volume for the behavior graph to be too high.</p>
    /// </li>
    /// <li>
    /// <p>
    /// <code>VOLUME_UNKNOWN</code> - Indicates that Detective is unable to verify the data
    /// volume for the member account. This is usually because the member account is not
    /// enrolled in Amazon GuardDuty. </p>
    /// </li>
    /// </ul>
    pub disabled_reason: std::option::Option<crate::model::MemberDisabledReason>,
    /// <p>The date and time that Detective sent the invitation to the member account. The value is in
    /// milliseconds since the epoch.</p>
    pub invited_time: std::option::Option<smithy_types::Instant>,
    /// <p>The date and time that the member account was last updated. The value is in milliseconds
    /// since the epoch.</p>
    pub updated_time: std::option::Option<smithy_types::Instant>,
    /// <p>The data volume in bytes per day for the member account.</p>
    pub volume_usage_in_bytes: std::option::Option<i64>,
    /// <p>The data and time when the member account data volume was last updated.</p>
    pub volume_usage_updated_time: std::option::Option<smithy_types::Instant>,
    /// <p>The member account data volume as a percentage of the maximum allowed data volume. 0
    /// indicates 0 percent, and 100 indicates 100 percent.</p>
    /// <p>Note that this is not the percentage of the behavior graph data volume.</p>
    /// <p>For example, the data volume for the behavior graph is 80 GB per day. The maximum data
    /// volume is 160 GB per day. If the data volume for the member account is 40 GB per day, then
    /// <code>PercentOfGraphUtilization</code> is 25. It represents 25% of the maximum allowed
    /// data volume. </p>
    pub percent_of_graph_utilization: std::option::Option<f64>,
    /// <p>The date and time when the graph utilization percentage was last updated.</p>
    pub percent_of_graph_utilization_updated_time: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for MemberDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MemberDetail");
        formatter.field("account_id", &self.account_id);
        formatter.field("email_address", &self.email_address);
        formatter.field("graph_arn", &self.graph_arn);
        formatter.field("master_id", &self.master_id);
        formatter.field("administrator_id", &self.administrator_id);
        formatter.field("status", &self.status);
        formatter.field("disabled_reason", &self.disabled_reason);
        formatter.field("invited_time", &self.invited_time);
        formatter.field("updated_time", &self.updated_time);
        formatter.field("volume_usage_in_bytes", &self.volume_usage_in_bytes);
        formatter.field("volume_usage_updated_time", &self.volume_usage_updated_time);
        formatter.field(
            "percent_of_graph_utilization",
            &self.percent_of_graph_utilization,
        );
        formatter.field(
            "percent_of_graph_utilization_updated_time",
            &self.percent_of_graph_utilization_updated_time,
        );
        formatter.finish()
    }
}
/// See [`MemberDetail`](crate::model::MemberDetail)
pub mod member_detail {
    /// A builder for [`MemberDetail`](crate::model::MemberDetail)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) account_id: std::option::Option<std::string::String>,
        pub(crate) email_address: std::option::Option<std::string::String>,
        pub(crate) graph_arn: std::option::Option<std::string::String>,
        pub(crate) master_id: std::option::Option<std::string::String>,
        pub(crate) administrator_id: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::MemberStatus>,
        pub(crate) disabled_reason: std::option::Option<crate::model::MemberDisabledReason>,
        pub(crate) invited_time: std::option::Option<smithy_types::Instant>,
        pub(crate) updated_time: std::option::Option<smithy_types::Instant>,
        pub(crate) volume_usage_in_bytes: std::option::Option<i64>,
        pub(crate) volume_usage_updated_time: std::option::Option<smithy_types::Instant>,
        pub(crate) percent_of_graph_utilization: std::option::Option<f64>,
        pub(crate) percent_of_graph_utilization_updated_time:
            std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The AWS account identifier for the member account.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input;
            self
        }
        /// <p>The AWS account root user email address for the member account.</p>
        pub fn email_address(mut self, input: impl Into<std::string::String>) -> Self {
            self.email_address = Some(input.into());
            self
        }
        pub fn set_email_address(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.email_address = input;
            self
        }
        /// <p>The ARN of the behavior graph that the member account was invited to.</p>
        pub fn graph_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.graph_arn = Some(input.into());
            self
        }
        pub fn set_graph_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.graph_arn = input;
            self
        }
        /// <p>The AWS account identifier of the administrator account for the behavior graph.</p>
        pub fn master_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.master_id = Some(input.into());
            self
        }
        pub fn set_master_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.master_id = input;
            self
        }
        /// <p>The AWS account identifier of the administrator account for the behavior graph.</p>
        pub fn administrator_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.administrator_id = Some(input.into());
            self
        }
        pub fn set_administrator_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.administrator_id = input;
            self
        }
        /// <p>The current membership status of the member account. The status can have one of the
        /// following values:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>INVITED</code> - Indicates that the member was sent an invitation but has
        /// not yet responded.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>VERIFICATION_IN_PROGRESS</code> - Indicates that Detective is verifying that the
        /// account identifier and email address provided for the member account match. If they
        /// do match, then Detective sends the invitation. If the email address and account
        /// identifier don't match, then the member cannot be added to the behavior graph.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>VERIFICATION_FAILED</code> - Indicates that the account and email address
        /// provided for the member account do not match, and Detective did not send an invitation to
        /// the account.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>ENABLED</code> - Indicates that the member account accepted the invitation
        /// to contribute to the behavior graph.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>ACCEPTED_BUT_DISABLED</code> - Indicates that the member account accepted
        /// the invitation but is prevented from contributing data to the behavior graph.
        /// <code>DisabledReason</code> provides the reason why the member account is not
        /// enabled.</p>
        /// </li>
        /// </ul>
        /// <p>Member accounts that declined an invitation or that were removed from the behavior graph
        /// are not included.</p>
        pub fn status(mut self, input: crate::model::MemberStatus) -> Self {
            self.status = Some(input);
            self
        }
        pub fn set_status(
            mut self,
            input: std::option::Option<crate::model::MemberStatus>,
        ) -> Self {
            self.status = input;
            self
        }
        /// <p>For member accounts with a status of <code>ACCEPTED_BUT_DISABLED</code>, the reason that
        /// the member account is not enabled.</p>
        /// <p>The reason can have one of the following values:</p>
        /// <ul>
        /// <li>
        /// <p>
        /// <code>VOLUME_TOO_HIGH</code> - Indicates that adding the member account would
        /// cause the data volume for the behavior graph to be too high.</p>
        /// </li>
        /// <li>
        /// <p>
        /// <code>VOLUME_UNKNOWN</code> - Indicates that Detective is unable to verify the data
        /// volume for the member account. This is usually because the member account is not
        /// enrolled in Amazon GuardDuty. </p>
        /// </li>
        /// </ul>
        pub fn disabled_reason(mut self, input: crate::model::MemberDisabledReason) -> Self {
            self.disabled_reason = Some(input);
            self
        }
        pub fn set_disabled_reason(
            mut self,
            input: std::option::Option<crate::model::MemberDisabledReason>,
        ) -> Self {
            self.disabled_reason = input;
            self
        }
        /// <p>The date and time that Detective sent the invitation to the member account. The value is in
        /// milliseconds since the epoch.</p>
        pub fn invited_time(mut self, input: smithy_types::Instant) -> Self {
            self.invited_time = Some(input);
            self
        }
        pub fn set_invited_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.invited_time = input;
            self
        }
        /// <p>The date and time that the member account was last updated. The value is in milliseconds
        /// since the epoch.</p>
        pub fn updated_time(mut self, input: smithy_types::Instant) -> Self {
            self.updated_time = Some(input);
            self
        }
        pub fn set_updated_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.updated_time = input;
            self
        }
        /// <p>The data volume in bytes per day for the member account.</p>
        pub fn volume_usage_in_bytes(mut self, input: i64) -> Self {
            self.volume_usage_in_bytes = Some(input);
            self
        }
        pub fn set_volume_usage_in_bytes(mut self, input: std::option::Option<i64>) -> Self {
            self.volume_usage_in_bytes = input;
            self
        }
        /// <p>The data and time when the member account data volume was last updated.</p>
        pub fn volume_usage_updated_time(mut self, input: smithy_types::Instant) -> Self {
            self.volume_usage_updated_time = Some(input);
            self
        }
        pub fn set_volume_usage_updated_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.volume_usage_updated_time = input;
            self
        }
        /// <p>The member account data volume as a percentage of the maximum allowed data volume. 0
        /// indicates 0 percent, and 100 indicates 100 percent.</p>
        /// <p>Note that this is not the percentage of the behavior graph data volume.</p>
        /// <p>For example, the data volume for the behavior graph is 80 GB per day. The maximum data
        /// volume is 160 GB per day. If the data volume for the member account is 40 GB per day, then
        /// <code>PercentOfGraphUtilization</code> is 25. It represents 25% of the maximum allowed
        /// data volume. </p>
        pub fn percent_of_graph_utilization(mut self, input: f64) -> Self {
            self.percent_of_graph_utilization = Some(input);
            self
        }
        pub fn set_percent_of_graph_utilization(mut self, input: std::option::Option<f64>) -> Self {
            self.percent_of_graph_utilization = input;
            self
        }
        /// <p>The date and time when the graph utilization percentage was last updated.</p>
        pub fn percent_of_graph_utilization_updated_time(
            mut self,
            input: smithy_types::Instant,
        ) -> Self {
            self.percent_of_graph_utilization_updated_time = Some(input);
            self
        }
        pub fn set_percent_of_graph_utilization_updated_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.percent_of_graph_utilization_updated_time = input;
            self
        }
        /// Consumes the builder and constructs a [`MemberDetail`](crate::model::MemberDetail)
        pub fn build(self) -> crate::model::MemberDetail {
            crate::model::MemberDetail {
                account_id: self.account_id,
                email_address: self.email_address,
                graph_arn: self.graph_arn,
                master_id: self.master_id,
                administrator_id: self.administrator_id,
                status: self.status,
                disabled_reason: self.disabled_reason,
                invited_time: self.invited_time,
                updated_time: self.updated_time,
                volume_usage_in_bytes: self.volume_usage_in_bytes,
                volume_usage_updated_time: self.volume_usage_updated_time,
                percent_of_graph_utilization: self.percent_of_graph_utilization,
                percent_of_graph_utilization_updated_time: self
                    .percent_of_graph_utilization_updated_time,
            }
        }
    }
}
impl MemberDetail {
    /// Creates a new builder-style object to manufacture [`MemberDetail`](crate::model::MemberDetail)
    pub fn builder() -> crate::model::member_detail::Builder {
        crate::model::member_detail::Builder::default()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum MemberDisabledReason {
    VolumeTooHigh,
    VolumeUnknown,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for MemberDisabledReason {
    fn from(s: &str) -> Self {
        match s {
            "VOLUME_TOO_HIGH" => MemberDisabledReason::VolumeTooHigh,
            "VOLUME_UNKNOWN" => MemberDisabledReason::VolumeUnknown,
            other => MemberDisabledReason::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for MemberDisabledReason {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(MemberDisabledReason::from(s))
    }
}
impl MemberDisabledReason {
    pub fn as_str(&self) -> &str {
        match self {
            MemberDisabledReason::VolumeTooHigh => "VOLUME_TOO_HIGH",
            MemberDisabledReason::VolumeUnknown => "VOLUME_UNKNOWN",
            MemberDisabledReason::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["VOLUME_TOO_HIGH", "VOLUME_UNKNOWN"]
    }
}
impl AsRef<str> for MemberDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[non_exhaustive]
#[derive(
    std::clone::Clone,
    std::cmp::Eq,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
    std::fmt::Debug,
    std::hash::Hash,
)]
pub enum MemberStatus {
    AcceptedButDisabled,
    Enabled,
    Invited,
    VerificationFailed,
    VerificationInProgress,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for MemberStatus {
    fn from(s: &str) -> Self {
        match s {
            "ACCEPTED_BUT_DISABLED" => MemberStatus::AcceptedButDisabled,
            "ENABLED" => MemberStatus::Enabled,
            "INVITED" => MemberStatus::Invited,
            "VERIFICATION_FAILED" => MemberStatus::VerificationFailed,
            "VERIFICATION_IN_PROGRESS" => MemberStatus::VerificationInProgress,
            other => MemberStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for MemberStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(MemberStatus::from(s))
    }
}
impl MemberStatus {
    pub fn as_str(&self) -> &str {
        match self {
            MemberStatus::AcceptedButDisabled => "ACCEPTED_BUT_DISABLED",
            MemberStatus::Enabled => "ENABLED",
            MemberStatus::Invited => "INVITED",
            MemberStatus::VerificationFailed => "VERIFICATION_FAILED",
            MemberStatus::VerificationInProgress => "VERIFICATION_IN_PROGRESS",
            MemberStatus::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &[
            "ACCEPTED_BUT_DISABLED",
            "ENABLED",
            "INVITED",
            "VERIFICATION_FAILED",
            "VERIFICATION_IN_PROGRESS",
        ]
    }
}
impl AsRef<str> for MemberStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>A behavior graph in Detective.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Graph {
    /// <p>The ARN of the behavior graph.</p>
    pub arn: std::option::Option<std::string::String>,
    /// <p>The date and time that the behavior graph was created. The value is in milliseconds
    /// since the epoch.</p>
    pub created_time: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Graph");
        formatter.field("arn", &self.arn);
        formatter.field("created_time", &self.created_time);
        formatter.finish()
    }
}
/// See [`Graph`](crate::model::Graph)
pub mod graph {
    /// A builder for [`Graph`](crate::model::Graph)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) arn: std::option::Option<std::string::String>,
        pub(crate) created_time: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The ARN of the behavior graph.</p>
        pub fn arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.arn = Some(input.into());
            self
        }
        pub fn set_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.arn = input;
            self
        }
        /// <p>The date and time that the behavior graph was created. The value is in milliseconds
        /// since the epoch.</p>
        pub fn created_time(mut self, input: smithy_types::Instant) -> Self {
            self.created_time = Some(input);
            self
        }
        pub fn set_created_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.created_time = input;
            self
        }
        /// Consumes the builder and constructs a [`Graph`](crate::model::Graph)
        pub fn build(self) -> crate::model::Graph {
            crate::model::Graph {
                arn: self.arn,
                created_time: self.created_time,
            }
        }
    }
}
impl Graph {
    /// Creates a new builder-style object to manufacture [`Graph`](crate::model::Graph)
    pub fn builder() -> crate::model::graph::Builder {
        crate::model::graph::Builder::default()
    }
}

/// <p>A member account that was included in a request but for which the request could not be
/// processed.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct UnprocessedAccount {
    /// <p>The AWS account identifier of the member account that was not processed.</p>
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The reason that the member account request could not be processed.</p>
    pub reason: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for UnprocessedAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("UnprocessedAccount");
        formatter.field("account_id", &self.account_id);
        formatter.field("reason", &self.reason);
        formatter.finish()
    }
}
/// See [`UnprocessedAccount`](crate::model::UnprocessedAccount)
pub mod unprocessed_account {
    /// A builder for [`UnprocessedAccount`](crate::model::UnprocessedAccount)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) account_id: std::option::Option<std::string::String>,
        pub(crate) reason: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The AWS account identifier of the member account that was not processed.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input;
            self
        }
        /// <p>The reason that the member account request could not be processed.</p>
        pub fn reason(mut self, input: impl Into<std::string::String>) -> Self {
            self.reason = Some(input.into());
            self
        }
        pub fn set_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.reason = input;
            self
        }
        /// Consumes the builder and constructs a [`UnprocessedAccount`](crate::model::UnprocessedAccount)
        pub fn build(self) -> crate::model::UnprocessedAccount {
            crate::model::UnprocessedAccount {
                account_id: self.account_id,
                reason: self.reason,
            }
        }
    }
}
impl UnprocessedAccount {
    /// Creates a new builder-style object to manufacture [`UnprocessedAccount`](crate::model::UnprocessedAccount)
    pub fn builder() -> crate::model::unprocessed_account::Builder {
        crate::model::unprocessed_account::Builder::default()
    }
}

/// <p>An AWS account that is the administrator account of or a member of a behavior
/// graph.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Account {
    /// <p>The account identifier of the AWS account.</p>
    pub account_id: std::option::Option<std::string::String>,
    /// <p>The AWS account root user email address for the AWS account.</p>
    pub email_address: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Account");
        formatter.field("account_id", &self.account_id);
        formatter.field("email_address", &self.email_address);
        formatter.finish()
    }
}
/// See [`Account`](crate::model::Account)
pub mod account {
    /// A builder for [`Account`](crate::model::Account)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) account_id: std::option::Option<std::string::String>,
        pub(crate) email_address: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The account identifier of the AWS account.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.account_id = Some(input.into());
            self
        }
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.account_id = input;
            self
        }
        /// <p>The AWS account root user email address for the AWS account.</p>
        pub fn email_address(mut self, input: impl Into<std::string::String>) -> Self {
            self.email_address = Some(input.into());
            self
        }
        pub fn set_email_address(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.email_address = input;
            self
        }
        /// Consumes the builder and constructs a [`Account`](crate::model::Account)
        pub fn build(self) -> crate::model::Account {
            crate::model::Account {
                account_id: self.account_id,
                email_address: self.email_address,
            }
        }
    }
}
impl Account {
    /// Creates a new builder-style object to manufacture [`Account`](crate::model::Account)
    pub fn builder() -> crate::model::account::Builder {
        crate::model::account::Builder::default()
    }
}
