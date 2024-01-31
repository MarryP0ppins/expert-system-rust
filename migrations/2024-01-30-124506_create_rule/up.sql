-- Your SQL goes here
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