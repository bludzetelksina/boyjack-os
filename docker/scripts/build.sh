#!/bin/bash
set -e

echo "=== Запуск сборки BoyJack OS ==="

# Опционально: обновляем зависимости Rust
cargo update

# Сборка ядра (предполагается, что есть отдельный крейт или путь)
echo "Сборка ядра..."
cargo build --manifest-path=kernel/Cargo.toml --release

# Сборка пользовательских компонентов
echo "Сборка пользовательских компонентов..."
cargo build --manifest-path=userland/Cargo.toml --release

# Если есть другие компоненты, добавить сюда их сборку
# echo "Сборка других компонентов..."
# cargo build --manifest-path=...

echo "=== Сборка завершена успешно ==="
