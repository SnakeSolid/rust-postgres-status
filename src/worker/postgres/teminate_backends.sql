select
  pg_terminate_backend(sa.pid)
from pg_stat_activity as sa
where sa.datname = $1
