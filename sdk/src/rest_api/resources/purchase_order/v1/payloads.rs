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

use crate::{
    purchase_order::store::{
        PurchaseOrder, PurchaseOrderAlternateId, PurchaseOrderVersion, PurchaseOrderVersionRevision,
    },
    rest_api::resources::paging::v1::Paging,
};

/// This is the representation of a PurchaseOrder from the REST API. This is a
/// subset of the fields in the corresponding store struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderSlice {
    pub purchase_order_uid: String,
    pub workflow_state: String,
    pub buyer_org_id: String,
    pub seller_org_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_version_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<PurchaseOrderVersionSlice>>,
    pub is_closed: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_ids: Option<Vec<PurchaseOrderAlternateIdSlice>>,
    pub created_at: i64,
    pub workflow_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

impl From<PurchaseOrder> for PurchaseOrderSlice {
    fn from(purchase_order: PurchaseOrder) -> Self {
        Self {
            purchase_order_uid: purchase_order.purchase_order_uid().to_string(),
            workflow_state: purchase_order.workflow_state().to_string(),
            buyer_org_id: purchase_order.buyer_org_id().to_string(),
            seller_org_id: purchase_order.seller_org_id().to_string(),
            accepted_version_id: purchase_order.accepted_version_id().map(ToOwned::to_owned),
            versions: Some(
                purchase_order
                    .versions()
                    .into_iter()
                    .map(PurchaseOrderVersionSlice::from)
                    .collect(),
            ),
            is_closed: purchase_order.is_closed(),
            alternate_ids: Some(
                purchase_order
                    .alternate_ids()
                    .into_iter()
                    .map(PurchaseOrderAlternateIdSlice::from)
                    .collect(),
            ),
            created_at: *purchase_order.created_at(),
            workflow_id: purchase_order.workflow_id().to_string(),
            service_id: purchase_order.service_id().map(String::from),
        }
    }
}

/// This is a struct that contains a list of `PurchaseOrderSlice`s as well as
/// paging information.
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderListSlice {
    pub data: Vec<PurchaseOrderSlice>,
    pub paging: Paging,
}

/// This is the representation of a PurchaseOrderVersion from the REST API.
/// This is a subset of the fields in the corresponding store struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderVersionSlice {
    pub version_id: String,
    pub is_draft: bool,
    pub current_revision_id: i64,
    pub revisions: Vec<PurchaseOrderRevisionSlice>,
    pub workflow_state: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

impl From<PurchaseOrderVersion> for PurchaseOrderVersionSlice {
    fn from(purchase_order_version: PurchaseOrderVersion) -> Self {
        Self {
            version_id: purchase_order_version.version_id().to_string(),
            is_draft: purchase_order_version.is_draft(),
            current_revision_id: *purchase_order_version.current_revision_id(),
            revisions: purchase_order_version
                .revisions()
                .into_iter()
                .map(PurchaseOrderRevisionSlice::from)
                .collect(),
            workflow_state: purchase_order_version.workflow_state().to_string(),
            service_id: purchase_order_version.service_id().map(String::from),
        }
    }
}

/// This is a struct that contains a list of `PurchaseOrderVersionSlice`s as
/// well as paging information.
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderVersionListSlice {
    pub data: Vec<PurchaseOrderVersionSlice>,
    pub paging: Paging,
}

/// This is the representation of a PurchaseOrderRevision from the REST API.
/// This is a subset of the fields in the corresponding store struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderRevisionSlice {
    pub revision_id: i64,
    pub submitter: String,
    pub created_at: i64,
    pub order_xml_v3_4: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

impl From<PurchaseOrderVersionRevision> for PurchaseOrderRevisionSlice {
    fn from(purchase_order_revision: PurchaseOrderVersionRevision) -> Self {
        Self {
            revision_id: *purchase_order_revision.revision_id(),
            submitter: purchase_order_revision.submitter().to_string(),
            created_at: purchase_order_revision.created_at(),
            order_xml_v3_4: purchase_order_revision.order_xml_v3_4().to_string(),
            service_id: purchase_order_revision.service_id().map(String::from),
        }
    }
}

/// This is a struct that contains a list of `PurchaseOrderRevisionSlice`s as
/// well as paging information.
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderRevisionListSlice {
    pub data: Vec<PurchaseOrderRevisionSlice>,
    pub paging: Paging,
}

/// This is the representation of a PurchaseOrderAlternateId from the REST API.
/// This is a subset of the fields in the corresponding store struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct PurchaseOrderAlternateIdSlice {
    purchase_order_uid: String,
    id_type: String,
    id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
}

impl From<PurchaseOrderAlternateId> for PurchaseOrderAlternateIdSlice {
    fn from(alternate_id: PurchaseOrderAlternateId) -> Self {
        Self {
            purchase_order_uid: alternate_id.purchase_order_uid().to_string(),
            id_type: alternate_id.id_type().to_string(),
            id: alternate_id.id().to_string(),
            service_id: alternate_id.service_id().map(String::from),
        }
    }
}
