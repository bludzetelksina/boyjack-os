#!/bin/bash
set -e

CONTAINER_NAME="boyjackos_build"
OUTPUT_DIR="../output"

echo "=== Запуск контейнера для сборки BoyJack OS ==="
docker run --name $CONTAINER_NAME boyjackos_image

echo "=== Копирование артефактов из контейнера ==="
mkdir -p $OUTPUT_DIR

# Копируем собранный образ OS (например, iso или img)
docker cp $CONTAINER_NAME:/boyjackos/build/boyjackos.iso $OUTPUT_DIR/

# Копируем бинарники ядра и пользовательских приложений
docker cp $CONTAINER_NAME:/boyjackos/build/kernel.bin $OUTPUT_DIR/
docker cp $CONTAINER_NAME:/boyjackos/build/userland_apps $OUTPUT_DIR/ -r

echo "=== Очистка контейнера ==="
docker rm $CONTAINER_NAME

echo "=== Развертывание завершено. Файлы находятся в $OUTPUT_DIR ==="
