// this code will not run
use barrel::{types, Migration, backend::Pg};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table("users", |t| {
        t.add_column("id", types::uuid().primary(true));
        t.add_column("email", types::varchar(200).nullable(false));
        t.add_column("first_name", types::varchar(200).nullable(false));
        t.add_column("last_name", types::varchar(200).nullable(false));
        t.add_column("username", types::varchar(50).nullable(false).unique(true));
    });

    m.make::<Pg>()
}
