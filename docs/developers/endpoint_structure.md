## User Endpoints
# Endpoint Structure
> This document provides a reference for all the supported services

## User Endpoints

| Method   | Endpoint                     | Description                       |
| -------- | ---------------------------- | --------------------------------- |
| `GET`    | `/users`                     | Get all users (maybe admin-only)  |
| `POST`   | `/users`                     | Register a new user               |
| `GET`    | `/users/:id`                 | Get a specific user               |
| `PUT`    | `/users/:id`                 | Update user profile               |
| `DELETE` | `/users/:id`                 | Delete/deactivate a user          |
| `PUT`    | `/users/:id/profile-picture` | Adds/updates user profile picture |
| `DELETE` | `/users/:id/profile-picture` | Removes user profile picture      |

## Session Endpoints

| Method   | Endpoint           | Description                      |
| -------- | ------------------ | -------------------------------- |
| `GET`    | `/session`         | Gets all active sessions         |
| `POST`   | `/session`         | Creates a new session for a user |
| `DELETE` | `/session/:id`     | Deletes a specified session      |

## Trip Invite Endpoints

| **Method** | **Endpoint**             | **Description**                        |
| ---------- | ------------------------ | -------------------------------------- |
| `GET`      | `/trip-invites`          | Get all trip invites                   |
| `POST`     | `/trip-invites`          | Add a trip invite                      |
| `GET`      | `/trip-invites/:tripId`  | Get all trip invites for specific trip |
| `PATCH`    | `/trip-invites/:tripId`  | Change status of trip invite           |
| `DELETE`   | `/trip-invites/:tripId`  | Delete trip invite                     |


## Trip Endpoints
  
| Method   | Endpoint                     | Description                                         |
| -------- | ---------------------------- | --------------------------------------------------- |
| `GET`    | `/trips`                     | Get all trips                                       |`
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
| `GET`    | `/trips/:id/budget-trackers` | Get all budget trackers                             |
| `POST`   | `/trips/:id/budget-trackers` | Create new budget tracker                           |
| `GET`    | `/trips/:id/invites`         | Get all outgoing invites                            |
| `POST`   | `/trips/:id/invites`         | Create new invite                                   |

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
| `GET`      | `/budget-trackers`     | Get all budget trackers     |
| `GET`      | `/budget-trackers/:id` | Get specific budget tracker |
| `PUT`      | `/budget-trackers/:id` | Update budget tracker       |
| `DELETE`   | `/budget-trackers/:id` | Delete budget tracker       |
