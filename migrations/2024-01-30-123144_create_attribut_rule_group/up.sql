-- Your SQL goes here
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