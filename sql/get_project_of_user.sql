select projects.name, projects.access_key
from main.users
inner join main.memberships using (user_id)
inner join main.projects using (project_id)
where user_id = $1;