// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateTags`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`create_tags`](crate::client::Client::create_tags).
            ///
            /// See [`crate::client::fluent_builders::CreateTags`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct CreateTags {
    _private: ()
}
impl CreateTags {
    /// Creates a new builder-style object to manufacture [`CreateTagsInput`](crate::input::CreateTagsInput).
    pub fn builder() -> crate::input::create_tags_input::Builder {
        crate::input::create_tags_input::Builder::default()
    }
    /// Creates a new `CreateTags` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateTags {
                type Output = std::result::Result<crate::output::CreateTagsOutput, crate::error::CreateTagsError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_create_tags_error(response)
                     } else {
                        crate::operation_deser::parse_create_tags_response(response)
                     }
                }
            }

/// Operation shape for `DescribeInstances`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`describe_instances`](crate::client::Client::describe_instances).
            ///
            /// See [`crate::client::fluent_builders::DescribeInstances`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct DescribeInstances {
    _private: ()
}
impl DescribeInstances {
    /// Creates a new builder-style object to manufacture [`DescribeInstancesInput`](crate::input::DescribeInstancesInput).
    pub fn builder() -> crate::input::describe_instances_input::Builder {
        crate::input::describe_instances_input::Builder::default()
    }
    /// Creates a new `DescribeInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeInstances {
                type Output = std::result::Result<crate::output::DescribeInstancesOutput, crate::error::DescribeInstancesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_describe_instances_error(response)
                     } else {
                        crate::operation_deser::parse_describe_instances_response(response)
                     }
                }
            }

/// Operation shape for `RunInstances`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`run_instances`](crate::client::Client::run_instances).
            ///
            /// See [`crate::client::fluent_builders::RunInstances`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct RunInstances {
    _private: ()
}
impl RunInstances {
    /// Creates a new builder-style object to manufacture [`RunInstancesInput`](crate::input::RunInstancesInput).
    pub fn builder() -> crate::input::run_instances_input::Builder {
        crate::input::run_instances_input::Builder::default()
    }
    /// Creates a new `RunInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RunInstances {
                type Output = std::result::Result<crate::output::RunInstancesOutput, crate::error::RunInstancesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_run_instances_error(response)
                     } else {
                        crate::operation_deser::parse_run_instances_response(response)
                     }
                }
            }

/// Operation shape for `TerminateInstances`.
            ///
            /// This is usually constructed for you using the the fluent builder returned by
            /// [`terminate_instances`](crate::client::Client::terminate_instances).
            ///
            /// See [`crate::client::fluent_builders::TerminateInstances`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug, )]
pub struct TerminateInstances {
    _private: ()
}
impl TerminateInstances {
    /// Creates a new builder-style object to manufacture [`TerminateInstancesInput`](crate::input::TerminateInstancesInput).
    pub fn builder() -> crate::input::terminate_instances_input::Builder {
        crate::input::terminate_instances_input::Builder::default()
    }
    /// Creates a new `TerminateInstances` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TerminateInstances {
                type Output = std::result::Result<crate::output::TerminateInstancesOutput, crate::error::TerminateInstancesError>;
                fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
                     if !response.status().is_success() && response.status().as_u16() != 200 {
                        crate::operation_deser::parse_terminate_instances_error(response)
                     } else {
                        crate::operation_deser::parse_terminate_instances_response(response)
                     }
                }
            }

