// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AddTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_tags`](crate::client::Client::add_tags).
///
/// See [`crate::client::fluent_builders::AddTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct AddTags {
    _private: (),
}
impl AddTags {
    /// Creates a new builder-style object to manufacture [`AddTagsInput`](crate::input::AddTagsInput)
    pub fn builder() -> crate::input::add_tags_input::Builder {
        crate::input::add_tags_input::Builder::default()
    }
    /// Creates a new `AddTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddTags {
    type Output = std::result::Result<crate::output::AddTagsOutput, crate::error::AddTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_tags_error(response)
        } else {
            crate::operation_deser::parse_add_tags_response(response)
        }
    }
}

/// Operation shape for `CancelQuery`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_query`](crate::client::Client::cancel_query).
///
/// See [`crate::client::fluent_builders::CancelQuery`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelQuery {
    _private: (),
}
impl CancelQuery {
    /// Creates a new builder-style object to manufacture [`CancelQueryInput`](crate::input::CancelQueryInput)
    pub fn builder() -> crate::input::cancel_query_input::Builder {
        crate::input::cancel_query_input::Builder::default()
    }
    /// Creates a new `CancelQuery` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelQuery {
    type Output =
        std::result::Result<crate::output::CancelQueryOutput, crate::error::CancelQueryError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_cancel_query_error(response)
        } else {
            crate::operation_deser::parse_cancel_query_response(response)
        }
    }
}

/// Operation shape for `CreateEventDataStore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_event_data_store`](crate::client::Client::create_event_data_store).
///
/// See [`crate::client::fluent_builders::CreateEventDataStore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateEventDataStore {
    _private: (),
}
impl CreateEventDataStore {
    /// Creates a new builder-style object to manufacture [`CreateEventDataStoreInput`](crate::input::CreateEventDataStoreInput)
    pub fn builder() -> crate::input::create_event_data_store_input::Builder {
        crate::input::create_event_data_store_input::Builder::default()
    }
    /// Creates a new `CreateEventDataStore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEventDataStore {
    type Output = std::result::Result<
        crate::output::CreateEventDataStoreOutput,
        crate::error::CreateEventDataStoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_event_data_store_error(response)
        } else {
            crate::operation_deser::parse_create_event_data_store_response(response)
        }
    }
}

/// Operation shape for `CreateTrail`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_trail`](crate::client::Client::create_trail).
///
/// See [`crate::client::fluent_builders::CreateTrail`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateTrail {
    _private: (),
}
impl CreateTrail {
    /// Creates a new builder-style object to manufacture [`CreateTrailInput`](crate::input::CreateTrailInput)
    pub fn builder() -> crate::input::create_trail_input::Builder {
        crate::input::create_trail_input::Builder::default()
    }
    /// Creates a new `CreateTrail` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateTrail {
    type Output =
        std::result::Result<crate::output::CreateTrailOutput, crate::error::CreateTrailError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_trail_error(response)
        } else {
            crate::operation_deser::parse_create_trail_response(response)
        }
    }
}

/// Operation shape for `DeleteEventDataStore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_event_data_store`](crate::client::Client::delete_event_data_store).
///
/// See [`crate::client::fluent_builders::DeleteEventDataStore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEventDataStore {
    _private: (),
}
impl DeleteEventDataStore {
    /// Creates a new builder-style object to manufacture [`DeleteEventDataStoreInput`](crate::input::DeleteEventDataStoreInput)
    pub fn builder() -> crate::input::delete_event_data_store_input::Builder {
        crate::input::delete_event_data_store_input::Builder::default()
    }
    /// Creates a new `DeleteEventDataStore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEventDataStore {
    type Output = std::result::Result<
        crate::output::DeleteEventDataStoreOutput,
        crate::error::DeleteEventDataStoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_event_data_store_error(response)
        } else {
            crate::operation_deser::parse_delete_event_data_store_response(response)
        }
    }
}

/// Operation shape for `DeleteTrail`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_trail`](crate::client::Client::delete_trail).
///
/// See [`crate::client::fluent_builders::DeleteTrail`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteTrail {
    _private: (),
}
impl DeleteTrail {
    /// Creates a new builder-style object to manufacture [`DeleteTrailInput`](crate::input::DeleteTrailInput)
    pub fn builder() -> crate::input::delete_trail_input::Builder {
        crate::input::delete_trail_input::Builder::default()
    }
    /// Creates a new `DeleteTrail` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteTrail {
    type Output =
        std::result::Result<crate::output::DeleteTrailOutput, crate::error::DeleteTrailError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_trail_error(response)
        } else {
            crate::operation_deser::parse_delete_trail_response(response)
        }
    }
}

