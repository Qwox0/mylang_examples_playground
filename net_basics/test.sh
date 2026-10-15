#!/usr/bin/env bash

set -e

script_dir="$(dirname "$(readlink -f "$0")")"

info() {
    echo -e "\e[92;1m+++\e[0m $1"
}

warn() {
    echo -e "\e[93;1m+++ WARN:\e[0m $1"
}

error() {
    echo -e "\e[91;1m+++ ERROR:\e[0m $1" >&2
    if [ ${2:-1} -ne 0 ]; then exit ${2:-1}; fi
}

server_out="./test_server.log"
client_out="./test_client.log"
send_delay=0.1

yellow=$'\e[33;1m'
cyan=$'\e[36;1m'
off=$'\e[0m'

cd "$script_dir"

set -x

variant="$1"
[ -d "$variant" ]

mylang build "$variant/server.mylang" --diagnostic-level=error
mylang build "$variant/client.mylang" --diagnostic-level=error

set +x

kill_decendents() {
    local children=$(pgrep -P $1 || true)
    for c in $children; do
        local child=$c
        kill_decendents $child
        kill -INT $child 2>/dev/null || true
    done
}
trap "kill_decendents $$" INT TERM EXIT

info "starting server"
{ stdbuf -oL -eL "./$variant/out/server" | tee "$server_out" | sed "s/^/$cyan[server]$off /"; } &
server_pid=$!
echo $server_pid

sleep 0.1

info "starting client"
{
    set -x
    sleep $send_delay
    echo "First message"
    sleep $send_delay
    echo "Second message"
    sleep $send_delay
    echo "quit"
    echo "ignored input"
    sleep 5
} | stdbuf -o0 -e0 "./$variant/out/client" |
    tee "$client_out" |
    sed "s/^/$yellow[client]$off /"

kill_decendents $server_pid

not() {
    local cmd=("$@")
    if "${cmd[@]}"; then false; else true; fi

}

info "verify $server_out"
(
    set -x
    grep "Listening on 0.0.0.0:12345" "$server_out" >/dev/null
    grep "Waiting for new connection ..." "$server_out" >/dev/null
    grep "New connection: [0-9]\+" "$server_out" >/dev/null
    grep "Waiting on connection [0-9]\+" "$server_out" >/dev/null
    grep "received 13 bytes\( from connection [0-9]\+\)\?: 'First message'" "$server_out" >/dev/null
    grep "wrote 13 bytes" "$server_out" >/dev/null
    grep "Waiting on connection [0-9]\+" "$server_out" >/dev/null
    grep "received 14 bytes from connection [0-9]\+: 'Second message'" "$server_out" >/dev/null
    grep "wrote 14 bytes" "$server_out" >/dev/null
    grep "Closing connection [0-9]\+ ..." "$server_out" >/dev/null

    not grep "quit" "$server_out" >/dev/null
    not grep "ignored input" "$client_out" >/dev/null
)

info "verify $client_out"
(
    set -x
    grep "Connecting to 127.0.0.1:12345" "$client_out" >/dev/null
    grep "Message from the server: 'Hello from the server'" "$client_out" >/dev/null
    grep "Sending message: 'First message'" "$client_out" >/dev/null
    grep "Message from the server: 'First message'" "$client_out" >/dev/null
    grep "Sending message: 'Second message'" "$client_out" >/dev/null
    grep "Message from the server: 'Second message'" "$client_out" >/dev/null
    grep "Closing socket ..." "$client_out" >/dev/null

    not grep "Sending message: 'quit'" "$client_out" >/dev/null
    not grep "ignored input" "$client_out" >/dev/null
    not grep "Server ended the connection" "$client_out" >/dev/null
)
