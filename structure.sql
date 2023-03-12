begin;

create table if not exists installation(
    installation_id serial primary key,
    name text not null,
    station text
);

create table if not exists vigicrues(
    time timestamptz,
    installation_id int references installation(installation_id),
    level float4,
    flow float4,

    primary key(time, installation_id)
);

commit;
