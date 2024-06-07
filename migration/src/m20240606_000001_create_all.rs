use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        // Replace the sample below with your own migration scripts
        db.execute_unprepared(
            "
            DROP TYPE IF EXISTS \"public\".\"operatorenum\";
            CREATE TYPE \"public\".\"operatorenum\" AS ENUM (
            'EQUAL',
            'NOT_EQUAL',
            'BELOW',
            'ABOVE',
            'NO_MORE_THAN',
            'NO_LESS_THAN'
            );

            DROP SEQUENCE IF EXISTS \"public\".\"answers_id_seq\";
            CREATE SEQUENCE \"public\".\"answers_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"attributes_id_seq\";
            CREATE SEQUENCE \"public\".\"attributes_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"attributesvalue_object_id_seq\";
            CREATE SEQUENCE \"public\".\"attributesvalue_object_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"attributesvalues_id_seq\";
            CREATE SEQUENCE \"public\".\"attributesvalues_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"clauses_id_seq\";
            CREATE SEQUENCE \"public\".\"clauses_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"histories_id_seq\";
            CREATE SEQUENCE \"public\".\"histories_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"objects_id_seq\";
            CREATE SEQUENCE \"public\".\"objects_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"questions_id_seq\";
            CREATE SEQUENCE \"public\".\"questions_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"rule_answer_id_seq\";
            CREATE SEQUENCE \"public\".\"rule_answer_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"rule_attributevalue_id_seq\";
            CREATE SEQUENCE \"public\".\"rule_attributevalue_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"rules_id_seq\";
            CREATE SEQUENCE \"public\".\"rules_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"systems_id_seq\";
            CREATE SEQUENCE \"public\".\"systems_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP SEQUENCE IF EXISTS \"public\".\"users_id_seq\";
            CREATE SEQUENCE \"public\".\"users_id_seq\" 
            INCREMENT 1
            MINVALUE  1
            MAXVALUE 2147483647
            START 1
            CACHE 1;

            DROP TABLE IF EXISTS \"public\".\"answers\";
            CREATE TABLE \"public\".\"answers\" (
            \"id\" int4 NOT NULL DEFAULT nextval('answers_id_seq'::regclass),
            \"question_id\" int4 NOT NULL,
            \"body\" varchar(128) COLLATE \"pg_catalog\".\"default\" NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"attributes\";
            CREATE TABLE \"public\".\"attributes\" (
            \"id\" int4 NOT NULL DEFAULT nextval('attributes_id_seq'::regclass),
            \"system_id\" int4 NOT NULL,
            \"name\" varchar(64) COLLATE \"pg_catalog\".\"default\" NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"attributesvalues\";
            CREATE TABLE \"public\".\"attributesvalues\" (
            \"id\" int4 NOT NULL DEFAULT nextval('attributesvalues_id_seq'::regclass),
            \"attribute_id\" int4 NOT NULL,
            \"value\" varchar(64) COLLATE \"pg_catalog\".\"default\" NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"clauses\";
            CREATE TABLE \"public\".\"clauses\" (
            \"id\" int4 NOT NULL DEFAULT nextval('clauses_id_seq'::regclass),
            \"rule_id\" int4 NOT NULL,
            \"compared_value\" varchar(64) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"logical_group\" varchar(36) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"operator\" \"public\".\"operatorenum\" NOT NULL,
            \"question_id\" int4 NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"histories\";
            CREATE TABLE \"public\".\"histories\" (
            \"id\" int4 NOT NULL DEFAULT nextval('histories_id_seq'::regclass),
            \"system_id\" int4 NOT NULL,
            \"user_id\" int4 NOT NULL,
            \"answered_questions\" varchar(9) COLLATE \"pg_catalog\".\"default\" NOT NULL DEFAULT '0/0'::character varying,
            \"results\" json NOT NULL,
            \"started_at\" timestamp(6) NOT NULL DEFAULT now(),
            \"finished_at\" timestamp(6) NOT NULL DEFAULT now()
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"object_attribute_attributevalue\";
            CREATE TABLE \"public\".\"object_attribute_attributevalue\" (
            \"id\" int4 NOT NULL DEFAULT nextval('attributesvalue_object_id_seq'::regclass),
            \"object_id\" int4 NOT NULL,
            \"attribute_value_id\" int4 NOT NULL,
            \"attribute_id\" int4 NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"objects\";
            CREATE TABLE \"public\".\"objects\" (
            \"id\" int4 NOT NULL DEFAULT nextval('objects_id_seq'::regclass),
            \"system_id\" int4 NOT NULL,
            \"name\" varchar(128) COLLATE \"pg_catalog\".\"default\" NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"questions\";
            CREATE TABLE \"public\".\"questions\" (
            \"id\" int4 NOT NULL DEFAULT nextval('questions_id_seq'::regclass),
            \"system_id\" int4 NOT NULL,
            \"body\" varchar(128) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"with_chooses\" bool NOT NULL DEFAULT false
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"rule_attribute_attributevalue\";
            CREATE TABLE \"public\".\"rule_attribute_attributevalue\" (
            \"id\" int4 NOT NULL DEFAULT nextval('rule_attributevalue_id_seq'::regclass),
            \"attribute_value_id\" int4 NOT NULL,
            \"rule_id\" int4 NOT NULL,
            \"attribute_id\" int4 NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"rule_question_answer\";
            CREATE TABLE \"public\".\"rule_question_answer\" (
            \"id\" int4 NOT NULL DEFAULT nextval('rule_answer_id_seq'::regclass),
            \"answer_id\" int4 NOT NULL,
            \"rule_id\" int4 NOT NULL,
            \"question_id\" int4 NOT NULL
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"rules\";
            CREATE TABLE \"public\".\"rules\" (
            \"id\" int4 NOT NULL DEFAULT nextval('rules_id_seq'::regclass),
            \"system_id\" int4 NOT NULL,
            \"attribute_rule\" bool NOT NULL DEFAULT true
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"systems\";
            CREATE TABLE \"public\".\"systems\" (
            \"id\" int4 NOT NULL DEFAULT nextval('systems_id_seq'::regclass),
            \"user_id\" int4 NOT NULL,
            \"about\" text COLLATE \"pg_catalog\".\"default\",
            \"created_at\" timestamp(6) NOT NULL DEFAULT now(),
            \"updated_at\" timestamp(6) NOT NULL DEFAULT now(),
            \"name\" varchar(128) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"private\" bool NOT NULL DEFAULT true,
            \"image_uri\" varchar(128) COLLATE \"pg_catalog\".\"default\" DEFAULT ''::character varying
            )
            ;

            DROP TABLE IF EXISTS \"public\".\"users\";
            CREATE TABLE \"public\".\"users\" (
            \"id\" int4 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
            \"email\" varchar(32) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"username\" varchar(16) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"created_at\" timestamp(6) NOT NULL DEFAULT now(),
            \"first_name\" varchar(16) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"last_name\" varchar(16) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"is_superuser\" bool NOT NULL DEFAULT false,
            \"password\" varchar(256) COLLATE \"pg_catalog\".\"default\" NOT NULL,
            \"verified\" bool NOT NULL DEFAULT false,
            \"verification_code\" varchar COLLATE \"pg_catalog\".\"default\",
            \"password_reset_at\" timestamp(6)
            )
            ;

            DROP FUNCTION IF EXISTS \"public\".\"diesel_manage_updated_at\"(\"_tbl\" regclass);
            CREATE OR REPLACE FUNCTION \"public\".\"diesel_manage_updated_at\"(\"_tbl\" regclass)
            RETURNS \"pg_catalog\".\"void\" AS $BODY$
            BEGIN
                EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                                FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
            END;
            $BODY$
            LANGUAGE plpgsql VOLATILE
            COST 100;

            DROP FUNCTION IF EXISTS \"public\".\"trigger_set_timestamp\"();
            CREATE OR REPLACE FUNCTION \"public\".\"trigger_set_timestamp\"()
            RETURNS \"pg_catalog\".\"trigger\" AS $BODY$
            BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
            END;
            $BODY$
            LANGUAGE plpgsql VOLATILE
            COST 100;

            ALTER SEQUENCE \"public\".\"answers_id_seq\"
            OWNED BY \"public\".\"answers\".\"id\";
            SELECT setval('\"public\".\"answers_id_seq\"', 145, true);

            ALTER SEQUENCE \"public\".\"attributes_id_seq\"
            OWNED BY \"public\".\"attributes\".\"id\";
            SELECT setval('\"public\".\"attributes_id_seq\"', 75, true);

            ALTER SEQUENCE \"public\".\"attributesvalue_object_id_seq\"
            OWNED BY \"public\".\"object_attribute_attributevalue\".\"id\";
            SELECT setval('\"public\".\"attributesvalue_object_id_seq\"', 101, true);

            ALTER SEQUENCE \"public\".\"attributesvalues_id_seq\"
            OWNED BY \"public\".\"attributesvalues\".\"id\";
            SELECT setval('\"public\".\"attributesvalues_id_seq\"', 228, true);

            ALTER SEQUENCE \"public\".\"clauses_id_seq\"
            OWNED BY \"public\".\"clauses\".\"id\";
            SELECT setval('\"public\".\"clauses_id_seq\"', 48, true);

            ALTER SEQUENCE \"public\".\"histories_id_seq\"
            OWNED BY \"public\".\"histories\".\"id\";
            SELECT setval('\"public\".\"histories_id_seq\"', 32, true);

            ALTER SEQUENCE \"public\".\"objects_id_seq\"
            OWNED BY \"public\".\"objects\".\"id\";
            SELECT setval('\"public\".\"objects_id_seq\"', 25, true);

            ALTER SEQUENCE \"public\".\"questions_id_seq\"
            OWNED BY \"public\".\"questions\".\"id\";
            SELECT setval('\"public\".\"questions_id_seq\"', 46, true);

            ALTER SEQUENCE \"public\".\"rule_answer_id_seq\"
            OWNED BY \"public\".\"rule_question_answer\".\"id\";
            SELECT setval('\"public\".\"rule_answer_id_seq\"', 16, true);

            ALTER SEQUENCE \"public\".\"rule_attributevalue_id_seq\"
            OWNED BY \"public\".\"rule_attribute_attributevalue\".\"id\";
            SELECT setval('\"public\".\"rule_attributevalue_id_seq\"', 22, true);

            ALTER SEQUENCE \"public\".\"rules_id_seq\"
            OWNED BY \"public\".\"rules\".\"id\";
            SELECT setval('\"public\".\"rules_id_seq\"', 31, true);

            ALTER SEQUENCE \"public\".\"systems_id_seq\"
            OWNED BY \"public\".\"systems\".\"id\";
            SELECT setval('\"public\".\"systems_id_seq\"', 118, true);

            ALTER SEQUENCE \"public\".\"users_id_seq\"
            OWNED BY \"public\".\"users\".\"id\";
            SELECT setval('\"public\".\"users_id_seq\"', 14, true);

            ALTER TABLE \"public\".\"answers\" ADD CONSTRAINT \"id_answers_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"attributes\" ADD CONSTRAINT \"id_attributes_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"attributesvalues\" ADD CONSTRAINT \"id_attributesvalues_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"clauses\" ADD CONSTRAINT \"id_clauses_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"histories\" ADD CONSTRAINT \"id_histories_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"object_attribute_attributevalue\" ADD CONSTRAINT \"attributesvalue_object_pkey\" PRIMARY KEY (\"object_id\", \"attribute_value_id\", \"attribute_id\");

            ALTER TABLE \"public\".\"objects\" ADD CONSTRAINT \"id_objects_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"questions\" ADD CONSTRAINT \"id_questions_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"rule_attribute_attributevalue\" ADD CONSTRAINT \"rule_attribute_value_pkey\" PRIMARY KEY (\"attribute_value_id\", \"rule_id\", \"attribute_id\");

            ALTER TABLE \"public\".\"rule_question_answer\" ADD CONSTRAINT \"rule_answer_pkey\" PRIMARY KEY (\"answer_id\", \"rule_id\", \"question_id\");

            ALTER TABLE \"public\".\"rules\" ADD CONSTRAINT \"id_rules_pkey\" PRIMARY KEY (\"id\");

            CREATE TRIGGER \"set_timestamp\" BEFORE UPDATE ON \"public\".\"systems\"
            FOR EACH ROW
            EXECUTE PROCEDURE \"public\".\"trigger_set_timestamp\"();

            ALTER TABLE \"public\".\"systems\" ADD CONSTRAINT \"name_systems_unique\" UNIQUE (\"name\");

            ALTER TABLE \"public\".\"systems\" ADD CONSTRAINT \"id_systems_pkey\" PRIMARY KEY (\"id\");

            CREATE INDEX \"idx-email\" ON \"public\".\"users\" USING btree (
            \"email\" COLLATE \"pg_catalog\".\"default\" \"pg_catalog\".\"text_ops\" ASC NULLS LAST
            );
            CREATE INDEX \"idx-verification_code\" ON \"public\".\"users\" USING btree (
            \"verification_code\" COLLATE \"pg_catalog\".\"default\" \"pg_catalog\".\"text_ops\" ASC NULLS LAST
            );

            ALTER TABLE \"public\".\"users\" ADD CONSTRAINT \"email_users_unique\" UNIQUE (\"email\");
            ALTER TABLE \"public\".\"users\" ADD CONSTRAINT \"username_users_unique\" UNIQUE (\"username\");

            ALTER TABLE \"public\".\"users\" ADD CONSTRAINT \"id_users_pkey\" PRIMARY KEY (\"id\");

            ALTER TABLE \"public\".\"answers\" ADD CONSTRAINT \"questions_answers_fkey\" FOREIGN KEY (\"question_id\") REFERENCES \"public\".\"questions\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"attributes\" ADD CONSTRAINT \"systems_attributes_fkey\" FOREIGN KEY (\"system_id\") REFERENCES \"public\".\"systems\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"attributesvalues\" ADD CONSTRAINT \"attributes_attributesvalues_fkey\" FOREIGN KEY (\"attribute_id\") REFERENCES \"public\".\"attributes\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"clauses\" ADD CONSTRAINT \"questions_clauses_fkey\" FOREIGN KEY (\"question_id\") REFERENCES \"public\".\"questions\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"clauses\" ADD CONSTRAINT \"rules_clauses_fkey\" FOREIGN KEY (\"rule_id\") REFERENCES \"public\".\"rules\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"histories\" ADD CONSTRAINT \"systems_histories_fkey\" FOREIGN KEY (\"system_id\") REFERENCES \"public\".\"systems\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"histories\" ADD CONSTRAINT \"users_histories_fkey\" FOREIGN KEY (\"user_id\") REFERENCES \"public\".\"users\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"object_attribute_attributevalue\" ADD CONSTRAINT \"attributes_attribute_fkey\" FOREIGN KEY (\"attribute_id\") REFERENCES \"public\".\"attributes\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"object_attribute_attributevalue\" ADD CONSTRAINT \"attributesvalue_object_objects_fkey\" FOREIGN KEY (\"object_id\") REFERENCES \"public\".\"objects\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"object_attribute_attributevalue\" ADD CONSTRAINT \"attributesvalues_attribute_value_fkey\" FOREIGN KEY (\"attribute_value_id\") REFERENCES \"public\".\"attributesvalues\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"objects\" ADD CONSTRAINT \"systems_objects_fkey\" FOREIGN KEY (\"system_id\") REFERENCES \"public\".\"systems\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"questions\" ADD CONSTRAINT \"systems_questions_fkey\" FOREIGN KEY (\"system_id\") REFERENCES \"public\".\"systems\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"rule_attribute_attributevalue\" ADD CONSTRAINT \"atribute_values_rule_attributevalues_fkey\" FOREIGN KEY (\"attribute_value_id\") REFERENCES \"public\".\"attributesvalues\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"rule_attribute_attributevalue\" ADD CONSTRAINT \"attributes_rule_attributevalues_fkey\" FOREIGN KEY (\"attribute_id\") REFERENCES \"public\".\"attributes\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"rule_attribute_attributevalue\" ADD CONSTRAINT \"rules_rule_attributevalues_fkey\" FOREIGN KEY (\"rule_id\") REFERENCES \"public\".\"rules\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"rule_question_answer\" ADD CONSTRAINT \"answers_rule_answers_fkey\" FOREIGN KEY (\"answer_id\") REFERENCES \"public\".\"answers\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"rule_question_answer\" ADD CONSTRAINT \"questions_rule_answers_fkey\" FOREIGN KEY (\"question_id\") REFERENCES \"public\".\"questions\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;
            ALTER TABLE \"public\".\"rule_question_answer\" ADD CONSTRAINT \"rules_rule_answers_fkey\" FOREIGN KEY (\"rule_id\") REFERENCES \"public\".\"rules\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"rules\" ADD CONSTRAINT \"systems_rules_fkey\" FOREIGN KEY (\"system_id\") REFERENCES \"public\".\"systems\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            ALTER TABLE \"public\".\"systems\" ADD CONSTRAINT \"users_systems_fkey\" FOREIGN KEY (\"user_id\") REFERENCES \"public\".\"users\" (\"id\") ON DELETE CASCADE ON UPDATE NO ACTION;

            
            "
        )
        .await?;

        Ok(())
    }
}
