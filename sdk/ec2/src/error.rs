// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Error type for the `CreateTags` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct CreateTagsError {
    /// Kind of error that occurred.
                    pub kind: CreateTagsErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `CreateTags` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum CreateTagsErrorKind {
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for CreateTagsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            CreateTagsErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for CreateTagsError {
    fn code(&self) -> Option<&str> {
        CreateTagsError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl CreateTagsError {
    /// Creates a new `CreateTagsError`.
                    pub fn new(kind: CreateTagsErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `CreateTagsError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: CreateTagsErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `CreateTagsError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: CreateTagsErrorKind::Unhandled(err.into()),
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
}
impl std::error::Error for CreateTagsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            CreateTagsErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `DescribeInstances` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct DescribeInstancesError {
    /// Kind of error that occurred.
                    pub kind: DescribeInstancesErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `DescribeInstances` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum DescribeInstancesErrorKind {
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for DescribeInstancesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            DescribeInstancesErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for DescribeInstancesError {
    fn code(&self) -> Option<&str> {
        DescribeInstancesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl DescribeInstancesError {
    /// Creates a new `DescribeInstancesError`.
                    pub fn new(kind: DescribeInstancesErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `DescribeInstancesError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: DescribeInstancesErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `DescribeInstancesError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: DescribeInstancesErrorKind::Unhandled(err.into()),
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
}
impl std::error::Error for DescribeInstancesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            DescribeInstancesErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `RunInstances` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct RunInstancesError {
    /// Kind of error that occurred.
                    pub kind: RunInstancesErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `RunInstances` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum RunInstancesErrorKind {
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for RunInstancesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            RunInstancesErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for RunInstancesError {
    fn code(&self) -> Option<&str> {
        RunInstancesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl RunInstancesError {
    /// Creates a new `RunInstancesError`.
                    pub fn new(kind: RunInstancesErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `RunInstancesError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: RunInstancesErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `RunInstancesError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: RunInstancesErrorKind::Unhandled(err.into()),
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
}
impl std::error::Error for RunInstancesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            RunInstancesErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

/// Error type for the `TerminateInstances` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub struct TerminateInstancesError {
    /// Kind of error that occurred.
                    pub kind: TerminateInstancesErrorKind,
                    /// Additional metadata about the error, including error code, message, and request ID.
                    pub (crate) meta: aws_smithy_types::Error
}
/// Types of errors that can occur for the `TerminateInstances` operation.
#[non_exhaustive]#[derive(std::fmt::Debug, )]
pub enum TerminateInstancesErrorKind {
    /// An unexpected error, e.g. invalid JSON returned by the service or an unknown error code
                    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for TerminateInstancesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TerminateInstancesErrorKind::Unhandled(_inner) => {
                _inner.fmt(f)
            }
        }
    }
}
impl aws_smithy_types::retry::ProvideErrorKind for TerminateInstancesError {
    fn code(&self) -> Option<&str> {
        TerminateInstancesError::code(self)
    }
    fn retryable_error_kind(&self) -> Option<aws_smithy_types::retry::ErrorKind> {
        None
    }
}
impl TerminateInstancesError {
    /// Creates a new `TerminateInstancesError`.
                    pub fn new(kind: TerminateInstancesErrorKind, meta: aws_smithy_types::Error) -> Self {
                        Self { kind, meta }
                    }
    
                    /// Creates the `TerminateInstancesError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>) -> Self {
                        Self {
                            kind: TerminateInstancesErrorKind::Unhandled(err.into()),
                            meta: Default::default()
                        }
                    }
    
                    /// Creates the `TerminateInstancesError::Unhandled` variant from a `aws_smithy_types::Error`.
                    pub fn generic(err: aws_smithy_types::Error) -> Self {
                        Self {
                            meta: err.clone(),
                            kind: TerminateInstancesErrorKind::Unhandled(err.into()),
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
}
impl std::error::Error for TerminateInstancesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            TerminateInstancesErrorKind::Unhandled(_inner) => {
                Some(_inner.as_ref())
            }
        }
    }
}

