// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// <p>Attribute associated with a resource.</p>
/// <p>Note the corresponding format required per type listed below:</p>
/// <dl>
/// <dt>IPV4</dt>
/// <dd>
/// <p>
/// <code>x.x.x.x</code>
/// </p>
/// <p>
/// <i>where x is an integer in the range [0,255]</i>
/// </p>
/// </dd>
/// <dt>IPV6</dt>
/// <dd>
/// <p>
/// <code>y : y : y : y : y : y : y : y</code>
/// </p>
/// <p>
/// <i>where y is a hexadecimal between 0 and FFFF. [0,
/// FFFF]</i>
/// </p>
/// </dd>
/// <dt>MAC_ADDRESS</dt>
/// <dd>
/// <p>
/// <code>^([0-9A-Fa-f]{2}[:-]){5}([0-9A-Fa-f]{2})$</code>
/// </p>
/// </dd>
/// <dt>FQDN</dt>
/// <dd>
/// <p>
/// <code>^[^<>{}\\\\/?,=\\p{Cntrl}]{1,256}$</code>
/// </p>
/// </dd>
/// </dl>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ResourceAttribute {
    /// <p>Type of resource.</p>
    pub r#type: std::option::Option<crate::model::ResourceAttributeType>,
    /// <p>Value of the resource type.</p>
    pub value: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ResourceAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ResourceAttribute");
        formatter.field("r#type", &self.r#type);
        formatter.field("value", &self.value);
        formatter.finish()
    }
}
/// See [`ResourceAttribute`](crate::model::ResourceAttribute)
pub mod resource_attribute {
    /// A builder for [`ResourceAttribute`](crate::model::ResourceAttribute)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) r#type: std::option::Option<crate::model::ResourceAttributeType>,
        pub(crate) value: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>Type of resource.</p>
        pub fn r#type(mut self, input: crate::model::ResourceAttributeType) -> Self {
            self.r#type = Some(input);
            self
        }
        pub fn set_type(
            mut self,
            input: std::option::Option<crate::model::ResourceAttributeType>,
        ) -> Self {
            self.r#type = input;
            self
        }
        /// <p>Value of the resource type.</p>
        pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
            self.value = Some(input.into());
            self
        }
        pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.value = input;
            self
        }
        /// Consumes the builder and constructs a [`ResourceAttribute`](crate::model::ResourceAttribute)
        pub fn build(self) -> crate::model::ResourceAttribute {
            crate::model::ResourceAttribute {
                r#type: self.r#type,
                value: self.value,
            }
        }
    }
}
impl ResourceAttribute {
    /// Creates a new builder-style object to manufacture [`ResourceAttribute`](crate::model::ResourceAttribute)
    pub fn builder() -> crate::model::resource_attribute::Builder {
        crate::model::resource_attribute::Builder::default()
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
pub enum ResourceAttributeType {
    BiosId,
    Fqdn,
    Ipv4Address,
    Ipv6Address,
    MacAddress,
    MotherboardSerialNumber,
    VmManagedObjectReference,
    VmManagerId,
    VmName,
    VmPath,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ResourceAttributeType {
    fn from(s: &str) -> Self {
        match s {
            "BIOS_ID" => ResourceAttributeType::BiosId,
            "FQDN" => ResourceAttributeType::Fqdn,
            "IPV4_ADDRESS" => ResourceAttributeType::Ipv4Address,
            "IPV6_ADDRESS" => ResourceAttributeType::Ipv6Address,
            "MAC_ADDRESS" => ResourceAttributeType::MacAddress,
            "MOTHERBOARD_SERIAL_NUMBER" => ResourceAttributeType::MotherboardSerialNumber,
            "VM_MANAGED_OBJECT_REFERENCE" => ResourceAttributeType::VmManagedObjectReference,
            "VM_MANAGER_ID" => ResourceAttributeType::VmManagerId,
            "VM_NAME" => ResourceAttributeType::VmName,
            "VM_PATH" => ResourceAttributeType::VmPath,
            other => ResourceAttributeType::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ResourceAttributeType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ResourceAttributeType::from(s))
    }
}
impl ResourceAttributeType {
    pub fn as_str(&self) -> &str {
        match self {
            ResourceAttributeType::BiosId => "BIOS_ID",
            ResourceAttributeType::Fqdn => "FQDN",
            ResourceAttributeType::Ipv4Address => "IPV4_ADDRESS",
            ResourceAttributeType::Ipv6Address => "IPV6_ADDRESS",
            ResourceAttributeType::MacAddress => "MAC_ADDRESS",
            ResourceAttributeType::MotherboardSerialNumber => "MOTHERBOARD_SERIAL_NUMBER",
            ResourceAttributeType::VmManagedObjectReference => "VM_MANAGED_OBJECT_REFERENCE",
            ResourceAttributeType::VmManagerId => "VM_MANAGER_ID",
            ResourceAttributeType::VmName => "VM_NAME",
            ResourceAttributeType::VmPath => "VM_PATH",
            ResourceAttributeType::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &[
            "BIOS_ID",
            "FQDN",
            "IPV4_ADDRESS",
            "IPV6_ADDRESS",
            "MAC_ADDRESS",
            "MOTHERBOARD_SERIAL_NUMBER",
            "VM_MANAGED_OBJECT_REFERENCE",
            "VM_MANAGER_ID",
            "VM_NAME",
            "VM_PATH",
        ]
    }
}
impl AsRef<str> for ResourceAttributeType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Task object encapsulating task information.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct Task {
    /// <p>Status of the task - Not Started, In-Progress, Complete.</p>
    pub status: std::option::Option<crate::model::Status>,
    /// <p>Details of task status as notified by a migration tool. A tool might use this field to
    /// provide clarifying information about the status that is unique to that tool or that
    /// explains an error state.</p>
    pub status_detail: std::option::Option<std::string::String>,
    /// <p>Indication of the percentage completion of the task.</p>
    pub progress_percent: std::option::Option<i32>,
}
impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("Task");
        formatter.field("status", &self.status);
        formatter.field("status_detail", &self.status_detail);
        formatter.field("progress_percent", &self.progress_percent);
        formatter.finish()
    }
}
/// See [`Task`](crate::model::Task)
pub mod task {
    /// A builder for [`Task`](crate::model::Task)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) status: std::option::Option<crate::model::Status>,
        pub(crate) status_detail: std::option::Option<std::string::String>,
        pub(crate) progress_percent: std::option::Option<i32>,
    }
    impl Builder {
        /// <p>Status of the task - Not Started, In-Progress, Complete.</p>
        pub fn status(mut self, input: crate::model::Status) -> Self {
            self.status = Some(input);
            self
        }
        pub fn set_status(mut self, input: std::option::Option<crate::model::Status>) -> Self {
            self.status = input;
            self
        }
        /// <p>Details of task status as notified by a migration tool. A tool might use this field to
        /// provide clarifying information about the status that is unique to that tool or that
        /// explains an error state.</p>
        pub fn status_detail(mut self, input: impl Into<std::string::String>) -> Self {
            self.status_detail = Some(input.into());
            self
        }
        pub fn set_status_detail(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.status_detail = input;
            self
        }
        /// <p>Indication of the percentage completion of the task.</p>
        pub fn progress_percent(mut self, input: i32) -> Self {
            self.progress_percent = Some(input);
            self
        }
        pub fn set_progress_percent(mut self, input: std::option::Option<i32>) -> Self {
            self.progress_percent = input;
            self
        }
        /// Consumes the builder and constructs a [`Task`](crate::model::Task)
        pub fn build(self) -> crate::model::Task {
            crate::model::Task {
                status: self.status,
                status_detail: self.status_detail,
                progress_percent: self.progress_percent,
            }
        }
    }
}
impl Task {
    /// Creates a new builder-style object to manufacture [`Task`](crate::model::Task)
    pub fn builder() -> crate::model::task::Builder {
        crate::model::task::Builder::default()
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
pub enum Status {
    Completed,
    Failed,
    InProgress,
    NotStarted,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "COMPLETED" => Status::Completed,
            "FAILED" => Status::Failed,
            "IN_PROGRESS" => Status::InProgress,
            "NOT_STARTED" => Status::NotStarted,
            other => Status::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for Status {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Status::from(s))
    }
}
impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Completed => "COMPLETED",
            Status::Failed => "FAILED",
            Status::InProgress => "IN_PROGRESS",
            Status::NotStarted => "NOT_STARTED",
            Status::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["COMPLETED", "FAILED", "IN_PROGRESS", "NOT_STARTED"]
    }
}
impl AsRef<str> for Status {
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
pub enum ApplicationStatus {
    Completed,
    InProgress,
    NotStarted,
    /// Unknown contains new variants that have been added since this code was generated.
    Unknown(String),
}
impl std::convert::From<&str> for ApplicationStatus {
    fn from(s: &str) -> Self {
        match s {
            "COMPLETED" => ApplicationStatus::Completed,
            "IN_PROGRESS" => ApplicationStatus::InProgress,
            "NOT_STARTED" => ApplicationStatus::NotStarted,
            other => ApplicationStatus::Unknown(other.to_owned()),
        }
    }
}
impl std::str::FromStr for ApplicationStatus {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(ApplicationStatus::from(s))
    }
}
impl ApplicationStatus {
    pub fn as_str(&self) -> &str {
        match self {
            ApplicationStatus::Completed => "COMPLETED",
            ApplicationStatus::InProgress => "IN_PROGRESS",
            ApplicationStatus::NotStarted => "NOT_STARTED",
            ApplicationStatus::Unknown(s) => s.as_ref(),
        }
    }
    pub fn values() -> &'static [&'static str] {
        &["COMPLETED", "IN_PROGRESS", "NOT_STARTED"]
    }
}
impl AsRef<str> for ApplicationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// <p>Summary of the AWS resource used for access control that is implicitly linked to your
/// AWS account.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ProgressUpdateStreamSummary {
    /// <p>The name of the ProgressUpdateStream. <i>Do not store personal data in this
    /// field.</i>
    /// </p>
    pub progress_update_stream_name: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ProgressUpdateStreamSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ProgressUpdateStreamSummary");
        formatter.field(
            "progress_update_stream_name",
            &self.progress_update_stream_name,
        );
        formatter.finish()
    }
}
/// See [`ProgressUpdateStreamSummary`](crate::model::ProgressUpdateStreamSummary)
pub mod progress_update_stream_summary {
    /// A builder for [`ProgressUpdateStreamSummary`](crate::model::ProgressUpdateStreamSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_update_stream_name: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The name of the ProgressUpdateStream. <i>Do not store personal data in this
        /// field.</i>
        /// </p>
        pub fn progress_update_stream_name(
            mut self,
            input: impl Into<std::string::String>,
        ) -> Self {
            self.progress_update_stream_name = Some(input.into());
            self
        }
        pub fn set_progress_update_stream_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.progress_update_stream_name = input;
            self
        }
        /// Consumes the builder and constructs a [`ProgressUpdateStreamSummary`](crate::model::ProgressUpdateStreamSummary)
        pub fn build(self) -> crate::model::ProgressUpdateStreamSummary {
            crate::model::ProgressUpdateStreamSummary {
                progress_update_stream_name: self.progress_update_stream_name,
            }
        }
    }
}
impl ProgressUpdateStreamSummary {
    /// Creates a new builder-style object to manufacture [`ProgressUpdateStreamSummary`](crate::model::ProgressUpdateStreamSummary)
    pub fn builder() -> crate::model::progress_update_stream_summary::Builder {
        crate::model::progress_update_stream_summary::Builder::default()
    }
}