/// Operation shape for `DescribeQuery`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_query`](crate::client::Client::describe_query).
///
/// See [`crate::client::fluent_builders::DescribeQuery`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeQuery {
    _private: (),
}
impl DescribeQuery {
    /// Creates a new builder-style object to manufacture [`DescribeQueryInput`](crate::input::DescribeQueryInput)
    pub fn builder() -> crate::input::describe_query_input::Builder {
        crate::input::describe_query_input::Builder::default()
    }
    /// Creates a new `DescribeQuery` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeQuery {
    type Output =
        std::result::Result<crate::output::DescribeQueryOutput, crate::error::DescribeQueryError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_query_error(response)
        } else {
            crate::operation_deser::parse_describe_query_response(response)
        }
    }
}

/// Operation shape for `DescribeTrails`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_trails`](crate::client::Client::describe_trails).
///
/// See [`crate::client::fluent_builders::DescribeTrails`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTrails {
    _private: (),
}
impl DescribeTrails {
    /// Creates a new builder-style object to manufacture [`DescribeTrailsInput`](crate::input::DescribeTrailsInput)
    pub fn builder() -> crate::input::describe_trails_input::Builder {
        crate::input::describe_trails_input::Builder::default()
    }
    /// Creates a new `DescribeTrails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTrails {
    type Output =
        std::result::Result<crate::output::DescribeTrailsOutput, crate::error::DescribeTrailsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_trails_error(response)
        } else {
            crate::operation_deser::parse_describe_trails_response(response)
        }
    }
}

/// Operation shape for `GetEventDataStore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_event_data_store`](crate::client::Client::get_event_data_store).
///
/// See [`crate::client::fluent_builders::GetEventDataStore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEventDataStore {
    _private: (),
}
impl GetEventDataStore {
    /// Creates a new builder-style object to manufacture [`GetEventDataStoreInput`](crate::input::GetEventDataStoreInput)
    pub fn builder() -> crate::input::get_event_data_store_input::Builder {
        crate::input::get_event_data_store_input::Builder::default()
    }
    /// Creates a new `GetEventDataStore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEventDataStore {
    type Output = std::result::Result<
        crate::output::GetEventDataStoreOutput,
        crate::error::GetEventDataStoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_event_data_store_error(response)
        } else {
            crate::operation_deser::parse_get_event_data_store_response(response)
        }
    }
}

/// Operation shape for `GetEventSelectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_event_selectors`](crate::client::Client::get_event_selectors).
///
/// See [`crate::client::fluent_builders::GetEventSelectors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEventSelectors {
    _private: (),
}
impl GetEventSelectors {
    /// Creates a new builder-style object to manufacture [`GetEventSelectorsInput`](crate::input::GetEventSelectorsInput)
    pub fn builder() -> crate::input::get_event_selectors_input::Builder {
        crate::input::get_event_selectors_input::Builder::default()
    }
    /// Creates a new `GetEventSelectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEventSelectors {
    type Output = std::result::Result<
        crate::output::GetEventSelectorsOutput,
        crate::error::GetEventSelectorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_event_selectors_error(response)
        } else {
            crate::operation_deser::parse_get_event_selectors_response(response)
        }
    }
}

/// Operation shape for `GetInsightSelectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_insight_selectors`](crate::client::Client::get_insight_selectors).
///
/// See [`crate::client::fluent_builders::GetInsightSelectors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetInsightSelectors {
    _private: (),
}
impl GetInsightSelectors {
    /// Creates a new builder-style object to manufacture [`GetInsightSelectorsInput`](crate::input::GetInsightSelectorsInput)
    pub fn builder() -> crate::input::get_insight_selectors_input::Builder {
        crate::input::get_insight_selectors_input::Builder::default()
    }
    /// Creates a new `GetInsightSelectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetInsightSelectors {
    type Output = std::result::Result<
        crate::output::GetInsightSelectorsOutput,
        crate::error::GetInsightSelectorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_insight_selectors_error(response)
        } else {
            crate::operation_deser::parse_get_insight_selectors_response(response)
        }
    }
}

/// Operation shape for `GetQueryResults`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_query_results`](crate::client::Client::get_query_results).
///
/// See [`crate::client::fluent_builders::GetQueryResults`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetQueryResults {
    _private: (),
}
impl GetQueryResults {
    /// Creates a new builder-style object to manufacture [`GetQueryResultsInput`](crate::input::GetQueryResultsInput)
    pub fn builder() -> crate::input::get_query_results_input::Builder {
        crate::input::get_query_results_input::Builder::default()
    }
    /// Creates a new `GetQueryResults` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetQueryResults {
    type Output = std::result::Result<
        crate::output::GetQueryResultsOutput,
        crate::error::GetQueryResultsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_query_results_error(response)
        } else {
            crate::operation_deser::parse_get_query_results_response(response)
        }
    }
}

