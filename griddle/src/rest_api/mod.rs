// Copyright 2018-2022 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "rest-api-actix-web-4")]
pub mod actix_web_4;
mod error;

pub use error::GriddleRestApiServerError;

#[derive(Clone, Debug, PartialEq)]
/// Indicates the service scope intended for incoming requests and is used by the REST API to
/// validate requests.
pub enum Scope {
    Global,
    Service,
}
