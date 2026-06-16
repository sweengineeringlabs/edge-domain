//! Core trait impls for [`NonEmptyString`].

use crate::api::valueobject::traits::ValueObject;
use crate::api::valueobject::traits::value_object_factory::ValueObjectFactory;
use crate::api::valueobject::types::NonEmptyString;

impl ValueObject for NonEmptyString {}

impl ValueObjectFactory for NonEmptyString {}
