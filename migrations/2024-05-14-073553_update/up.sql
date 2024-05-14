-- Your SQL goes here
ALTER TABLE "public"."clauses" 
  ALTER COLUMN "logical_group" TYPE varchar(36) USING "logical_group"::varchar(36);