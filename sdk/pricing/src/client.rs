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

/// An ergonomic service client for `AWSPriceListService`.
///
/// This client allows ergonomic access to a `AWSPriceListService`-shaped service.
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
    pub fn describe_services(&self) -> fluent_builders::DescribeServices<C, M, R> {
        fluent_builders::DescribeServices::new(self.handle.clone())
    }
    pub fn get_attribute_values(&self) -> fluent_builders::GetAttributeValues<C, M, R> {
        fluent_builders::GetAttributeValues::new(self.handle.clone())
    }
    pub fn get_products(&self) -> fluent_builders::GetProducts<C, M, R> {
        fluent_builders::GetProducts::new(self.handle.clone())
    }
}
pub mod fluent_builders {
    #[derive(std::fmt::Debug)]
    pub struct DescribeServices<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::describe_services_input::Builder,
    }
    impl<C, M, R> DescribeServices<C, M, R>
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
            crate::output::DescribeServicesOutput,
            smithy_http::result::SdkError<crate::error::DescribeServicesError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::DescribeServicesInputOperationOutputAlias,
                crate::output::DescribeServicesOutput,
                crate::error::DescribeServicesError,
                crate::input::DescribeServicesInputOperationRetryAlias,
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
        /// <p>The code for the service whose information you want to retrieve, such as <code>AmazonEC2</code>.
        /// You can use
        /// the <code>ServiceCode</code> to filter the results in a <code>GetProducts</code> call.
        /// To retrieve a list of all services, leave this blank.</p>
        pub fn service_code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.service_code(inp);
            self
        }
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_service_code(input);
            self
        }
        /// <p>The format version that you want the response to be in.</p>
        /// <p>Valid values are: <code>aws_v1</code>
        /// </p>
        pub fn format_version(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.format_version(inp);
            self
        }
        pub fn set_format_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_format_version(input);
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results that you want returned in the response.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetAttributeValues<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_attribute_values_input::Builder,
    }
    impl<C, M, R> GetAttributeValues<C, M, R>
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
            crate::output::GetAttributeValuesOutput,
            smithy_http::result::SdkError<crate::error::GetAttributeValuesError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetAttributeValuesInputOperationOutputAlias,
                crate::output::GetAttributeValuesOutput,
                crate::error::GetAttributeValuesError,
                crate::input::GetAttributeValuesInputOperationRetryAlias,
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
        /// <p>The service code for the service whose attributes you want to retrieve. For example, if you want
        /// the retrieve an EC2 attribute, use <code>AmazonEC2</code>.</p>
        pub fn service_code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.service_code(inp);
            self
        }
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_service_code(input);
            self
        }
        /// <p>The name of the attribute that you want to retrieve the values for, such as <code>volumeType</code>.</p>
        pub fn attribute_name(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.attribute_name(inp);
            self
        }
        pub fn set_attribute_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_attribute_name(input);
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return in response.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    #[derive(std::fmt::Debug)]
    pub struct GetProducts<
        C = smithy_client::erase::DynConnector,
        M = aws_hyper::AwsMiddleware,
        R = smithy_client::retry::Standard,
    > {
        handle: std::sync::Arc<super::Handle<C, M, R>>,
        inner: crate::input::get_products_input::Builder,
    }
    impl<C, M, R> GetProducts<C, M, R>
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
            crate::output::GetProductsOutput,
            smithy_http::result::SdkError<crate::error::GetProductsError>,
        >
        where
            R::Policy: smithy_client::bounds::SmithyRetryPolicy<
                crate::input::GetProductsInputOperationOutputAlias,
                crate::output::GetProductsOutput,
                crate::error::GetProductsError,
                crate::input::GetProductsInputOperationRetryAlias,
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
        /// <p>The code for the service whose products you want to retrieve. </p>
        pub fn service_code(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.service_code(inp);
            self
        }
        pub fn set_service_code(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_service_code(input);
            self
        }
        /// Appends an item to `Filters`.
        ///
        /// To override the contents of this collection use [`set_filters`](Self::set_filters).
        /// <p>The list of filters that limit the returned products. only products that match all filters
        /// are returned.</p>
        pub fn filters(mut self, inp: impl Into<crate::model::Filter>) -> Self {
            self.inner = self.inner.filters(inp);
            self
        }
        pub fn set_filters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Filter>>,
        ) -> Self {
            self.inner = self.inner.set_filters(input);
            self
        }
        /// <p>The format version that you want the response to be in.</p>
        /// <p>Valid values are: <code>aws_v1</code>
        /// </p>
        pub fn format_version(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.format_version(inp);
            self
        }
        pub fn set_format_version(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_format_version(input);
            self
        }
        /// <p>The pagination token that indicates the next set of results that you want to retrieve.</p>
        pub fn next_token(mut self, inp: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(inp);
            self
        }
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of results to return in the response.</p>
        pub fn max_results(mut self, inp: i32) -> Self {
            self.inner = self.inner.max_results(inp);
            self
        }
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
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
