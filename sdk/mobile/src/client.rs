// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    client: smithy_client::Client<C, M, R>,
    conf: crate::Config,
}

/// An ergonomic service client for `AWSMobileService`.
///
/// This client allows ergonomic access to a `AWSMobileService`-shaped service.
/// Each method corresponds to an endpoint defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
///
/// # Using a Client
///
/// Once you have a client set up, you can access the service's endpoints
/// by calling the appropriate method on [`Client`]. Each such method
/// returns a request builder for that endpoint, with methods for setting
/// the various fields of the request. Once your request is complete, use
/// the `send` method to send the request. `send` returns a future, which
/// you then have to `.await` to get the service's response.
///
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#c-builder
/// [SigV4-signed requests]: https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html
#[derive(std::fmt::Debug)]
pub struct Client<
    C = smithy_client::erase::DynConnector,
    M = aws_hyper::AwsMiddleware,
    R = smithy_client::retry::Standard,
> {
    handle: std::sync::Arc<Handle<C, M, R>>,
}

impl<C, M, R> std::clone::Clone for Client<C, M, R> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

#[doc(inline)]
pub use smithy_client::Builder;

impl<C, M, R> From<smithy_client::Client<C, M, R>> for Client<C, M, R> {
    fn from(client: smithy_client::Client<C, M, R>) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl<C, M, R> Client<C, M, R> {
    pub fn with_config(client: smithy_client::Client<C, M, R>, conf: crate::Config) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl<C, M, R> Client<C, M, R>
where
    C: smithy_client::bounds::SmithyConnector,
    M: smithy_client::bounds::SmithyMiddleware<C>,
    R: smithy_client::retry::NewRequestPolicy,
{
    pub fn create_project(&self) -> fluent_builders::CreateProject<C, M, R> {
        fluent_builders::CreateProject::new(self.handle.clone())
    }
    pub fn delete_project(&self) -> fluent_builders::DeleteProject<C, M, R> {
        fluent_builders::DeleteProject::new(self.handle.clone())
    }
    pub fn describe_bundle(&self) -> fluent_builders::DescribeBundle<C, M, R> {
        fluent_builders::DescribeBundle::new(self.handle.clone())
    }
    pub fn describe_project(&self) -> fluent_builders::DescribeProject<C, M, R> {
        fluent_builders::DescribeProject::new(self.handle.clone())
    }
    pub fn export_bundle(&self) -> fluent_builders::ExportBundle<C, M, R> {
        fluent_builders::ExportBundle::new(self.handle.clone())
    }
    pub fn export_project(&self) -> fluent_builders::ExportProject<C, M, R> {
        fluent_builders::ExportProject::new(self.handle.clone())
    }
    pub fn list_bundles(&self) -> fluent_builders::ListBundles<C, M, R> {
        fluent_builders::ListBundles::new(self.handle.clone())
    }
    pub fn list_projects(&self) -> fluent_builders::ListProjects<C, M, R> {
        fluent_builders::ListProjects::new(self.handle.clone())
    }
    pub fn update_project(&self) -> fluent_builders::UpdateProject<C, M, R> {
        fluent_builders::UpdateProject::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct CreateProject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::create_project_input::Builder,
    }
    impl<C, M, R> CreateProject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::CreateProjectOutput,
            smithy_http::result::SdkError<crate::error::CreateProjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::CreateProjectInputOperationOutputAlias,
                crate::output::CreateProjectOutput,
                crate::error::CreateProjectError,
                crate::input::CreateProjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Name of the project.
        /// </p>
        pub fn name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.name(inp);
            self
        }
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_name(input);
            self
        }
        /// <p>
        /// Default region where project resources should be created.
        /// </p>
        pub fn region(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.region(inp);
            self
        }
        pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_region(input);
            self
        }
        /// <p>
        /// ZIP or YAML file which contains configuration settings to be used when creating
        /// the project. This may be the contents of the file downloaded from the URL provided
        /// in an export project operation.
        /// </p>
        pub fn contents(mut self, inp: smithy_types::Blob) -> Self {
            self.inner = self.inner.contents(inp);
            self
        }
        pub fn set_contents(mut self, input: std::option::Option<smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_contents(input);
            self
        }
        /// <p>
        /// Unique identifier for an exported snapshot of project configuration. This
        /// snapshot identifier is included in the share URL when a project is exported.
        /// </p>
        pub fn snapshot_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.snapshot_id(inp);
            self
        }
        pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_snapshot_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DeleteProject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::delete_project_input::Builder,
    }
    impl<C, M, R> DeleteProject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DeleteProjectOutput,
            smithy_http::result::SdkError<crate::error::DeleteProjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DeleteProjectInputOperationOutputAlias,
                crate::output::DeleteProjectOutput,
                crate::error::DeleteProjectError,
                crate::input::DeleteProjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(inp);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeBundle<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_bundle_input::Builder,
    }
    impl<C, M, R> DescribeBundle<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeBundleOutput,
            smithy_http::result::SdkError<crate::error::DescribeBundleError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeBundleInputOperationOutputAlias,
                crate::output::DescribeBundleOutput,
                crate::error::DescribeBundleError,
                crate::input::DescribeBundleInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Unique bundle identifier.
        /// </p>
        pub fn bundle_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bundle_id(inp);
            self
        }
        pub fn set_bundle_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bundle_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct DescribeProject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_project_input::Builder,
    }
    impl<C, M, R> DescribeProject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::DescribeProjectOutput,
            smithy_http::result::SdkError<crate::error::DescribeProjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeProjectInputOperationOutputAlias,
                crate::output::DescribeProjectOutput,
                crate::error::DescribeProjectError,
                crate::input::DescribeProjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(inp);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
        /// <p>
        /// If set to true, causes AWS Mobile Hub to synchronize information from other services, e.g., update state of AWS CloudFormation stacks in the AWS Mobile Hub project.
        /// </p>
        pub fn sync_from_resources(mut self, inp: bool) -> Self {
            self.inner = self.inner.sync_from_resources(inp);
            self
        }
        pub fn set_sync_from_resources(mut self, input: std::option::Option<bool>) -> Self {
            self.inner = self.inner.set_sync_from_resources(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ExportBundle<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::export_bundle_input::Builder,
    }
    impl<C, M, R> ExportBundle<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ExportBundleOutput,
            smithy_http::result::SdkError<crate::error::ExportBundleError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ExportBundleInputOperationOutputAlias,
                crate::output::ExportBundleOutput,
                crate::error::ExportBundleError,
                crate::input::ExportBundleInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Unique bundle identifier.
        /// </p>
        pub fn bundle_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.bundle_id(inp);
            self
        }
        pub fn set_bundle_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_bundle_id(input);
            self
        }
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(inp);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
        /// <p>
        /// Developer desktop or target application platform.
        /// </p>
        pub fn platform(mut self, inp: crate::model::Platform) -> Self {
            self.inner = self.inner.platform(inp);
            self
        }
        pub fn set_platform(mut self, input: std::option::Option<crate::model::Platform>) -> Self {
            self.inner = self.inner.set_platform(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ExportProject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::export_project_input::Builder,
    }
    impl<C, M, R> ExportProject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ExportProjectOutput,
            smithy_http::result::SdkError<crate::error::ExportProjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ExportProjectInputOperationOutputAlias,
                crate::output::ExportProjectOutput,
                crate::error::ExportProjectError,
                crate::input::ExportProjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(inp);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListBundles<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_bundles_input::Builder,
    }
    impl<C, M, R> ListBundles<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListBundlesOutput,
            smithy_http::result::SdkError<crate::error::ListBundlesError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListBundlesInputOperationOutputAlias,
                crate::output::ListBundlesOutput,
                crate::error::ListBundlesError,
                crate::input::ListBundlesInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Maximum number of records to list in a single response.
        /// </p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>
        /// Pagination token. Set to null to start listing bundles from start.
        /// If non-null pagination token is returned in a result, then pass its
        /// value in here in another request to list more bundles.
        /// </p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct ListProjects<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::list_projects_input::Builder,
    }
    impl<C, M, R> ListProjects<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::ListProjectsOutput,
            smithy_http::result::SdkError<crate::error::ListProjectsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::ListProjectsInputOperationOutputAlias,
                crate::output::ListProjectsOutput,
                crate::error::ListProjectsError,
                crate::input::ListProjectsInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// Maximum number of records to list in a single response.
        /// </p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>
        /// Pagination token. Set to null to start listing projects from start.
        /// If non-null pagination token is returned in a result, then pass its
        /// value in here in another request to list more projects.
        /// </p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct UpdateProject<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::update_project_input::Builder,
    }
    impl<C, M, R> UpdateProject<C, M, R>
    where
        C: smithy_client::bounds::SmithyConnector,
        M: smithy_client::bounds::SmithyMiddleware<C>,
        R: smithy_client::retry::NewRequestPolicy,
    {
        pub(crate) fn new(handle: std::sync::Arc<super::Handle<C, M, R>>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }
        pub async fn send(
            self,
        ) -> std::result::Result<
            crate::output::UpdateProjectOutput,
            smithy_http::result::SdkError<crate::error::UpdateProjectError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::UpdateProjectInputOperationOutputAlias,
                crate::output::UpdateProjectOutput,
                crate::error::UpdateProjectError,
                crate::input::UpdateProjectInputOperationRetryAlias,
            >,
        {
            let input = self
                .inner
                .build()
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            let op = input
                .make_operation(&self.handle.conf)
                .map_err(|err| smithy_http::result::SdkError::ConstructionFailure(err.into()))?;
            self.handle.client.call(op).await
        }
        /// <p>
        /// ZIP or YAML file which contains project configuration to be updated. This should
        /// be the contents of the file downloaded from the URL provided in an export project
        /// operation.
        /// </p>
        pub fn contents(mut self, inp: smithy_types::Blob) -> Self {
            self.inner = self.inner.contents(inp);
            self
        }
        pub fn set_contents(mut self, input: std::option::Option<smithy_types::Blob>) -> Self {
            self.inner = self.inner.set_contents(input);
            self
        }
        /// <p>
        /// Unique project identifier.
        /// </p>
        pub fn project_id(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.project_id(inp);
            self
        }
        pub fn set_project_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_project_id(input);
            self
        }
    }
}
impl<C> Client<C, aws_hyper::AwsMiddleware, smithy_client::retry::Standard> {
    pub fn from_conf_conn(conf: crate::Config, conn: C) -> Self {
        let client = aws_hyper::Client::new(conn);
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
impl
    Client<
        smithy_client::erase::DynConnector,
        aws_hyper::AwsMiddleware,
        smithy_client::retry::Standard,
    >
{
    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn new(config: &aws_types::config::Config) -> Self {
        Self::from_conf(config.into())
    }

    #[cfg(any(feature = "rustls", feature = "native-tls"))]
    pub fn from_conf(conf: crate::Config) -> Self {
        let client = aws_hyper::Client::https();
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
