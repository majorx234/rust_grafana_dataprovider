# ApiEndpointsListMetrics200ResponseInnerPayloadsInner

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**label** | **String** | The label of the payload. If the value is empty, use the name as the label. | [optional] [default to None]
**name** | **String** | The name of the payload.  | 
**r#type** | **String** | If the value is select, the UI of the payload is a radio box. If the value is multi-select, the UI of the payload is a multi selection box. if the value is input, the UI of the payload is an input box. if the value is textarea, the UI of the payload is a multiline input box. The default is input. | [optional] [default to Some("input".to_string())]
**placeholder** | **String** | Input box / selection box prompt information. | [optional] [default to None]
**reload_metric** | **bool** | Whether to overload the metrics API after modifying the value of the payload. | [optional] [default to Some(false)]
**width** | **i32** | Set the input / selection box width to a multiple of 8px. | [optional] [default to None]
**options** | [**Vec<models::ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner>**](api_endpoints_list_metrics_200_response_inner_payloads_inner_options_inner.md) | If the payload type is select / multi-select, the list is the configuration of the option list. | [optional] [default to None]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


