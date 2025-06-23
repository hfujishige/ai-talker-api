-- AuthType
CREATE TYPE auth_type AS ENUM ('userpass', 'md5', 'other');

-- DtmfMode
CREATE TYPE dtmf_mode AS ENUM ('auto', 'rfc2833', 'info', 'inband', 'none');

-- MediaEncryption
CREATE TYPE media_encryption AS ENUM ('no', 'sdes', 'dtls', 'zrtp');

-- TransportType
CREATE TYPE transport_type AS ENUM ('udp', 'tcp', 'tls', 'ws', 'wss');

-- TurnOnOff
CREATE TYPE turn_on_off AS ENUM ('yes', 'no');
