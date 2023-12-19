#!/bin/bash
rm -f database.db
cat dump.sqlite.sql | sqlite3 database.db