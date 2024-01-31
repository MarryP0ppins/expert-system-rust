-- Your SQL goes here
CREATE TABLE attributes (
    id serial NOT NULL,
    system_id integer NOT NULL,
    name character varying(128) NOT NULL,
    CONSTRAINT id_attributes_pkey PRIMARY KEY (id),
    CONSTRAINT systems_attributes_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

CREATE TABLE attributesValues (
    id serial NOT NULL,
    attribute_id integer NOT NULL,
    value character varying(128) NOT NULL,
    CONSTRAINT id_attributesValues_pkey PRIMARY KEY (id),
    CONSTRAINT attributes_attributesValues_fkey FOREIGN KEY (attribute_id)
        REFERENCES public.attributes (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

CREATE TABLE objects (
    id serial NOT NULL,
    system_id integer NOT NULL,
    name character varying(128) NOT NULL,
    CONSTRAINT id_objects_pkey PRIMARY KEY (id),
    CONSTRAINT systems_objects_fkey FOREIGN KEY (system_id)
        REFERENCES public.systems (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID
);

CREATE TABLE attributesValue_object (
    id serial NOT NULL,
    object_id integer NOT NULL,
    attribute_value_id integer NOT NULL,
    CONSTRAINT attributesValue_object_objects_fkey FOREIGN KEY (object_id)
        REFERENCES public.objects (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT attributesValues_attribute_value_fkey FOREIGN KEY (attribute_value_id)
        REFERENCES public.attributesValues (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE CASCADE
        NOT VALID,
    CONSTRAINT attributesValue_object_pkey PRIMARY KEY (object_id, attribute_value_id)
);