#!/usr/bin/env zsh

# test values
tuser_ids=("1001" "1002" "1003")

# execute unit test
for tuser_id in ${tuser_ids[@]}; do
    echo "Deleting account with ID: ${tuser_id}"
    http DELETE http://localhost:3000/api/v1/pjsip_realtime/accounts/${tuser_id}
done
