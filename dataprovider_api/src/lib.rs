#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::blacklisted_name)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.1";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiPeriodEndpointsPeriodTagKeysResponse {
    /// List of ad hoc filter keys
    ListOfAdHocFilterKeys
    (Vec<models::ApiEndpointsTagKeys200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiPeriodEndpointsPeriodTagValuesResponse {
    /// A valid response
    AValidResponse
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiPeriodEndpointsPeriodDatasourceHealthResponse {
    /// Positive health check
    PositiveHealthCheck
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiPeriodEndpointsPeriodVariableResponse {
    /// Variable items
    VariableItems
    (models::ApiEndpointsVariable200Response)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse {
    /// Success
    Success
    (Vec<models::ApiEndpointsListMetricPayloadOptions200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiPeriodEndpointsPeriodListMetricsResponse {
    /// Success
    Success
    (Vec<models::ApiEndpointsListMetrics200ResponseInner>)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ApiPeriodEndpointsPeriodQueryResponse {
    /// Can be a 'timeseries' or 'table' response
    CanBeA
    (Vec<models::ApiEndpointsQuery200ResponseInner>)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Keys for ad hoc filters
    async fn api_period_endpoints_period_tag_keys(
        &self,
        context: &C) -> Result<ApiPeriodEndpointsPeriodTagKeysResponse, ApiError>;

    /// Values for ad hoc filters
    async fn api_period_endpoints_period_tag_values(
        &self,
        api_endpoints_tag_values_request: models::ApiEndpointsTagValuesRequest,
        context: &C) -> Result<ApiPeriodEndpointsPeriodTagValuesResponse, ApiError>;

    /// Test connection
    async fn api_period_endpoints_period_datasource_health(
        &self,
        context: &C) -> Result<ApiPeriodEndpointsPeriodDatasourceHealthResponse, ApiError>;

    /// Variable
    async fn api_period_endpoints_period_variable(
        &self,
        api_endpoints_variable_request: models::ApiEndpointsVariableRequest,
        context: &C) -> Result<ApiPeriodEndpointsPeriodVariableResponse, ApiError>;

    /// List the available payload options.
    async fn api_period_endpoints_period_list_metric_payload_options(
        &self,
        api_endpoints_list_metric_payload_options_request: Option<models::ApiEndpointsListMetricPayloadOptionsRequest>,
        context: &C) -> Result<ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse, ApiError>;

    /// List available metrics
    async fn api_period_endpoints_period_list_metrics(
        &self,
        metric_obj: Option<models::MetricObj>,
        context: &C) -> Result<ApiPeriodEndpointsPeriodListMetricsResponse, ApiError>;

    /// Query
    async fn api_period_endpoints_period_query(
        &self,
        api_endpoints_query_request: models::ApiEndpointsQueryRequest,
        context: &C) -> Result<ApiPeriodEndpointsPeriodQueryResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Keys for ad hoc filters
    async fn api_period_endpoints_period_tag_keys(
        &self,
        ) -> Result<ApiPeriodEndpointsPeriodTagKeysResponse, ApiError>;

    /// Values for ad hoc filters
    async fn api_period_endpoints_period_tag_values(
        &self,
        api_endpoints_tag_values_request: models::ApiEndpointsTagValuesRequest,
        ) -> Result<ApiPeriodEndpointsPeriodTagValuesResponse, ApiError>;

    /// Test connection
    async fn api_period_endpoints_period_datasource_health(
        &self,
        ) -> Result<ApiPeriodEndpointsPeriodDatasourceHealthResponse, ApiError>;

    /// Variable
    async fn api_period_endpoints_period_variable(
        &self,
        api_endpoints_variable_request: models::ApiEndpointsVariableRequest,
        ) -> Result<ApiPeriodEndpointsPeriodVariableResponse, ApiError>;

    /// List the available payload options.
    async fn api_period_endpoints_period_list_metric_payload_options(
        &self,
        api_endpoints_list_metric_payload_options_request: Option<models::ApiEndpointsListMetricPayloadOptionsRequest>,
        ) -> Result<ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse, ApiError>;

    /// List available metrics
    async fn api_period_endpoints_period_list_metrics(
        &self,
        metric_obj: Option<models::MetricObj>,
        ) -> Result<ApiPeriodEndpointsPeriodListMetricsResponse, ApiError>;

    /// Query
    async fn api_period_endpoints_period_query(
        &self,
        api_endpoints_query_request: models::ApiEndpointsQueryRequest,
        ) -> Result<ApiPeriodEndpointsPeriodQueryResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Keys for ad hoc filters
    async fn api_period_endpoints_period_tag_keys(
        &self,
        ) -> Result<ApiPeriodEndpointsPeriodTagKeysResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_period_endpoints_period_tag_keys(&context).await
    }

    /// Values for ad hoc filters
    async fn api_period_endpoints_period_tag_values(
        &self,
        api_endpoints_tag_values_request: models::ApiEndpointsTagValuesRequest,
        ) -> Result<ApiPeriodEndpointsPeriodTagValuesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_period_endpoints_period_tag_values(api_endpoints_tag_values_request, &context).await
    }

    /// Test connection
    async fn api_period_endpoints_period_datasource_health(
        &self,
        ) -> Result<ApiPeriodEndpointsPeriodDatasourceHealthResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_period_endpoints_period_datasource_health(&context).await
    }

    /// Variable
    async fn api_period_endpoints_period_variable(
        &self,
        api_endpoints_variable_request: models::ApiEndpointsVariableRequest,
        ) -> Result<ApiPeriodEndpointsPeriodVariableResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_period_endpoints_period_variable(api_endpoints_variable_request, &context).await
    }

    /// List the available payload options.
    async fn api_period_endpoints_period_list_metric_payload_options(
        &self,
        api_endpoints_list_metric_payload_options_request: Option<models::ApiEndpointsListMetricPayloadOptionsRequest>,
        ) -> Result<ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_period_endpoints_period_list_metric_payload_options(api_endpoints_list_metric_payload_options_request, &context).await
    }

    /// List available metrics
    async fn api_period_endpoints_period_list_metrics(
        &self,
        metric_obj: Option<models::MetricObj>,
        ) -> Result<ApiPeriodEndpointsPeriodListMetricsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_period_endpoints_period_list_metrics(metric_obj, &context).await
    }

    /// Query
    async fn api_period_endpoints_period_query(
        &self,
        api_endpoints_query_request: models::ApiEndpointsQueryRequest,
        ) -> Result<ApiPeriodEndpointsPeriodQueryResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().api_period_endpoints_period_query(api_endpoints_query_request, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;
