use axum::http::StatusCode;
use sqlx::{MySql, Transaction};

use crate::infrastructure::models::{
    errors::{deletion_error::DeletionError, registration_error::RegistrationError},
    pjsip_realtime::sip_udp::{PsAorForUdp, PsAuthForUdp, PsEndpointForUdp},
};

// registration method
pub async fn exec_insert_udp_pjsip_account(
    transaction: &mut Transaction<'_, MySql>,
    auth: PsAuthForUdp,
    aor: PsAorForUdp,
    endpoint: PsEndpointForUdp,
) -> Result<StatusCode, RegistrationError> {
    // Validate the input data
    // 事前バリデーション
    if auth.id.is_empty() || aor.id.is_empty() || endpoint.id.is_empty() {
        return Err(RegistrationError::ValidationError(
            "ID cannot be empty".to_string(),
        ));
    }

    // Insert SQL statements for pjsip_realtime tables with placeholders
    let auth_insert = r#"
        insert into ps_auths (id, auth_type, password, username)
        values (?, ?, ?, ?)"#;
    let aor_insert: &str = r#"
        insert into ps_aors (id, default_expiration, max_contacts, minimum_expiration,
                             qualify_frequency, maximum_expiration, qualify_timeout)
        values (?, ?, ?, ?, ?, ?, ?)"#;
    let endpoint_insert: &str = r#"
        insert into ps_endpoints (id, transport, aors, context, disallow, allow, direct_media,
                                  force_rport, rewrite_contact, rtp_symmetric, media_encryption,
                                  from_domain, from_user)
        values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#;
    // TODO : define result types MySqlQueryResult to PgQueryResult after migrate mysql to postgres
    // Execute the insert queries
    let auth_result = sqlx::query(auth_insert)
        .bind(auth.id)
        .bind(auth.auth_type)
        .bind(auth.password)
        .bind(auth.username)
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let aor_result = sqlx::query(aor_insert)
        .bind(aor.id)
        .bind(aor.default_expiration)
        .bind(aor.max_contacts)
        .bind(aor.minimum_expiration)
        .bind(aor.qualify_frequency)
        .bind(aor.maximum_expiration)
        .bind(aor.qualify_timeout)
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let endpoint_result = sqlx::query(endpoint_insert)
        .bind(endpoint.id)
        .bind(endpoint.transport)
        .bind(endpoint.aors)
        .bind(endpoint.context)
        .bind(endpoint.disallow)
        .bind(endpoint.allow)
        .bind(endpoint.direct_media)
        .bind(endpoint.force_rport)
        .bind(endpoint.rewrite_contact)
        .bind(endpoint.rtp_symmetric)
        .bind(endpoint.media_encryption)
        .bind(endpoint.from_domain)
        .bind(endpoint.from_user)
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    // if any of the insertions failed, return an error
    if auth_result.rows_affected() == 0
        || aor_result.rows_affected() == 0
        || endpoint_result.rows_affected() == 0
    {
        return Err(RegistrationError::InsertionFailed);
    }
    // return OK status with a success message
    Ok(StatusCode::CREATED)
}

pub async fn exec_delete_pjsip_account(
    transaction: &mut Transaction<'_, MySql>,
    account_id: String,
) -> Result<StatusCode, DeletionError> {
    if account_id.is_empty() {
        return Err(DeletionError::ValidationError(
            "Account ID cannot be empty".to_string(),
        ));
    }

    // SQL statements to delete from pjsip_realtime tables
    let auth_delete = r#"delete from ps_auths where id = ? "#;
    let aor_delete = r#"delete from ps_aors where id = ? "#;
    let endpoint_delete = r#"delete from ps_endpoints where id = ? "#;

    let auth_result = sqlx::query(auth_delete)
        .bind(&account_id)
        .execute(&mut **transaction)
        .await
        .map_err(DeletionError::from)?;
    let aor_result = sqlx::query(aor_delete)
        .bind(&account_id)
        .execute(&mut **transaction)
        .await
        .map_err(DeletionError::from)?;
    let endpoint_result = sqlx::query(endpoint_delete)
        .bind(&account_id)
        .execute(&mut **transaction)
        .await
        .map_err(DeletionError::from)?;
    // Check if any of the deletions failed
    if auth_result.rows_affected() == 0
        || aor_result.rows_affected() == 0
        || endpoint_result.rows_affected() == 0
    {
        return Err(DeletionError::DeletionFailed);
    }
    // OK Return
    Ok(StatusCode::NO_CONTENT)
}
