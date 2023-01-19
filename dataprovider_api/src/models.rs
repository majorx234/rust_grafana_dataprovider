#![allow(unused_qualifications)]

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsListMetricPayloadOptions200ResponseInner {
    /// The label of the option in the drop-down box. If the value is empty, use the value as the label.
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    /// The value of the option in the drop-down box.
    #[serde(rename = "value")]
    pub value: String,

}

impl ApiEndpointsListMetricPayloadOptions200ResponseInner {
    #[allow(clippy::new_without_default)]
    pub fn new(value: String, ) -> ApiEndpointsListMetricPayloadOptions200ResponseInner {
        ApiEndpointsListMetricPayloadOptions200ResponseInner {
            label: None,
            value,
        }
    }
}

/// Converts the ApiEndpointsListMetricPayloadOptions200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsListMetricPayloadOptions200ResponseInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.label.as_ref().map(|label| {
                vec![
                    "label".to_string(),
                    label.to_string(),
                ].join(",")
            }),


            Some("value".to_string()),
            Some(self.value.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsListMetricPayloadOptions200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsListMetricPayloadOptions200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub label: Vec<String>,
            pub value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsListMetricPayloadOptions200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "label" => intermediate_rep.label.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsListMetricPayloadOptions200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsListMetricPayloadOptions200ResponseInner {
            label: intermediate_rep.label.into_iter().next(),
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in ApiEndpointsListMetricPayloadOptions200ResponseInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptions200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptions200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptions200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsListMetricPayloadOptions200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptions200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsListMetricPayloadOptions200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsListMetricPayloadOptions200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsListMetricPayloadOptionsRequest {
    /// Current metric.
    #[serde(rename = "metric")]
    pub metric: String,

    /// Current payload.
    #[serde(rename = "payload")]
    pub payload: serde_json::Value,

    /// The payload name of the option list needs to be obtained.
    #[serde(rename = "name")]
    pub name: String,

}

impl ApiEndpointsListMetricPayloadOptionsRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(metric: String, payload: serde_json::Value, name: String, ) -> ApiEndpointsListMetricPayloadOptionsRequest {
        ApiEndpointsListMetricPayloadOptionsRequest {
            metric,
            payload,
            name,
        }
    }
}

/// Converts the ApiEndpointsListMetricPayloadOptionsRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsListMetricPayloadOptionsRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("metric".to_string()),
            Some(self.metric.to_string()),

            // Skipping payload in query parameter serialization


            Some("name".to_string()),
            Some(self.name.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsListMetricPayloadOptionsRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsListMetricPayloadOptionsRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub metric: Vec<String>,
            pub payload: Vec<serde_json::Value>,
            pub name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsListMetricPayloadOptionsRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "metric" => intermediate_rep.metric.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "payload" => intermediate_rep.payload.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsListMetricPayloadOptionsRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsListMetricPayloadOptionsRequest {
            metric: intermediate_rep.metric.into_iter().next().ok_or_else(|| "metric missing in ApiEndpointsListMetricPayloadOptionsRequest".to_string())?,
            payload: intermediate_rep.payload.into_iter().next().ok_or_else(|| "payload missing in ApiEndpointsListMetricPayloadOptionsRequest".to_string())?,
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ApiEndpointsListMetricPayloadOptionsRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptionsRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptionsRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptionsRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsListMetricPayloadOptionsRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsListMetricPayloadOptionsRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsListMetricPayloadOptionsRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsListMetricPayloadOptionsRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsListMetrics200ResponseInner {
    /// If the value is empty, use the value as the label
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    /// The value of the option.
    #[serde(rename = "value")]
    pub value: String,

    /// Configuration parameters of the payload.
    #[serde(rename = "payloads")]
    pub payloads: Vec<models::ApiEndpointsListMetrics200ResponseInnerPayloadsInner>,

}

impl ApiEndpointsListMetrics200ResponseInner {
    #[allow(clippy::new_without_default)]
    pub fn new(value: String, payloads: Vec<models::ApiEndpointsListMetrics200ResponseInnerPayloadsInner>, ) -> ApiEndpointsListMetrics200ResponseInner {
        ApiEndpointsListMetrics200ResponseInner {
            label: None,
            value,
            payloads,
        }
    }
}

/// Converts the ApiEndpointsListMetrics200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsListMetrics200ResponseInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.label.as_ref().map(|label| {
                vec![
                    "label".to_string(),
                    label.to_string(),
                ].join(",")
            }),


            Some("value".to_string()),
            Some(self.value.to_string()),

            // Skipping payloads in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsListMetrics200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsListMetrics200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub label: Vec<String>,
            pub value: Vec<String>,
            pub payloads: Vec<Vec<models::ApiEndpointsListMetrics200ResponseInnerPayloadsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsListMetrics200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "label" => intermediate_rep.label.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "payloads" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsListMetrics200ResponseInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsListMetrics200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsListMetrics200ResponseInner {
            label: intermediate_rep.label.into_iter().next(),
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in ApiEndpointsListMetrics200ResponseInner".to_string())?,
            payloads: intermediate_rep.payloads.into_iter().next().ok_or_else(|| "payloads missing in ApiEndpointsListMetrics200ResponseInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsListMetrics200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsListMetrics200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsListMetrics200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsListMetrics200ResponseInnerPayloadsInner {
    /// The label of the payload. If the value is empty, use the name as the label.
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    /// The name of the payload. 
    #[serde(rename = "name")]
    pub name: String,

    /// If the value is select, the UI of the payload is a radio box. If the value is multi-select, the UI of the payload is a multi selection box. if the value is input, the UI of the payload is an input box. if the value is textarea, the UI of the payload is a multiline input box. The default is input.
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,

    /// Input box / selection box prompt information.
    #[serde(rename = "placeholder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub placeholder: Option<String>,

    /// Whether to overload the metrics API after modifying the value of the payload.
    #[serde(rename = "reloadMetric")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub reload_metric: Option<bool>,

    /// Set the input / selection box width to a multiple of 8px.
    #[serde(rename = "width")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub width: Option<i32>,

    /// If the payload type is select / multi-select, the list is the configuration of the option list.
    #[serde(rename = "options")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub options: Option<Vec<models::ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner>>,

}

impl ApiEndpointsListMetrics200ResponseInnerPayloadsInner {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, ) -> ApiEndpointsListMetrics200ResponseInnerPayloadsInner {
        ApiEndpointsListMetrics200ResponseInnerPayloadsInner {
            label: None,
            name,
            r#type: Some("input".to_string()),
            placeholder: None,
            reload_metric: Some(false),
            width: None,
            options: None,
        }
    }
}

/// Converts the ApiEndpointsListMetrics200ResponseInnerPayloadsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsListMetrics200ResponseInnerPayloadsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.label.as_ref().map(|label| {
                vec![
                    "label".to_string(),
                    label.to_string(),
                ].join(",")
            }),


            Some("name".to_string()),
            Some(self.name.to_string()),


            self.r#type.as_ref().map(|r#type| {
                vec![
                    "type".to_string(),
                    r#type.to_string(),
                ].join(",")
            }),


            self.placeholder.as_ref().map(|placeholder| {
                vec![
                    "placeholder".to_string(),
                    placeholder.to_string(),
                ].join(",")
            }),


            self.reload_metric.as_ref().map(|reload_metric| {
                vec![
                    "reloadMetric".to_string(),
                    reload_metric.to_string(),
                ].join(",")
            }),


            self.width.as_ref().map(|width| {
                vec![
                    "width".to_string(),
                    width.to_string(),
                ].join(",")
            }),

            // Skipping options in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsListMetrics200ResponseInnerPayloadsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsListMetrics200ResponseInnerPayloadsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub label: Vec<String>,
            pub name: Vec<String>,
            pub r#type: Vec<String>,
            pub placeholder: Vec<String>,
            pub reload_metric: Vec<bool>,
            pub width: Vec<i32>,
            pub options: Vec<Vec<models::ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsListMetrics200ResponseInnerPayloadsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "label" => intermediate_rep.label.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "placeholder" => intermediate_rep.placeholder.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "reloadMetric" => intermediate_rep.reload_metric.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "width" => intermediate_rep.width.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "options" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsListMetrics200ResponseInnerPayloadsInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsListMetrics200ResponseInnerPayloadsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsListMetrics200ResponseInnerPayloadsInner {
            label: intermediate_rep.label.into_iter().next(),
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in ApiEndpointsListMetrics200ResponseInnerPayloadsInner".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next(),
            placeholder: intermediate_rep.placeholder.into_iter().next(),
            reload_metric: intermediate_rep.reload_metric.into_iter().next(),
            width: intermediate_rep.width.into_iter().next(),
            options: intermediate_rep.options.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsListMetrics200ResponseInnerPayloadsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsListMetrics200ResponseInnerPayloadsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsListMetrics200ResponseInnerPayloadsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner {
    /// The label of the payload select option.
    #[serde(rename = "label")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    /// The label of the payload value.
    #[serde(rename = "value")]
    pub value: String,

}

impl ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner {
    #[allow(clippy::new_without_default)]
    pub fn new(value: String, ) -> ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner {
        ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner {
            label: None,
            value,
        }
    }
}

/// Converts the ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.label.as_ref().map(|label| {
                vec![
                    "label".to_string(),
                    label.to_string(),
                ].join(",")
            }),


            Some("value".to_string()),
            Some(self.value.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub label: Vec<String>,
            pub value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "label" => intermediate_rep.label.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner {
            label: intermediate_rep.label.into_iter().next(),
            value: intermediate_rep.value.into_iter().next().ok_or_else(|| "value missing in ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsListMetrics200ResponseInnerPayloadsInnerOptionsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQuery200ResponseInner {
    #[serde(rename = "target")]
    pub target: String,

    #[serde(rename = "datapoints")]
    pub datapoints: Vec<Vec<f64>>,

    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "columns")]
    pub columns: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>,

    #[serde(rename = "rows")]
    pub rows: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>,

}