/// <p>MigrationTaskSummary includes <code>MigrationTaskName</code>,
/// <code>ProgressPercent</code>, <code>ProgressUpdateStream</code>, <code>Status</code>,
/// and <code>UpdateDateTime</code> for each task.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MigrationTaskSummary {
    /// <p>An AWS resource used for access control. It should uniquely identify the migration tool
    /// as it is used for all updates made by the tool.</p>
    pub progress_update_stream: std::option::Option<std::string::String>,
    /// <p>Unique identifier that references the migration task. <i>Do not store personal
    /// data in this field.</i>
    /// </p>
    pub migration_task_name: std::option::Option<std::string::String>,
    /// <p>Status of the task.</p>
    pub status: std::option::Option<crate::model::Status>,
    /// <p>Indication of the percentage completion of the task.</p>
    pub progress_percent: std::option::Option<i32>,
    /// <p>Detail information of what is being done within the overall status state.</p>
    pub status_detail: std::option::Option<std::string::String>,
    /// <p>The timestamp when the task was gathered.</p>
    pub update_date_time: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for MigrationTaskSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MigrationTaskSummary");
        formatter.field("progress_update_stream", &self.progress_update_stream);
        formatter.field("migration_task_name", &self.migration_task_name);
        formatter.field("status", &self.status);
        formatter.field("progress_percent", &self.progress_percent);
        formatter.field("status_detail", &self.status_detail);
        formatter.field("update_date_time", &self.update_date_time);
        formatter.finish()
    }
}
/// See [`MigrationTaskSummary`](crate::model::MigrationTaskSummary)
pub mod migration_task_summary {
    /// A builder for [`MigrationTaskSummary`](crate::model::MigrationTaskSummary)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_update_stream: std::option::Option<std::string::String>,
        pub(crate) migration_task_name: std::option::Option<std::string::String>,
        pub(crate) status: std::option::Option<crate::model::Status>,
        pub(crate) progress_percent: std::option::Option<i32>,
        pub(crate) status_detail: std::option::Option<std::string::String>,
        pub(crate) update_date_time: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>An AWS resource used for access control. It should uniquely identify the migration tool
        /// as it is used for all updates made by the tool.</p>
        pub fn progress_update_stream(mut self, input: impl Into<std::string::String>) -> Self {
            self.progress_update_stream = Some(input.into());
            self
        }
        pub fn set_progress_update_stream(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.progress_update_stream = input;
            self
        }
        /// <p>Unique identifier that references the migration task. <i>Do not store personal
        /// data in this field.</i>
        /// </p>
        pub fn migration_task_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.migration_task_name = Some(input.into());
            self
        }
        pub fn set_migration_task_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.migration_task_name = input;
            self
        }
        /// <p>Status of the task.</p>
        pub fn status(mut self, input: crate::model::Status) -> Self {
            self.status = Some(input);
            self
        }
        pub fn set_status(mut self, input: std::option::Option<crate::model::Status>) -> Self {
            self.status = input;
            self
        }
        /// <p>Indication of the percentage completion of the task.</p>
        pub fn progress_percent(mut self, input: i32) -> Self {
            self.progress_percent = Some(input);
            self
        }
        pub fn set_progress_percent(mut self, input: std::option::Option<i32>) -> Self {
            self.progress_percent = input;
            self
        }
        /// <p>Detail information of what is being done within the overall status state.</p>
        pub fn status_detail(mut self, input: impl Into<std::string::String>) -> Self {
            self.status_detail = Some(input.into());
            self
        }
        pub fn set_status_detail(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.status_detail = input;
            self
        }
        /// <p>The timestamp when the task was gathered.</p>
        pub fn update_date_time(mut self, input: smithy_types::Instant) -> Self {
            self.update_date_time = Some(input);
            self
        }
        pub fn set_update_date_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.update_date_time = input;
            self
        }
        /// Consumes the builder and constructs a [`MigrationTaskSummary`](crate::model::MigrationTaskSummary)
        pub fn build(self) -> crate::model::MigrationTaskSummary {
            crate::model::MigrationTaskSummary {
                progress_update_stream: self.progress_update_stream,
                migration_task_name: self.migration_task_name,
                status: self.status,
                progress_percent: self.progress_percent,
                status_detail: self.status_detail,
                update_date_time: self.update_date_time,
            }
        }
    }
}
impl MigrationTaskSummary {
    /// Creates a new builder-style object to manufacture [`MigrationTaskSummary`](crate::model::MigrationTaskSummary)
    pub fn builder() -> crate::model::migration_task_summary::Builder {
        crate::model::migration_task_summary::Builder::default()
    }
}

