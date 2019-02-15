select
  db.datname as name,
  extract(epoch from (pg_stat_file('base/' || db.oid || '/PG_VERSION')).modification)::bigint as date,
  pg_database_size(datname) as size
from pg_database as db
