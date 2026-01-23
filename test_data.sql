-- Insert sample data for testing
INSERT INTO pjsip_realtime_accounts (
    id, username, password, transport, context, from_domain, from_user, created_at, updated_at
) VALUES 
(
    '01HX1234567890ABCDEFGHIJKL', 
    'test_user_1', 
    'test_password_1', 
    'UDP', 
    'from-sbc', 
    'example.com', 
    'test_user_1',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
),
(
    '01HX1234567890ABCDEFGHIJKM', 
    'test_user_2', 
    'test_password_2', 
    'TCP', 
    'users', 
    'example.org', 
    'test_user_2',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
),
(
    '01HX1234567890ABCDEFGHIJKN', 
    'test_user_3', 
    'test_password_3', 
    'TLS', 
    'secure', 
    'secure.example.com', 
    'test_user_3',
    CURRENT_TIMESTAMP,
    CURRENT_TIMESTAMP
)
ON CONFLICT (username) DO NOTHING;
