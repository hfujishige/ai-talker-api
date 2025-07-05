use sqlx::PgPool;

// test helper function to create a test app
pub(crate) async fn reset_pjsip_realtime_database(pool: &PgPool) {
    sqlx::query("DELETE FROM pjsip_realtime_accounts")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM ps_auths")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM ps_aors")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM ps_endpoints")
        .execute(pool)
        .await
        .unwrap();
}
