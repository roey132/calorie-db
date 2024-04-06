ALTER TABLE "product_measures" DROP CONSTRAINT product_measures_product_id_fkey;
ALTER TABLE "user_meals" DROP CONSTRAINT user_meals_product_id_fkey;
ALTER TABLE "products" DROP CONSTRAINT products_user_id_fkey;
ALTER TABLE "user_meals" DROP CONSTRAINT user_meals_user_id_fkey;

DROP TABLE product_measures;
DROP TABLE user_meals;
DROP TABLE users;
DROP TABLE products;