# Используем официальный образ Rust с поддержкой сборки
FROM rust:latest

# Устанавливаем необходимые инструменты для сборки asm и других компонентов
RUN apt-get update && apt-get install -y \
    build-essential \
    nasm \
    llvm \
    clang \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Создаем рабочую директорию внутри контейнера
WORKDIR /usr/src/boyjack-os

# Копируем файлы проекта в контейнер
COPY . .

# Собираем проект в релизной конфигурации
RUN cargo build --release

# Если нужны дополнительные шаги (например, сборка asm модулей), добавьте их здесь
# RUN nasm -f elf64 kernel/arch/x86_64/boot.s -o kernel/arch/x86_64/boot.o

# Указываем команду по умолчанию (например, ничего не запускать, просто сборка)
CMD ["bash"]
