// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_app_instance_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAppInstanceInput,
) {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1);
    }
    if let Some(var_2) = &input.metadata {
        object.key("Metadata").string(var_2);
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3);
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
            {
                let mut object_7 = array_5.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_7, item_6);
                object_7.finish();
            }
        }
        array_5.finish();
    }
}

pub fn serialize_structure_crate_input_create_app_instance_admin_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAppInstanceAdminInput,
) {
    if let Some(var_8) = &input.app_instance_admin_arn {
        object.key("AppInstanceAdminArn").string(var_8);
    }
}

pub fn serialize_structure_crate_input_create_app_instance_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAppInstanceUserInput,
) {
    if let Some(var_9) = &input.app_instance_arn {
        object.key("AppInstanceArn").string(var_9);
    }
    if let Some(var_10) = &input.app_instance_user_id {
        object.key("AppInstanceUserId").string(var_10);
    }
    if let Some(var_11) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_11);
    }
    if let Some(var_12) = &input.metadata {
        object.key("Metadata").string(var_12);
    }
    if let Some(var_13) = &input.name {
        object.key("Name").string(var_13);
    }
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("Tags").start_array();
        for item_16 in var_14 {
            {
                let mut object_17 = array_15.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_17, item_16);
                object_17.finish();
            }
        }
        array_15.finish();
    }
}

pub fn serialize_structure_crate_input_put_app_instance_retention_settings_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutAppInstanceRetentionSettingsInput,
) {
    if let Some(var_18) = &input.app_instance_retention_settings {
        let mut object_19 = object.key("AppInstanceRetentionSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_app_instance_retention_settings(
            &mut object_19,
            var_18,
        );
        object_19.finish();
    }
}

pub fn serialize_structure_crate_input_update_app_instance_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAppInstanceInput,
) {
    if let Some(var_20) = &input.metadata {
        object.key("Metadata").string(var_20);
    }
    if let Some(var_21) = &input.name {
        object.key("Name").string(var_21);
    }
}

pub fn serialize_structure_crate_input_update_app_instance_user_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAppInstanceUserInput,
) {
    if let Some(var_22) = &input.metadata {
        object.key("Metadata").string(var_22);
    }
    if let Some(var_23) = &input.name {
        object.key("Name").string(var_23);
    }
}

pub fn serialize_structure_crate_model_tag(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) {
    if let Some(var_24) = &input.key {
        object.key("Key").string(var_24);
    }
    if let Some(var_25) = &input.value {
        object.key("Value").string(var_25);
    }
}

pub fn serialize_structure_crate_model_app_instance_retention_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AppInstanceRetentionSettings,
) {
    if let Some(var_26) = &input.channel_retention_settings {
        let mut object_27 = object.key("ChannelRetentionSettings").start_object();
        crate::json_ser::serialize_structure_crate_model_channel_retention_settings(
            &mut object_27,
            var_26,
        );
        object_27.finish();
    }
}

pub fn serialize_structure_crate_model_channel_retention_settings(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChannelRetentionSettings,
) {
    if let Some(var_28) = &input.retention_days {
        object.key("RetentionDays").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_28).into()),
        );
    }
}
