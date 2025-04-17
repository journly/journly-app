DELETE FROM public.users WHERE user_id = $user_id
RETURNING $table_fields