#!/usr/bin/bash
set -xe
#set -o pipefail

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
DB_USER=${POSTGRES_USER:=donkec}
# Check if a custom password has been set, otherwise default to 'password'
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database name has been set, otherwise default to 'newsletter'
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a custom port has been set, otherwise default to '5432'
DB_PORT="${POSTGRES_PORT:=5432}"

if [[ "${SKIP_DOCKER}" ]]; then
    # Launch postgres using Docker
    docker run \
        -e POSTGRES_USER=${DB_USER} \              # set env variables to be used inside container
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \      # set env variables to be used inside container
        -e POSTGRES_DB=${DB_NAME} \                # set env variables to be used inside container
        -p "${DB_PORT}":5432 \                     # map database port
        --name subs_docker                         # container name
        -d postgres \                              # detach from terminal, so it doesn't lock. "postgres" is image to load
        postgres -N 1000                           # Increased maximum number of connections for testing purposes
fi

# example how to run pgadmin using Docker
# docker run \
#   -e PGADMIN_DEFAULT_EMAIL=admin@example.com \
#   -e PGADMIN_DEFAULT_PASSWORD=securepass \
#   -p 5050:80 \
#   --name pgadmin4_visualizer \
#   -d dpage/pgadmin4

echo "IP for PgAdmin4"
echo "$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' optimistic_bose)"

# Keep pinging Postgres until it's ready to accept commands
until PGPASSWORD="${DB_PASSWORD}" psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
# Creates the database specified in your DATABASE_URL
sqlx database create
sqlx migrate run
>&2 echo "Postgres has been migrated, ready to go!"