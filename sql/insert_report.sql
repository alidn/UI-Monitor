with project as (
    select project_id from main.projects
    where access_key = $1
)
insert into main.reports (project_id, session_id, timestamp)
select project.project_id, $2, $3
from project
returning report_id;