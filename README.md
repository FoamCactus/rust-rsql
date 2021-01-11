# rust-rsql
rsql parser in rust

Being able to filter api endpoint returns with arbitrary filters is useful.
Having dedicated api endpoints for everything only scales so far.
This crate defines a language for url query specification of those filters.
It is roughly equivalent to https://github.com/jirutka/rsql-parser except that I havent nailed down how to use it in the backend.

Rough roadmap forward
  builders
  to_string/url_encoded impl
  Diesel integration
    derive macros?
