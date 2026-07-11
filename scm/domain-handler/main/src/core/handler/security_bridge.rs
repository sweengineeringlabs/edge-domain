//! Bridge from `edge_security_runtime::SecurityContext` to the local
//! `domain-handler` decoupling boundary (SEA `no_foreign_type`).

use edge_security_runtime::SecurityContext;

use crate::api::SecurityPrincipal;

impl SecurityPrincipal for SecurityContext {}
