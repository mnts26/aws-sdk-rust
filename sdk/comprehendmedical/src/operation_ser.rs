// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_describe_entities_detection_v2_job(
    input: &crate::input::DescribeEntitiesDetectionV2JobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_entities_detection_v2_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_icd10_cm_inference_job(
    input: &crate::input::DescribeIcd10CmInferenceJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_icd10_cm_inference_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_phi_detection_job(
    input: &crate::input::DescribePhiDetectionJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_phi_detection_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_describe_rx_norm_inference_job(
    input: &crate::input::DescribeRxNormInferenceJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_describe_rx_norm_inference_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_detect_entities(
    input: &crate::input::DetectEntitiesInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_detect_entities_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_detect_entities_v2(
    input: &crate::input::DetectEntitiesV2Input,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_detect_entities_v2_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_detect_phi(
    input: &crate::input::DetectPhiInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_detect_phi_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_infer_icd10_cm(
    input: &crate::input::InferIcd10CmInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_infer_icd10_cm_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_infer_rx_norm(
    input: &crate::input::InferRxNormInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_infer_rx_norm_input(&mut object, input);
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_entities_detection_v2_jobs(
    input: &crate::input::ListEntitiesDetectionV2JobsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_entities_detection_v2_jobs_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_icd10_cm_inference_jobs(
    input: &crate::input::ListIcd10CmInferenceJobsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_icd10_cm_inference_jobs_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_phi_detection_jobs(
    input: &crate::input::ListPhiDetectionJobsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_phi_detection_jobs_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_list_rx_norm_inference_jobs(
    input: &crate::input::ListRxNormInferenceJobsInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_list_rx_norm_inference_jobs_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_entities_detection_v2_job(
    input: &crate::input::StartEntitiesDetectionV2JobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_entities_detection_v2_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_icd10_cm_inference_job(
    input: &crate::input::StartIcd10CmInferenceJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_icd10_cm_inference_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_phi_detection_job(
    input: &crate::input::StartPhiDetectionJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_phi_detection_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_start_rx_norm_inference_job(
    input: &crate::input::StartRxNormInferenceJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_start_rx_norm_inference_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_stop_entities_detection_v2_job(
    input: &crate::input::StopEntitiesDetectionV2JobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_stop_entities_detection_v2_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_stop_icd10_cm_inference_job(
    input: &crate::input::StopIcd10CmInferenceJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_stop_icd10_cm_inference_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_stop_phi_detection_job(
    input: &crate::input::StopPhiDetectionJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_stop_phi_detection_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_stop_rx_norm_inference_job(
    input: &crate::input::StopRxNormInferenceJobInput,
) -> Result<smithy_http::body::SdkBody, smithy_types::Error> {
    let mut out = String::new();
    let mut object = smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::json_ser::serialize_structure_crate_input_stop_rx_norm_inference_job_input(
        &mut object,
        input,
    );
    object.finish();
    Ok(smithy_http::body::SdkBody::from(out))
}
