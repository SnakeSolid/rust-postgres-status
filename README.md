# PgRestore Status

Tool to monitor PostgreSQL database creation dates and size.

## Usage

To start postgres-status with default configuration:

```bash
./pgrestore-web
```

Optional arguments:

* `-a` (`--address`) ADDR: Address to listen on, default value - localhost;
* `-p` (`--port`) PORT: Port to listen on, default value - 8080;
* `-c` (`--config`) PATH: Path to configuration file, default value - config.yaml;
* `-h` (`--help`): Show help and exit.

## Dependencies

This utility requires administrative rights for used PostgreSQL role. This requirement related with using
file access function `pg_stat_file` to check modification date of `PG_VERSION` file.

## Configuration Example

This utility requires only access to execute queries on the server. So disk size must be defined in configuration file.
Parameter `server.disk.offset` required to correctly calculate used disk space. Used space is defined as following:

```
disk.used = server.disk.offset + sum(database.size for database in databases)
```

Simple configuration example:

```yaml
---
update_interval: 3600 # update interval in seconds

server: # serve description
  disk: # server disk description
    offset: 1000 # fixed size offset
    capacity: 6250907656192 # disk capacity
    soft_threshold: 5000726124953 # soft limit (when progress bar is yellow)
    hard_threshold: 5625816890572 # hard limit (when progress bar is red)
  host: "localhost" # server host name
  port: 5432 # server port
  role: "anaconda" # role
  password: "anaconda" # password
  service_databases: # list of service databases (will be gray in table)
    - postgres
    - template0
    - template1
```
