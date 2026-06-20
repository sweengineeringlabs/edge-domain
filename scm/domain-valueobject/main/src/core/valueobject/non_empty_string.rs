//! Core trait impls for [`NonEmptyString`].

use crate::api::ValueObject;
use crate::api::ValueObjectBootstrap;
use crate::api::NonEmptyString;

impl ValueObject for NonEmptyString {}

impl ValueObjectBootstrap for NonEmptyString {}
