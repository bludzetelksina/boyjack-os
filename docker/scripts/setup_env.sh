#!/bin/bash
set -e

echo "=== Установка окружения для сборки BoyJack OS ==="

# Обновление пакетов
apt-get update && apt-get upgrade -y

# Установка базовых инструментов
apt-get install -y build-essential curl git wget pkg-config libssl-dev

# Установка Rust (через rustup)
if ! command -v rustc &> /dev/null; then
    echo "Установка Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    echo "Rust уже установлен"
fi

# Проверка установки Rust
rustc --version
cargo --version

# Установка nasm для сборки ASM модулей
apt-get install -y nasm

# Установка Docker CLI (если контейнер поддерживает Docker внутри Docker)
if ! command -v docker &> /dev/null; then
    echo "Установка Docker CLI..."
    apt-get install -y apt-transport-https ca-certificates curl gnupg-agent software-properties-common
    curl -fsSL https://download.docker.com/linux/debian/gpg | apt-key add -
    add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/debian $(lsb_release -cs) stable"
    apt-get update
    apt-get install -y docker-ce-cli
else
    echo "Docker CLI уже установлен"
fi

echo "=== Окружение установлено успешно ==="