/// <p>Object representing the on-premises resource being migrated.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DiscoveredResource {
    /// <p>The configurationId in Application Discovery Service that uniquely identifies the
    /// on-premise resource.</p>
    pub configuration_id: std::option::Option<std::string::String>,
    /// <p>A description that can be free-form text to record additional detail about the
    /// discovered resource for clarity or later reference.</p>
    pub description: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for DiscoveredResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DiscoveredResource");
        formatter.field("configuration_id", &self.configuration_id);
        formatter.field("description", &self.description);
        formatter.finish()
    }
}
/// See [`DiscoveredResource`](crate::model::DiscoveredResource)
pub mod discovered_resource {
    /// A builder for [`DiscoveredResource`](crate::model::DiscoveredResource)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) configuration_id: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The configurationId in Application Discovery Service that uniquely identifies the
        /// on-premise resource.</p>
        pub fn configuration_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.configuration_id = Some(input.into());
            self
        }
        pub fn set_configuration_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.configuration_id = input;
            self
        }
        /// <p>A description that can be free-form text to record additional detail about the
        /// discovered resource for clarity or later reference.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// Consumes the builder and constructs a [`DiscoveredResource`](crate::model::DiscoveredResource)
        pub fn build(self) -> crate::model::DiscoveredResource {
            crate::model::DiscoveredResource {
                configuration_id: self.configuration_id,
                description: self.description,
            }
        }
    }
}
impl DiscoveredResource {
    /// Creates a new builder-style object to manufacture [`DiscoveredResource`](crate::model::DiscoveredResource)
    pub fn builder() -> crate::model::discovered_resource::Builder {
        crate::model::discovered_resource::Builder::default()
    }
}

