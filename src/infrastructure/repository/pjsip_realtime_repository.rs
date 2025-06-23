use axum::http::StatusCode;
use sqlx::{Postgres, Transaction};

use crate::infrastructure::models::{
    errors::registration_error::RegistrationError,
    pjsip_realtime::sip_udp::{PsAorForUdp, PsAuthForUdp, PsEndpointForUdp},
};

// registration method
pub async fn exec_insert_udp_pjsip_account(
    transaction: &mut Transaction<'_, Postgres>,
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
    // NOTE: This requires the enum definitions to have the correct `#[sqlx(type_name = "...")]` attribute.
    let auth_insert = r#"
        insert into ps_auths (id, auth_type, password, username)
        values ($1, $2::pjsip_auth_type_values_v2, $3, $4)"#;
    let aor_insert: &str = r#"
        insert into ps_aors (id, default_expiration, max_contacts, minimum_expiration,
                             qualify_frequency, maximum_expiration, qualify_timeout,
                             remove_existing, remove_unavailable)
        values ($1, $2, $3, $4, $5, $6, $7, $8::ast_bool_values, $9::ast_bool_values)"#;
    let endpoint_insert: &str = r#"
        insert into ps_endpoints (id, transport, aors, context, disallow, allow, direct_media,
                                  force_rport, rewrite_contact, rtp_symmetric, media_encryption,
                                  from_domain, from_user, dtmf_mode)
        values ($1, $2::transport_type, $3, $4, $5, $6, $7::ast_bool_values,
                $8::ast_bool_values, $9::ast_bool_values, $10::ast_bool_values, $11::pjsip_media_encryption_values,
                $12, $13, $14::pjsip_dtmf_mode_values_v3)"#;
    // TODO : define result types MySqlQueryResult to PgQueryResult after migrate mysql to postgres)

    let auth_result = sqlx::query(auth_insert)
        .bind(auth.id)
        .bind(auth.auth_type.to_string())
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
        .bind(aor.remove_existing.to_string())
        .bind(aor.remove_unavailable.to_string())
        .execute(&mut **transaction)
        .await
        .map_err(RegistrationError::from)?;
    let endpoint_result = sqlx::query(endpoint_insert)
        .bind(endpoint.id)
        .bind(endpoint.transport.to_string())
        .bind(endpoint.aors)
        .bind(endpoint.context)
        .bind(endpoint.disallow)
        .bind(endpoint.allow)
        .bind(endpoint.direct_media.to_string())
        .bind(endpoint.force_rport.to_string())
        .bind(endpoint.rewrite_contact.to_string())
        .bind(endpoint.rtp_symmetric.to_string())
        .bind(endpoint.media_encryption.to_string())
        .bind(endpoint.from_domain)
        .bind(endpoint.from_user)
        .bind(endpoint.dtmf_mode.to_string())
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
    transaction: &mut Transaction<'_, Postgres>,
    account_id: String,
) -> Result<(), sqlx::Error> {
    // テーブル名を複数形に修正
    sqlx::query("DELETE FROM ps_endpoints WHERE id = $1")
        .bind(&account_id)
        .execute(&mut **transaction)
        .await?;

    sqlx::query("DELETE FROM ps_aors WHERE id = $1")
        .bind(&account_id)
        .execute(&mut **transaction)
        .await?;

    sqlx::query("DELETE FROM ps_auths WHERE id = $1")
        .bind(&account_id)
        .execute(&mut **transaction)
        .await?;

    Ok(())
}
