-- Your SQL goes here

ALTER TABLE rules DROP COLUMN compared_value;
ALTER TABLE rules DROP COLUMN logical_group;
ALTER TABLE rules DROP COLUMN operator;

CREATE TABLE clauses (
    id serial NOT NULL,
    rule_id integer,
    compared_value character varying(64) NOT NULL,
    logical_group integer NOT NULL,
    operator OperatorEnum NOT NULL,
    CONSTRAINT id_clauses_pkey PRIMARY KEY (id),
    CONSTRAINT rules_clauses_fkey FOREIGN KEY (rule_id)
        REFERENCES public.rules (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);