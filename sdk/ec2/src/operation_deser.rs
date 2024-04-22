// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]pub fn parse_create_tags_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateTagsOutput, crate::error::CreateTagsError> {
    let generic = crate::xml_deser::parse_http_generic_error(response).map_err(crate::error::CreateTagsError::unhandled)?;
    Err(crate::error::CreateTagsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_create_tags_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateTagsOutput, crate::error::CreateTagsError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::create_tags_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_describe_instances_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeInstancesOutput, crate::error::DescribeInstancesError> {
    let generic = crate::xml_deser::parse_http_generic_error(response).map_err(crate::error::DescribeInstancesError::unhandled)?;
    Err(crate::error::DescribeInstancesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_describe_instances_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeInstancesOutput, crate::error::DescribeInstancesError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::describe_instances_output::Builder::default();
        let _ = response;
        output = crate::xml_deser::deser_operation_crate_operation_describe_instances(response.body().as_ref(), output).map_err(crate::error::DescribeInstancesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_run_instances_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RunInstancesOutput, crate::error::RunInstancesError> {
    let generic = crate::xml_deser::parse_http_generic_error(response).map_err(crate::error::RunInstancesError::unhandled)?;
    Err(crate::error::RunInstancesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_run_instances_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RunInstancesOutput, crate::error::RunInstancesError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::run_instances_output::Builder::default();
        let _ = response;
        output = crate::xml_deser::deser_operation_crate_operation_run_instances(response.body().as_ref(), output).map_err(crate::error::RunInstancesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_terminate_instances_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TerminateInstancesOutput, crate::error::TerminateInstancesError> {
    let generic = crate::xml_deser::parse_http_generic_error(response).map_err(crate::error::TerminateInstancesError::unhandled)?;
    Err(crate::error::TerminateInstancesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]pub fn parse_terminate_instances_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::TerminateInstancesOutput, crate::error::TerminateInstancesError> {
    Ok({
        #[allow(unused_mut)]let mut output = crate::output::terminate_instances_output::Builder::default();
        let _ = response;
        output = crate::xml_deser::deser_operation_crate_operation_terminate_instances(response.body().as_ref(), output).map_err(crate::error::TerminateInstancesError::unhandled)?;
        output.build()
    })
}

