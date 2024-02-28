-- Your SQL goes here

ALTER TABLE "public"."clauses" 
  ADD COLUMN "question_id" int4 NOT NULL DEFAULT 3,
  ADD CONSTRAINT "questions_clauses_fkey" FOREIGN KEY ("question_id") REFERENCES "public"."questions" ("id") ON DELETE CASCADE ON UPDATE NO ACTION;

ALTER TABLE "public"."systems" 
  ALTER COLUMN "image_uri" TYPE varchar(128) COLLATE "pg_catalog"."default" USING "image_uri"::varchar(128);