impl ApiEndpointsQuery200ResponseInner {
    #[allow(clippy::new_without_default)]
    pub fn new(target: String, datapoints: Vec<Vec<f64>>, r#type: String, columns: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>, rows: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>, ) -> ApiEndpointsQuery200ResponseInner {
        ApiEndpointsQuery200ResponseInner {
            target,
            datapoints,
            r#type,
            columns,
            rows,
        }
    }
}

/// Converts the ApiEndpointsQuery200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQuery200ResponseInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("target".to_string()),
            Some(self.target.to_string()),

            // Skipping datapoints in query parameter serialization


            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping columns in query parameter serialization

            // Skipping rows in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQuery200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQuery200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub target: Vec<String>,
            pub datapoints: Vec<Vec<Vec<f64>>>,
            pub r#type: Vec<String>,
            pub columns: Vec<Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>>,
            pub rows: Vec<Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQuery200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "target" => intermediate_rep.target.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "datapoints" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQuery200ResponseInner".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "columns" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQuery200ResponseInner".to_string()),
                    "rows" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQuery200ResponseInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQuery200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQuery200ResponseInner {
            target: intermediate_rep.target.into_iter().next().ok_or_else(|| "target missing in ApiEndpointsQuery200ResponseInner".to_string())?,
            datapoints: intermediate_rep.datapoints.into_iter().next().ok_or_else(|| "datapoints missing in ApiEndpointsQuery200ResponseInner".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ApiEndpointsQuery200ResponseInner".to_string())?,
            columns: intermediate_rep.columns.into_iter().next().ok_or_else(|| "columns missing in ApiEndpointsQuery200ResponseInner".to_string())?,
            rows: intermediate_rep.rows.into_iter().next().ok_or_else(|| "rows missing in ApiEndpointsQuery200ResponseInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQuery200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQuery200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQuery200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQuery200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQuery200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQuery200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQuery200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// timeseries case
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQuery200ResponseInnerAnyOf {
    #[serde(rename = "target")]
    pub target: String,

    #[serde(rename = "datapoints")]
    pub datapoints: Vec<Vec<f64>>,

}

impl ApiEndpointsQuery200ResponseInnerAnyOf {
    #[allow(clippy::new_without_default)]
    pub fn new(target: String, datapoints: Vec<Vec<f64>>, ) -> ApiEndpointsQuery200ResponseInnerAnyOf {
        ApiEndpointsQuery200ResponseInnerAnyOf {
            target,
            datapoints,
        }
    }
}

/// Converts the ApiEndpointsQuery200ResponseInnerAnyOf value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQuery200ResponseInnerAnyOf {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("target".to_string()),
            Some(self.target.to_string()),

            // Skipping datapoints in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQuery200ResponseInnerAnyOf value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQuery200ResponseInnerAnyOf {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub target: Vec<String>,
            pub datapoints: Vec<Vec<Vec<f64>>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQuery200ResponseInnerAnyOf".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "target" => intermediate_rep.target.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "datapoints" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQuery200ResponseInnerAnyOf".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQuery200ResponseInnerAnyOf".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQuery200ResponseInnerAnyOf {
            target: intermediate_rep.target.into_iter().next().ok_or_else(|| "target missing in ApiEndpointsQuery200ResponseInnerAnyOf".to_string())?,
            datapoints: intermediate_rep.datapoints.into_iter().next().ok_or_else(|| "datapoints missing in ApiEndpointsQuery200ResponseInnerAnyOf".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQuery200ResponseInnerAnyOf - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQuery200ResponseInnerAnyOf as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQuery200ResponseInnerAnyOf - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// table case
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQuery200ResponseInnerAnyOf1 {
    // Note: inline enums are not fully supported by openapi-generator
    #[serde(rename = "type")]
    pub r#type: String,

    #[serde(rename = "columns")]
    pub columns: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>,

    #[serde(rename = "rows")]
    pub rows: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>,

}

impl ApiEndpointsQuery200ResponseInnerAnyOf1 {
    #[allow(clippy::new_without_default)]
    pub fn new(r#type: String, columns: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>, rows: Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>, ) -> ApiEndpointsQuery200ResponseInnerAnyOf1 {
        ApiEndpointsQuery200ResponseInnerAnyOf1 {
            r#type,
            columns,
            rows,
        }
    }
}

/// Converts the ApiEndpointsQuery200ResponseInnerAnyOf1 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQuery200ResponseInnerAnyOf1 {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("type".to_string()),
            Some(self.r#type.to_string()),

            // Skipping columns in query parameter serialization

            // Skipping rows in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQuery200ResponseInnerAnyOf1 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQuery200ResponseInnerAnyOf1 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub columns: Vec<Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>>,
            pub rows: Vec<Vec<models::ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQuery200ResponseInnerAnyOf1".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "columns" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQuery200ResponseInnerAnyOf1".to_string()),
                    "rows" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQuery200ResponseInnerAnyOf1".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQuery200ResponseInnerAnyOf1".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQuery200ResponseInnerAnyOf1 {
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ApiEndpointsQuery200ResponseInnerAnyOf1".to_string())?,
            columns: intermediate_rep.columns.into_iter().next().ok_or_else(|| "columns missing in ApiEndpointsQuery200ResponseInnerAnyOf1".to_string())?,
            rows: intermediate_rep.rows.into_iter().next().ok_or_else(|| "rows missing in ApiEndpointsQuery200ResponseInnerAnyOf1".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQuery200ResponseInnerAnyOf1 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQuery200ResponseInnerAnyOf1 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQuery200ResponseInnerAnyOf1 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,

}

impl ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner {
    #[allow(clippy::new_without_default)]
    pub fn new(text: String, ) -> ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner {
        ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner {
            text,
            r#type: None,
        }
    }
}

/// Converts the ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("text".to_string()),
            Some(self.text.to_string()),


            self.r#type.as_ref().map(|r#type| {
                vec![
                    "type".to_string(),
                    r#type.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text: Vec<String>,
            pub r#type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner {
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQuery200ResponseInnerAnyOf1ColumnsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner {
}

impl ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner {
        ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner {
        }
    }
}

/// Converts the ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQuery200ResponseInnerAnyOf1RowsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQueryRequest {
    #[serde(rename = "panelId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub panel_id: Option<models::ApiEndpointsQueryRequestPanelId>,

    #[serde(rename = "range")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range: Option<models::ApiEndpointsQueryRequestRange>,

    #[serde(rename = "rangeRaw")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range_raw: Option<models::RawTimeFrame>,

    #[serde(rename = "interval")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interval: Option<String>,

    #[serde(rename = "intervalMs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interval_ms: Option<f64>,

    #[serde(rename = "maxDataPoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_data_points: Option<f64>,

    #[serde(rename = "targets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub targets: Option<Vec<models::ApiEndpointsQueryRequestTargetsInner>>,

    #[serde(rename = "scopedVars")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub scoped_vars: Option<serde_json::Value>,

    #[serde(rename = "adhocFilters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub adhoc_filters: Option<Vec<models::ApiEndpointsQueryRequestAdhocFiltersInner>>,

}

impl ApiEndpointsQueryRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsQueryRequest {
        ApiEndpointsQueryRequest {
            panel_id: None,
            range: None,
            range_raw: None,
            interval: None,
            interval_ms: None,
            max_data_points: None,
            targets: None,
            scoped_vars: None,
            adhoc_filters: None,
        }
    }
}

/// Converts the ApiEndpointsQueryRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQueryRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping panelId in query parameter serialization

            // Skipping range in query parameter serialization

            // Skipping rangeRaw in query parameter serialization


            self.interval.as_ref().map(|interval| {
                vec![
                    "interval".to_string(),
                    interval.to_string(),
                ].join(",")
            }),


            self.interval_ms.as_ref().map(|interval_ms| {
                vec![
                    "intervalMs".to_string(),
                    interval_ms.to_string(),
                ].join(",")
            }),


            self.max_data_points.as_ref().map(|max_data_points| {
                vec![
                    "maxDataPoints".to_string(),
                    max_data_points.to_string(),
                ].join(",")
            }),

            // Skipping targets in query parameter serialization

            // Skipping scopedVars in query parameter serialization

            // Skipping adhocFilters in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQueryRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQueryRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub panel_id: Vec<models::ApiEndpointsQueryRequestPanelId>,
            pub range: Vec<models::ApiEndpointsQueryRequestRange>,
            pub range_raw: Vec<models::RawTimeFrame>,
            pub interval: Vec<String>,
            pub interval_ms: Vec<f64>,
            pub max_data_points: Vec<f64>,
            pub targets: Vec<Vec<models::ApiEndpointsQueryRequestTargetsInner>>,
            pub scoped_vars: Vec<serde_json::Value>,
            pub adhoc_filters: Vec<Vec<models::ApiEndpointsQueryRequestAdhocFiltersInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQueryRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "panelId" => intermediate_rep.panel_id.push(<models::ApiEndpointsQueryRequestPanelId as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "range" => intermediate_rep.range.push(<models::ApiEndpointsQueryRequestRange as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "rangeRaw" => intermediate_rep.range_raw.push(<models::RawTimeFrame as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "interval" => intermediate_rep.interval.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "intervalMs" => intermediate_rep.interval_ms.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "maxDataPoints" => intermediate_rep.max_data_points.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "targets" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQueryRequest".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "scopedVars" => intermediate_rep.scoped_vars.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "adhocFilters" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsQueryRequest".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQueryRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQueryRequest {
            panel_id: intermediate_rep.panel_id.into_iter().next(),
            range: intermediate_rep.range.into_iter().next(),
            range_raw: intermediate_rep.range_raw.into_iter().next(),
            interval: intermediate_rep.interval.into_iter().next(),
            interval_ms: intermediate_rep.interval_ms.into_iter().next(),
            max_data_points: intermediate_rep.max_data_points.into_iter().next(),
            targets: intermediate_rep.targets.into_iter().next(),
            scoped_vars: intermediate_rep.scoped_vars.into_iter().next(),
            adhoc_filters: intermediate_rep.adhoc_filters.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQueryRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQueryRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQueryRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQueryRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQueryRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQueryRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQueryRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQueryRequestAdhocFiltersInner {
    #[serde(rename = "key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,

    #[serde(rename = "operator")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub operator: Option<String>,

    #[serde(rename = "value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,

}

impl ApiEndpointsQueryRequestAdhocFiltersInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsQueryRequestAdhocFiltersInner {
        ApiEndpointsQueryRequestAdhocFiltersInner {
            key: None,
            operator: None,
            value: None,
        }
    }
}

/// Converts the ApiEndpointsQueryRequestAdhocFiltersInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQueryRequestAdhocFiltersInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.key.as_ref().map(|key| {
                vec![
                    "key".to_string(),
                    key.to_string(),
                ].join(",")
            }),


            self.operator.as_ref().map(|operator| {
                vec![
                    "operator".to_string(),
                    operator.to_string(),
                ].join(",")
            }),


            self.value.as_ref().map(|value| {
                vec![
                    "value".to_string(),
                    value.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQueryRequestAdhocFiltersInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQueryRequestAdhocFiltersInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
            pub operator: Vec<String>,
            pub value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQueryRequestAdhocFiltersInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "operator" => intermediate_rep.operator.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "value" => intermediate_rep.value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQueryRequestAdhocFiltersInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQueryRequestAdhocFiltersInner {
            key: intermediate_rep.key.into_iter().next(),
            operator: intermediate_rep.operator.into_iter().next(),
            value: intermediate_rep.value.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQueryRequestAdhocFiltersInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQueryRequestAdhocFiltersInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQueryRequestAdhocFiltersInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQueryRequestAdhocFiltersInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQueryRequestAdhocFiltersInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQueryRequestAdhocFiltersInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQueryRequestAdhocFiltersInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQueryRequestPanelId {
}

impl ApiEndpointsQueryRequestPanelId {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsQueryRequestPanelId {
        ApiEndpointsQueryRequestPanelId {
        }
    }
}

/// Converts the ApiEndpointsQueryRequestPanelId value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQueryRequestPanelId {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQueryRequestPanelId value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQueryRequestPanelId {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQueryRequestPanelId".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQueryRequestPanelId".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQueryRequestPanelId {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQueryRequestPanelId> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQueryRequestPanelId>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQueryRequestPanelId>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQueryRequestPanelId - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQueryRequestPanelId> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQueryRequestPanelId as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQueryRequestPanelId - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQueryRequestRange {
    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<chrono::DateTime::<chrono::Utc>>,

    #[serde(rename = "raw")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub raw: Option<models::RawTimeFrame>,

}

impl ApiEndpointsQueryRequestRange {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsQueryRequestRange {
        ApiEndpointsQueryRequestRange {
            from: None,
            to: None,
            raw: None,
        }
    }
}

/// Converts the ApiEndpointsQueryRequestRange value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQueryRequestRange {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping from in query parameter serialization

            // Skipping to in query parameter serialization

            // Skipping raw in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQueryRequestRange value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQueryRequestRange {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub from: Vec<chrono::DateTime::<chrono::Utc>>,
            pub to: Vec<chrono::DateTime::<chrono::Utc>>,
            pub raw: Vec<models::RawTimeFrame>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQueryRequestRange".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "from" => intermediate_rep.from.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(<chrono::DateTime::<chrono::Utc> as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "raw" => intermediate_rep.raw.push(<models::RawTimeFrame as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQueryRequestRange".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQueryRequestRange {
            from: intermediate_rep.from.into_iter().next(),
            to: intermediate_rep.to.into_iter().next(),
            raw: intermediate_rep.raw.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQueryRequestRange> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQueryRequestRange>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQueryRequestRange>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQueryRequestRange - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQueryRequestRange> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQueryRequestRange as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQueryRequestRange - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsQueryRequestTargetsInner {
    #[serde(rename = "target")]
    pub target: String,

    #[serde(rename = "refId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ref_id: Option<String>,

    /// arbitrary \"additional data\" the user can pass in
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<serde_json::Value>,

}

impl ApiEndpointsQueryRequestTargetsInner {
    #[allow(clippy::new_without_default)]
    pub fn new(target: String, ) -> ApiEndpointsQueryRequestTargetsInner {
        ApiEndpointsQueryRequestTargetsInner {
            target,
            ref_id: None,
            payload: None,
        }
    }
}

/// Converts the ApiEndpointsQueryRequestTargetsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsQueryRequestTargetsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("target".to_string()),
            Some(self.target.to_string()),


            self.ref_id.as_ref().map(|ref_id| {
                vec![
                    "refId".to_string(),
                    ref_id.to_string(),
                ].join(",")
            }),

            // Skipping payload in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsQueryRequestTargetsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsQueryRequestTargetsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub target: Vec<String>,
            pub ref_id: Vec<String>,
            pub payload: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsQueryRequestTargetsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "target" => intermediate_rep.target.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "refId" => intermediate_rep.ref_id.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "payload" => intermediate_rep.payload.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsQueryRequestTargetsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsQueryRequestTargetsInner {
            target: intermediate_rep.target.into_iter().next().ok_or_else(|| "target missing in ApiEndpointsQueryRequestTargetsInner".to_string())?,
            ref_id: intermediate_rep.ref_id.into_iter().next(),
            payload: intermediate_rep.payload.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsQueryRequestTargetsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsQueryRequestTargetsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsQueryRequestTargetsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsQueryRequestTargetsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsQueryRequestTargetsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsQueryRequestTargetsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsQueryRequestTargetsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsTagKeys200ResponseInner {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,

    #[serde(rename = "text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub text: Option<String>,

}

impl ApiEndpointsTagKeys200ResponseInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsTagKeys200ResponseInner {
        ApiEndpointsTagKeys200ResponseInner {
            r#type: None,
            text: None,
        }
    }
}

/// Converts the ApiEndpointsTagKeys200ResponseInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsTagKeys200ResponseInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.r#type.as_ref().map(|r#type| {
                vec![
                    "type".to_string(),
                    r#type.to_string(),
                ].join(",")
            }),


            self.text.as_ref().map(|text| {
                vec![
                    "text".to_string(),
                    text.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsTagKeys200ResponseInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsTagKeys200ResponseInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub r#type: Vec<String>,
            pub text: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsTagKeys200ResponseInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsTagKeys200ResponseInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsTagKeys200ResponseInner {
            r#type: intermediate_rep.r#type.into_iter().next(),
            text: intermediate_rep.text.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsTagKeys200ResponseInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsTagKeys200ResponseInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsTagKeys200ResponseInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsTagKeys200ResponseInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsTagKeys200ResponseInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsTagKeys200ResponseInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsTagKeys200ResponseInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsTagValuesRequest {
    #[serde(rename = "key")]
    pub key: String,

}

impl ApiEndpointsTagValuesRequest {
    #[allow(clippy::new_without_default)]
    pub fn new(key: String, ) -> ApiEndpointsTagValuesRequest {
        ApiEndpointsTagValuesRequest {
            key,
        }
    }
}

/// Converts the ApiEndpointsTagValuesRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsTagValuesRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("key".to_string()),
            Some(self.key.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsTagValuesRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsTagValuesRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub key: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsTagValuesRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "key" => intermediate_rep.key.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsTagValuesRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsTagValuesRequest {
            key: intermediate_rep.key.into_iter().next().ok_or_else(|| "key missing in ApiEndpointsTagValuesRequest".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsTagValuesRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsTagValuesRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsTagValuesRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsTagValuesRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsTagValuesRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsTagValuesRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsTagValuesRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsVariable200Response {
    #[serde(rename = "fields")]
    pub fields: Vec<models::DataframeFieldsInner>,

}

impl ApiEndpointsVariable200Response {
    #[allow(clippy::new_without_default)]
    pub fn new(fields: Vec<models::DataframeFieldsInner>, ) -> ApiEndpointsVariable200Response {
        ApiEndpointsVariable200Response {
            fields,
        }
    }
}

/// Converts the ApiEndpointsVariable200Response value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsVariable200Response {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping fields in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsVariable200Response value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsVariable200Response {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub fields: Vec<Vec<models::DataframeFieldsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsVariable200Response".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in ApiEndpointsVariable200Response".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsVariable200Response".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsVariable200Response {
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in ApiEndpointsVariable200Response".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsVariable200Response> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsVariable200Response>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsVariable200Response>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsVariable200Response - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsVariable200Response> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsVariable200Response as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsVariable200Response - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsVariable200ResponseOneOfInner {
    #[serde(rename = "__text")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub __text: Option<String>,

    #[serde(rename = "__value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub __value: Option<String>,

}

impl ApiEndpointsVariable200ResponseOneOfInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsVariable200ResponseOneOfInner {
        ApiEndpointsVariable200ResponseOneOfInner {
            __text: None,
            __value: None,
        }
    }
}

/// Converts the ApiEndpointsVariable200ResponseOneOfInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsVariable200ResponseOneOfInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.__text.as_ref().map(|__text| {
                vec![
                    "__text".to_string(),
                    __text.to_string(),
                ].join(",")
            }),


            self.__value.as_ref().map(|__value| {
                vec![
                    "__value".to_string(),
                    __value.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsVariable200ResponseOneOfInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsVariable200ResponseOneOfInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub __text: Vec<String>,
            pub __value: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsVariable200ResponseOneOfInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "__text" => intermediate_rep.__text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "__value" => intermediate_rep.__value.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsVariable200ResponseOneOfInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsVariable200ResponseOneOfInner {
            __text: intermediate_rep.__text.into_iter().next(),
            __value: intermediate_rep.__value.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsVariable200ResponseOneOfInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsVariable200ResponseOneOfInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsVariable200ResponseOneOfInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsVariable200ResponseOneOfInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsVariable200ResponseOneOfInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsVariable200ResponseOneOfInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsVariable200ResponseOneOfInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsVariableRequest {
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<models::ApiEndpointsVariableRequestPayload>,

    #[serde(rename = "range")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub range: Option<models::ApiEndpointsQueryRequestRange>,

}

impl ApiEndpointsVariableRequest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ApiEndpointsVariableRequest {
        ApiEndpointsVariableRequest {
            payload: None,
            range: None,
        }
    }
}

/// Converts the ApiEndpointsVariableRequest value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsVariableRequest {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping payload in query parameter serialization

            // Skipping range in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsVariableRequest value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsVariableRequest {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub payload: Vec<models::ApiEndpointsVariableRequestPayload>,
            pub range: Vec<models::ApiEndpointsQueryRequestRange>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsVariableRequest".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "payload" => intermediate_rep.payload.push(<models::ApiEndpointsVariableRequestPayload as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "range" => intermediate_rep.range.push(<models::ApiEndpointsQueryRequestRange as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsVariableRequest".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsVariableRequest {
            payload: intermediate_rep.payload.into_iter().next(),
            range: intermediate_rep.range.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsVariableRequest> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsVariableRequest>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsVariableRequest>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsVariableRequest - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsVariableRequest> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsVariableRequest as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsVariableRequest - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ApiEndpointsVariableRequestPayload {
    #[serde(rename = "target")]
    pub target: String,

    #[serde(rename = "variables")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub variables: Option<serde_json::Value>,

}

impl ApiEndpointsVariableRequestPayload {
    #[allow(clippy::new_without_default)]
    pub fn new(target: String, ) -> ApiEndpointsVariableRequestPayload {
        ApiEndpointsVariableRequestPayload {
            target,
            variables: None,
        }
    }
}

/// Converts the ApiEndpointsVariableRequestPayload value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ApiEndpointsVariableRequestPayload {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("target".to_string()),
            Some(self.target.to_string()),

            // Skipping variables in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ApiEndpointsVariableRequestPayload value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ApiEndpointsVariableRequestPayload {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub target: Vec<String>,
            pub variables: Vec<serde_json::Value>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ApiEndpointsVariableRequestPayload".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "target" => intermediate_rep.target.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "variables" => intermediate_rep.variables.push(<serde_json::Value as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ApiEndpointsVariableRequestPayload".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ApiEndpointsVariableRequestPayload {
            target: intermediate_rep.target.into_iter().next().ok_or_else(|| "target missing in ApiEndpointsVariableRequestPayload".to_string())?,
            variables: intermediate_rep.variables.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ApiEndpointsVariableRequestPayload> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ApiEndpointsVariableRequestPayload>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ApiEndpointsVariableRequestPayload>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ApiEndpointsVariableRequestPayload - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ApiEndpointsVariableRequestPayload> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ApiEndpointsVariableRequestPayload as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ApiEndpointsVariableRequestPayload - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Dataframe {
    #[serde(rename = "fields")]
    pub fields: Vec<models::DataframeFieldsInner>,

}

impl Dataframe {
    #[allow(clippy::new_without_default)]
    pub fn new(fields: Vec<models::DataframeFieldsInner>, ) -> Dataframe {
        Dataframe {
            fields,
        }
    }
}

/// Converts the Dataframe value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Dataframe {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping fields in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Dataframe value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Dataframe {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub fields: Vec<Vec<models::DataframeFieldsInner>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Dataframe".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "fields" => return std::result::Result::Err("Parsing a container in this style is not supported in Dataframe".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Dataframe".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Dataframe {
            fields: intermediate_rep.fields.into_iter().next().ok_or_else(|| "fields missing in Dataframe".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Dataframe> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Dataframe>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Dataframe>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Dataframe - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Dataframe> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Dataframe as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Dataframe - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DataframeFieldsInner {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "values")]
    pub values: Vec<serde_json::Value>,

}

impl DataframeFieldsInner {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, values: Vec<serde_json::Value>, ) -> DataframeFieldsInner {
        DataframeFieldsInner {
            name,
            values,
        }
    }
}

/// Converts the DataframeFieldsInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DataframeFieldsInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),

            // Skipping values in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DataframeFieldsInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DataframeFieldsInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub values: Vec<Vec<serde_json::Value>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DataframeFieldsInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "values" => return std::result::Result::Err("Parsing a container in this style is not supported in DataframeFieldsInner".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DataframeFieldsInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DataframeFieldsInner {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in DataframeFieldsInner".to_string())?,
            values: intermediate_rep.values.into_iter().next().ok_or_else(|| "values missing in DataframeFieldsInner".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DataframeFieldsInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DataframeFieldsInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DataframeFieldsInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DataframeFieldsInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DataframeFieldsInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DataframeFieldsInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DataframeFieldsInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MetricObj {
    #[serde(rename = "metric")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub metric: Option<String>,

    #[serde(rename = "payload")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub payload: Option<models::Payload>,

}

impl MetricObj {
    #[allow(clippy::new_without_default)]
    pub fn new() -> MetricObj {
        MetricObj {
            metric: None,
            payload: None,
        }
    }
}

/// Converts the MetricObj value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MetricObj {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.metric.as_ref().map(|metric| {
                vec![
                    "metric".to_string(),
                    metric.to_string(),
                ].join(",")
            }),

            // Skipping payload in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MetricObj value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MetricObj {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub metric: Vec<String>,
            pub payload: Vec<models::Payload>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MetricObj".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "metric" => intermediate_rep.metric.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "payload" => intermediate_rep.payload.push(<models::Payload as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MetricObj".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MetricObj {
            metric: intermediate_rep.metric.into_iter().next(),
            payload: intermediate_rep.payload.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MetricObj> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MetricObj>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MetricObj>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MetricObj - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MetricObj> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MetricObj as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MetricObj - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


/// The currently selected/entered payload options and values. Key is the name of the payload, and value is the value of the payload.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Payload {
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub namespace: Option<String>,

}

impl Payload {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Payload {
        Payload {
            namespace: None,
        }
    }
}

/// Converts the Payload value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Payload {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.namespace.as_ref().map(|namespace| {
                vec![
                    "namespace".to_string(),
                    namespace.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Payload value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Payload {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub namespace: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Payload".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "namespace" => intermediate_rep.namespace.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Payload".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Payload {
            namespace: intermediate_rep.namespace.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Payload> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Payload>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Payload>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Payload - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Payload> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Payload as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Payload - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct RawTimeFrame {
    #[serde(rename = "from")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub from: Option<String>,

    #[serde(rename = "to")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub to: Option<String>,

}

impl RawTimeFrame {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RawTimeFrame {
        RawTimeFrame {
            from: None,
            to: None,
        }
    }
}

/// Converts the RawTimeFrame value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for RawTimeFrame {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.from.as_ref().map(|from| {
                vec![
                    "from".to_string(),
                    from.to_string(),
                ].join(",")
            }),


            self.to.as_ref().map(|to| {
                vec![
                    "to".to_string(),
                    to.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a RawTimeFrame value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for RawTimeFrame {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub from: Vec<String>,
            pub to: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing RawTimeFrame".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "from" => intermediate_rep.from.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "to" => intermediate_rep.to.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing RawTimeFrame".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(RawTimeFrame {
            from: intermediate_rep.from.into_iter().next(),
            to: intermediate_rep.to.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<RawTimeFrame> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<RawTimeFrame>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<RawTimeFrame>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for RawTimeFrame - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<RawTimeFrame> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <RawTimeFrame as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into RawTimeFrame - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

