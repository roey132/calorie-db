DROP TRIGGER IF EXISTS update_table_modtime ON product_measures;
DROP TRIGGER IF EXISTS update_table_modtime ON user_meals;
DROP TRIGGER IF EXISTS update_table_modtime ON products;
DROP TRIGGER IF EXISTS update_table_modtime ON users;

DROP FUNCTION IF EXISTS update_table_modtime;


ALTER TABLE "product_measures" DROP CONSTRAINT product_measures_product_id_fkey;
ALTER TABLE "user_meals" DROP CONSTRAINT user_meals_product_id_fkey;
ALTER TABLE "products" DROP CONSTRAINT products_user_id_fkey;
ALTER TABLE "user_meals" DROP CONSTRAINT user_meals_user_id_fkey;

DROP TABLE product_measures;
DROP TABLE user_meals;
DROP TABLE users;
DROP TABLE products;
