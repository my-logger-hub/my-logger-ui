use rest_api_shared::{ProductScopeHttpModel, ProductScopesHttpModel};

use crate::my_logger_grpc::{GetProductScopesResponse, ProductScopeGrpcModel};

impl From<GetProductScopesResponse> for ProductScopesHttpModel {
    fn from(value: GetProductScopesResponse) -> Self {
        Self {
            default_scope: value.default_scope,
            scopes: value.scopes.into_iter().map(|itm| itm.into()).collect(),
        }
    }
}

impl From<ProductScopeGrpcModel> for ProductScopeHttpModel {
    fn from(value: ProductScopeGrpcModel) -> Self {
        Self {
            name: value.name,
            applications: value.applications,
        }
    }
}
