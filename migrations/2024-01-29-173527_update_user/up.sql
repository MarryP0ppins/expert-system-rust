-- Your SQL goes here

ALTER TABLE systems DROP CONSTRAINT users_systems_fkey;

ALTER TABLE systems RENAME COLUMN "user" TO user_id;

ALTER TABLE systems ADD CONSTRAINT users_systems_fkey FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID