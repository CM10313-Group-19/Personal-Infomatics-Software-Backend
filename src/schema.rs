// @generated automatically by Diesel CLI.

diesel::table! {
    meals (MealId) {
        MealId -> Integer,
        MealName -> Varchar,
        Date -> Datetime,
        Calories -> Integer,
    }
}
