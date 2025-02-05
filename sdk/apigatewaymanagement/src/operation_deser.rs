// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_connection_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteConnectionOutput, crate::error::DeleteConnectionError>
{
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DeleteConnectionError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DeleteConnectionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ForbiddenException" => {
            crate::error::DeleteConnectionError {
                meta: generic,
                kind: crate::error::DeleteConnectionErrorKind::ForbiddenException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::forbidden_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_forbidden_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DeleteConnectionError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "GoneException" => crate::error::DeleteConnectionError {
            meta: generic,
            kind: crate::error::DeleteConnectionErrorKind::GoneException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::gone_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_gone_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteConnectionError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "LimitExceededException" => crate::error::DeleteConnectionError {
            meta: generic,
            kind: crate::error::DeleteConnectionErrorKind::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DeleteConnectionError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DeleteConnectionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_connection_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteConnectionOutput, crate::error::DeleteConnectionError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_connection_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_connection_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetConnectionOutput, crate::error::GetConnectionError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetConnectionError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetConnectionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ForbiddenException" => {
            crate::error::GetConnectionError {
                meta: generic,
                kind: crate::error::GetConnectionErrorKind::ForbiddenException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::forbidden_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_forbidden_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetConnectionError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "GoneException" => crate::error::GetConnectionError {
            meta: generic,
            kind: crate::error::GetConnectionErrorKind::GoneException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::gone_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_gone_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetConnectionError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "LimitExceededException" => crate::error::GetConnectionError {
            meta: generic,
            kind: crate::error::GetConnectionErrorKind::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetConnectionError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetConnectionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_connection_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetConnectionOutput, crate::error::GetConnectionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_connection_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_connection(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetConnectionError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_post_to_connection_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::PostToConnectionOutput, crate::error::PostToConnectionError>
{
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::PostToConnectionError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::PostToConnectionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ForbiddenException" => {
            crate::error::PostToConnectionError {
                meta: generic,
                kind: crate::error::PostToConnectionErrorKind::ForbiddenException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::forbidden_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_forbidden_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::PostToConnectionError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "GoneException" => crate::error::PostToConnectionError {
            meta: generic,
            kind: crate::error::PostToConnectionErrorKind::GoneException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::gone_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_gone_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::PostToConnectionError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "LimitExceededException" => crate::error::PostToConnectionError {
            meta: generic,
            kind: crate::error::PostToConnectionErrorKind::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_limit_exceeded_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::PostToConnectionError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "PayloadTooLargeException" => crate::error::PostToConnectionError {
            meta: generic,
            kind: crate::error::PostToConnectionErrorKind::PayloadTooLargeException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::payload_too_large_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_payload_too_large_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::PostToConnectionError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::PostToConnectionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_post_to_connection_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::PostToConnectionOutput, crate::error::PostToConnectionError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::post_to_connection_output::Builder::default();
        let _ = response;
        output.build()
    })
}