/// Operation shape for `GetTrail`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_trail`](crate::client::Client::get_trail).
///
/// See [`crate::client::fluent_builders::GetTrail`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTrail {
    _private: (),
}
impl GetTrail {
    /// Creates a new builder-style object to manufacture [`GetTrailInput`](crate::input::GetTrailInput)
    pub fn builder() -> crate::input::get_trail_input::Builder {
        crate::input::get_trail_input::Builder::default()
    }
    /// Creates a new `GetTrail` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTrail {
    type Output = std::result::Result<crate::output::GetTrailOutput, crate::error::GetTrailError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_trail_error(response)
        } else {
            crate::operation_deser::parse_get_trail_response(response)
        }
    }
}

/// Operation shape for `GetTrailStatus`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_trail_status`](crate::client::Client::get_trail_status).
///
/// See [`crate::client::fluent_builders::GetTrailStatus`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetTrailStatus {
    _private: (),
}
impl GetTrailStatus {
    /// Creates a new builder-style object to manufacture [`GetTrailStatusInput`](crate::input::GetTrailStatusInput)
    pub fn builder() -> crate::input::get_trail_status_input::Builder {
        crate::input::get_trail_status_input::Builder::default()
    }
    /// Creates a new `GetTrailStatus` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetTrailStatus {
    type Output =
        std::result::Result<crate::output::GetTrailStatusOutput, crate::error::GetTrailStatusError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_trail_status_error(response)
        } else {
            crate::operation_deser::parse_get_trail_status_response(response)
        }
    }
}

/// Operation shape for `ListEventDataStores`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_event_data_stores`](crate::client::Client::list_event_data_stores).
///
/// See [`crate::client::fluent_builders::ListEventDataStores`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEventDataStores {
    _private: (),
}
impl ListEventDataStores {
    /// Creates a new builder-style object to manufacture [`ListEventDataStoresInput`](crate::input::ListEventDataStoresInput)
    pub fn builder() -> crate::input::list_event_data_stores_input::Builder {
        crate::input::list_event_data_stores_input::Builder::default()
    }
    /// Creates a new `ListEventDataStores` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEventDataStores {
    type Output = std::result::Result<
        crate::output::ListEventDataStoresOutput,
        crate::error::ListEventDataStoresError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_event_data_stores_error(response)
        } else {
            crate::operation_deser::parse_list_event_data_stores_response(response)
        }
    }
}

/// Operation shape for `ListPublicKeys`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_public_keys`](crate::client::Client::list_public_keys).
///
/// See [`crate::client::fluent_builders::ListPublicKeys`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListPublicKeys {
    _private: (),
}
impl ListPublicKeys {
    /// Creates a new builder-style object to manufacture [`ListPublicKeysInput`](crate::input::ListPublicKeysInput)
    pub fn builder() -> crate::input::list_public_keys_input::Builder {
        crate::input::list_public_keys_input::Builder::default()
    }
    /// Creates a new `ListPublicKeys` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListPublicKeys {
    type Output =
        std::result::Result<crate::output::ListPublicKeysOutput, crate::error::ListPublicKeysError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_public_keys_error(response)
        } else {
            crate::operation_deser::parse_list_public_keys_response(response)
        }
    }
}

/// Operation shape for `ListQueries`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_queries`](crate::client::Client::list_queries).
///
/// See [`crate::client::fluent_builders::ListQueries`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListQueries {
    _private: (),
}
impl ListQueries {
    /// Creates a new builder-style object to manufacture [`ListQueriesInput`](crate::input::ListQueriesInput)
    pub fn builder() -> crate::input::list_queries_input::Builder {
        crate::input::list_queries_input::Builder::default()
    }
    /// Creates a new `ListQueries` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListQueries {
    type Output =
        std::result::Result<crate::output::ListQueriesOutput, crate::error::ListQueriesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_queries_error(response)
        } else {
            crate::operation_deser::parse_list_queries_response(response)
        }
    }
}

