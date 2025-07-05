#!/usr/bin/env zsh
http POST http://127.0.0.1:3000/api/v1/pjsip_realtime/accounts_with_id \
  id="unit1234567890" \
  username="taro yamada" \
  password="123456" \
  transport="udp" \
  context="default" \
  from_domain="from_domain" \
  from_user="from_user"
