#!/usr/bin/env zsh

# sip-udp signal test accounts
typeset -A tuser1
typeset -A tuser2
typeset -A tuser3

tuser1=(id '1001' \
       username 'udp_user' \
       password '1234567890' \
       transport 'udp' \
       context 'from-sbc' \
       from_domain 'example.com' \
       from_user '1001')
tuser2=(id '1002' \
        username '1002' \
        password '1234567890' \
        transport 'udp' \
        context 'from-sbc' \
        from_domain 'example.com' \
        from_user '1002')
tuser3=(id '1003' \
        username '1003' \
        password '1234567890' \
        transport 'udp' \
        context 'from-sbc' \
        from_domain 'example.jp' \
        from_user '1003')

# Array of user variable names
user_names=(tuser1 tuser2 tuser3)

# server
schema=http
host=127.0.0.1:3000

# execute
for user_name in ${user_names[@]}; do
    # Get the associative array by name
    typeset -A current_user
    eval "current_user=(\${(kv)${user_name}})"
    
    echo "Creating account: ${current_user[id]} (${current_user[username]})"
    
    http POST "${schema}://${host}/api/v1/pjsip_realtime/accounts_with_id" \
    id="${current_user[id]}" \
    username="${current_user[username]}" \
    password="${current_user[password]}" \
    transport="${current_user[transport]}" \
    context="${current_user[context]}" \
    from_domain="${current_user[from_domain]}" \
    from_user="${current_user[from_user]}"
    
    echo "---"
done
