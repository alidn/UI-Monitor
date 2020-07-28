select *
from main.reports
where session_id = $1
order by timestamp asc;