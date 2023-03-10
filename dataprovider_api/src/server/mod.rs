use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     ApiPeriodEndpointsPeriodTagKeysResponse,
     ApiPeriodEndpointsPeriodTagValuesResponse,
     ApiPeriodEndpointsPeriodDatasourceHealthResponse,
     ApiPeriodEndpointsPeriodVariableResponse,
     ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse,
     ApiPeriodEndpointsPeriodListMetricsResponse,
     ApiPeriodEndpointsPeriodQueryResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/$",
            r"^/metric-payload-options$",
            r"^/metrics$",
            r"^/query$",
            r"^/tag-keys$",
            r"^/tag-values$",
            r"^/variable$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_: usize = 0;
    pub(crate) static ID_METRIC_PAYLOAD_OPTIONS: usize = 1;
    pub(crate) static ID_METRICS: usize = 2;
    pub(crate) static ID_QUERY: usize = 3;
    pub(crate) static ID_TAG_KEYS: usize = 4;
    pub(crate) static ID_TAG_VALUES: usize = 5;
    pub(crate) static ID_VARIABLE: usize = 6;
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString>  + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString>  + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // ApiPeriodEndpointsPeriodTagKeys - POST /tag-keys
            hyper::Method::POST if path.matched(paths::ID_TAG_KEYS) => {
                                let result = api_impl.api_period_endpoints_period_tag_keys(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiPeriodEndpointsPeriodTagKeysResponse::ListOfAdHocFilterKeys
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for API_PERIOD_ENDPOINTS_PERIOD_TAG_KEYS_LIST_OF_AD_HOC_FILTER_KEYS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ApiPeriodEndpointsPeriodTagValues - POST /tag-values
            hyper::Method::POST if path.matched(paths::ID_TAG_VALUES) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_api_endpoints_tag_values_request: Option<models::ApiEndpointsTagValuesRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_api_endpoints_tag_values_request) => param_api_endpoints_tag_values_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter ApiEndpointsTagValuesRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ApiEndpointsTagValuesRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_api_endpoints_tag_values_request = match param_api_endpoints_tag_values_request {
                                    Some(param_api_endpoints_tag_values_request) => param_api_endpoints_tag_values_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter ApiEndpointsTagValuesRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter ApiEndpointsTagValuesRequest")),
                                };

                                let result = api_impl.api_period_endpoints_period_tag_values(
                                            param_api_endpoints_tag_values_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiPeriodEndpointsPeriodTagValuesResponse::AValidResponse
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ApiEndpointsTagValuesRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ApiEndpointsTagValuesRequest")),
                        }
            },

            // ApiPeriodEndpointsPeriodDatasourceHealth - GET /
            hyper::Method::GET if path.matched(paths::ID_) => {
                                let result = api_impl.api_period_endpoints_period_datasource_health(
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiPeriodEndpointsPeriodDatasourceHealthResponse::PositiveHealthCheck
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ApiPeriodEndpointsPeriodVariable - POST /variable
            hyper::Method::POST if path.matched(paths::ID_VARIABLE) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_api_endpoints_variable_request: Option<models::ApiEndpointsVariableRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_api_endpoints_variable_request) => param_api_endpoints_variable_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter ApiEndpointsVariableRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ApiEndpointsVariableRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_api_endpoints_variable_request = match param_api_endpoints_variable_request {
                                    Some(param_api_endpoints_variable_request) => param_api_endpoints_variable_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter ApiEndpointsVariableRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter ApiEndpointsVariableRequest")),
                                };

                                let result = api_impl.api_period_endpoints_period_variable(
                                            param_api_endpoints_variable_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiPeriodEndpointsPeriodVariableResponse::VariableItems
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for API_PERIOD_ENDPOINTS_PERIOD_VARIABLE_VARIABLE_ITEMS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ApiEndpointsVariableRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ApiEndpointsVariableRequest")),
                        }
            },

            // ApiPeriodEndpointsPeriodListMetricPayloadOptions - POST /metric-payload-options
            hyper::Method::POST if path.matched(paths::ID_METRIC_PAYLOAD_OPTIONS) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_api_endpoints_list_metric_payload_options_request: Option<models::ApiEndpointsListMetricPayloadOptionsRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_api_endpoints_list_metric_payload_options_request) => param_api_endpoints_list_metric_payload_options_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.api_period_endpoints_period_list_metric_payload_options(
                                            param_api_endpoints_list_metric_payload_options_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse::Success
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for API_PERIOD_ENDPOINTS_PERIOD_LIST_METRIC_PAYLOAD_OPTIONS_SUCCESS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ApiEndpointsListMetricPayloadOptionsRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ApiEndpointsListMetricPayloadOptionsRequest")),
                        }
            },

            // ApiPeriodEndpointsPeriodListMetrics - POST /metrics
            hyper::Method::POST if path.matched(paths::ID_METRICS) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_metric_obj: Option<models::MetricObj> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_metric_obj) => param_metric_obj,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.api_period_endpoints_period_list_metrics(
                                            param_metric_obj,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiPeriodEndpointsPeriodListMetricsResponse::Success
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for API_PERIOD_ENDPOINTS_PERIOD_LIST_METRICS_SUCCESS"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter MetricObj: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter MetricObj")),
                        }
            },

            // ApiPeriodEndpointsPeriodQuery - POST /query
            hyper::Method::POST if path.matched(paths::ID_QUERY) => {
                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_api_endpoints_query_request: Option<models::ApiEndpointsQueryRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&*body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_api_endpoints_query_request) => param_api_endpoints_query_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter ApiEndpointsQueryRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ApiEndpointsQueryRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_api_endpoints_query_request = match param_api_endpoints_query_request {
                                    Some(param_api_endpoints_query_request) => param_api_endpoints_query_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter ApiEndpointsQueryRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter ApiEndpointsQueryRequest")),
                                };

                                let result = api_impl.api_period_endpoints_period_query(
                                            param_api_endpoints_query_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ApiPeriodEndpointsPeriodQueryResponse::CanBeA
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for API_PERIOD_ENDPOINTS_PERIOD_QUERY_CAN_BE_A"));
                                                    let body = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ApiEndpointsQueryRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ApiEndpointsQueryRequest")),
                        }
            },

            _ if path.matched(paths::ID_) => method_not_allowed(),
            _ if path.matched(paths::ID_METRIC_PAYLOAD_OPTIONS) => method_not_allowed(),
            _ if path.matched(paths::ID_METRICS) => method_not_allowed(),
            _ if path.matched(paths::ID_QUERY) => method_not_allowed(),
            _ if path.matched(paths::ID_TAG_KEYS) => method_not_allowed(),
            _ if path.matched(paths::ID_TAG_VALUES) => method_not_allowed(),
            _ if path.matched(paths::ID_VARIABLE) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // ApiPeriodEndpointsPeriodTagKeys - POST /tag-keys
            hyper::Method::POST if path.matched(paths::ID_TAG_KEYS) => Some("ApiPeriodEndpointsPeriodTagKeys"),
            // ApiPeriodEndpointsPeriodTagValues - POST /tag-values
            hyper::Method::POST if path.matched(paths::ID_TAG_VALUES) => Some("ApiPeriodEndpointsPeriodTagValues"),
            // ApiPeriodEndpointsPeriodDatasourceHealth - GET /
            hyper::Method::GET if path.matched(paths::ID_) => Some("ApiPeriodEndpointsPeriodDatasourceHealth"),
            // ApiPeriodEndpointsPeriodVariable - POST /variable
            hyper::Method::POST if path.matched(paths::ID_VARIABLE) => Some("ApiPeriodEndpointsPeriodVariable"),
            // ApiPeriodEndpointsPeriodListMetricPayloadOptions - POST /metric-payload-options
            hyper::Method::POST if path.matched(paths::ID_METRIC_PAYLOAD_OPTIONS) => Some("ApiPeriodEndpointsPeriodListMetricPayloadOptions"),
            // ApiPeriodEndpointsPeriodListMetrics - POST /metrics
            hyper::Method::POST if path.matched(paths::ID_METRICS) => Some("ApiPeriodEndpointsPeriodListMetrics"),
            // ApiPeriodEndpointsPeriodQuery - POST /query
            hyper::Method::POST if path.matched(paths::ID_QUERY) => Some("ApiPeriodEndpointsPeriodQuery"),
            _ => None,
        }
    }
}
