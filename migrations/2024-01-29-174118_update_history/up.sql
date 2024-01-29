-- Your SQL goes here
ALTER TABLE histories DROP CONSTRAINT users_histories_fkey;
ALTER TABLE histories DROP CONSTRAINT systems_histories_fkey;
ALTER TABLE questions DROP CONSTRAINT systems_questions_fkey;
ALTER TABLE answers DROP CONSTRAINT questions_answers_fkey;

ALTER TABLE histories RENAME COLUMN "user" TO user_id;
ALTER TABLE histories RENAME COLUMN system TO system_id;
ALTER TABLE questions RENAME COLUMN system TO system_id;
ALTER TABLE answers RENAME COLUMN question TO question_id;

ALTER TABLE histories ADD CONSTRAINT users_histories_fkey FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID;

ALTER TABLE histories ADD CONSTRAINT systems_histories_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID;

ALTER TABLE questions ADD CONSTRAINT systems_questions_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID;

ALTER TABLE answers ADD CONSTRAINT questions_answers_fkey FOREIGN KEY (question_id)
        REFERENCES public.questions (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID;