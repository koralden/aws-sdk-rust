// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `DeleteObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DeleteObjectError {
    /// Kind of error that occurred.
    pub kind: DeleteObjectErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DeleteObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DeleteObjectErrorKind {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFoundException(crate::error::ContainerNotFoundException),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DeleteObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DeleteObjectErrorKind::ContainerNotFoundException(_inner) => _inner.fmt(f),
            DeleteObjectErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            DeleteObjectErrorKind::ObjectNotFoundException(_inner) => _inner.fmt(f),
            DeleteObjectErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DeleteObjectError {
    fn code(&self) -> Option<&str> {
        DeleteObjectError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DeleteObjectError {
    /// Creates a new `DeleteObjectError`.
    pub fn new(kind: DeleteObjectErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DeleteObjectError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DeleteObjectErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DeleteObjectError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DeleteObjectErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DeleteObjectErrorKind::ContainerNotFoundException`.
    pub fn is_container_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteObjectErrorKind::ContainerNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `DeleteObjectErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, DeleteObjectErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `DeleteObjectErrorKind::ObjectNotFoundException`.
    pub fn is_object_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DeleteObjectErrorKind::ObjectNotFoundException(_)
        )
    }
}
impl std::error::Error for DeleteObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DeleteObjectErrorKind::ContainerNotFoundException(_inner) => Some(_inner),
            DeleteObjectErrorKind::InternalServerError(_inner) => Some(_inner),
            DeleteObjectErrorKind::ObjectNotFoundException(_inner) => Some(_inner),
            DeleteObjectErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `DescribeObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct DescribeObjectError {
    /// Kind of error that occurred.
    pub kind: DescribeObjectErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `DescribeObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum DescribeObjectErrorKind {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFoundException(crate::error::ContainerNotFoundException),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeObjectErrorKind::ContainerNotFoundException(_inner) => _inner.fmt(f),
            DescribeObjectErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            DescribeObjectErrorKind::ObjectNotFoundException(_inner) => _inner.fmt(f),
            DescribeObjectErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeObjectError {
    fn code(&self) -> Option<&str> {
        DescribeObjectError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeObjectError {
    /// Creates a new `DescribeObjectError`.
    pub fn new(kind: DescribeObjectErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `DescribeObjectError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: DescribeObjectErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `DescribeObjectError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: DescribeObjectErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `DescribeObjectErrorKind::ContainerNotFoundException`.
    pub fn is_container_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeObjectErrorKind::ContainerNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `DescribeObjectErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, DescribeObjectErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `DescribeObjectErrorKind::ObjectNotFoundException`.
    pub fn is_object_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            DescribeObjectErrorKind::ObjectNotFoundException(_)
        )
    }
}
impl std::error::Error for DescribeObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeObjectErrorKind::ContainerNotFoundException(_inner) => Some(_inner),
            DescribeObjectErrorKind::InternalServerError(_inner) => Some(_inner),
            DescribeObjectErrorKind::ObjectNotFoundException(_inner) => Some(_inner),
            DescribeObjectErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `GetObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct GetObjectError {
    /// Kind of error that occurred.
    pub kind: GetObjectErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `GetObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum GetObjectErrorKind {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFoundException(crate::error::ContainerNotFoundException),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(crate::error::InternalServerError),
    /// <p>Could not perform an operation on an object that does not exist.</p>
    ObjectNotFoundException(crate::error::ObjectNotFoundException),
    /// <p>The requested content range is not valid.</p>
    RequestedRangeNotSatisfiableException(crate::error::RequestedRangeNotSatisfiableException),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for GetObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GetObjectErrorKind::ContainerNotFoundException(_inner) => _inner.fmt(f),
            GetObjectErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            GetObjectErrorKind::ObjectNotFoundException(_inner) => _inner.fmt(f),
            GetObjectErrorKind::RequestedRangeNotSatisfiableException(_inner) => _inner.fmt(f),
            GetObjectErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for GetObjectError {
    fn code(&self) -> Option<&str> {
        GetObjectError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl GetObjectError {
    /// Creates a new `GetObjectError`.
    pub fn new(kind: GetObjectErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `GetObjectError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: GetObjectErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `GetObjectError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: GetObjectErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `GetObjectErrorKind::ContainerNotFoundException`.
    pub fn is_container_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetObjectErrorKind::ContainerNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `GetObjectErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, GetObjectErrorKind::InternalServerError(_))
    }
    /// Returns `true` if the error kind is `GetObjectErrorKind::ObjectNotFoundException`.
    pub fn is_object_not_found_exception(&self) -> bool {
        matches!(&self.kind, GetObjectErrorKind::ObjectNotFoundException(_))
    }
    /// Returns `true` if the error kind is `GetObjectErrorKind::RequestedRangeNotSatisfiableException`.
    pub fn is_requested_range_not_satisfiable_exception(&self) -> bool {
        matches!(
            &self.kind,
            GetObjectErrorKind::RequestedRangeNotSatisfiableException(_)
        )
    }
}
impl std::error::Error for GetObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            GetObjectErrorKind::ContainerNotFoundException(_inner) => Some(_inner),
            GetObjectErrorKind::InternalServerError(_inner) => Some(_inner),
            GetObjectErrorKind::ObjectNotFoundException(_inner) => Some(_inner),
            GetObjectErrorKind::RequestedRangeNotSatisfiableException(_inner) => Some(_inner),
            GetObjectErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `ListItems` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct ListItemsError {
    /// Kind of error that occurred.
    pub kind: ListItemsErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `ListItems` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum ListItemsErrorKind {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFoundException(crate::error::ContainerNotFoundException),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(crate::error::InternalServerError),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for ListItemsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ListItemsErrorKind::ContainerNotFoundException(_inner) => _inner.fmt(f),
            ListItemsErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            ListItemsErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for ListItemsError {
    fn code(&self) -> Option<&str> {
        ListItemsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl ListItemsError {
    /// Creates a new `ListItemsError`.
    pub fn new(kind: ListItemsErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `ListItemsError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: ListItemsErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `ListItemsError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: ListItemsErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `ListItemsErrorKind::ContainerNotFoundException`.
    pub fn is_container_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            ListItemsErrorKind::ContainerNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `ListItemsErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, ListItemsErrorKind::InternalServerError(_))
    }
}
impl std::error::Error for ListItemsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ListItemsErrorKind::ContainerNotFoundException(_inner) => Some(_inner),
            ListItemsErrorKind::InternalServerError(_inner) => Some(_inner),
            ListItemsErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// Error type for the `PutObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub struct PutObjectError {
    /// Kind of error that occurred.
    pub kind: PutObjectErrorKind,
    /// Additional metadata about the error, including error code, message, and request ID.
    pub(crate) meta: aws_smithy_types::Error,
}
/// Types of errors that can occur for the `PutObject` operation.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum PutObjectErrorKind {
    /// <p>The specified container was not found for the specified account.</p>
    ContainerNotFoundException(crate::error::ContainerNotFoundException),
    /// <p>The service is temporarily unavailable.</p>
    InternalServerError(crate::error::InternalServerError),
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for PutObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            PutObjectErrorKind::ContainerNotFoundException(_inner) => _inner.fmt(f),
            PutObjectErrorKind::InternalServerError(_inner) => _inner.fmt(f),
            PutObjectErrorKind::Unhandled(_inner) => _inner.fmt(f),
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for PutObjectError {
    fn code(&self) -> Option<&str> {
        PutObjectError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl PutObjectError {
    /// Creates a new `PutObjectError`.
    pub fn new(kind: PutObjectErrorKind, meta: aws_smithy_types::Error) -> Self {
        Self { kind, meta }
    }

    /// Creates the `PutObjectError::Unhandled` variant from any error type.
    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
        Self {
            kind: PutObjectErrorKind::Unhandled(err.into()),
            meta: Default::default(),
        }
    }

    /// Creates the `PutObjectError::Unhandled` variant from a `aws_smithy_types::Error`.
    pub fn generic(err: aws_smithy_types::Error) -> Self {
        Self {
            meta: err.clone(),
            kind: PutObjectErrorKind::Unhandled(err.into()),
        }
    }

    /// Returns the error message if one is available.
    pub fn message(&self) -> Option<&str> {
        self.meta.message()
    }

    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    pub fn meta(&self) -> &aws_smithy_types::Error {
        &self.meta
    }

    /// Returns the request ID if it's available.
    pub fn request_id(&self) -> Option<&str> {
        self.meta.request_id()
    }

    /// Returns the error code if it's available.
    pub fn code(&self) -> Option<&str> {
        self.meta.code()
    }
    /// Returns `true` if the error kind is `PutObjectErrorKind::ContainerNotFoundException`.
    pub fn is_container_not_found_exception(&self) -> bool {
        matches!(
            &self.kind,
            PutObjectErrorKind::ContainerNotFoundException(_)
        )
    }
    /// Returns `true` if the error kind is `PutObjectErrorKind::InternalServerError`.
    pub fn is_internal_server_error(&self) -> bool {
        matches!(&self.kind, PutObjectErrorKind::InternalServerError(_))
    }
}
impl std::error::Error for PutObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            PutObjectErrorKind::ContainerNotFoundException(_inner) => Some(_inner),
            PutObjectErrorKind::InternalServerError(_inner) => Some(_inner),
            PutObjectErrorKind::Unhandled(_inner) => Some(_inner.as_ref()),
        }
    }
}

/// <p>The service is temporarily unavailable.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct InternalServerError {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("InternalServerError");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl InternalServerError {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InternalServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalServerError")?;
        if let Some(inner_1) = &self.message {
            write!(f, ": {}", inner_1)?;
        }
        Ok(())
    }
}
impl std::error::Error for InternalServerError {}
/// See [`InternalServerError`](crate::error::InternalServerError)
pub mod internal_server_error {
    /// A builder for [`InternalServerError`](crate::error::InternalServerError)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`InternalServerError`](crate::error::InternalServerError)
        pub fn build(self) -> crate::error::InternalServerError {
            crate::error::InternalServerError {
                message: self.message,
            }
        }
    }
}
impl InternalServerError {
    /// Creates a new builder-style object to manufacture [`InternalServerError`](crate::error::InternalServerError)
    pub fn builder() -> crate::error::internal_server_error::Builder {
        crate::error::internal_server_error::Builder::default()
    }
}

/// <p>The specified container was not found for the specified account.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ContainerNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ContainerNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ContainerNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ContainerNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ContainerNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ContainerNotFoundException")?;
        if let Some(inner_2) = &self.message {
            write!(f, ": {}", inner_2)?;
        }
        Ok(())
    }
}
impl std::error::Error for ContainerNotFoundException {}
/// See [`ContainerNotFoundException`](crate::error::ContainerNotFoundException)
pub mod container_not_found_exception {
    /// A builder for [`ContainerNotFoundException`](crate::error::ContainerNotFoundException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ContainerNotFoundException`](crate::error::ContainerNotFoundException)
        pub fn build(self) -> crate::error::ContainerNotFoundException {
            crate::error::ContainerNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ContainerNotFoundException {
    /// Creates a new builder-style object to manufacture [`ContainerNotFoundException`](crate::error::ContainerNotFoundException)
    pub fn builder() -> crate::error::container_not_found_exception::Builder {
        crate::error::container_not_found_exception::Builder::default()
    }
}

/// <p>The requested content range is not valid.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct RequestedRangeNotSatisfiableException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for RequestedRangeNotSatisfiableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RequestedRangeNotSatisfiableException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl RequestedRangeNotSatisfiableException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for RequestedRangeNotSatisfiableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RequestedRangeNotSatisfiableException")?;
        if let Some(inner_3) = &self.message {
            write!(f, ": {}", inner_3)?;
        }
        Ok(())
    }
}
impl std::error::Error for RequestedRangeNotSatisfiableException {}
/// See [`RequestedRangeNotSatisfiableException`](crate::error::RequestedRangeNotSatisfiableException)
pub mod requested_range_not_satisfiable_exception {
    /// A builder for [`RequestedRangeNotSatisfiableException`](crate::error::RequestedRangeNotSatisfiableException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`RequestedRangeNotSatisfiableException`](crate::error::RequestedRangeNotSatisfiableException)
        pub fn build(self) -> crate::error::RequestedRangeNotSatisfiableException {
            crate::error::RequestedRangeNotSatisfiableException {
                message: self.message,
            }
        }
    }
}
impl RequestedRangeNotSatisfiableException {
    /// Creates a new builder-style object to manufacture [`RequestedRangeNotSatisfiableException`](crate::error::RequestedRangeNotSatisfiableException)
    pub fn builder() -> crate::error::requested_range_not_satisfiable_exception::Builder {
        crate::error::requested_range_not_satisfiable_exception::Builder::default()
    }
}

/// <p>Could not perform an operation on an object that does not exist.</p>
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct ObjectNotFoundException {
    #[allow(missing_docs)] // documentation missing in model
    pub message: std::option::Option<std::string::String>,
}
impl std::fmt::Debug for ObjectNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("ObjectNotFoundException");
        formatter.field("message", &self.message);
        formatter.finish()
    }
}
impl ObjectNotFoundException {
    /// Returns the error message.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ObjectNotFoundException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ObjectNotFoundException")?;
        if let Some(inner_4) = &self.message {
            write!(f, ": {}", inner_4)?;
        }
        Ok(())
    }
}
impl std::error::Error for ObjectNotFoundException {}
/// See [`ObjectNotFoundException`](crate::error::ObjectNotFoundException)
pub mod object_not_found_exception {
    /// A builder for [`ObjectNotFoundException`](crate::error::ObjectNotFoundException)
    #[non_exhaustive]
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) message: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
            self.message = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.message = input;
            self
        }
        /// Consumes the builder and constructs a [`ObjectNotFoundException`](crate::error::ObjectNotFoundException)
        pub fn build(self) -> crate::error::ObjectNotFoundException {
            crate::error::ObjectNotFoundException {
                message: self.message,
            }
        }
    }
}
impl ObjectNotFoundException {
    /// Creates a new builder-style object to manufacture [`ObjectNotFoundException`](crate::error::ObjectNotFoundException)
    pub fn builder() -> crate::error::object_not_found_exception::Builder {
        crate::error::object_not_found_exception::Builder::default()
    }
}
