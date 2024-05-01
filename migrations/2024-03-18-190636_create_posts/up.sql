CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE "products" (
  "product_id" SERIAL PRIMARY KEY NOT NULL,
  "product_name" varchar NOT NULL,
  "calories_per_gram" float NOT NULL,
  "user_id" uuid,
  "create_time" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "update_time" timestamp,
  "delete_time" timestamp
);

CREATE TABLE "product_measures" (
  "product_id" integer NOT NULL,
  "measure_id" SERIAL PRIMARY KEY NOT NULL,
  "default_measure" boolean,
  "measure_name" varchar NOT NULL,
  "measure_calories" float NOT NULL,
  "create_time" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "update_time" timestamp,
  "delete_time" timestamp
);

CREATE TABLE "users" (
  "user_id" uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  "username" varchar NOT NULL UNIQUE,
  "password" varchar NOT NULL,
  "create_time" timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  "update_time" timestamp
);

CREATE TYPE meal_type AS ENUM ('Calories','Measure','Product');

CREATE TABLE "user_meals" (
  "meal_id" SERIAL PRIMARY KEY,
  "user_id" uuid NOT NULL,
  "meal_type" meal_type NOT NULL,
  "product_id" integer,
  "product_grams" integer,
  "measure_id" integer,
  "measure_count" float,
  "calories" float,
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

ALTER TABLE "user_meals" 
ADD CONSTRAINT meal_type_check
CHECK (
  (meal_type = 'Calories' AND
   calories IS NOT NULL AND
   product_id IS NULL AND
   measure_id IS NULL AND
   measure_count IS NULL AND
   product_grams IS NULL)
  OR
  (meal_type = 'Product' AND
   product_id IS NOT NULL AND
   product_grams IS NOT NULL AND
   measure_id IS NULL AND
   calories IS NULL AND
   measure_count IS NULL)
  OR 
  (meal_type = 'Measure' AND
   measure_id IS NOT NULL AND
   measure_count IS NOT NULL AND
   product_id IS NOT NULL AND
   product_grams IS NULL AND
   calories IS NULL)
);

create view user_meals_calculated as 
select um.meal_id,
um.user_id,
um.meal_type,
p.product_name,
um.product_grams,
pm.measure_name,
um.measure_count,
um.meal_date,
case 
	when  meal_type = 'Calories' then um.calories
	when meal_type = 'Product' then p.calories_per_gram * um.product_grams
	when meal_type = 'Measure' then pm.measure_calories * um.measure_count
else 
	null
end as calc_calories
from user_meals um 
left join products p on p.product_id = um.product_id 
left join product_measures pm on pm.product_id = um.product_id and pm.measure_id = um.measure_id;

