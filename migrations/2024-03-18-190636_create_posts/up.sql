CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "products" (
  "product_id" SERIAL PRIMARY KEY NOT NULL,
  "product_name" varchar NOT NULL,
  "calories_per_gram" float NOT NULL,
  "user_id" uuid,
  "create_time" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "update_time" timestamp
);

CREATE TABLE "product_measures" (
  "product_id" integer NOT NULL,
  "measure_id" SERIAL PRIMARY KEY NOT NULL,
  "default_measure" boolean,
  "measure_name" varchar NOT NULL,
  "measure_calories" float NOT NULL,
  "create_time" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "update_time" timestamp
);

CREATE TABLE "users" (
  "user_id" uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  "username" varchar NOT NULL UNIQUE,
  "password" varchar NOT NULL,
  "create_time" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "update_time" timestamp
);

CREATE TABLE "user_meals" (
  "meal_id" integer PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "product_id" integer NOT NULL,
  "product_grams" integer,
  "measure_id" integer,
  "measure_count" float,
  "calories" float NOT NULL,
  "meal_name" varchar,
  "meal_note" varchar,
  "meal_date" date NOT NULL,
  "create_time" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "update_time" timestamp
);

ALTER TABLE "product_measures" ADD FOREIGN KEY ("product_id") REFERENCES "products" ("product_id");

ALTER TABLE "user_meals" ADD FOREIGN KEY ("product_id") REFERENCES "products" ("product_id");

ALTER TABLE "products" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

ALTER TABLE "user_meals" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

CREATE OR REPLACE FUNCTION update_modified_column()
RETURNS TRIGGER AS $$
BEGIN
   NEW.update_time = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_table_modtime
BEFORE UPDATE ON products
FOR EACH ROW
EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_table_modtime
BEFORE UPDATE ON user_meals
FOR EACH ROW
EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_table_modtime
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_modified_column();

CREATE TRIGGER update_table_modtime
BEFORE UPDATE ON product_measures
FOR EACH ROW
EXECUTE FUNCTION update_modified_column();