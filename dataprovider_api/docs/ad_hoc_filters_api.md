# ad_hoc_filters_api

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
**api.endpoints.tag_keys**](ad_hoc_filters_api.md#api.endpoints.tag_keys) | **POST** /tag-keys | Keys for ad hoc filters
**api.endpoints.tag_values**](ad_hoc_filters_api.md#api.endpoints.tag_values) | **POST** /tag-values | Values for ad hoc filters


# **api.endpoints.tag_keys**
> Vec<models::ApiEndpointsTagKeys200ResponseInner> api.endpoints.tag_keys()
Keys for ad hoc filters

returns possible keys for ad hoc filters

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**Vec<models::ApiEndpointsTagKeys200ResponseInner>**](api_endpoints_tag_keys_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **api.endpoints.tag_values**
> api.endpoints.tag_values(api_endpoints_tag_values_request)
Values for ad hoc filters

returns possible values for ad hoc filters

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **api_endpoints_tag_values_request** | [**ApiEndpointsTagValuesRequest**](ApiEndpointsTagValuesRequest.md)|  | 

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

