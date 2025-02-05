// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_associate_member_account_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateMemberAccountInput,
) {
    if let Some(var_1) = &input.member_account_id {
        object.key("memberAccountId").string(var_1);
    }
}

pub fn serialize_structure_crate_input_associate_s3_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AssociateS3ResourcesInput,
) {
    if let Some(var_2) = &input.member_account_id {
        object.key("memberAccountId").string(var_2);
    }
    if let Some(var_3) = &input.s3_resources {
        let mut array_4 = object.key("s3Resources").start_array();
        for item_5 in var_3 {
            {
                let mut object_6 = array_4.value().start_object();
                crate::json_ser::serialize_structure_crate_model_s3_resource_classification(
                    &mut object_6,
                    item_5,
                );
                object_6.finish();
            }
        }
        array_4.finish();
    }
}

pub fn serialize_structure_crate_input_disassociate_member_account_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateMemberAccountInput,
) {
    if let Some(var_7) = &input.member_account_id {
        object.key("memberAccountId").string(var_7);
    }
}

pub fn serialize_structure_crate_input_disassociate_s3_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisassociateS3ResourcesInput,
) {
    if let Some(var_8) = &input.member_account_id {
        object.key("memberAccountId").string(var_8);
    }
    if let Some(var_9) = &input.associated_s3_resources {
        let mut array_10 = object.key("associatedS3Resources").start_array();
        for item_11 in var_9 {
            {
                let mut object_12 = array_10.value().start_object();
                crate::json_ser::serialize_structure_crate_model_s3_resource(
                    &mut object_12,
                    item_11,
                );
                object_12.finish();
            }
        }
        array_10.finish();
    }
}

pub fn serialize_structure_crate_input_list_member_accounts_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListMemberAccountsInput,
) {
    if let Some(var_13) = &input.next_token {
        object.key("nextToken").string(var_13);
    }
    if let Some(var_14) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_14).into()),
        );
    }
}

pub fn serialize_structure_crate_input_list_s3_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListS3ResourcesInput,
) {
    if let Some(var_15) = &input.member_account_id {
        object.key("memberAccountId").string(var_15);
    }
    if let Some(var_16) = &input.next_token {
        object.key("nextToken").string(var_16);
    }
    if let Some(var_17) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            smithy_types::Number::NegInt((*var_17).into()),
        );
    }
}

pub fn serialize_structure_crate_input_update_s3_resources_input(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateS3ResourcesInput,
) {
    if let Some(var_18) = &input.member_account_id {
        object.key("memberAccountId").string(var_18);
    }
    if let Some(var_19) = &input.s3_resources_update {
        let mut array_20 = object.key("s3ResourcesUpdate").start_array();
        for item_21 in var_19 {
            {
                let mut object_22 = array_20.value().start_object();
                crate::json_ser::serialize_structure_crate_model_s3_resource_classification_update(
                    &mut object_22,
                    item_21,
                );
                object_22.finish();
            }
        }
        array_20.finish();
    }
}

pub fn serialize_structure_crate_model_s3_resource_classification(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3ResourceClassification,
) {
    if let Some(var_23) = &input.bucket_name {
        object.key("bucketName").string(var_23);
    }
    if let Some(var_24) = &input.prefix {
        object.key("prefix").string(var_24);
    }
    if let Some(var_25) = &input.classification_type {
        let mut object_26 = object.key("classificationType").start_object();
        crate::json_ser::serialize_structure_crate_model_classification_type(
            &mut object_26,
            var_25,
        );
        object_26.finish();
    }
}

pub fn serialize_structure_crate_model_s3_resource(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3Resource,
) {
    if let Some(var_27) = &input.bucket_name {
        object.key("bucketName").string(var_27);
    }
    if let Some(var_28) = &input.prefix {
        object.key("prefix").string(var_28);
    }
}

pub fn serialize_structure_crate_model_s3_resource_classification_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::S3ResourceClassificationUpdate,
) {
    if let Some(var_29) = &input.bucket_name {
        object.key("bucketName").string(var_29);
    }
    if let Some(var_30) = &input.prefix {
        object.key("prefix").string(var_30);
    }
    if let Some(var_31) = &input.classification_type_update {
        let mut object_32 = object.key("classificationTypeUpdate").start_object();
        crate::json_ser::serialize_structure_crate_model_classification_type_update(
            &mut object_32,
            var_31,
        );
        object_32.finish();
    }
}

pub fn serialize_structure_crate_model_classification_type(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ClassificationType,
) {
    if let Some(var_33) = &input.one_time {
        object.key("oneTime").string(var_33.as_str());
    }
    if let Some(var_34) = &input.continuous {
        object.key("continuous").string(var_34.as_str());
    }
}

pub fn serialize_structure_crate_model_classification_type_update(
    object: &mut smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ClassificationTypeUpdate,
) {
    if let Some(var_35) = &input.one_time {
        object.key("oneTime").string(var_35.as_str());
    }
    if let Some(var_36) = &input.continuous {
        object.key("continuous").string(var_36.as_str());
    }
}
