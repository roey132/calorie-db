-- Your SQL goes here
ALTER TABLE user_meals
ADD COLUMN protein FLOAT;

ALTER TABLE user_meals
ADD COLUMN carbs FLOAT;

ALTER TABLE user_meals
ADD COLUMN fats FLOAT; 

ALTER TABLE "user_meals" DROP CONSTRAINT meal_type_check;

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
   measure_count IS NULL AND
   carbs IS NULL AND
   fats IS NULL AND
   protein IS NULL)
  OR 
  (meal_type = 'Measure' AND
   measure_id IS NOT NULL AND
   measure_count IS NOT NULL AND
   product_id IS NULL AND
   product_grams IS NULL AND
   calories IS NULL AND
   carbs IS NULL AND
   fats IS NULL AND
   protein IS NULL)
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
um.meal_name,
um.meal_date,

case 
	when meal_type = 'Calories' then um.calories
	when meal_type = 'Product' then p.calories_per_gram * um.product_grams
	when meal_type = 'Measure' then pm.measure_calories * um.measure_count
else 
	null
end as calc_calories,
case 
    when meal_type = 'Product' then p.protein_per_gram * um.product_grams
    when meal_type = 'Measure' then pc.product_protein_per_gram * (pm.measure_calories / pc.product_calories_per_gram) * um.measure_count
    when meal_type = 'Calories' then um.protein
else 
    null
end as protein_grams,
case 
    when meal_type = 'Product' then p.carbs_per_gram * um.product_grams
    when meal_type = 'Measure' then pc.product_carbs_per_gram * (pm.measure_calories / pc.product_calories_per_gram) * um.measure_count
    when meal_type = 'Calories' then um.carbs
else 
    null
end as carbs_grams,
case 
    when meal_type = 'Product' then p.fats_per_gram * um.product_grams
    when meal_type = 'Measure' then pc.product_fats_per_gram * (pm.measure_calories / pc.product_calories_per_gram) * um.measure_count
    when meal_type = 'Calories' then um.fats
else 
    null
end as fats_grams
from user_meals um 
left join products p on p.product_id = um.product_id 
left join product_measures pm ON pm.measure_id = um.measure_id
left join (SELECT pm.measure_id,
                p.calories_per_gram AS product_calories_per_gram,
                p.fats_per_gram AS product_fats_per_gram,
                p.protein_per_gram AS product_protein_per_gram,
                p.carbs_per_gram AS product_carbs_per_gram
           FROM product_measures pm
           LEFT JOIN products p ON pm.product_id = p.product_id) pc ON um.measure_id = pc.measure_id;