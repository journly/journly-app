## User Endpoints
# Endpoint Structure
> This document provides a reference for all the supported services

## User Endpoints

| Method   | Endpoint     | Description                      |
| -------- | ------------ | -------------------------------- |
| `GET`    | `/users`     | Get all users (maybe admin-only) |
| `POST`   | `/users`     | Register a new user              |
| `GET`    | `/users/:id` | Get a specific user              |
| `PUT`    | `/users/:id` | Update user profile              |
| `DELETE` | `/users/:id` | Delete/deactivate a user         |

## Auth Endpoints

| Method | Endpoint       | Description                    |
| ------ | -------------- | ------------------------------ |
| `POST` | `/auth/login`  | Authenticate user              |
| `POST` | `/auth/logout` | Log out                        |
| `GET`  | `/auth/me`     | Get current authenticated user |

## Trip Endpoints

| Method   | Endpoint                     | Description                                         |
| -------- | ---------------------------- | --------------------------------------------------- |
| `GET`    | `/trips`                     | Get all trips                                       |
| `POST`   | `/trips`                     | Create new trip                                     |
| `GET`    | `/trips?user={id}`           | Get all trips for a specific user                   |
| `GET`    | `/trips/:id`                 | Get a specific trip with all its sections + widgets |
| `PUT`    | `/trips/:id`                 | Update trip                                         |
| `DELETE` | `/trips/:id`                 | Delete specific trip                                |
| `GET`    | `/trips/:id/sections`        | Get all sections in a trip                          |
| `PUT`    | `/trips/:id/sections`        | Update sections                                     |
| `DELETE` | `/trips/:id/sections/:id`    | Delete specific section                             |
| `GET`    | `/trips/:id/itineraries`     | Get all trip itineraries                            |
| `POST`   | `/trips/:id/itineraries`     | Create new trip itinerary                           |
| `GET`    | `/trips/:id/budget_trackers` | Get all budget trackers                             |
| `POST`   | `/trips/:id/budget_trackers` | Create new budget tracker                           |

## Itinerary Endpoints

| **Method** | **Endpoint**       | **Description**        |
| ---------- | ------------------ | ---------------------- |
| `GET`      | `/itineraries`     | Get all itineraries    |
| `GET`      | `/itineraries/:id` | Get specific itinerary |
| `PUT`      | `/itineraries/:id` | Update itinerary       |
| `DELETE`   | `/itineraries/:id` | Delete itinerary       |

## Budget Tracker Endpoints


| **Method** | **Endpoint**           | **Description**             |
| ---------- | ---------------------- | --------------------------- |
| `GET`      | `/budget_trackers`     | Get all budget trackers     |
| `GET`      | `/budget_trackers/:id` | Get specific budget tracker |
| `PUT`      | `/budget_trackers/:id` | Update budget tracker       |
| `DELETE`   | `/budget_trackers/:id` | Delete budget tracker       |
