#!/usr/bin/env bash

function caller() {
  #env -0 | sort -z | tr '\0' '\n'
  # 如果只有一个参数且包含特殊字符，视为完整命令
  if [[ $# -eq 1 && ($1 == *"|"* || $1 == *"&&"* || $1 == *">"* || $1 == *"<"*) ]]; then
    ebc_info "执行复合命令: $1"
    bash -c "$1"
  else
    ebc_info "执行命令: $*"
    "$@"
  fi
}

function ebc_success() {
  echo -e "\e[1;32m$*\e[0m"
}

function ebc_info() {
  echo -e "\e[1;36m$*\e[0m"
}

function ebc_warn() {
  echo -e "\e[1;33m$*\e[0m"
}

function ebc_error() {
  echo -e "\e[1;31m$*\e[0m"
}

function ebc_debug() {
  echo -e "\e[1;35m$*\e[0m"
}

function load_envs() {
  local load_file=$1
  local env_filter=$2
  echo -e "load_file: $load_file, env_filter: $env_filter"

  [ ! -f "$load_file" ] && echo "部署环境配置不存在: $load_file,请配置部署环境变量!" && exit

  #export $(grep $env_filter "$load_file" | xargs)
  mapfile -t vars < <(grep "$env_filter" "$load_file")
  export "${vars[@]}"

  #env | grep $env_filter
  env -0 | sort -z | tr '\0' '\n' | grep -a "$env_filter"
  #ENV_VAR=$(printf '${%s} ' $(env | grep $env_filter | cut -d'=' -f1))
  #echo $ENV_VAR
}
