#!/usr/bin/env bash

DB_USER="root"
DB_PASSWORD="h6redmine"
DB_NAME="lokoda"
DB_PORT="3600"

export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@localhost/${DB_NAME}

sqlx database create
sqlx migrate run