/// <p>An ARN of the AWS cloud resource target receiving the migration (e.g., AMI, EC2
/// instance, RDS instance, etc.).</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CreatedArtifact {
    /// <p>An ARN that uniquely identifies the result of a migration task.</p>
    pub name: std::option::Option<std::string::String>,
    /// <p>A description that can be free-form text to record additional detail about the artifact
    /// for clarity or for later reference.</p>
    pub description: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for CreatedArtifact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreatedArtifact");
        formatter.field("name", &self.name);
        formatter.field("description", &self.description);
        formatter.finish()
    }
}
/// See [`CreatedArtifact`](crate::model::CreatedArtifact)
pub mod created_artifact {
    /// A builder for [`CreatedArtifact`](crate::model::CreatedArtifact)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) description: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>An ARN that uniquely identifies the result of a migration task.</p>
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// <p>A description that can be free-form text to record additional detail about the artifact
        /// for clarity or for later reference.</p>
        pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
            self.description = Some(input.into());
            self
        }
        pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.description = input;
            self
        }
        /// Consumes the builder and constructs a [`CreatedArtifact`](crate::model::CreatedArtifact)
        pub fn build(self) -> crate::model::CreatedArtifact {
            crate::model::CreatedArtifact {
                name: self.name,
                description: self.description,
            }
        }
    }
}
impl CreatedArtifact {
    /// Creates a new builder-style object to manufacture [`CreatedArtifact`](crate::model::CreatedArtifact)
    pub fn builder() -> crate::model::created_artifact::Builder {
        crate::model::created_artifact::Builder::default()
    }
}

