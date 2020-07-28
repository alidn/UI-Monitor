select distinct session_id
from main.reports
where project_id = $1;