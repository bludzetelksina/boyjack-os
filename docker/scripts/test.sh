#!/bin/bash
set -e

echo "=== Запуск тестов BoyJack OS ==="

# Компиляция ядра и компонентов
echo "Сборка проекта..."
cargo build --all

# Запуск модульных тестов (Rust)
echo "Запуск модульных тестов..."
cargo test --all

# Проверка наличия бинарников
echo "Проверка сгенерированных бинарных файлов..."
if [ -f "target/debug/kernel" ]; then
    echo "Ядро собрано успешно."
else
    echo "Ошибка: ядро не собрано!"
    exit 1
fi

if [ -f "target/debug/userland_shell" ]; then
    echo "Shell собрано успешно."
else
    echo "Ошибка: shell не собран!"
    exit 1
fi

# Здесь можно добавить другие проверки, например, запуск эмулятора QEMU (если настроено)
# echo "Запуск тестовой виртуальной машины..."
# qemu-system-x86_64 -kernel target/debug/kernel -serial stdio -display none -no-reboot -monitor none & sleep 10
# Проверка логов и статуса запуска

echo "=== Все тесты выполнены успешно ==="
