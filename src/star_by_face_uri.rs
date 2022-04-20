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
use std::io::Read;
use json::JsonValue;
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use crate::{Gender, StarByFaceError};

pub struct StarByFace {
    #[doc(hidden)]
    response: Option<String>,
    #[doc(hidden)]
    error_message: Option<String>
}

#[doc(hidden)]
struct StarByFaceInternal {
    image_uri: String
}

impl StarByFace {

    /// `image_uri` Image URI.
    pub fn new(image_uri: &str) -> Self {
        let internal = StarByFaceInternal::new(image_uri);
        let response = internal.http();
        let error_message = internal.error_message();
        Self {
            response,
            error_message
        }
    }

    #[doc(hidden)]
    fn is_error(&self) -> Option<std::string::String> {
        match self.error_message.clone() {
            Some(message) => Some(message),
            None => None
        }
    }

    #[doc(hidden)]
    fn females_data(&self) -> Option<Vec<std::string::String>> {
        match self.response.clone() {
            Some(response) => {
                match Regex::new(r#"<div id="female-celebs-result">(?s).*"#) {
                    Ok(regex) => match regex.captures(&response) {
                        Some(cap) => {
                            match cap.get(0) {
                                Some(data) => {
                                    match Regex::new(r#"<div (.*?)similarity="(.*?)">(.*?)\s*(.*?)\s*</div>\s*</div>\s*(.*?)\s*(.*?)href="(.*?)"(.*?)\s*(.*?)\s*(.*?)src="(.*?)" />\s*(.*?)\s*(.*?)\s*(.*?)\s*<p(.*?)>(.*?)</p>"#) {
                                        Ok(regex) => {
                                            let mut list = vec![];
                                            let mut json = JsonValue::new_object();
                                            for cap in regex.captures_iter(data.as_str()) {
                                                json["similarity"] = format!("{}%",cap[2].to_string()).into();
                                                json["name"] = cap[16].into();
                                                json["gender"] = "female".into();
                                                json["image"] = cap[11].replace(" ","%20").into();
                                                json["wiki"] = cap[7].into();
                                                list.push(json.to_string())
                                            }
                                            Some(list)
                                        },
                                        Err(_) => None
                                    }
                                },
                                None => None
                            }
                        },
                        None => None
                    },
                    Err(_) => None
                }
            },
            None => None
        }
    }

    #[doc(hidden)]
    fn males_data(&self) -> Option<Vec<std::string::String>> {
        match self.response.clone() {
            Some(response) => {
                match Regex::new(r#"<div id="male-celebs-result">(?s).*<div id="female-celebs-result">"#) {
                    Ok(regex) => match regex.captures(&response) {
                        Some(cap) => {
                            match cap.get(0) {
                                Some(data) => {
                                    match Regex::new(r#"<div (.*?)similarity="(.*?)">(.*?)\s*(.*?)\s*</div>\s*</div>\s*(.*?)\s*(.*?)href="(.*?)"(.*?)\s*(.*?)\s*(.*?)src="(.*?)" />\s*(.*?)\s*(.*?)\s*(.*?)\s*<p(.*?)>(.*?)</p>"#) {
                                        Ok(regex) => {
                                            let mut list = vec![];
                                            let mut json = JsonValue::new_object();
                                            for cap in regex.captures_iter(data.as_str()) {
                                                json["similarity"] = format!("{}%",cap[2].to_string()).into();
                                                json["name"] = cap[16].into();
                                                json["gender"] = "male".into();
                                                json["image"] = cap[11].replace(" ","%20").into();
                                                json["wiki"] = cap[7].into();
                                                list.push(json.to_string())
                                            }
                                            Some(list)
                                        },
                                        Err(_) => None
                                    }
                                },
                                None => None
                            }
                        },
                        None => None
                    },
                    Err(_) => None
                }
            },
            None => None
        }
    }

    /// Return warning message.
    pub fn warning(&self) -> Option<std::string::String> {
        match self.response.clone() {
            Some(res) => match Regex::new(r#"showError(.*?);"#) {
                Ok(regex) => {
                    match regex.captures(&res) {
                        Some(cap) => match cap.get(1) {
                            Some(data) => Some(data.as_str().replace("(","").replace(")","").replace("\"","")),
                            None => None
                        },
                        None => None
                    }
                },
                Err(_) => None
            },
            None => None
        }
    }

    /// Return vector of JSON Objects.
    pub fn get_data(&self) -> Result<Vec<std::string::String>, StarByFaceError>  {
        if let Some(error) = self.is_error() {
            return Err(StarByFaceError::Error(error))
        }
        let females = match self.females_data() {
            Some(data) => data,
            None => vec![]
        };
        let males = match self.males_data() {
            Some(data) => data,
            None => vec![]
        };
        let mut list = vec![];
        for i in females {
            list.push(i)
        }
        for i in males {
            list.push(i)
        }
        if list.is_empty() {
            Err(StarByFaceError::Null(String::from("null")))
        } else {
            Ok(list)
        }
    }

