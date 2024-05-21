-- Your SQL goes here
ALTER TABLE "user_meals" ADD FOREIGN KEY ("measure_id") REFERENCES "product_measures" ("measure_id");

ALTER TABLE "user_meals" DROP CONSTRAINT meal_type_check;

UPDATE "user_meals"
SET product_id = NULL
WHERE meal_type = 'Measure';

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
   product_id IS NULL AND
   product_grams IS NULL AND
   calories IS NULL)
);

DROP VIEW user_meals_calculated;
CREATE VIEW user_meals_calculated as 
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
left join product_measures pm ON pm.measure_id = um.measure_id;