-- Your SQL goes here
ALTER TABLE AttributeRuleGroup_atributeValue DROP CONSTRAINT attributeRuleGroup_atributeValue_pkey;

ALTER TABLE AttributeRuleGroup_atributeValue DROP CONSTRAINT attributeRuleGroup_atributeValue_attributeRuleGroups_fkey;


ALTER TABLE rules DROP CONSTRAINT attributeRuleGroup_rules_fkey;
DROP TABLE AttributeRuleGroups;
DROP TABLE AttributeRuleGroup_atributeValue;

CREATE TABLE rule_attributevalue (
    id serial NOT NULL,
    attribute_value_id integer NOT NULL,
    rule_id integer NOT NULL,
    CONSTRAINT atribute_values_rule_attributevalues_fkey FOREIGN KEY (attribute_value_id)
        REFERENCES public.attributesvalues (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT rules_rule_attributevalues_fkey FOREIGN KEY (rule_id)
        REFERENCES public.rules (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT rule_attribute_value_pkey PRIMARY KEY (attribute_value_id, rule_id)
);