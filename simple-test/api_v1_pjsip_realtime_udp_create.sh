#!/usr/bin/env zsh
http POST http://127.0.0.1:3000/api/v1/pjsip_realtime/accounts \
  username="1001" \
  password="1234567890" \
  transport="udp" \
  context="from-sipproxy" \
  from_domain="example.com" \
  from_user="1001"
