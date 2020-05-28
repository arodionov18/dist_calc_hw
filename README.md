# Интернет магазин
Базовый API для интернет магазина

## Архитектура
![Архитектура](https://github.com/arodionov18/dist_calc_hw/raw/hw4/architecture.png)

## Методы REST API
Должны быть реализованые следующие методы:
* добавить новый товар;
* редактировать товар по `id`;
* удалить товар по `id`;
* получить определенный товар по `id`;
* получить полный список товаров.

## Товар
Описание товара состоит из:
- названия;
- уникального кода (`id`)
- категории число или название

## Postman
https://documenter.getpostman.com/view/10599009/SzRxUpUG?version=latest

## Запуск
Для запуска требуется собрать докер образ и запустить через `docker-compose`

```
cd hw1
docker build -t hw1_app:latest . && docker-compose up
```
## Бонусные баллы
Реализовано хранение в бд и пагинация.