/// <p>The state of an application discovered through Migration Hub import, the AWS Agentless
/// Discovery Connector, or the AWS Application Discovery Agent.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ApplicationState {
    /// <p>The configurationId from the Application Discovery Service that uniquely identifies an
    /// application.</p>
    pub application_id: std::option::Option<std::string::String>,
    /// <p>The current status of an application.</p>
    pub application_status: std::option::Option<crate::model::ApplicationStatus>,
    /// <p>The timestamp when the application status was last updated.</p>
    pub last_updated_time: std::option::Option<smithy_types::Instant>,
}
impl std::fmt::Debug for ApplicationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ApplicationState");
        formatter.field("application_id", &self.application_id);
        formatter.field("application_status", &self.application_status);
        formatter.field("last_updated_time", &self.last_updated_time);
        formatter.finish()
    }
}
/// See [`ApplicationState`](crate::model::ApplicationState)
pub mod application_state {
    /// A builder for [`ApplicationState`](crate::model::ApplicationState)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_id: std::option::Option<std::string::String>,
        pub(crate) application_status: std::option::Option<crate::model::ApplicationStatus>,
        pub(crate) last_updated_time: std::option::Option<smithy_types::Instant>,
    }
    impl Builder {
        /// <p>The configurationId from the Application Discovery Service that uniquely identifies an
        /// application.</p>
        pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_id = Some(input.into());
            self
        }
        pub fn set_application_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_id = input;
            self
        }
        /// <p>The current status of an application.</p>
        pub fn application_status(mut self, input: crate::model::ApplicationStatus) -> Self {
            self.application_status = Some(input);
            self
        }
        pub fn set_application_status(
            mut self,
            input: std::option::Option<crate::model::ApplicationStatus>,
        ) -> Self {
            self.application_status = input;
            self
        }
        /// <p>The timestamp when the application status was last updated.</p>
        pub fn last_updated_time(mut self, input: smithy_types::Instant) -> Self {
            self.last_updated_time = Some(input);
            self
        }
        pub fn set_last_updated_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.last_updated_time = input;
            self
        }
        /// Consumes the builder and constructs a [`ApplicationState`](crate::model::ApplicationState)
        pub fn build(self) -> crate::model::ApplicationState {
            crate::model::ApplicationState {
                application_id: self.application_id,
                application_status: self.application_status,
                last_updated_time: self.last_updated_time,
            }
        }
    }
}
impl ApplicationState {
    /// Creates a new builder-style object to manufacture [`ApplicationState`](crate::model::ApplicationState)
    pub fn builder() -> crate::model::application_state::Builder {
        crate::model::application_state::Builder::default()
    }
}

