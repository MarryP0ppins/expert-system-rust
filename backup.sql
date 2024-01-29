----USER_MODEL----

CREATE TABLE users (
    id serial NOT NULL,
    email character varying(32) NOT NULL,
    username character varying(16) NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    first_name character varying(16) NOT NULL,
    last_name character varying(16) NOT NULL,
    is_superuser boolean NOT NULL DEFAULT false,
    password character varying(16) NOT NULL,
    CONSTRAINT id_users_pkey PRIMARY KEY (id),
    CONSTRAINT email_users_unique UNIQUE (email)
        INCLUDE(email),
    CONSTRAINT username_users_unique UNIQUE (username)
        INCLUDE(username)
);

----SYSTEM_MODEL----

CREATE TABLE systems (
    id serial NOT NULL,
    user_id integer NOT NULL,
    about text,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    name character varying(128) NOT NULL,
    private boolean NOT NULL DEFAULT True,
    CONSTRAINT id_systems_pkey PRIMARY KEY (id),
    CONSTRAINT name_systems_unique UNIQUE (name)
        INCLUDE(name),
    CONSTRAINT users_systems_fkey FOREIGN KEY (user_id)
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

CREATE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON systems
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

----HISTORY_MODEL----

CREATE TABLE histories (
    id serial NOT NULL,
    system_id integer NOT NULL,
    user_id integer NOT NULL,
    started_at timestamp NOT NULL DEFAULT NOW(),
    finished_at timestamp NOT NULL DEFAULT NOW(),
    answered_questions character varying(8) NOT NULL DEFAULT '0/0',
    results json NOT NULL,
    CONSTRAINT id_histories_pkey PRIMARY KEY (id),
    CONSTRAINT users_histories_fkey FOREIGN KEY (user_id)
        REFERENCES public."users" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT systems_histories_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

----QUESTION_MODEL----

CREATE TABLE questions (
    id serial NOT NULL,
    system_id integer NOT NULL, 
    body character varying(64) NOT NULL,
    with_chooses boolean NOT NULL DEFAULT False,
    CONSTRAINT id_questions_pkey PRIMARY KEY (id),
    CONSTRAINT systems_questions_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

----ANSWER_MODEL----

CREATE TABLE answers
(
    id serial NOT NULL,
    question_id integer NOT NULL,
    body character varying(128) NOT NULL,
    CONSTRAINT id_answers_pkey PRIMARY KEY (id),
    CONSTRAINT questions_answers_fkey FOREIGN KEY (question_id)
        REFERENCES public.questions (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);