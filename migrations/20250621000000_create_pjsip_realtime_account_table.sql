/*
 This SQL script creates a table named "pjsip_realtime_accounts" based on the PjsipRealtimeAccountWithId struct.
 The table stores PJSIP realtime account configuration data with the following columns:
 - id: VARCHAR primary key for the account identifier (ULID)
 - username: VARCHAR for the SIP username
 - password: VARCHAR for the SIP password
 - transport: VARCHAR for the transport type (UDP, TCP, TLS, WS, WSS)
 - context: VARCHAR for the dialplan context
 - from_domain: VARCHAR for the From header domain
 - from_user: VARCHAR for the From header user
 - created_at: timestamp for record creation
 - updated_at: timestamp for record modification
*/

-- Drop the table if it exists
DROP TABLE IF EXISTS pjsip_realtime_accounts;

-- Create the pjsip_realtime_accounts table
CREATE TABLE pjsip_realtime_accounts (
    id VARCHAR(255) PRIMARY KEY,  -- ULID is 26 characters
    username VARCHAR(50) NOT NULL,
    password VARCHAR(255) NOT NULL,
    transport VARCHAR(10) NOT NULL
                          CHECK (transport IN ('udp', 'tcp', 'tls', 'ws', 'wss')),
    context VARCHAR(100) NOT NULL,
    from_domain VARCHAR(255) NOT NULL,
    from_user VARCHAR(50) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes for faster lookups
CREATE INDEX idx_pjsip_accounts_username 
          ON pjsip_realtime_accounts(username);
CREATE INDEX idx_pjsip_accounts_transport 
          ON pjsip_realtime_accounts(transport);
CREATE INDEX idx_pjsip_accounts_context 
          ON pjsip_realtime_accounts(context);
CREATE INDEX idx_pjsip_accounts_from_domain 
          ON pjsip_realtime_accounts(from_domain);

-- Create a unique constraint on username to prevent duplicates
CREATE UNIQUE INDEX idx_pjsip_accounts_username_unique 
                 ON pjsip_realtime_accounts(username);
