/*
 * Copyright 2022 XXIV
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
//! Celebrity look alike face-recognition API
//!
//! There should be only one person in the photo.
//! Recommendations: The face should be clearly
//! visible, it’s better to use frontal photos. Face
//! recognition accuracy depends on the resolution
//! and quality of a face image.
//!
//! The system detects the face and creates a facial
//! pattern. System facial point detection can locate
//! the key components of faces, including eyebrows,
//! eyes, nose, mouth and position.
//!
//! The Neural Network compares the person with celebrity
//! faces and suggests the most similar ones.
mod star_by_face_uri;
mod star_by_face_file;
mod error;
pub use star_by_face_uri::StarByFace;
pub use star_by_face_file::StarByFaceFile;
pub use error::StarByFaceError;

pub enum Gender {
    FEMALE,
    MALE
}

pub enum Key {
    SIMILARITY,
    NAME,
    GENDER,
    IMAGE,
    WIKI
}

/// Return value of the given key.
pub fn get(data: String, key: Key) -> Result<std::string::String, StarByFaceError> {
    match json::parse(&data) {
        Ok(json) => {
            match key {
                Key::SIMILARITY => Ok(json["similarity"].to_string()),
                Key::NAME => Ok(json["name"].to_string()),
                Key::GENDER => Ok(json["gender"].to_string()),
                Key::IMAGE => Ok(json["image"].to_string()),
                Key::WIKI => Ok(json["wiki"].to_string())
            }
        },
        Err(_) => Err(StarByFaceError::Null(String::from("null")))
    }
}

/// Return vector of values of the given key.
pub fn get_list(data: Vec<String>, key: Key) -> Result<Vec<std::string::String>, StarByFaceError> {
    let mut list = vec![];
    for i in &data {
        match json::parse(i) {
            Ok(json) => {
                match key {
                    Key::SIMILARITY => list.push(json["similarity"].to_string()),
                    Key::NAME => list.push(json["name"].to_string()),
                    Key::GENDER => list.push(json["gender"].to_string()),
                    Key::IMAGE => list.push(json["image"].to_string()),
                    Key::WIKI => list.push(json["wiki"].to_string())
                }
            },
            Err(_) => return Err(StarByFaceError::Null(String::from("null")))
        }
    }
    if !list.is_empty() {
        Ok(list)
    } else {
        Err(StarByFaceError::Null(String::from("null")))
    }
}