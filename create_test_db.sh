#!/bin/bash
rm -f test.db
cat test.dump.sqlite.sql | sqlite3 test.db