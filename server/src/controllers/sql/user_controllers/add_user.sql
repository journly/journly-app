INSERT INTO public.users(id, email, first_name, last_name, username)
VALUES ($1, $2, $3, $4, $5, $6)
RETURNING $table_fields;