CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "products" (
  "product_id" integer PRIMARY KEY NOT NULL,
  "product_name" varchar NOT NULL,
  "calories_1gram" integer,
  "user_id" uuid,
  "create_time" timestamp NOT NULL,
  "update_time" timestamp
);

CREATE TABLE "product_measures" (
  "product_id" integer NOT NULL,
  "measure_id" integer PRIMARY KEY NOT NULL,
  "is_primary_measure" bool,
  "measure_name" varchar NOT NULL,
  "measure_grams" integer NOT NULL,
  "create_time" timestamp NOT NULL,
  "update_time" timestamp
);

CREATE TABLE "users" (
  "user_id" uuid PRIMARY KEY NOT NULL,
  "username" varchar NOT NULL,
  "password" varchar NOT NULL,
  "create_time" timestamp NOT NULL,
  "update_time" timestamp
);

CREATE TABLE "user_meals" (
  "meal_id" integer PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "product_id" integer,
  "measure_id" integer,
  "measure_count" integer,
  "calories" integer,
  "meal_name" varchar,
  "meal_note" varchar,
  "meal_time" timestamptz NOT NULL,
  "create_time" timestamp NOT NULL,
  "update_time" timestamp
);

ALTER TABLE "product_measures" ADD FOREIGN KEY ("product_id") REFERENCES "products" ("product_id");

ALTER TABLE "user_meals" ADD FOREIGN KEY ("product_id") REFERENCES "products" ("product_id");

ALTER TABLE "products" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

ALTER TABLE "user_meals" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");