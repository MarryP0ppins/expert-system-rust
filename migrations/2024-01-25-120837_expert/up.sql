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
    "user" integer NOT NULL,
    about text,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    name character varying(128) NOT NULL,
    private boolean NOT NULL DEFAULT True,
    CONSTRAINT id_systems_pkey PRIMARY KEY (id),
    CONSTRAINT name_systems_unique UNIQUE (name)
        INCLUDE(name),
    CONSTRAINT users_systems_fkey FOREIGN KEY ("user")
        REFERENCES public.users (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

--DROP FUNCTION IF EXISTS trigger_set_timestamp;
CREATE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

--DROP TRIGGER IF EXISTS set_timestamp;
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON systems
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

----HISTORY_MODEL----

CREATE TABLE histories (
    id serial NOT NULL,
    system integer NOT NULL,
    "user" integer NOT NULL,
    answered_questions character varying(8) NOT NULL DEFAULT '0/0',
    results json NOT NULL,
    CONSTRAINT id_histories_pkey PRIMARY KEY (id),
    CONSTRAINT users_histories_fkey FOREIGN KEY ("user")
        REFERENCES public."users" (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID,
    CONSTRAINT systems_histories_fkey FOREIGN KEY (system)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);

----QUESTION_MODEL----

CREATE TABLE questions (
    id serial NOT NULL,
    system integer NOT NULL, 
    body character varying(64) NOT NULL,
    with_chooses boolean NOT NULL DEFAULT False,
    CONSTRAINT id_questions_pkey PRIMARY KEY (id),
    CONSTRAINT systems_questions_fkey FOREIGN KEY (system)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
);