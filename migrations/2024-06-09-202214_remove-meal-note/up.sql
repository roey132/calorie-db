ALTER TABLE user_meals
DROP COLUMN meal_note;

ALTER TABLE "user_meals"
DROP CONSTRAINT meal_type_check;

UPDATE "user_meals"
SET meal_name = null
WHERE meal_type != 'Calories';
UPDATE "user_meals"
SET meal_name = null
WHERE meal_type != 'Calories';

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
   measure_count IS NULL  AND 
   meal_name IS NULL)
  OR 
  (meal_type = 'Measure' AND
   measure_id IS NOT NULL AND
   measure_count IS NOT NULL AND
   product_id IS NULL AND
   product_grams IS NULL AND
   calories IS NULL AND 
   meal_name IS NULL)
);