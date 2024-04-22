// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct TerminateInstancesOutput  {
    /// <p>Information about the terminated instances.</p>
    pub terminating_instances: std::option::Option<std::vec::Vec<crate::model::InstanceStateChange>>,
}
impl TerminateInstancesOutput {
    /// <p>Information about the terminated instances.</p>
    pub fn terminating_instances(&self) -> std::option::Option<& [crate::model::InstanceStateChange]> {
        self.terminating_instances.as_deref()
    }
}
impl  std::fmt::Debug for TerminateInstancesOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("TerminateInstancesOutput");
        formatter.field("terminating_instances", &self.terminating_instances);
        formatter.finish()
    }
}
/// See [`TerminateInstancesOutput`](crate::output::TerminateInstancesOutput).
pub mod terminate_instances_output {
    
    /// A builder for [`TerminateInstancesOutput`](crate::output::TerminateInstancesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) terminating_instances: std::option::Option<std::vec::Vec<crate::model::InstanceStateChange>>,
    }
    impl Builder {
        /// Appends an item to `terminating_instances`.
        ///
        /// To override the contents of this collection use [`set_terminating_instances`](Self::set_terminating_instances).
        ///
        /// <p>Information about the terminated instances.</p>
        pub fn terminating_instances(mut self, input: crate::model::InstanceStateChange) -> Self {
            let mut v = self.terminating_instances.unwrap_or_default();
                            v.push(input);
                            self.terminating_instances = Some(v);
                            self
        }
        /// <p>Information about the terminated instances.</p>
        pub fn set_terminating_instances(mut self, input: std::option::Option<std::vec::Vec<crate::model::InstanceStateChange>>) -> Self {
            self.terminating_instances = input; self
        }
        /// Consumes the builder and constructs a [`TerminateInstancesOutput`](crate::output::TerminateInstancesOutput).
        pub fn build(self) -> crate::output::TerminateInstancesOutput {
            crate::output::TerminateInstancesOutput {
                terminating_instances: self.terminating_instances
                ,
            }
        }
    }
    
    
}
impl TerminateInstancesOutput {
    /// Creates a new builder-style object to manufacture [`TerminateInstancesOutput`](crate::output::TerminateInstancesOutput).
    pub fn builder() -> crate::output::terminate_instances_output::Builder {
        crate::output::terminate_instances_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct CreateTagsOutput  {
}
impl  std::fmt::Debug for CreateTagsOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CreateTagsOutput");
        formatter.finish()
    }
}
/// See [`CreateTagsOutput`](crate::output::CreateTagsOutput).
pub mod create_tags_output {
    
    /// A builder for [`CreateTagsOutput`](crate::output::CreateTagsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
    }
    impl Builder {
        /// Consumes the builder and constructs a [`CreateTagsOutput`](crate::output::CreateTagsOutput).
        pub fn build(self) -> crate::output::CreateTagsOutput {
            crate::output::CreateTagsOutput {
            }
        }
    }
    
    
}
impl CreateTagsOutput {
    /// Creates a new builder-style object to manufacture [`CreateTagsOutput`](crate::output::CreateTagsOutput).
    pub fn builder() -> crate::output::create_tags_output::Builder {
        crate::output::create_tags_output::Builder::default()
    }
}

/// <p>Describes a launch request for one or more instances, and includes owner, requester, and security group information that applies to all instances in the launch request.</p>
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct RunInstancesOutput  {
    /// <p>[EC2-Classic only] The security groups.</p>
    pub groups: std::option::Option<std::vec::Vec<crate::model::GroupIdentifier>>,
    /// <p>The instances.</p>
    pub instances: std::option::Option<std::vec::Vec<crate::model::Instance>>,
    /// <p>The ID of the Amazon Web Services account that owns the reservation.</p>
    pub owner_id: std::option::Option<std::string::String>,
    /// <p>The ID of the requester that launched the instances on your behalf (for example, Amazon Web Services Management Console or Auto Scaling).</p>
    pub requester_id: std::option::Option<std::string::String>,
    /// <p>The ID of the reservation.</p>
    pub reservation_id: std::option::Option<std::string::String>,
}
impl RunInstancesOutput {
    /// <p>[EC2-Classic only] The security groups.</p>
    pub fn groups(&self) -> std::option::Option<& [crate::model::GroupIdentifier]> {
        self.groups.as_deref()
    }
    /// <p>The instances.</p>
    pub fn instances(&self) -> std::option::Option<& [crate::model::Instance]> {
        self.instances.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the reservation.</p>
    pub fn owner_id(&self) -> std::option::Option<& str> {
        self.owner_id.as_deref()
    }
    /// <p>The ID of the requester that launched the instances on your behalf (for example, Amazon Web Services Management Console or Auto Scaling).</p>
    pub fn requester_id(&self) -> std::option::Option<& str> {
        self.requester_id.as_deref()
    }
    /// <p>The ID of the reservation.</p>
    pub fn reservation_id(&self) -> std::option::Option<& str> {
        self.reservation_id.as_deref()
    }
}
impl  std::fmt::Debug for RunInstancesOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("RunInstancesOutput");
        formatter.field("groups", &self.groups);
        formatter.field("instances", &self.instances);
        formatter.field("owner_id", &self.owner_id);
        formatter.field("requester_id", &self.requester_id);
        formatter.field("reservation_id", &self.reservation_id);
        formatter.finish()
    }
}
/// See [`RunInstancesOutput`](crate::output::RunInstancesOutput).
pub mod run_instances_output {
    
