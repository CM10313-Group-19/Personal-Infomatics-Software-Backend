CREATE TABLE IF NOT EXISTS users (
    user_id int NOT NULL AUTO_INCREMENT, 
    email varchar(255) NOT NULL,
    password varchar(255) NOT NULL,
    date_of_birth DATE not null,
    PRIMARY KEY(user_id)
);
CREATE TABLE IF NOT EXISTS meals (
    meal_id int NOT NULL AUTO_INCREMENT,
    user_id int NOT NULL,
    meal_name varchar(255) NOT NULL,
    date_eaten DATETIME NOT NULL,
    calories int NOT NULL,
    PRIMARY KEY(meal_id),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);
