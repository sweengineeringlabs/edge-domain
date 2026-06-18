//! Core trait impls for [`NonEmptyString`].

use crate::api::ValueObject;
use crate::api::ValueObjectFactory;
use crate::api::NonEmptyString;

impl ValueObject for NonEmptyString {}

impl ValueObjectFactory for NonEmptyString {}
