select distinct tags.name from main.projects
inner join main.reports using(project_id)
inner join main.report_tags using(report_id)
inner join main.tags using (tag_id)
where access_key = $1;