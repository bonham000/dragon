use super::schema::users;

#[derive(Serialize, Deserialize, Debug)]
pub struct MaybeUser {
    pub email: String,
    pub name: String,
    pub family_name: String,
    pub given_name: String,
    pub photo_url: String,
    pub push_token: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[table_name = "users"]
pub struct InsertableUser {
    pub uuid: String,
    pub email: String,
    pub name: String,
    pub family_name: String,
    pub given_name: String,
    pub photo_url: String,
    pub push_token: String,
    pub experience_points: i64,
    pub score_history: String,
    pub settings: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Identifiable, PartialEq, Eq)]
#[table_name = "users"]
pub struct SavedUser {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub family_name: String,
    pub given_name: String,
    pub photo_url: String,
    pub uuid: String,
    pub push_token: String,
    pub experience_points: i64,
    pub score_history: String,
    pub settings: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ListScore {
    pub complete: bool,
    pub list_key: String,
    pub list_index: i32,
    pub number_words_completed: i32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ScoreHistory {
    pub mc_english: bool,
    pub mc_mandarin: bool,
    pub quiz_text: bool,
    pub mandarin_pronunciation: bool,
    pub list_02_score: ListScore,
    pub list_03_score: ListScore,
    pub list_04_score: ListScore,
    pub list_05_score: ListScore,
    pub list_06_score: ListScore,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct UserSettings {
    pub language_setting: String,
    pub app_difficulty_setting: String,
    pub auto_proceed_question: bool,
    pub disable_audio: bool,
}