    /// A builder for [`RunInstancesOutput`](crate::output::RunInstancesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) groups: std::option::Option<std::vec::Vec<crate::model::GroupIdentifier>>,
        pub(crate) instances: std::option::Option<std::vec::Vec<crate::model::Instance>>,
        pub(crate) owner_id: std::option::Option<std::string::String>,
        pub(crate) requester_id: std::option::Option<std::string::String>,
        pub(crate) reservation_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `groups`.
        ///
        /// To override the contents of this collection use [`set_groups`](Self::set_groups).
        ///
        /// <p>[EC2-Classic only] The security groups.</p>
        pub fn groups(mut self, input: crate::model::GroupIdentifier) -> Self {
            let mut v = self.groups.unwrap_or_default();
                            v.push(input);
                            self.groups = Some(v);
                            self
        }
        /// <p>[EC2-Classic only] The security groups.</p>
        pub fn set_groups(mut self, input: std::option::Option<std::vec::Vec<crate::model::GroupIdentifier>>) -> Self {
            self.groups = input; self
        }
        /// Appends an item to `instances`.
        ///
        /// To override the contents of this collection use [`set_instances`](Self::set_instances).
        ///
        /// <p>The instances.</p>
        pub fn instances(mut self, input: crate::model::Instance) -> Self {
            let mut v = self.instances.unwrap_or_default();
                            v.push(input);
                            self.instances = Some(v);
                            self
        }
        /// <p>The instances.</p>
        pub fn set_instances(mut self, input: std::option::Option<std::vec::Vec<crate::model::Instance>>) -> Self {
            self.instances = input; self
        }
        /// <p>The ID of the Amazon Web Services account that owns the reservation.</p>
        pub fn owner_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.owner_id = Some(input.into());
            self
        }
        /// <p>The ID of the Amazon Web Services account that owns the reservation.</p>
        pub fn set_owner_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.owner_id = input; self
        }
        /// <p>The ID of the requester that launched the instances on your behalf (for example, Amazon Web Services Management Console or Auto Scaling).</p>
        pub fn requester_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.requester_id = Some(input.into());
            self
        }
        /// <p>The ID of the requester that launched the instances on your behalf (for example, Amazon Web Services Management Console or Auto Scaling).</p>
        pub fn set_requester_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.requester_id = input; self
        }
        /// <p>The ID of the reservation.</p>
        pub fn reservation_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.reservation_id = Some(input.into());
            self
        }
        /// <p>The ID of the reservation.</p>
        pub fn set_reservation_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.reservation_id = input; self
        }
        /// Consumes the builder and constructs a [`RunInstancesOutput`](crate::output::RunInstancesOutput).
        pub fn build(self) -> crate::output::RunInstancesOutput {
            crate::output::RunInstancesOutput {
                groups: self.groups
                ,
                instances: self.instances
                ,
                owner_id: self.owner_id
                ,
                requester_id: self.requester_id
                ,
                reservation_id: self.reservation_id
                ,
            }
        }
    }
    
    
}
impl RunInstancesOutput {
    /// Creates a new builder-style object to manufacture [`RunInstancesOutput`](crate::output::RunInstancesOutput).
    pub fn builder() -> crate::output::run_instances_output::Builder {
        crate::output::run_instances_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]#[derive(std::clone::Clone, std::cmp::PartialEq, )]
pub struct DescribeInstancesOutput  {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Information about the reservations.</p>
    pub reservations: std::option::Option<std::vec::Vec<crate::model::Reservation>>,
}
impl DescribeInstancesOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<& str> {
        self.next_token.as_deref()
    }
    /// <p>Information about the reservations.</p>
    pub fn reservations(&self) -> std::option::Option<& [crate::model::Reservation]> {
        self.reservations.as_deref()
    }
}
impl  std::fmt::Debug for DescribeInstancesOutput  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeInstancesOutput");
        formatter.field("next_token", &self.next_token);
        formatter.field("reservations", &self.reservations);
        formatter.finish()
    }
}
/// See [`DescribeInstancesOutput`](crate::output::DescribeInstancesOutput).
pub mod describe_instances_output {
    
    /// A builder for [`DescribeInstancesOutput`](crate::output::DescribeInstancesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug, )]
    pub struct Builder {
        pub(crate) next_token: std::option::Option<std::string::String>,
        pub(crate) reservations: std::option::Option<std::vec::Vec<crate::model::Reservation>>,
    }
    impl Builder {
        /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input; self
        }
        /// Appends an item to `reservations`.
        ///
        /// To override the contents of this collection use [`set_reservations`](Self::set_reservations).
        ///
        /// <p>Information about the reservations.</p>
        pub fn reservations(mut self, input: crate::model::Reservation) -> Self {
            let mut v = self.reservations.unwrap_or_default();
                            v.push(input);
                            self.reservations = Some(v);
                            self
        }
        /// <p>Information about the reservations.</p>
        pub fn set_reservations(mut self, input: std::option::Option<std::vec::Vec<crate::model::Reservation>>) -> Self {
            self.reservations = input; self
        }
        /// Consumes the builder and constructs a [`DescribeInstancesOutput`](crate::output::DescribeInstancesOutput).
        pub fn build(self) -> crate::output::DescribeInstancesOutput {
            crate::output::DescribeInstancesOutput {
                next_token: self.next_token
                ,
                reservations: self.reservations
                ,
            }
        }
    }
    
    
}
impl DescribeInstancesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeInstancesOutput`](crate::output::DescribeInstancesOutput).
    pub fn builder() -> crate::output::describe_instances_output::Builder {
        crate::output::describe_instances_output::Builder::default()
    }
}
