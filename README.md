# Системное программирование на Rust, Копанов Антон, Задача 4

## Описание

Этот проект представляет собой клиент-серверное приложение, написанное на языке программирования Rust. Сервер принимает
подключения от клиентов, обрабатывает их запросы и ведет логирование событий и производительности.
В этом проекте я сфокусировался на использовании асинхронных возможностей Rust, а также на работе с сетью и
многопоточностью, использовав библиотеку tokio и Rust.

## Установка

1. Убедитесь, что у вас установлен Rust и Cargo. Если нет, следуйте инструкциям
   на [официальном сайте Rust](https://www.rust-lang.org/).

2. Клонируйте репозиторий проекта:
    ```sh
    git clone <URL_репозитория>
    cd <имя_проекта>
    ```

3. Установите зависимости:
    ```sh
    cargo build
    ```

## Запуск сервера

Для запуска сервера используйте следующую команду:

```sh
cargo run --bin server -- --address <адрес> --log-file <имя_файла_лога>
```

Запуск клиента
Для запуска клиента используйте следующую команду:

```sh
cargo run --bin client -- --address <адрес>
```

## Логирование

Сервер использует библиотеку tracing для логирования событий и производительности. Логи сохраняются в файл, указанный в
параметре --log-file.  
С помощью него можно анализировать все происходящие события и производительность сервера, а также статистику по запросам
клиентов.
По умолчанию логи сохраняются в файле server.log, а адерс сервера - 127.0.0.1:8080.

## Использование

Для сервера нужно дождаться, пока клиент подключится к нему.
После этого ввести число, чтобы начать угадывание.

Для клиента нужно дождаться, пока сервер напишет сообщение о начале игры.
Затем вводить числа, чтобы угадать число.

