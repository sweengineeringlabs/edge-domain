//! Core implementations — `DomainExtension` and `DomainSpi` for `NoopDomainExtension`.

use crate::api::{DomainExtension, DomainSpi, NoopDomainExtension};

impl DomainExtension for NoopDomainExtension {}

impl DomainSpi for NoopDomainExtension {}
