#!/usr/bin/env zsh
account_id="testid123456"
http DELETE http://localhost:3000/api/v1/pjsip_realtime/accounts/${account_id}