/// Operation shape for `ListTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags`](crate::client::Client::list_tags).
///
/// See [`crate::client::fluent_builders::ListTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTags {
    _private: (),
}
impl ListTags {
    /// Creates a new builder-style object to manufacture [`ListTagsInput`](crate::input::ListTagsInput)
    pub fn builder() -> crate::input::list_tags_input::Builder {
        crate::input::list_tags_input::Builder::default()
    }
    /// Creates a new `ListTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTags {
    type Output = std::result::Result<crate::output::ListTagsOutput, crate::error::ListTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_error(response)
        } else {
            crate::operation_deser::parse_list_tags_response(response)
        }
    }
}

/// Operation shape for `ListTrails`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_trails`](crate::client::Client::list_trails).
///
/// See [`crate::client::fluent_builders::ListTrails`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTrails {
    _private: (),
}
impl ListTrails {
    /// Creates a new builder-style object to manufacture [`ListTrailsInput`](crate::input::ListTrailsInput)
    pub fn builder() -> crate::input::list_trails_input::Builder {
        crate::input::list_trails_input::Builder::default()
    }
    /// Creates a new `ListTrails` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTrails {
    type Output =
        std::result::Result<crate::output::ListTrailsOutput, crate::error::ListTrailsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_trails_error(response)
        } else {
            crate::operation_deser::parse_list_trails_response(response)
        }
    }
}

/// Operation shape for `LookupEvents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`lookup_events`](crate::client::Client::lookup_events).
///
/// See [`crate::client::fluent_builders::LookupEvents`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct LookupEvents {
    _private: (),
}
impl LookupEvents {
    /// Creates a new builder-style object to manufacture [`LookupEventsInput`](crate::input::LookupEventsInput)
    pub fn builder() -> crate::input::lookup_events_input::Builder {
        crate::input::lookup_events_input::Builder::default()
    }
    /// Creates a new `LookupEvents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for LookupEvents {
    type Output =
        std::result::Result<crate::output::LookupEventsOutput, crate::error::LookupEventsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_lookup_events_error(response)
        } else {
            crate::operation_deser::parse_lookup_events_response(response)
        }
    }
}

/// Operation shape for `PutEventSelectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_event_selectors`](crate::client::Client::put_event_selectors).
///
/// See [`crate::client::fluent_builders::PutEventSelectors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutEventSelectors {
    _private: (),
}
impl PutEventSelectors {
    /// Creates a new builder-style object to manufacture [`PutEventSelectorsInput`](crate::input::PutEventSelectorsInput)
    pub fn builder() -> crate::input::put_event_selectors_input::Builder {
        crate::input::put_event_selectors_input::Builder::default()
    }
    /// Creates a new `PutEventSelectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutEventSelectors {
    type Output = std::result::Result<
        crate::output::PutEventSelectorsOutput,
        crate::error::PutEventSelectorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_event_selectors_error(response)
        } else {
            crate::operation_deser::parse_put_event_selectors_response(response)
        }
    }
}

/// Operation shape for `PutInsightSelectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`put_insight_selectors`](crate::client::Client::put_insight_selectors).
///
/// See [`crate::client::fluent_builders::PutInsightSelectors`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct PutInsightSelectors {
    _private: (),
}
impl PutInsightSelectors {
    /// Creates a new builder-style object to manufacture [`PutInsightSelectorsInput`](crate::input::PutInsightSelectorsInput)
    pub fn builder() -> crate::input::put_insight_selectors_input::Builder {
        crate::input::put_insight_selectors_input::Builder::default()
    }
    /// Creates a new `PutInsightSelectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for PutInsightSelectors {
    type Output = std::result::Result<
        crate::output::PutInsightSelectorsOutput,
        crate::error::PutInsightSelectorsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_put_insight_selectors_error(response)
        } else {
            crate::operation_deser::parse_put_insight_selectors_response(response)
        }
    }
}

/// Operation shape for `RemoveTags`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`remove_tags`](crate::client::Client::remove_tags).
///
/// See [`crate::client::fluent_builders::RemoveTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RemoveTags {
    _private: (),
}
impl RemoveTags {
    /// Creates a new builder-style object to manufacture [`RemoveTagsInput`](crate::input::RemoveTagsInput)
    pub fn builder() -> crate::input::remove_tags_input::Builder {
        crate::input::remove_tags_input::Builder::default()
    }
    /// Creates a new `RemoveTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RemoveTags {
    type Output =
        std::result::Result<crate::output::RemoveTagsOutput, crate::error::RemoveTagsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_remove_tags_error(response)
        } else {
            crate::operation_deser::parse_remove_tags_response(response)
        }
    }
}

