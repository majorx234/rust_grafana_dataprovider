# visualization_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**api.endpoints.list_metric_payload_options**](visualization_api.md#api.endpoints.list_metric_payload_options) | **POST** /metric-payload-options | List the available payload options.
**api.endpoints.list_metrics**](visualization_api.md#api.endpoints.list_metrics) | **POST** /metrics | List available metrics
**api.endpoints.query**](visualization_api.md#api.endpoints.query) | **POST** /query | Query


# **api.endpoints.list_metric_payload_options**
> Vec<models::ApiEndpointsListMetricPayloadOptions200ResponseInner> api.endpoints.list_metric_payload_options(optional)
List the available payload options.

When the payload `type` is `select` or `multi-select` and the payload `options` configuration is empty, expanding the drop-down menu will trigger this API. The request body will carry the current metric and payload.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **api_endpoints_list_metric_payload_options_request** | [**ApiEndpointsListMetricPayloadOptionsRequest**](ApiEndpointsListMetricPayloadOptionsRequest.md)|  | 

### Return type

[**Vec<models::ApiEndpointsListMetricPayloadOptions200ResponseInner>**](api_endpoints_list_metric_payload_options_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api.endpoints.list_metrics**
> Vec<models::ApiEndpointsListMetrics200ResponseInner> api.endpoints.list_metrics(optional)
List available metrics

In `Panel > Queries` page. It will send the request to obtain the available metrics. The request body will carry the current metric and payload. In the `Builder` mode, if the `reloadMetric` value in the load configuration is true, the api will also be triggered when the value is modified / switched.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **metric_obj** | [**MetricObj**](MetricObj.md)|  | 

### Return type

[**Vec<models::ApiEndpointsListMetrics200ResponseInner>**](api_endpoints_list_metrics_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api.endpoints.query**
> Vec<models::ApiEndpointsQuery200ResponseInner> api.endpoints.query(api_endpoints_query_request)
Query

Returns metrics data

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_endpoints_query_request** | [**ApiEndpointsQueryRequest**](ApiEndpointsQueryRequest.md)|  | 

### Return type

[**Vec<models::ApiEndpointsQuery200ResponseInner>**](api_endpoints_query_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

