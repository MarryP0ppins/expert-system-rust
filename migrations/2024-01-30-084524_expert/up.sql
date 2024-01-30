-- Your SQL goes here
ALTER TABLE questionRuleGroup_answer DROP CONSTRAINT questionRuleGroup_answer_pkey;

ALTER TABLE questionRuleGroup_answer RENAME COLUMN questionRuleGroup_id TO question_rule_group_id;

ALTER TABLE questionRuleGroup_answer ADD CONSTRAINT questionRuleGroup_answer_pkey PRIMARY KEY (answer_id, question_rule_group_id);