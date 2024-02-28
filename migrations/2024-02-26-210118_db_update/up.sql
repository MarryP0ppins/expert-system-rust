-- Your SQL goes here
ALTER TABLE "public"."rule_answer" 
  DROP CONSTRAINT "rule_answer_pkey",
  ADD COLUMN "question_id" int4 NOT NULL,
  ADD CONSTRAINT "rule_answer_pkey" PRIMARY KEY ("answer_id", "rule_id", "question_id"),
  ADD CONSTRAINT "questions_rule_answers_fkey" FOREIGN KEY ("question_id") REFERENCES "public"."questions" ("id") ON DELETE CASCADE ON UPDATE NO ACTION;


ALTER TABLE "public"."rule_attributevalue" 
  DROP CONSTRAINT "rule_attribute_value_pkey",
  ADD COLUMN "attribute_id" int4 NOT NULL,
  ADD CONSTRAINT "rule_attribute_value_pkey" PRIMARY KEY ("attribute_value_id", "rule_id", "attribute_id"),
  ADD CONSTRAINT "attributes_rule_attributevalues_fkey" FOREIGN KEY ("attribute_id") REFERENCES "public"."attributes" ("id") ON DELETE CASCADE ON UPDATE NO ACTION;