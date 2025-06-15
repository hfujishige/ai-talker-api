#!/usr/bin/env zsh
http POST http://localhost:3000/api/v1/pjsip_realtime/accounts \
  username="taro yamada" \
  password="123456" \
  transport="TransportUdp" \
  context="default" \
  from_domain="" \
  from_user=""
