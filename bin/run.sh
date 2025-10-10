#!/usr/bin/env bash
set -e

sc_dir="$(
  cd "$(dirname "$0")" >/dev/null 2>&1 || exit
  pwd -P
)"

rs_path=${sc_dir/opentelemetry-demo*/opentelemetry-demo}
source $rs_path/bin/libs/headers.sh

Case=${1:-"run"}

ebc_debug "解析命令参数> run.sh $Case"

shopt -s expand_aliases
alias dc='docker-compose -f docker-compose.yml --env-file .env'
#alias dc='docker-compose -f docker-compose-os.yml --env-file .env'

case "$Case" in
help)
  ebc_debug "说明: run.sh 命令快捷参数"
  ebc_debug "用法: run.sh <Case>"
  ebc_debug "示例: run.sh restart"
  ;;
info)
    host=http://localhost:8080
    host=http://0.wh.zsc.iirii.com:8815
    ebc_debug "Web store: $host/"
    ebc_debug "Grafana: $host/grafana/"
    ebc_debug "Load Generator UI: $host/loadgen/"
    ebc_debug "Jaeger UI: $host/jaeger/ui/"
    ebc_debug "Tracetest UI: http://localhost:11633/, only when using make run-tracetesting"
    ebc_debug "Flagd configurator UI: $host/feature"
    ;;
restart)
    #export ENVOY_PORT=8815
    #docker compose up --force-recreate --remove-orphans --detach

  dc down

  #dc up -d --pull=always
  dc up -d

  dc logs -f
 ;;
exec)
  ebc_debug "dc exec -it otel-collector /otelcol-contrib --help"
  dc exec -it otel-collector /otelcol-contrib --help
 ;;
ps)
  dc ps -a
 ;;
logs)
  dc logs -f
 ;;
stop)
  dc down
  docker ps -a
 ;;
*)
  echo "[参数命令不合法]case: $Case [remove]"
  exit 1
  ;;
esac
