# Fullstack template

A template to write fullstack applications with Rust and VueJS

## Start Server

- Start development Server: `docker compose up --build -d`
- Start production Server: `docker compose -f docker-compose.yml up --build -d`

## Access components

- Database-Dev: `localhost:5432`
- Backend-Dev: `localhost:8080`
- Frontend-Dev: `localhost:3000`
- Frontend-Prod: `localhost:8000`

The Backend and the Database are not accessable from the outside, if the compose is startet in a production environment.
