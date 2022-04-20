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
use std::error::Error;
use std::fmt::{Display, Result, Formatter};

#[derive(Debug)]
pub enum StarByFaceError {
    Error(String),
    Null(String),
}

impl Error for StarByFaceError {}

impl Display for StarByFaceError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            StarByFaceError::Error(ref message) => write!(f, "{}",message),
            StarByFaceError::Null(ref message) => write!(f, "{}",message)
        }
    }
}