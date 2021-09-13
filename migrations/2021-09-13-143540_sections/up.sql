-- Your SQL goes here
create table sections (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    -- Title of the section
    title varchar(255) UNIQUE NOT NULL,
    -- Optional description of the section
    description text,
    -- Wether to show the section in the default list on the homepage
    is_default boolean NOT NULL DEFAULT false,
    -- Wether the posts should contain titles or not
    has_titles boolean NOT NULL DEFAULT true
);

create table posts (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY,

    section_id uuid NOT NULL REFERENCES sections(id) ON DELETE CASCADE,
    -- Title of the post
    -- Wether this is NULL or not is enforced using the enforce_post_titles trigger
    title varchar(255),
    -- Post date, defaults to today
    publish_date date NOT NULL DEFAULT now(),
    -- Content of the post
    content text NOT NULL
);

create function enforce_post_titles() returns trigger as $enforce_post_titles$
    begin
        -- Check for a wrongfully null title
        if new.title is null and exists (
            select 1 from sections where id = new.section_id and has_titles
        ) then
            raise exception 'Expected a post title, but got null.';
        end if;

        if new.title is not null and exists (
            select 1 from sections where id = new.section_id and not has_titles
        ) then
            raise exception 'Expected an empty post title, but got a value.';
        end if;

        return new;
    end;
$enforce_post_titles$ language plpgsql;

create trigger insert_enforce_post_titles
    before insert on posts
    for each row
    execute function enforce_post_titles();

create trigger update_enforce_post_titles
    before update of title on posts
    for each row
    when (old.title is distinct from new.title)
    execute function enforce_post_titles();
