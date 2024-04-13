# Entelur

Telegram bot for managing expenses and splitting them between friends and family.

# Building

```
cargo build
```

# Running

Currently only SQLite backend is supported. Sqlite driver code is bundled in the executable via rusqlite so no additional dependencies required.

```
Usage: entelur [OPTIONS] --backend <BACKEND> --connection-string <CONNECTION_STRING>

Options:
  -p, --parallel-readers <PARALLEL_READERS>
          Number parallel readers allowed in DB (Only for SQLite) [default: 5]
  -b, --backend <BACKEND>
          [possible values: sqlite, inmemory]
  -c, --connection-string <CONNECTION_STRING>
          
  -h, --help
          Print help
  -V, --version
          Print version
```

To run using sqlite use the following - 

```
./entelur -b sqlite -c ~/.data/db.sqlite
```