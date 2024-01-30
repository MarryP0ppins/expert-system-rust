-- Your SQL goes here
CREATE TABLE questionRuleGroups (
    id serial NOT NULL,
    system_id integer NOT NULL,
    CONSTRAINT id_questionRuleGroup_pkey PRIMARY KEY (id)
);


CREATE TABLE questionRuleGroup_answer (
    id serial NOT NULL,
    answer_id integer NOT NULL,
    questionRuleGroup_id integer NOT NULL,
    CONSTRAINT answers_questionRuleGroup_answer_fkey FOREIGN KEY (answer_id)
        REFERENCES public.answers (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT questionRuleGroups_questionRuleGroup_answer_fkey FOREIGN KEY (questionRuleGroup_id)
        REFERENCES public.questionRuleGroups (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT questionRuleGroup_answer_pkey PRIMARY KEY (answer_id, questionRuleGroup_id)
);