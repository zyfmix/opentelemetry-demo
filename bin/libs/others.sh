#!/usr/bin/env bash
set -e

source "${BASH_SOURCE%/*}/headers.sh"

function parse_iirii_server() {
    Server=$1

    [[ ! "$Server" =~ ^[^@]+@[^:]+:[0-9]+$ ]] && ebc_error "Server format error: $Server, 参数格式: <user>@<*>.iirii.com:<port>" && exit

    ssh_user=${Server%@*}
    ssh_server=${Server##*@}
    server_host=${ssh_server%:*}
    server_port=${ssh_server##*:}
    server_target=${server_host%".iirii.com"}

    [[ "$OSTYPE" == "linux-gnu"* ]] && server_path=$(echo "$server_target" | tr '.' '\n' | tac | paste -sd.)
    [[ "$OSTYPE" == "darwin"* ]]  && server_path=$(echo "$server_target" | tr '.' '\n' | tac | gpaste -s -d '.')

    ebc_debug "[Server: $Server] Extract ssh_user: $ssh_user, ssh_server: $ssh_server, server_host: $server_host, server_port: $server_port, server_target: $server_target, server_path: $server_path"

    local -n ref2=$2
    local -n ref3=$3
    local -n ref4=$4
    local -n ref5=$5
    local -n ref6=$6

    ref2=$ssh_user
    ref3=$server_host
    ref4=$server_port
    ref5=$server_target
    ref6=$server_path
}
