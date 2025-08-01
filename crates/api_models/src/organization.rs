use common_enums::OrganizationType;
use common_utils::{id_type, pii};
use utoipa::ToSchema;
pub struct OrganizationNew {
    pub org_id: id_type::OrganizationId,
    pub org_type: OrganizationType,
    pub org_name: Option<String>,
}

impl OrganizationNew {
    pub fn new(org_type: OrganizationType, org_name: Option<String>) -> Self {
        Self {
            org_id: id_type::OrganizationId::default(),
            org_type,
            org_name,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct OrganizationId {
    pub organization_id: id_type::OrganizationId,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct OrganizationCreateRequest {
    /// Name of the organization
    pub organization_name: String,

    /// Details about the organization
    #[schema(value_type = Option<Object>)]
    pub organization_details: Option<pii::SecretSerdeValue>,

    /// Metadata is useful for storing additional, unstructured information on an object.
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<pii::SecretSerdeValue>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, ToSchema)]
#[serde(deny_unknown_fields)]
pub struct OrganizationUpdateRequest {
    /// Name of the organization
    pub organization_name: Option<String>,

    /// Details about the organization
    #[schema(value_type = Option<Object>)]
    pub organization_details: Option<pii::SecretSerdeValue>,

    /// Metadata is useful for storing additional, unstructured information on an object.
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<pii::SecretSerdeValue>,

    /// Platform merchant id is unique distiguisher for special merchant in the platform org
    #[schema(value_type = String)]
    pub platform_merchant_id: Option<id_type::MerchantId>,
}
#[cfg(feature = "v1")]
#[derive(Debug, serde::Serialize, Clone, ToSchema)]
pub struct OrganizationResponse {
    /// The unique identifier for the Organization
    #[schema(value_type = String, max_length = 64, min_length = 1, example = "org_q98uSGAYbjEwqs0mJwnz")]
    pub organization_id: id_type::OrganizationId,

    /// Name of the Organization
    pub organization_name: Option<String>,

    /// Details about the organization
    #[schema(value_type = Option<Object>)]
    pub organization_details: Option<pii::SecretSerdeValue>,

    /// Metadata is useful for storing additional, unstructured information on an object.
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<pii::SecretSerdeValue>,
    pub modified_at: time::PrimitiveDateTime,
    pub created_at: time::PrimitiveDateTime,

    /// Organization Type of the organization
    #[schema(value_type = Option<OrganizationType>, example = "standard")]
    pub organization_type: Option<OrganizationType>,
}

#[cfg(feature = "v2")]
#[derive(Debug, serde::Serialize, Clone, ToSchema)]
pub struct OrganizationResponse {
    /// The unique identifier for the Organization
    #[schema(value_type = String, max_length = 64, min_length = 1, example = "org_q98uSGAYbjEwqs0mJwnz")]
    pub id: id_type::OrganizationId,

    /// Name of the Organization
    pub organization_name: Option<String>,

    /// Details about the organization
    #[schema(value_type = Option<Object>)]
    pub organization_details: Option<pii::SecretSerdeValue>,

    /// Metadata is useful for storing additional, unstructured information on an object.
    #[schema(value_type = Option<Object>)]
    pub metadata: Option<pii::SecretSerdeValue>,
    pub modified_at: time::PrimitiveDateTime,
    pub created_at: time::PrimitiveDateTime,

    /// Organization Type of the organization
    #[schema(value_type = Option<OrganizationType>, example = "standard")]
    pub organization_type: Option<OrganizationType>,
}
