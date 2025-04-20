DELETE FROM public.users WHERE id = $user_id
RETURNING $table_fields;