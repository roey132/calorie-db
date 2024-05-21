-- This file should undo anything in `up.sql`
ALTER TABLE "user_meals" DROP CONSTRAINT "user_meals_measure_id_fkey";

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
left join product_measures pm on pm.product_id = um.product_id and pm.measure_id = um.measure_id;