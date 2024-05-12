-- Your SQL goes here
ALTER TABLE "public"."attributesvalue_object" 
  DROP CONSTRAINT "attributesvalue_object_pkey",
  ADD COLUMN "attribute_id" int4 NOT NULL,
  ADD CONSTRAINT "attributesvalue_object_pkey" PRIMARY KEY ("object_id", "attribute_value_id", "attribute_id"),
  ADD CONSTRAINT "attributes_attribute_fkey" FOREIGN KEY ("attribute_id") REFERENCES "public"."attributes" ("id") ON DELETE CASCADE ON UPDATE NO ACTION;