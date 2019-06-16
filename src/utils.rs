use ::uuid::Uuid;
use rocket::http::Status;
use rocket::Response;

use super::types::{InitialUserData, InsertableUser, ListScore, ScoreHistory, UserSettings};

pub fn get_failure_status() -> Response<'static> {
    Response::build()
        .status(Status::InternalServerError)
        .finalize()
}

pub fn create_new_user(user: InitialUserData) -> InsertableUser {
    let default_score_history = ScoreHistory {
        mc_english: false,
        mc_mandarin: false,
        quiz_text: false,
        mandarin_pronunciation: false,
        list_02_score: ListScore {
            complete: false,
            list_index: 0,
            list_key: "2".to_string(),
            number_words_completed: 0,
        },
        list_03_score: ListScore {
            complete: false,
            list_index: 1,
            list_key: "3".to_string(),
            number_words_completed: 0,
        },
        list_04_score: ListScore {
            complete: false,
            list_index: 2,
            list_key: "4".to_string(),
            number_words_completed: 0,
        },
        list_05_score: ListScore {
            complete: false,
            list_index: 3,
            list_key: "5".to_string(),
            number_words_completed: 0,
        },
        list_06_score: ListScore {
            complete: false,
            list_index: 4,
            list_key: "6".to_string(),
            number_words_completed: 0,
        },
    };

    let default_settings = UserSettings {
        disable_audio: false,
        auto_proceed_question: true,
        language_setting: "simplified".to_string(),
        app_difficulty_setting: "MEDIUM".to_string(),
    };

    InsertableUser {
        uuid: Uuid::new_v4().to_string(),
        email: "".to_string(),
        username: "".to_string(),
        experience_points: 0,
        push_token: user.push_token,
        settings: serde_json::to_string(&default_settings).unwrap(),
        score_history: serde_json::to_string(&default_score_history).unwrap(),
    }
}
