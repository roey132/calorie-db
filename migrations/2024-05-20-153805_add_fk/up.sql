-- Your SQL goes here
ALTER TABLE "user_meals" ADD FOREIGN KEY ("measure_id") REFERENCES "product_measures" ("measure_id");
