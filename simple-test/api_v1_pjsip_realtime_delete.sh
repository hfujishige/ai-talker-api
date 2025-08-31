#!/usr/bin/env zsh

# test values
# account_id="testid123456"
account_id="1001"

# execute unit test
http DELETE http://localhost:3000/api/v1/pjsip_realtime/accounts/${account_id}
