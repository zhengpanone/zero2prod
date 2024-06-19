-- Add down migration script here

drop index counter_records_counter_id_index ;

drop table counter_records;

DROP index counters_user_id_index;

DROP TABLE counters;