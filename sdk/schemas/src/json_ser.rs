// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_discoverer_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateDiscovererInput,
) {
    if input.cross_account {
        object.key("CrossAccount").boolean(input.cross_account);
    }
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1);
    }
    if let Some(var_2) = &input.source_arn {
        object.key("SourceArn").string(var_2);
    }
    if let Some(var_3) = &input.tags {
        let mut object_4 = object.key("tags").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5).string(value_6);
            }
        }
        object_4.finish();
    }
}

pub fn serialize_structure_crate_input_create_registry_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRegistryInput,
) {
    if let Some(var_7) = &input.description {
        object.key("Description").string(var_7);
    }
    if let Some(var_8) = &input.tags {
        let mut object_9 = object.key("tags").start_object();
        for (key_10, value_11) in var_8 {
            {
                object_9.key(key_10).string(value_11);
            }
        }
        object_9.finish();
    }
}

pub fn serialize_structure_crate_input_create_schema_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSchemaInput,
) {
    if let Some(var_12) = &input.content {
        object.key("Content").string(var_12);
    }
    if let Some(var_13) = &input.description {
        object.key("Description").string(var_13);
    }
    if let Some(var_14) = &input.tags {
        let mut object_15 = object.key("tags").start_object();
        for (key_16, value_17) in var_14 {
            {
                object_15.key(key_16).string(value_17);
            }
        }
        object_15.finish();
    }
    if let Some(var_18) = &input.r#type {
        object.key("Type").string(var_18.as_str());
    }
}

pub fn serialize_structure_crate_input_get_discovered_schema_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetDiscoveredSchemaInput,
) {
    if let Some(var_19) = &input.events {
        let mut array_20 = object.key("Events").start_array();
        for item_21 in var_19 {
            {
                array_20.value().string(item_21);
            }
        }
        array_20.finish();
    }
    if let Some(var_22) = &input.r#type {
        object.key("Type").string(var_22.as_str());
    }
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) {
    if let Some(var_23) = &input.policy {
        object.key("Policy").string(var_23);
    }
    if let Some(var_24) = &input.revision_id {
        object.key("RevisionId").string(var_24);
    }
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) {
    if let Some(var_25) = &input.tags {
        let mut object_26 = object.key("tags").start_object();
        for (key_27, value_28) in var_25 {
            {
                object_26.key(key_27).string(value_28);
            }
        }
        object_26.finish();
    }
}

pub fn serialize_structure_crate_input_update_discoverer_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateDiscovererInput,
) {
    if input.cross_account {
        object.key("CrossAccount").boolean(input.cross_account);
    }
    if let Some(var_29) = &input.description {
        object.key("Description").string(var_29);
    }
}

pub fn serialize_structure_crate_input_update_registry_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRegistryInput,
) {
    if let Some(var_30) = &input.description {
        object.key("Description").string(var_30);
    }
}

pub fn serialize_structure_crate_input_update_schema_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSchemaInput,
) {
    if let Some(var_31) = &input.client_token_id {
        object.key("ClientTokenId").string(var_31);
    }
    if let Some(var_32) = &input.content {
        object.key("Content").string(var_32);
    }
    if let Some(var_33) = &input.description {
        object.key("Description").string(var_33);
    }
    if let Some(var_34) = &input.r#type {
        object.key("Type").string(var_34.as_str());
    }
}
