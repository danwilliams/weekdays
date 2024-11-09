# Weekdays

The Weekdays crate provides a simple [`Weekdays`](https://docs.rs/weekdays/latest/weekdays/struct.Weekdays.html)
type for representing days of the week bit-mapped in a single byte, and
functionality for working with them.

This is particularly useful when representing a set of days in a database field
or similar.

## Bit layout

Each bit represents a day of the week. The bits are ordered from most
significant to least significant, starting from Monday, with the least
significant bit representing Sunday.

```text
Monday
| Tuesday
| | Wednesday
| | | Thursday
| | | | Friday
| | | | | Saturday
| | | | | | Sunday
1 1 1 1 1 1 1
```


## Feature flags

The following feature flags are available:

  - `chrono`: Enables conversion to and from the [`Weekday`](https://docs.rs/chrono/latest/chrono/enum.Weekday.html)
    type from the [Chrono](https://crates.io/crates/chrono) crate.
  - `postgres`: Implements the [`ToSql`](https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.ToSql.html)
    and [`FromSql`](https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.FromSql.html)
    traits for use with [tokio-postgres](https://crates.io/crates/tokio-postgres).
  - `serde`: Enables serialisation and deserialisation with [Serde](https://crates.io/crates/serde)
    by implementing the [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html)
    and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html)
    traits.

Additionally:

  - `default`: Has no features enabled.
  - `full`: Enables all features.


## Database schema

When using the `postgres` feature, the expectation is that the database field
will be configured as `BITS(7)`, i.e. a 7-bit bitfield.


