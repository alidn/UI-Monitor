with inserted_project as (
    insert into main.projects (name)
        values ($1) returning project_id, name, access_key
)
   , inserted_membership as (
    insert
        into main.memberships (user_id, project_id)
            select $2, inserted_project.project_id
            from inserted_project
            returning project_id
)
select inserted_project.name, inserted_project.access_key
from inserted_project
