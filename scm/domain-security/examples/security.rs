//! Basic `Security` usage example.

use edge_domain_security::{Security, SecurityBootstrap, SecurityContext};

struct TestSecurity;
impl SecurityBootstrap for TestSecurity {}

fn main() {
    let guard = TestSecurity::noop_guard();

    let open: SecurityContext = TestSecurity::unauthenticated();
    println!("open route: {:?}", guard.enforce(&open));

    let principal = TestSecurity::anonymous_principal();
    let authed: SecurityContext = TestSecurity::authenticated(Box::new(principal));
    println!("authed route: {:?}", guard.enforce(&authed));

    let claims = [("role".to_string(), "admin".to_string())].into();
    let from_claims = TestSecurity::from_claims(claims).expect("claims must not be empty");
    println!("role: {:?}", from_claims.claim("role"));
}
