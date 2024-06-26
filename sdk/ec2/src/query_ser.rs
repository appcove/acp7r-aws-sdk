// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]pub fn serialize_structure_crate_model_tag(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Tag) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_1 = writer.prefix("Key");
    if let Some(var_2) = &input.key {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]let mut scope_3 = writer.prefix("Value");
    if let Some(var_4) = &input.value {
        scope_3.string(var_4);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_filter(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Filter) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_5 = writer.prefix("Name");
    if let Some(var_6) = &input.name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]let mut scope_7 = writer.prefix("Value");
    if let Some(var_8) = &input.values {
        let mut list_10 = scope_7.start_list(true, Some("item"));
        for item_9 in var_8 {
            #[allow(unused_mut)]let mut entry_11 = list_10.entry();
            entry_11.string(item_9);
        }
        list_10.finish();
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_block_device_mapping(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::BlockDeviceMapping) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_12 = writer.prefix("DeviceName");
    if let Some(var_13) = &input.device_name {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]let mut scope_14 = writer.prefix("Ebs");
    if let Some(var_15) = &input.ebs {
        crate::query_ser::serialize_structure_crate_model_ebs_block_device(scope_14, var_15)?;
    }
    #[allow(unused_mut)]let mut scope_16 = writer.prefix("NoDevice");
    if let Some(var_17) = &input.no_device {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]let mut scope_18 = writer.prefix("VirtualName");
    if let Some(var_19) = &input.virtual_name {
        scope_18.string(var_19);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_capacity_reservation_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CapacityReservationSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_20 = writer.prefix("CapacityReservationPreference");
    if let Some(var_21) = &input.capacity_reservation_preference {
        scope_20.string(var_21.as_str());
    }
    #[allow(unused_mut)]let mut scope_22 = writer.prefix("CapacityReservationTarget");
    if let Some(var_23) = &input.capacity_reservation_target {
        crate::query_ser::serialize_structure_crate_model_capacity_reservation_target(scope_22, var_23)?;
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_cpu_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CpuOptionsRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_24 = writer.prefix("CoreCount");
    if let Some(var_25) = &input.core_count {
        scope_24.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_25).into()));
    }
    #[allow(unused_mut)]let mut scope_26 = writer.prefix("ThreadsPerCore");
    if let Some(var_27) = &input.threads_per_core {
        scope_26.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_27).into()));
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_credit_specification_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CreditSpecificationRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_28 = writer.prefix("CpuCredits");
    if let Some(var_29) = &input.cpu_credits {
        scope_28.string(var_29);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_elastic_gpu_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ElasticGpuSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_30 = writer.prefix("Type");
    if let Some(var_31) = &input.r#type {
        scope_30.string(var_31);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_elastic_inference_accelerator(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ElasticInferenceAccelerator) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_32 = writer.prefix("Count");
    if let Some(var_33) = &input.count {
        scope_32.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_33).into()));
    }
    #[allow(unused_mut)]let mut scope_34 = writer.prefix("Type");
    if let Some(var_35) = &input.r#type {
        scope_34.string(var_35);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_enclave_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::EnclaveOptionsRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_36 = writer.prefix("Enabled");
    if let Some(var_37) = &input.enabled {
        scope_36.boolean(*var_37);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_hibernation_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::HibernationOptionsRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_38 = writer.prefix("Configured");
    if let Some(var_39) = &input.configured {
        scope_38.boolean(*var_39);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_iam_instance_profile_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::IamInstanceProfileSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_40 = writer.prefix("Arn");
    if let Some(var_41) = &input.arn {
        scope_40.string(var_41);
    }
    #[allow(unused_mut)]let mut scope_42 = writer.prefix("Name");
    if let Some(var_43) = &input.name {
        scope_42.string(var_43);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_instance_market_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::InstanceMarketOptionsRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_44 = writer.prefix("MarketType");
    if let Some(var_45) = &input.market_type {
        scope_44.string(var_45.as_str());
    }
    #[allow(unused_mut)]let mut scope_46 = writer.prefix("SpotOptions");
    if let Some(var_47) = &input.spot_options {
        crate::query_ser::serialize_structure_crate_model_spot_market_options(scope_46, var_47)?;
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_instance_ipv6_address(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::InstanceIpv6Address) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_48 = writer.prefix("Ipv6Address");
    if let Some(var_49) = &input.ipv6_address {
        scope_48.string(var_49);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_launch_template_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::LaunchTemplateSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_50 = writer.prefix("LaunchTemplateId");
    if let Some(var_51) = &input.launch_template_id {
        scope_50.string(var_51);
    }
    #[allow(unused_mut)]let mut scope_52 = writer.prefix("LaunchTemplateName");
    if let Some(var_53) = &input.launch_template_name {
        scope_52.string(var_53);
    }
    #[allow(unused_mut)]let mut scope_54 = writer.prefix("Version");
    if let Some(var_55) = &input.version {
        scope_54.string(var_55);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_license_configuration_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::LicenseConfigurationRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_56 = writer.prefix("LicenseConfigurationArn");
    if let Some(var_57) = &input.license_configuration_arn {
        scope_56.string(var_57);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_instance_maintenance_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::InstanceMaintenanceOptionsRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_58 = writer.prefix("AutoRecovery");
    if let Some(var_59) = &input.auto_recovery {
        scope_58.string(var_59.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_instance_metadata_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::InstanceMetadataOptionsRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_60 = writer.prefix("HttpEndpoint");
    if let Some(var_61) = &input.http_endpoint {
        scope_60.string(var_61.as_str());
    }
    #[allow(unused_mut)]let mut scope_62 = writer.prefix("HttpProtocolIpv6");
    if let Some(var_63) = &input.http_protocol_ipv6 {
        scope_62.string(var_63.as_str());
    }
    #[allow(unused_mut)]let mut scope_64 = writer.prefix("HttpPutResponseHopLimit");
    if let Some(var_65) = &input.http_put_response_hop_limit {
        scope_64.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_65).into()));
    }
    #[allow(unused_mut)]let mut scope_66 = writer.prefix("HttpTokens");
    if let Some(var_67) = &input.http_tokens {
        scope_66.string(var_67.as_str());
    }
    #[allow(unused_mut)]let mut scope_68 = writer.prefix("InstanceMetadataTags");
    if let Some(var_69) = &input.instance_metadata_tags {
        scope_68.string(var_69.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_run_instances_monitoring_enabled(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::RunInstancesMonitoringEnabled) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_70 = writer.prefix("Enabled");
    if let Some(var_71) = &input.enabled {
        scope_70.boolean(*var_71);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_instance_network_interface_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::InstanceNetworkInterfaceSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_72 = writer.prefix("AssociateCarrierIpAddress");
    if let Some(var_73) = &input.associate_carrier_ip_address {
        scope_72.boolean(*var_73);
    }
    #[allow(unused_mut)]let mut scope_74 = writer.prefix("AssociatePublicIpAddress");
    if let Some(var_75) = &input.associate_public_ip_address {
        scope_74.boolean(*var_75);
    }
    #[allow(unused_mut)]let mut scope_76 = writer.prefix("DeleteOnTermination");
    if let Some(var_77) = &input.delete_on_termination {
        scope_76.boolean(*var_77);
    }
    #[allow(unused_mut)]let mut scope_78 = writer.prefix("Description");
    if let Some(var_79) = &input.description {
        scope_78.string(var_79);
    }
    #[allow(unused_mut)]let mut scope_80 = writer.prefix("DeviceIndex");
    if let Some(var_81) = &input.device_index {
        scope_80.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_81).into()));
    }
    #[allow(unused_mut)]let mut scope_82 = writer.prefix("SecurityGroupId");
    if let Some(var_83) = &input.groups {
        let mut list_85 = scope_82.start_list(true, Some("SecurityGroupId"));
        for item_84 in var_83 {
            #[allow(unused_mut)]let mut entry_86 = list_85.entry();
            entry_86.string(item_84);
        }
        list_85.finish();
    }
    #[allow(unused_mut)]let mut scope_87 = writer.prefix("InterfaceType");
    if let Some(var_88) = &input.interface_type {
        scope_87.string(var_88);
    }
    #[allow(unused_mut)]let mut scope_89 = writer.prefix("Ipv4PrefixCount");
    if let Some(var_90) = &input.ipv4_prefix_count {
        scope_89.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_90).into()));
    }
    #[allow(unused_mut)]let mut scope_91 = writer.prefix("Ipv4Prefix");
    if let Some(var_92) = &input.ipv4_prefixes {
        let mut list_94 = scope_91.start_list(true, Some("item"));
        for item_93 in var_92 {
            #[allow(unused_mut)]let mut entry_95 = list_94.entry();
            crate::query_ser::serialize_structure_crate_model_ipv4_prefix_specification_request(entry_95, item_93)?;
        }
        list_94.finish();
    }
    #[allow(unused_mut)]let mut scope_96 = writer.prefix("Ipv6AddressCount");
    if let Some(var_97) = &input.ipv6_address_count {
        scope_96.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_97).into()));
    }
    #[allow(unused_mut)]let mut scope_98 = writer.prefix("Ipv6Addresses");
    if let Some(var_99) = &input.ipv6_addresses {
        let mut list_101 = scope_98.start_list(true, Some("item"));
        for item_100 in var_99 {
            #[allow(unused_mut)]let mut entry_102 = list_101.entry();
            crate::query_ser::serialize_structure_crate_model_instance_ipv6_address(entry_102, item_100)?;
        }
        list_101.finish();
    }
    #[allow(unused_mut)]let mut scope_103 = writer.prefix("Ipv6PrefixCount");
    if let Some(var_104) = &input.ipv6_prefix_count {
        scope_103.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_104).into()));
    }
    #[allow(unused_mut)]let mut scope_105 = writer.prefix("Ipv6Prefix");
    if let Some(var_106) = &input.ipv6_prefixes {
        let mut list_108 = scope_105.start_list(true, Some("item"));
        for item_107 in var_106 {
            #[allow(unused_mut)]let mut entry_109 = list_108.entry();
            crate::query_ser::serialize_structure_crate_model_ipv6_prefix_specification_request(entry_109, item_107)?;
        }
        list_108.finish();
    }
    #[allow(unused_mut)]let mut scope_110 = writer.prefix("NetworkCardIndex");
    if let Some(var_111) = &input.network_card_index {
        scope_110.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_111).into()));
    }
    #[allow(unused_mut)]let mut scope_112 = writer.prefix("NetworkInterfaceId");
    if let Some(var_113) = &input.network_interface_id {
        scope_112.string(var_113);
    }
    #[allow(unused_mut)]let mut scope_114 = writer.prefix("PrivateIpAddress");
    if let Some(var_115) = &input.private_ip_address {
        scope_114.string(var_115);
    }
    #[allow(unused_mut)]let mut scope_116 = writer.prefix("PrivateIpAddresses");
    if let Some(var_117) = &input.private_ip_addresses {
        let mut list_119 = scope_116.start_list(true, Some("item"));
        for item_118 in var_117 {
            #[allow(unused_mut)]let mut entry_120 = list_119.entry();
            crate::query_ser::serialize_structure_crate_model_private_ip_address_specification(entry_120, item_118)?;
        }
        list_119.finish();
    }
    #[allow(unused_mut)]let mut scope_121 = writer.prefix("SecondaryPrivateIpAddressCount");
    if let Some(var_122) = &input.secondary_private_ip_address_count {
        scope_121.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_122).into()));
    }
    #[allow(unused_mut)]let mut scope_123 = writer.prefix("SubnetId");
    if let Some(var_124) = &input.subnet_id {
        scope_123.string(var_124);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_placement(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Placement) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_125 = writer.prefix("Affinity");
    if let Some(var_126) = &input.affinity {
        scope_125.string(var_126);
    }
    #[allow(unused_mut)]let mut scope_127 = writer.prefix("AvailabilityZone");
    if let Some(var_128) = &input.availability_zone {
        scope_127.string(var_128);
    }
    #[allow(unused_mut)]let mut scope_129 = writer.prefix("GroupName");
    if let Some(var_130) = &input.group_name {
        scope_129.string(var_130);
    }
    #[allow(unused_mut)]let mut scope_131 = writer.prefix("HostId");
    if let Some(var_132) = &input.host_id {
        scope_131.string(var_132);
    }
    #[allow(unused_mut)]let mut scope_133 = writer.prefix("HostResourceGroupArn");
    if let Some(var_134) = &input.host_resource_group_arn {
        scope_133.string(var_134);
    }
    #[allow(unused_mut)]let mut scope_135 = writer.prefix("PartitionNumber");
    if let Some(var_136) = &input.partition_number {
        scope_135.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_136).into()));
    }
    #[allow(unused_mut)]let mut scope_137 = writer.prefix("SpreadDomain");
    if let Some(var_138) = &input.spread_domain {
        scope_137.string(var_138);
    }
    #[allow(unused_mut)]let mut scope_139 = writer.prefix("Tenancy");
    if let Some(var_140) = &input.tenancy {
        scope_139.string(var_140.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_private_dns_name_options_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::PrivateDnsNameOptionsRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_141 = writer.prefix("EnableResourceNameDnsAAAARecord");
    if let Some(var_142) = &input.enable_resource_name_dns_aaaa_record {
        scope_141.boolean(*var_142);
    }
    #[allow(unused_mut)]let mut scope_143 = writer.prefix("EnableResourceNameDnsARecord");
    if let Some(var_144) = &input.enable_resource_name_dns_a_record {
        scope_143.boolean(*var_144);
    }
    #[allow(unused_mut)]let mut scope_145 = writer.prefix("HostnameType");
    if let Some(var_146) = &input.hostname_type {
        scope_145.string(var_146.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_tag_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::TagSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_147 = writer.prefix("ResourceType");
    if let Some(var_148) = &input.resource_type {
        scope_147.string(var_148.as_str());
    }
    #[allow(unused_mut)]let mut scope_149 = writer.prefix("Tag");
    if let Some(var_150) = &input.tags {
        let mut list_152 = scope_149.start_list(true, Some("item"));
        for item_151 in var_150 {
            #[allow(unused_mut)]let mut entry_153 = list_152.entry();
            crate::query_ser::serialize_structure_crate_model_tag(entry_153, item_151)?;
        }
        list_152.finish();
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_ebs_block_device(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::EbsBlockDevice) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_154 = writer.prefix("DeleteOnTermination");
    if let Some(var_155) = &input.delete_on_termination {
        scope_154.boolean(*var_155);
    }
    #[allow(unused_mut)]let mut scope_156 = writer.prefix("Encrypted");
    if let Some(var_157) = &input.encrypted {
        scope_156.boolean(*var_157);
    }
    #[allow(unused_mut)]let mut scope_158 = writer.prefix("Iops");
    if let Some(var_159) = &input.iops {
        scope_158.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_159).into()));
    }
    #[allow(unused_mut)]let mut scope_160 = writer.prefix("KmsKeyId");
    if let Some(var_161) = &input.kms_key_id {
        scope_160.string(var_161);
    }
    #[allow(unused_mut)]let mut scope_162 = writer.prefix("OutpostArn");
    if let Some(var_163) = &input.outpost_arn {
        scope_162.string(var_163);
    }
    #[allow(unused_mut)]let mut scope_164 = writer.prefix("SnapshotId");
    if let Some(var_165) = &input.snapshot_id {
        scope_164.string(var_165);
    }
    #[allow(unused_mut)]let mut scope_166 = writer.prefix("Throughput");
    if let Some(var_167) = &input.throughput {
        scope_166.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_167).into()));
    }
    #[allow(unused_mut)]let mut scope_168 = writer.prefix("VolumeSize");
    if let Some(var_169) = &input.volume_size {
        scope_168.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_169).into()));
    }
    #[allow(unused_mut)]let mut scope_170 = writer.prefix("VolumeType");
    if let Some(var_171) = &input.volume_type {
        scope_170.string(var_171.as_str());
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_capacity_reservation_target(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::CapacityReservationTarget) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_172 = writer.prefix("CapacityReservationId");
    if let Some(var_173) = &input.capacity_reservation_id {
        scope_172.string(var_173);
    }
    #[allow(unused_mut)]let mut scope_174 = writer.prefix("CapacityReservationResourceGroupArn");
    if let Some(var_175) = &input.capacity_reservation_resource_group_arn {
        scope_174.string(var_175);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_spot_market_options(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::SpotMarketOptions) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_176 = writer.prefix("BlockDurationMinutes");
    if let Some(var_177) = &input.block_duration_minutes {
        scope_176.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_177).into()));
    }
    #[allow(unused_mut)]let mut scope_178 = writer.prefix("InstanceInterruptionBehavior");
    if let Some(var_179) = &input.instance_interruption_behavior {
        scope_178.string(var_179.as_str());
    }
    #[allow(unused_mut)]let mut scope_180 = writer.prefix("MaxPrice");
    if let Some(var_181) = &input.max_price {
        scope_180.string(var_181);
    }
    #[allow(unused_mut)]let mut scope_182 = writer.prefix("SpotInstanceType");
    if let Some(var_183) = &input.spot_instance_type {
        scope_182.string(var_183.as_str());
    }
    #[allow(unused_mut)]let mut scope_184 = writer.prefix("ValidUntil");
    if let Some(var_185) = &input.valid_until {
        scope_184.date_time(var_185, aws_smithy_types::date_time::Format::DateTime)?;
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_ipv4_prefix_specification_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Ipv4PrefixSpecificationRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_186 = writer.prefix("Ipv4Prefix");
    if let Some(var_187) = &input.ipv4_prefix {
        scope_186.string(var_187);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_ipv6_prefix_specification_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::Ipv6PrefixSpecificationRequest) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_188 = writer.prefix("Ipv6Prefix");
    if let Some(var_189) = &input.ipv6_prefix {
        scope_188.string(var_189);
    }
    Ok(())
}

#[allow(unused_mut)]pub fn serialize_structure_crate_model_private_ip_address_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::PrivateIpAddressSpecification) -> Result<(), aws_smithy_http::operation::SerializationError> {
    #[allow(unused_mut)]let mut scope_190 = writer.prefix("Primary");
    if let Some(var_191) = &input.primary {
        scope_190.boolean(*var_191);
    }
    #[allow(unused_mut)]let mut scope_192 = writer.prefix("PrivateIpAddress");
    if let Some(var_193) = &input.private_ip_address {
        scope_192.string(var_193);
    }
    Ok(())
}

