# Taken from rust-postgres repository
> Postgres to Rust type mappings.

The following implementations are provided by this crate, along with the
corresponding Postgres types:

| Rust type                         | Postgres type(s)                              |
|-----------------------------------|-----------------------------------------------|
| `bool`                            | BOOL                                          |
| `i8`                              | "char"                                        |
| `i16`                             | SMALLINT, SMALLSERIAL                         |
| `i32`                             | INT, SERIAL                                   |
| `u32`                             | OID                                           |
| `i64`                             | BIGINT, BIGSERIAL                             |
| `f32`                             | REAL                                          |
| `f64`                             | DOUBLE PRECISION                              |
| `&str`/`String`                   | VARCHAR, CHAR(n), TEXT, CITEXT, NAME, UNKNOWN |
|                                   | LTREE, LQUERY, LTXTQUERY                      |
| `&[u8]`/`Vec<u8>`                 | BYTEA                                         |
| `HashMap<String, Option<String>>` | HSTORE                                        |
| `SystemTime`                      | TIMESTAMP, TIMESTAMP WITH TIME ZONE           |
| `IpAddr`                          | INET                                          |

In addition, some implementations are provided for types in third party
crates. These are disabled by default; to opt into one of these
implementations, activate the Cargo feature corresponding to the crate's
name prefixed by `with-`. For example, the `with-serde_json-1` feature enables
the implementation for the `serde_json::Value` type.

| Rust type                       | Postgres type(s)                    |
|---------------------------------|-------------------------------------|
| `chrono::NaiveDateTime`         | TIMESTAMP                           |
| `chrono::DateTime<Utc>`         | TIMESTAMP WITH TIME ZONE            |
| `chrono::DateTime<Local>`       | TIMESTAMP WITH TIME ZONE            |
| `chrono::DateTime<FixedOffset>` | TIMESTAMP WITH TIME ZONE            |
| `chrono::NaiveDate`             | DATE                                |
| `chrono::NaiveTime`             | TIME                                |
| `time::PrimitiveDateTime`       | TIMESTAMP                           |
| `time::OffsetDateTime`          | TIMESTAMP WITH TIME ZONE            |
| `time::Date`                    | DATE                                |
| `time::Time`                    | TIME                                |
| `eui48::MacAddress`             | MACADDR                             |
| `geo_types::Point<f64>`         | POINT                               |
| `geo_types::Rect<f64>`          | BOX                                 |
| `geo_types::LineString<f64>`    | PATH                                |
| `serde_json::Value`             | JSON, JSONB                         |
| `uuid::Uuid`                    | UUID                                |
| `bit_vec::BitVec`               | BIT, VARBIT                         |
| `eui48::MacAddress`             | MACADDR                             |
| `cidr::InetCidr`                | CIDR                                |
| `cidr::InetAddr`                | INET                                |
| `smol_str::SmolStr`             | VARCHAR, CHAR(n), TEXT, CITEXT,     |
|                                 | NAME, UNKNOWN, LTREE, LQUERY,       |
|                                 | LTXTQUERY                           |

