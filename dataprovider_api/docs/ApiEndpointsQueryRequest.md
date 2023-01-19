# ApiEndpointsQueryRequest

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**panel_id** | [***models::ApiEndpointsQueryRequestPanelId**](api_endpoints_query_request_panelId.md) |  | [optional] [default to None]
**range** | [***models::ApiEndpointsQueryRequestRange**](api_endpoints_query_request_range.md) |  | [optional] [default to None]
**range_raw** | [***models::RawTimeFrame**](raw-time-frame.md) |  | [optional] [default to None]
**interval** | **String** |  | [optional] [default to None]
**interval_ms** | **f64** |  | [optional] [default to None]
**max_data_points** | **f64** |  | [optional] [default to None]
**targets** | [**Vec<models::ApiEndpointsQueryRequestTargetsInner>**](api_endpoints_query_request_targets_inner.md) |  | [optional] [default to None]
**scoped_vars** | [***serde_json::Value**](.md) |  | [optional] [default to None]
**adhoc_filters** | [**Vec<models::ApiEndpointsQueryRequestAdhocFiltersInner>**](api_endpoints_query_request_adhocFilters_inner.md) |  | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


