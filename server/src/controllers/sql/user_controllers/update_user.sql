UPDATE public.users 
SET $new_info WHERE id = $user_id
RETURNING $table_fields;