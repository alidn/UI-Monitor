with maybe_inserted_tag as (
    insert into main.tags
        (name)
        values ($1)
        on conflict do nothing
        returning tag_id, name)
        ,
     u as (
         select tag_id, name
         from maybe_inserted_tag
         union
         (select tag_id, name from main.tags where name = $2)
     )
select tag_id, name
from u
where tag_id is not null
