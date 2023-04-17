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
CREATE TABLE IF NOT EXISTS weights (
    weight_id int NOT NULL AUTO_INCREMENT,
    user_id int NOT NULL,
    day_measured DATE NOT NULL,
    value int NOT null,
    PRIMARY KEY(weight_id),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);
CREATE TABLE IF NOT EXISTS exercise (
    exercise_id int NOT NULL AUTO_INCREMENT,
    user_id int NOT NULL,
    duration time NOT NULL,
    distance float NOT NULL,
    exercise_date date NOT NULL,
    exercise_type varchar(255) NOT NULL,
    PRIMARY KEY(exercise_id),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);
CREATE TABLE IF NOT EXISTS sleep (
    sleep_id int NOT NULL AUTO_INCREMENT,
    user_id int NOT NULL,
    start_time time NOT NULL,
    end_time time NOT NULL,
    sleep_date date NOT NULL,
    PRIMARY KEY(sleep_id),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);
CREATE TABLE IF NOT EXISTS five_a_day (
    user_id int NOT NULL AUTO_INCREMENT,
    day date NOT NULL,
    value int NOT NULL,
    PRIMARY KEY(user_id, day),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);
CREATE TABLE IF NOT EXISTS goals (
    goal_id int NOT NULL AUTO_INCREMENT,
    user_id int NOT NULL,
    date_created date NOT NULL,
    value varchar(255) NOT NULL,
    completed bool NOT NULL,
    PRIMARY KEY(goal_id),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
)
