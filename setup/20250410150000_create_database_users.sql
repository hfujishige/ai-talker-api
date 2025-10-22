-- Create read-write user for API
DO $$
DECLARE
    rw_password TEXT := COALESCE(current_setting('app.api_user_rw_password', true), 'q4p05yOt9V9g');
    ro_password TEXT := COALESCE(current_setting('app.api_user_ro_password', true), 'ovag1YeMGqwU');
BEGIN
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'api_user_rw') THEN
        EXECUTE format('CREATE USER api_user_rw WITH PASSWORD %L', rw_password);
        COMMENT ON ROLE api_user_rw IS 'API user with read-write permissions';
    END IF;
    
    IF NOT EXISTS (SELECT FROM pg_catalog.pg_roles WHERE rolname = 'api_user_ro') THEN
        EXECUTE format('CREATE USER api_user_ro WITH PASSWORD %L', ro_password);
        COMMENT ON ROLE api_user_ro IS 'API user with read-only permissions';
    END IF;
END
$$;

-- Priviledges assignment for api_user_ro

-- grant privileges to asterisk database
GRANT CONNECT ON DATABASE asterisk TO api_user_ro;

-- grant privileges on asterisk database
\c asterisk
-- Grant read-only privileges to api_user_ro
GRANT USAGE ON SCHEMA public TO api_user_ro;
GRANT SELECT ON ALL TABLES IN SCHEMA public TO api_user_ro;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO api_user_ro;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT ON TABLES TO api_user_ro;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO api_user_ro;

-- Grant privileges to access PostgreSQL system catalogs for table existence checks
GRANT SELECT ON pg_catalog.pg_tables TO api_user_ro;
GRANT SELECT ON information_schema.tables TO api_user_ro;
GRANT SELECT ON information_schema.columns TO api_user_ro;

GRANT SELECT ON TABLE ps_auths TO api_user_ro;
GRANT SELECT ON TABLE ps_aors TO api_user_ro;
GRANT SELECT ON TABLE ps_endpoints TO api_user_ro;
GRANT SELECT ON TABLE ps_registrations TO api_user_ro;


-- Priviledges assignment for api_user_rw
-- grant privileges to asterisk database
GRANT CONNECT ON DATABASE asterisk TO api_user_rw;

-- grant privileges on asterisk database
\c asterisk
-- Grant read-write privileges to api_user_rw
GRANT USAGE ON SCHEMA public TO api_user_rw;
GRANT CREATE ON SCHEMA public TO api_user_rw;
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO api_user_rw;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO api_user_rw;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO api_user_rw;
ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT USAGE, SELECT ON SEQUENCES TO api_user_rw;

-- Grant privileges for migration tables (sqlx uses _sqlx_migrations table)
GRANT CREATE ON DATABASE asterisk TO api_user_rw;

-- Grant privileges to access PostgreSQL system catalogs for table existence checks
GRANT SELECT ON pg_catalog.pg_tables TO api_user_rw;
GRANT SELECT ON information_schema.tables TO api_user_rw;
GRANT SELECT ON information_schema.columns TO api_user_rw;

-- asterisk pjsip realtime tables
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE ps_auths TO api_user_rw;
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE ps_aors TO api_user_rw;
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE ps_endpoints TO api_user_rw;
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLE ps_registrations TO api_user_rw;