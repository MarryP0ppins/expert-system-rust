----USER_MODEL----

CREATE TABLE users (
    id serial NOT NULL,
    email character varying(32) NOT NULL,
    username character varying(16) NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    first_name character varying(16) NOT NULL,
    last_name character varying(16) NOT NULL,
    is_superuser boolean NOT NULL DEFAULT false,
    password character varying(256) NOT NULL,
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

----QUESTION_RULE_GROUP_MODEL----

CREATE TABLE questionRuleGroups (
    id serial NOT NULL,
    system_id integer NOT NULL,
    CONSTRAINT id_questionRuleGroup_pkey PRIMARY KEY (id)
);

----QUESTION_RULE_GROUP_ANSWER_MODEL----

CREATE TABLE questionRuleGroup_answer (
    id serial NOT NULL,
    answer_id integer NOT NULL,
    question_rule_group_id integer NOT NULL,
    CONSTRAINT answers_questionRuleGroup_answer_fkey FOREIGN KEY (answer_id)
        REFERENCES public.answers (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT questionRuleGroups_questionRuleGroup_answer_fkey FOREIGN KEY (question_rule_group_id)
        REFERENCES public.questionRuleGroups (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT questionRuleGroup_answer_pkey PRIMARY KEY (answer_id, question_rule_group_id)
);


CREATE TABLE attributes (
    id serial NOT NULL,
    system_id integer NOT NULL,
    name character varying(128) NOT NULL,
    CONSTRAINT id_attributes_pkey PRIMARY KEY (id),
    CONSTRAINT systems_attributes_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

CREATE TABLE attributesValues (
    id serial NOT NULL,
    attribute_id integer NOT NULL,
    value character varying(128) NOT NULL,
    CONSTRAINT id_attributesValues_pkey PRIMARY KEY (id),
    CONSTRAINT attributes_attributesValues_fkey FOREIGN KEY (attribute_id)
        REFERENCES public.attributes (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

CREATE TABLE objects (
    id serial NOT NULL,
    system_id integer NOT NULL,
    name character varying(128) NOT NULL,
    CONSTRAINT id_objects_pkey PRIMARY KEY (id),
    CONSTRAINT systems_objects_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

CREATE TABLE attributesValue_object (
    id serial NOT NULL,
    object_id integer NOT NULL,
    attribute_value_id integer NOT NULL,
    CONSTRAINT attributesValue_object_objects_fkey FOREIGN KEY (object_id)
        REFERENCES public.objects (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT attributesValues_attribute_value_fkey FOREIGN KEY (attribute_value_id)
        REFERENCES public.attributesValues (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT attributesValue_object_pkey PRIMARY KEY (object_id, attribute_value_id)
);


CREATE TABLE AttributeRuleGroups (
    id serial NOT NULL,
    system_id integer NOT NULL,
    CONSTRAINT id_attributeRuleGroup_pkey PRIMARY KEY (id)
);

CREATE TABLE AttributeRuleGroup_atributeValue (
    id serial NOT NULL,
    attribute_value_id integer NOT NULL,
    attribute_rule_group_id integer NOT NULL,
    CONSTRAINT attributeRuleGroup_atributeValue_atributesValues_fkey FOREIGN KEY (attribute_value_id)
        REFERENCES public.attributesvalues (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT attributeRuleGroup_atributeValue_attributeRuleGroups_fkey FOREIGN KEY (attribute_rule_group_id)
        REFERENCES public.attributeRuleGroups (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT attributeRuleGroup_atributeValue_pkey PRIMARY KEY (attribute_value_id, attribute_rule_group_id)
);


CREATE TYPE OperatorEnum AS ENUM
    ('EQUAL', 'NOT_EQUAL', 'BELOW', 'ABOVE', 'NO_MORE_THAN', 'NO_LESS_THAN');



CREATE TABLE rules (
    id serial NOT NULL,
    attribute_rule_group_id integer,
    question_rule_group_id integer,
    compared_value character varying(64) NOT NULL,
    logical_group integer NOT NULL,
    operator OperatorEnum NOT NULL,
    CONSTRAINT id_rules_pkey PRIMARY KEY (id),
    CONSTRAINT attributeRuleGroup_rules_fkey FOREIGN KEY (attribute_rule_group_id)
        REFERENCES public.attributerulegroups (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT questionRuleGroup_rules_fkey FOREIGN KEY (question_rule_group_id)
        REFERENCES public.questionrulegroups (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);