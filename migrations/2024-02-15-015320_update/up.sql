-- Your SQL goes here

DROP TABLE questionRuleGroup_answer;
/*
ALTER TABLE questionrulegroup_answer DROP CONSTRAINT questionRuleGroups_questionRuleGroup_answer_fkey;

ALTER TABLE questionRuleGroup_answer RENAME rule_answer;

ALTER TABLE rule_answer RENAME COLUMN questionRuleGroup_id TO rule_id;

ALTER TABLE rule_answer ADD CONSTRAINT rules_rule_answers_fkey FOREIGN KEY (rule_id)
        REFERENCES public.rule (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID;

ALTER TABLE rule_answer ADD CONSTRAINT rule_answer_pkey PRIMARY KEY (answer_id, rule_id);

DROP TABLE questionRuleGroups;

*/