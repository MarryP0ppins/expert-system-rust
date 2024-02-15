-- Your SQL goes here

ALTER TABLE rules RENAME COLUMN attribute_rule_group_id TO system_id;
ALTER TABLE rules DROP COLUMN question_rule_group_id;
ALTER TABLE rules  ADD CONSTRAINT systems_rules_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID;