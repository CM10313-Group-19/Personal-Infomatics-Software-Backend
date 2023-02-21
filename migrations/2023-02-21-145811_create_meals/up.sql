-- Your SQL goes here
CREATE TABLE meals (
    MealId int AUTO_INCREMENT NOT NULL,
    MealName varchar(255) NOT NULL,
    Date DATETIME NOT NULL,
    Calories int NOT NULL,
    PRIMARY KEY(MealId)
);
