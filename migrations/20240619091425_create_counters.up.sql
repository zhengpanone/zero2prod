-- Add up migration script here
create table counters(
  id integer primary key not null,
  user_id integer not null,
  name text not null,
  value integer not null,
  step integer not null,
  input_step integer not null,
  sequence integer not null,
  create_at timestamp not null default CURRENT_TIMESTAMP,
  update_at timestamp not null default CURRENT_TIMESTAMP
);

create index counters_user_id_index on counters (user_id asc, sequence desc);

create table counter_records(
  id integer primary key not null,
  counter_id integer not null,
  step integer not null, 
  'begin' integer not null,
  'end' integer not null,
  create_at timestamp not null default CURRENT_TIMESTAMP,
  update_at timestamp not null default CURRENT_TIMESTAMP
);

create index counter_records_counter_id_index on counter_records (counter_id);