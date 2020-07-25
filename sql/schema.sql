create table if not exists main.users
(
    user_id  serial primary key,
    username varchar(50),
    email    varchar(254) not null unique,
    password varchar(20)  not null
);

create table if not exists main.projects
(
    project_id serial primary key,
    access_key uuid not null unique default main.gen_random_uuid(),
    name       varchar(20)
);

create table if not exists main.memberships
(
    user_id    integer not null,
    project_id integer not null,
    primary key (user_id, project_id),
    foreign key (user_id) references main.users (user_id),
    foreign key (project_id) references main.projects (project_id)
);

create table if not exists main.reports
(
    report_id  serial primary key,
    project_id integer     not null references main.projects (project_id),
    session_id uuid unique not null,
    timestamp  timestamp   not null default current_timestamp
);

create table if not exists main.tags
(
    tag_id serial primary key,
    name   varchar(20)
);

create table if not exists main.report_tags
(
    report_id integer not null references main.reports (report_id),
    tag_id    integer not null references main.tags (tag_id),
    primary key (report_id, tag_id)
);
