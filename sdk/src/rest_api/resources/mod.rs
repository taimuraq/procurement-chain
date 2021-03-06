// Copyright 2018-2021 Cargill Incorporated
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

#[cfg(feature = "rest-api-resources-agent")]
pub mod agents;
#[cfg(feature = "rest-api-resources-batches")]
pub mod batches;
pub mod error;
#[cfg(feature = "rest-api-resources-location")]
pub mod locations;
#[cfg(feature = "rest-api-resources-organization")]
pub mod organizations;
pub mod paging;
#[cfg(feature = "rest-api-resources-product")]
pub mod products;
#[cfg(feature = "rest-api-resources-purchase-order")]
pub mod purchase_order;
#[cfg(feature = "rest-api-resources-role")]
pub mod roles;
#[cfg(feature = "rest-api-resources-schema")]
pub mod schemas;
#[cfg(any(
    feature = "rest-api-resources-submit",
    feature = "rest-api-resources-batch-tracking"
))]
pub mod submit;
#[cfg(feature = "rest-api-resources-track-and-trace")]
pub mod track_and_trace;
