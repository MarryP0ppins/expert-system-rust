-- Your SQL goes here

ALTER TABLE rules DROP CONSTRAINT questionRuleGroup_rules_fkey;
DROP TABLE questionRuleGroups;

CREATE TABLE rule_answer (
    id serial NOT NULL,
    answer_id integer NOT NULL,
    rule_id integer NOT NULL,
    CONSTRAINT answers_rule_answers_fkey FOREIGN KEY (answer_id)
        REFERENCES public.answers (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT rules_rule_answers_fkey FOREIGN KEY (rule_id)
        REFERENCES public.rules (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT rule_answer_pkey PRIMARY KEY (answer_id, rule_id)
);