    /// `result` Gender enum.
    ///
    /// Return vector of JSON Objects.
    pub fn get_data_by_gender(&self,result: Gender) -> Result<Vec<std::string::String>, StarByFaceError>  {
        if let Some(error) = self.is_error() {
            return Err(StarByFaceError::Error(error))
        }
        let females = match self.females_data() {
            Some(data) => data,
            None => vec![]
        };
        let males = match self.males_data() {
            Some(data) => data,
            None => vec![]
        };
        match result {
            Gender::FEMALE => if !females.is_empty() {
                Ok(females)
            } else {
                Err(StarByFaceError::Null(String::from("null")))
            },
            Gender::MALE => if !males.is_empty() {
                Ok(males)
            } else {
                Err(StarByFaceError::Null(String::from("null")))
            }
        }
    }

    /// Return vector of JSON Objects.
    pub fn highest_similarity(&self) -> Result<Vec<std::string::String>, StarByFaceError> {
        if let Some(error) = self.is_error() {
            return Err(StarByFaceError::Error(error))
        }
        let females = match self.females_data() {
            Some(data) => data,
            None => vec![]
        };
        let males = match self.males_data() {
            Some(data) => data,
            None => vec![]
        };
        let mut list = vec![];
        if !females.is_empty() {
            list.push(females[0].to_string())
        }
        if !males.is_empty() {
            list.push(males[0].to_string())
        }
        if !list.is_empty() {
            Ok(list)
        } else {
            Err(StarByFaceError::Null(String::from("null")))
        }
    }

    /// `result` Gender enum.
    ///
    /// Return JSON Object.
    pub fn highest_similarity_by_gender(&self,result: Gender) -> Result<std::string::String, StarByFaceError> {
        if let Some(error) = self.is_error() {
            return Err(StarByFaceError::Error(error))
        }
        let females = match self.females_data() {
            Some(data) => data,
            None => vec![]
        };
        let males = match self.males_data() {
            Some(data) => data,
            None => vec![]
        };
        match result {
            Gender::FEMALE => if !females.is_empty() {
                Ok(females[0].to_string())
            } else {
                Err(StarByFaceError::Null(String::from("null")))
            },
            Gender::MALE => if !males.is_empty() {
                Ok(males[0].to_string())
            } else {
                Err(StarByFaceError::Null(String::from("null")))
            }
        }
    }

    /// Return vector of JSON Objects.
    pub fn lowest_similarity(&self) -> Result<Vec<std::string::String>, StarByFaceError> {
        if let Some(error) = self.is_error() {
            return Err(StarByFaceError::Error(error))
        }
        let females = match self.females_data() {
            Some(data) => data,
            None => vec![]
        };
        let males = match self.males_data() {
            Some(data) => data,
            None => vec![]
        };
        let mut list = vec![];
        if !females.is_empty() {
            list.push(females[females.len() - 1].to_string())
        }
        if !males.is_empty() {
            list.push(males[males.len() - 1].to_string())
        }
        if !list.is_empty() {
            Ok(list)
        } else {
            Err(StarByFaceError::Null(String::from("null")))
        }
    }

    /// `result` Gender enum.
    ///
    /// Return JSON Object.
    pub fn lowest_similarity_by_gender(&self,result: Gender) -> Result<std::string::String, StarByFaceError> {
        if let Some(error) = self.is_error() {
            return Err(StarByFaceError::Error(error))
        }
        let females = match self.females_data() {
            Some(data) => data,
            None => vec![]
        };
        let males = match self.males_data() {
            Some(data) => data,
            None => vec![]
        };
        match result {
            Gender::FEMALE => if !females.is_empty() {
                Ok(females[females.len() - 1].to_string())
            } else {
                Err(StarByFaceError::Null(String::from("null")))
            },
            Gender::MALE => if !males.is_empty() {
                Ok(males[males.len() - 1].to_string())
            } else {
                Err(StarByFaceError::Null(String::from("null")))
            }
        }
    }
}

impl StarByFaceInternal {
    fn new(image_uri: &str) -> Self {
        Self {
            image_uri: image_uri.to_string()
        }
    }

    fn http(&self) -> Option<std::string::String> {
        match reqwest::blocking::Client::new().post("https://starbyface.com/Home/LooksLike")
            .header(CONTENT_TYPE,"application/json; utf-8")
            .body(format!("{{\"url\": \"{}\"}}", self.image_uri))
            .send() {
            Ok(mut data) => {
                let mut body = String::new();
                match data.read_to_string(&mut body) {
                    Ok(_) => Some(body),
                    Err(_) => None
                }
            },
            Err(_) => None
        }
    }

    fn error_message(&self) -> Option<std::string::String> {
        match self.http() {
            Some(res) => match Regex::new(r#""errorMsg":"(.*?)""#) {
                Ok(regex) => {
                    match regex.captures(&res) {
                        Some(cap) => match cap.get(1) {
                            Some(data) => if data.as_str().contains("Parameter is not valid") {
                                Some("Invalid Image".to_string())
                            } else {
                                Some(data.as_str().to_string())
                            },
                            None => None
                        },
                        None => None
                    }
                },
                Err(_) => None
            },
            None => None
        }
    }
}