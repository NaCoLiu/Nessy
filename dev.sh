#!/bin/bash
# filepath: cargo-watch-ignore.sh

# 检测是否在 PowerShell 中运行
if [[ -n "$PSModulePath" || -n "$POWERSHELL_DISTRIBUTION_CHANNEL" ]]; then
    # PowerShell 环境
    echo "在 PowerShell 中运行..."
    
    # 获取命令行参数，默认为 run
    if [ -z "$1" ]; then
        COMMAND="run"
    else
        COMMAND="$1"
    fi
    
    # 执行 cargo watch
    cargo watch -i config.json -x "$COMMAND"
else
    # Bash 环境
    echo "在 Bash 中运行..."
    
    # 获取命令行参数，默认为 run
    COMMAND=${1:-run}
    
    # 执行 cargo watch
    cargo watch -i config.json -x "$COMMAND"
fi