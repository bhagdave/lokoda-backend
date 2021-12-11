#!/usr/bin/env bash

DB_USER=${MYSQL_USER:=root}
DB_PASSWORD="${MYSQL_PASSWORD:=h6redmine}"
DB_NAME="${MYSQL_DB:=lokoda}"
DB_PORT="${MYSQL_PORT:=3600}"

export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

sqlx database create
sqlx migrate run
