#!/usr/bin/env zsh
# test params
typeset -A tuser

# for unit test
# tuser=(id unit1234567890 \
#        username 'taro yamada' \
#        password 123456 \
#        transport udp \
#        context default \
#        from_domain 'from_domain' \
#        from_user 'from_user')

# for kamailio+asterisk sip signal integ test.
tuser=(id 1001 \
       username 'sipintegtest' \
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
