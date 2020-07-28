select tags.name, tags.tag_id
from main.reports
inner join main.report_tags using (report_id)
inner join main.tags using (tag_id)
where report_id = $1;