/// <p>Represents a migration task in a migration tool.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct MigrationTask {
    /// <p>A name that identifies the vendor of the migration tool being used.</p>
    pub progress_update_stream: std::option::Option<std::string::String>,
    /// <p>Unique identifier that references the migration task. <i>Do not store personal
    /// data in this field.</i>
    /// </p>
    pub migration_task_name: std::option::Option<std::string::String>,
    /// <p>Task object encapsulating task information.</p>
    pub task: std::option::Option<crate::model::Task>,
    /// <p>The timestamp when the task was gathered.</p>
    pub update_date_time: std::option::Option<smithy_types::Instant>,
    /// <p>Information about the resource that is being migrated. This data will be used to map the
    /// task to a resource in the Application Discovery Service repository.</p>
    pub resource_attribute_list:
        std::option::Option<std::vec::Vec<crate::model::ResourceAttribute>>,
}
impl std::fmt::Debug for MigrationTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("MigrationTask");
        formatter.field("progress_update_stream", &self.progress_update_stream);
        formatter.field("migration_task_name", &self.migration_task_name);
        formatter.field("task", &self.task);
        formatter.field("update_date_time", &self.update_date_time);
        formatter.field("resource_attribute_list", &self.resource_attribute_list);
        formatter.finish()
    }
}
/// See [`MigrationTask`](crate::model::MigrationTask)
pub mod migration_task {
    /// A builder for [`MigrationTask`](crate::model::MigrationTask)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) progress_update_stream: std::option::Option<std::string::String>,
        pub(crate) migration_task_name: std::option::Option<std::string::String>,
        pub(crate) task: std::option::Option<crate::model::Task>,
        pub(crate) update_date_time: std::option::Option<smithy_types::Instant>,
        pub(crate) resource_attribute_list:
            std::option::Option<std::vec::Vec<crate::model::ResourceAttribute>>,
    }
    impl Builder {
        /// <p>A name that identifies the vendor of the migration tool being used.</p>
        pub fn progress_update_stream(mut self, input: impl Into<std::string::String>) -> Self {
            self.progress_update_stream = Some(input.into());
            self
        }
        pub fn set_progress_update_stream(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.progress_update_stream = input;
            self
        }
        /// <p>Unique identifier that references the migration task. <i>Do not store personal
        /// data in this field.</i>
        /// </p>
        pub fn migration_task_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.migration_task_name = Some(input.into());
            self
        }
        pub fn set_migration_task_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.migration_task_name = input;
            self
        }
        /// <p>Task object encapsulating task information.</p>
        pub fn task(mut self, input: crate::model::Task) -> Self {
            self.task = Some(input);
            self
        }
        pub fn set_task(mut self, input: std::option::Option<crate::model::Task>) -> Self {
            self.task = input;
            self
        }
        /// <p>The timestamp when the task was gathered.</p>
        pub fn update_date_time(mut self, input: smithy_types::Instant) -> Self {
            self.update_date_time = Some(input);
            self
        }
        pub fn set_update_date_time(
            mut self,
            input: std::option::Option<smithy_types::Instant>,
        ) -> Self {
            self.update_date_time = input;
            self
        }
        pub fn resource_attribute_list(
            mut self,
            input: impl Into<crate::model::ResourceAttribute>,
        ) -> Self {
            let mut v = self.resource_attribute_list.unwrap_or_default();
            v.push(input.into());
            self.resource_attribute_list = Some(v);
            self
        }
        pub fn set_resource_attribute_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ResourceAttribute>>,
        ) -> Self {
            self.resource_attribute_list = input;
            self
        }
        /// Consumes the builder and constructs a [`MigrationTask`](crate::model::MigrationTask)
        pub fn build(self) -> crate::model::MigrationTask {
            crate::model::MigrationTask {
                progress_update_stream: self.progress_update_stream,
                migration_task_name: self.migration_task_name,
                task: self.task,
                update_date_time: self.update_date_time,
                resource_attribute_list: self.resource_attribute_list,
            }
        }
    }
}
impl MigrationTask {
    /// Creates a new builder-style object to manufacture [`MigrationTask`](crate::model::MigrationTask)
    pub fn builder() -> crate::model::migration_task::Builder {
        crate::model::migration_task::Builder::default()
    }
}
