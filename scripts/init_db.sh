#!/usr/bin/env bash
set -x
set -e pipefail

if ! [ -x "$(command -v psql)" ]; then
  echo >&2 "Error: psql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

# Check if a custom user has been set, otherwise default to 'postgres'
DB_USER=${POSTGRES_USER:=postgres}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"


# set env variables to be used inside container
# set env variables to be used inside container
# set env variables to be used inside container
# map database port
# container name
# detach from terminal, so it doesn't lock. "postgres" is image to load
# Increased maximum number of connections for testing purposes

# Allow to skip Docker if a dockerized Postgres database is already running
if [ -z "${SKIP_DOCKER}" ]
then
  docker run \
      -e POSTGRES_USER=${DB_USER} \
      -e POSTGRES_PASSWORD=${DB_PASSWORD} \
      -e POSTGRES_DB=${DB_NAME} \
      -p "${DB_PORT}":5432 \
      --name subs_docker \
      -d postgres \
      postgres -N 1000
fi

#example how to run pgadmin using Docker
if [ -z "${SKIP_DOCKER}" ]; then
    docker run \
        -e PGADMIN_DEFAULT_EMAIL=admin@example.com \
        -e PGADMIN_DEFAULT_PASSWORD=securepass \
        -p 5050:80 \
        --name pgadmin4_visualizer \
        -d dpage/pgadmin4
fi

echo "postgres IP for PgAdmin4 page on localhost:5050"
echo "$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' subs_docker)"

# Keep pinging Postgres until it's ready to accept commands
until PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
# Creates the database specified in your DATABASE_URL
sqlx database create # ovo nam cak i ne treba jer smo kroz argumente dockeru rekli da napravi postgres database
# sqlx migrate add create_subscriptions_table # ovo treba samo jednom pokrenut
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"