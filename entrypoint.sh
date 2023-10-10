#!/bin/bash
set -e 

echo "*** Initializing Setup Environment ***"

user=$1
echo "User is: $user"

echo "*** Deleting Old State ***"
delete_old_state (){
    /usr/bin/node purge-chain --base-path /tmp/$user --chain local -y || echo "no state found"
}

echo "*** Starting Node ***"
run_node (){
    if [ $user == "alice" ]; then
        /usr/bin/node \
        --base-path /tmp/alice \
        --chain local \
        --alice \
        --port 30333 \
        --rpc-port 9944 \
        --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
        --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
        --validator
        echo "initialised Alice Node"
    elif [ $user == "bob" ]; then
        until /usr/bin/node \
            --base-path /tmp/bob \
            --chain local \
            --bob \
            --port 30334 \
            --rpc-port 9946 \
            --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
            --validator \
            --bootnodes /dns/provider1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp 
        do     
	        sleep 1
        done
        echo "initialised Bob Node" 
    elif [ $user == "charlie" ]; then
        until /usr/bin/node \
            --base-path /tmp/charlie \
            --chain local \
            --charlie \
            --port 30334 \
            --rpc-port 9946 \
            --telemetry-url "wss://telemetry.polkadot.io/submit/ 0" \
            --validator \
            --bootnodes /dns/provider1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp 
        do     
	        sleep 1
        done
        echo "initialised Charlie Node" 
    fi      
}

delete_old_state && run_node

