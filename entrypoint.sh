#!/usr/bin/env bash
# This script is meant to be run on Unix/Linux based systems
set -e 

echo "*** Initializing Setup Environment ***"

user=$1
echo "User is: $user"

echo "*** Deleting Old State ***"
delete_old_state (){
    /usr/bin/node purge-chain --base-path /tmp/$user --chain local -y
}

echo "*** Starting Node ***"
run_node (){
    if [ $user == "alice" ]; then
        ./target/release/node-template \
        --base-path /tmp/alice \
        --chain local \
        --alice \
        --port 30333 \
        --rpc-port 9944 \
        --ws-external \
        --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
        --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
        --validator
    elif [ $user == "bob"]; then
        until ./target/release/node-template \
            --base-path /tmp/bob \
            --chain local \
            --bob \
            --port 30334 \
            --rpc-port 9946 \
            --ws-external \
            --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
            --validator \
            --bootnodes /ip4/provider1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp  > /dev/null 2>&1
        do     
	        sleep 1
        done 
    fi      
}

delete_old_state && run_node