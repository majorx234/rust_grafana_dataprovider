//! Main library entry point for dataprovider_api implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use dataprovider_api::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        dataprovider_api::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use dataprovider_api::{
    Api,
    ApiPeriodEndpointsPeriodTagKeysResponse,
    ApiPeriodEndpointsPeriodTagValuesResponse,
    ApiPeriodEndpointsPeriodDatasourceHealthResponse,
    ApiPeriodEndpointsPeriodVariableResponse,
    ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse,
    ApiPeriodEndpointsPeriodListMetricsResponse,
    ApiPeriodEndpointsPeriodQueryResponse,
};
use dataprovider_api::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Keys for ad hoc filters
    async fn api_period_endpoints_period_tag_keys(
        &self,
        context: &C) -> Result<ApiPeriodEndpointsPeriodTagKeysResponse, ApiError>
    {
        let context = context.clone();
        info!("api_period_endpoints_period_tag_keys() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Values for ad hoc filters
    async fn api_period_endpoints_period_tag_values(
        &self,
        api_endpoints_tag_values_request: models::ApiEndpointsTagValuesRequest,
        context: &C) -> Result<ApiPeriodEndpointsPeriodTagValuesResponse, ApiError>
    {
        let context = context.clone();
        info!("api_period_endpoints_period_tag_values({:?}) - X-Span-ID: {:?}", api_endpoints_tag_values_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Test connection
    async fn api_period_endpoints_period_datasource_health(
        &self,
        context: &C) -> Result<ApiPeriodEndpointsPeriodDatasourceHealthResponse, ApiError>
    {
        let context = context.clone();
        info!("api_period_endpoints_period_datasource_health() - X-Span-ID: {:?}", context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Variable
    async fn api_period_endpoints_period_variable(
        &self,
        api_endpoints_variable_request: models::ApiEndpointsVariableRequest,
        context: &C) -> Result<ApiPeriodEndpointsPeriodVariableResponse, ApiError>
    {
        let context = context.clone();
        info!("api_period_endpoints_period_variable({:?}) - X-Span-ID: {:?}", api_endpoints_variable_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List the available payload options.
    async fn api_period_endpoints_period_list_metric_payload_options(
        &self,
        api_endpoints_list_metric_payload_options_request: Option<models::ApiEndpointsListMetricPayloadOptionsRequest>,
        context: &C) -> Result<ApiPeriodEndpointsPeriodListMetricPayloadOptionsResponse, ApiError>
    {
        let context = context.clone();
        info!("api_period_endpoints_period_list_metric_payload_options({:?}) - X-Span-ID: {:?}", api_endpoints_list_metric_payload_options_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List available metrics
    async fn api_period_endpoints_period_list_metrics(
        &self,
        metric_obj: Option<models::MetricObj>,
        context: &C) -> Result<ApiPeriodEndpointsPeriodListMetricsResponse, ApiError>
    {
        let context = context.clone();
        info!("api_period_endpoints_period_list_metrics({:?}) - X-Span-ID: {:?}", metric_obj, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Query
    async fn api_period_endpoints_period_query(
        &self,
        api_endpoints_query_request: models::ApiEndpointsQueryRequest,
        context: &C) -> Result<ApiPeriodEndpointsPeriodQueryResponse, ApiError>
    {
        let context = context.clone();
        info!("api_period_endpoints_period_query({:?}) - X-Span-ID: {:?}", api_endpoints_query_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}
