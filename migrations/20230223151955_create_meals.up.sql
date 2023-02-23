CREATE TABLE IF NOT EXISTS meals (
     meal_id int AUTO_INCREMENT NOT NULL,
     meal_name varchar(255) NOT NULL,
     date_eaten DATETIME NOT NULL,
     calories int NOT NULL,
     PRIMARY KEY(meal_id)
 );
