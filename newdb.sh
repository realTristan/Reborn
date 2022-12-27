#!bin/bash
echo "Clearing Database.."
sqlx database drop --database-url "sqlite:database.db"
y
echo "Database Dropped.."
sqlx database create --database-url "sqlite:database.db"
echo "Created New Database.."
sqlx migrate run --database-url "sqlite:database.db"
echo "Successfully Cleared Database!"