/// Operation shape for `RestoreEventDataStore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`restore_event_data_store`](crate::client::Client::restore_event_data_store).
///
/// See [`crate::client::fluent_builders::RestoreEventDataStore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct RestoreEventDataStore {
    _private: (),
}
impl RestoreEventDataStore {
    /// Creates a new builder-style object to manufacture [`RestoreEventDataStoreInput`](crate::input::RestoreEventDataStoreInput)
    pub fn builder() -> crate::input::restore_event_data_store_input::Builder {
        crate::input::restore_event_data_store_input::Builder::default()
    }
    /// Creates a new `RestoreEventDataStore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RestoreEventDataStore {
    type Output = std::result::Result<
        crate::output::RestoreEventDataStoreOutput,
        crate::error::RestoreEventDataStoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_restore_event_data_store_error(response)
        } else {
            crate::operation_deser::parse_restore_event_data_store_response(response)
        }
    }
}

/// Operation shape for `StartLogging`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_logging`](crate::client::Client::start_logging).
///
/// See [`crate::client::fluent_builders::StartLogging`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartLogging {
    _private: (),
}
impl StartLogging {
    /// Creates a new builder-style object to manufacture [`StartLoggingInput`](crate::input::StartLoggingInput)
    pub fn builder() -> crate::input::start_logging_input::Builder {
        crate::input::start_logging_input::Builder::default()
    }
    /// Creates a new `StartLogging` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartLogging {
    type Output =
        std::result::Result<crate::output::StartLoggingOutput, crate::error::StartLoggingError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_logging_error(response)
        } else {
            crate::operation_deser::parse_start_logging_response(response)
        }
    }
}

/// Operation shape for `StartQuery`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_query`](crate::client::Client::start_query).
///
/// See [`crate::client::fluent_builders::StartQuery`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartQuery {
    _private: (),
}
impl StartQuery {
    /// Creates a new builder-style object to manufacture [`StartQueryInput`](crate::input::StartQueryInput)
    pub fn builder() -> crate::input::start_query_input::Builder {
        crate::input::start_query_input::Builder::default()
    }
    /// Creates a new `StartQuery` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartQuery {
    type Output =
        std::result::Result<crate::output::StartQueryOutput, crate::error::StartQueryError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_query_error(response)
        } else {
            crate::operation_deser::parse_start_query_response(response)
        }
    }
}

/// Operation shape for `StopLogging`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`stop_logging`](crate::client::Client::stop_logging).
///
/// See [`crate::client::fluent_builders::StopLogging`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StopLogging {
    _private: (),
}
impl StopLogging {
    /// Creates a new builder-style object to manufacture [`StopLoggingInput`](crate::input::StopLoggingInput)
    pub fn builder() -> crate::input::stop_logging_input::Builder {
        crate::input::stop_logging_input::Builder::default()
    }
    /// Creates a new `StopLogging` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StopLogging {
    type Output =
        std::result::Result<crate::output::StopLoggingOutput, crate::error::StopLoggingError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_stop_logging_error(response)
        } else {
            crate::operation_deser::parse_stop_logging_response(response)
        }
    }
}

/// Operation shape for `UpdateEventDataStore`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_event_data_store`](crate::client::Client::update_event_data_store).
///
/// See [`crate::client::fluent_builders::UpdateEventDataStore`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateEventDataStore {
    _private: (),
}
impl UpdateEventDataStore {
    /// Creates a new builder-style object to manufacture [`UpdateEventDataStoreInput`](crate::input::UpdateEventDataStoreInput)
    pub fn builder() -> crate::input::update_event_data_store_input::Builder {
        crate::input::update_event_data_store_input::Builder::default()
    }
    /// Creates a new `UpdateEventDataStore` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateEventDataStore {
    type Output = std::result::Result<
        crate::output::UpdateEventDataStoreOutput,
        crate::error::UpdateEventDataStoreError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_event_data_store_error(response)
        } else {
            crate::operation_deser::parse_update_event_data_store_response(response)
        }
    }
}

/// Operation shape for `UpdateTrail`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_trail`](crate::client::Client::update_trail).
///
/// See [`crate::client::fluent_builders::UpdateTrail`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateTrail {
    _private: (),
}
impl UpdateTrail {
    /// Creates a new builder-style object to manufacture [`UpdateTrailInput`](crate::input::UpdateTrailInput)
    pub fn builder() -> crate::input::update_trail_input::Builder {
        crate::input::update_trail_input::Builder::default()
    }
    /// Creates a new `UpdateTrail` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateTrail {
    type Output =
        std::result::Result<crate::output::UpdateTrailOutput, crate::error::UpdateTrailError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_trail_error(response)
        } else {
            crate::operation_deser::parse_update_trail_response(response)
        }
    }
}
