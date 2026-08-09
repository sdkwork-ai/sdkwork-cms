//! CMS backend-api gateway route manifest derived from the authored route manifest.

use sdkwork_web_core::{HttpMethod, HttpRoute, HttpRouteManifest};

use crate::manifest::{RouteDefinition, RouteManifest};

fn http_method(method: &str) -> HttpMethod {
    match method {
        "POST" => HttpMethod::Post,
        "PUT" => HttpMethod::Put,
        "PATCH" => HttpMethod::Patch,
        "DELETE" => HttpMethod::Delete,
        _ => HttpMethod::Get,
    }
}

fn http_route(definition: &RouteDefinition) -> HttpRoute {
    let method = http_method(definition.method);
    let route = match definition.auth_mode {
        "dual-token" => {
            HttpRoute::dual_token(method, definition.path, "cms", definition.operation_id)
        }
        "api-key" => HttpRoute::api_key(method, definition.path, "cms", definition.operation_id),
        _ => HttpRoute::public(method, definition.path, "cms", definition.operation_id),
    };
    let route = if definition.permission.is_empty() {
        route
    } else {
        route.with_required_permission(definition.permission)
    };
    if definition.idempotent {
        route.with_idempotent(true)
    } else {
        route
    }
}

pub fn gateway_route_manifest_from(manifest: &RouteManifest) -> HttpRouteManifest {
    HttpRouteManifest::from_owned_routes(
        manifest
            .routes
            .iter()
            .map(http_route)
            .collect::<Vec<_>>(),
    )
}

pub fn gateway_route_manifest() -> HttpRouteManifest {
    gateway_route_manifest_from(&crate::manifest::cms_backend_api_manifest())
}
