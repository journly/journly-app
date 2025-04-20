INSERT INTO public.users(id, display_name, username, password_hash)
VALUES ($1, $2, $3, $4)
RETURNING $table_fields;