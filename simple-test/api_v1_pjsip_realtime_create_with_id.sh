#!/usr/bin/env zsh
# test params
typeset -A tuser

# Add SIP account
tuser=(id 1001 \
       username '1001' \
       password 1234567890 \
       transport udp \
       context default \
       from_domain 'example.com' \
       from_user '1001')

# server
schema=http
host=127.0.0.1:3000

# execute
http POST "${schema}://${host}/api/v1/pjsip_realtime/accounts_with_id" \
  id="${tuser[id]}" \
  username="${tuser[username]}" \
  password="${tuser[password]}" \
  transport="${tuser[transport]}" \
  context="${tuser[context]}" \
  from_domain="${tuser[from_domain]}" \
  from_user="${tuser[from_user]}"
