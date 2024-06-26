// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_structure_crate_output_list_cidr_blocks_output_next_token(input: &crate::output::ListCidrBlocksOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_cidr_collections_output_next_token(input: &crate::output::ListCidrCollectionsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_cidr_locations_output_next_token(input: &crate::output::ListCidrLocationsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_health_checks_output_next_marker(input: &crate::output::ListHealthChecksOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_marker {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_hosted_zones_output_next_marker(input: &crate::output::ListHostedZonesOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_marker {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn reflens_structure_crate_output_list_query_logging_configs_output_next_token(input: &crate::output::ListQueryLoggingConfigsOutput) -> std::option::Option<& std::string::String> {
                    let input = match &input.next_token {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_cidr_blocks_output_cidr_blocks(input: crate::output::ListCidrBlocksOutput) -> std::option::Option<std::vec::Vec<crate::model::CidrBlockSummary>> {
                    let input = match input.cidr_blocks {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_cidr_collections_output_cidr_collections(input: crate::output::ListCidrCollectionsOutput) -> std::option::Option<std::vec::Vec<crate::model::CollectionSummary>> {
                    let input = match input.cidr_collections {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_cidr_locations_output_cidr_locations(input: crate::output::ListCidrLocationsOutput) -> std::option::Option<std::vec::Vec<crate::model::LocationSummary>> {
                    let input = match input.cidr_locations {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_health_checks_output_health_checks(input: crate::output::ListHealthChecksOutput) -> std::option::Option<std::vec::Vec<crate::model::HealthCheck>> {
                    let input = match input.health_checks {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_hosted_zones_output_hosted_zones(input: crate::output::ListHostedZonesOutput) -> std::option::Option<std::vec::Vec<crate::model::HostedZone>> {
                    let input = match input.hosted_zones {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

pub(crate) fn lens_structure_crate_output_list_query_logging_configs_output_query_logging_configs(input: crate::output::ListQueryLoggingConfigsOutput) -> std::option::Option<std::vec::Vec<crate::model::QueryLoggingConfig>> {
                    let input = match input.query_logging_configs {
                        None => return None,
                        Some(t) => t
                    };
Some(input)
                }

