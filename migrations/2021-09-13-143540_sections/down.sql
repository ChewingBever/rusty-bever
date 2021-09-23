-- This file should undo anything in `up.sql`
drop trigger insert_enforce_post_titles on posts;
drop trigger update_enforce_post_titles on posts;
drop function enforce_post_titles;

drop table posts cascade;
drop table sections cascade;
