// @generated automatically by Diesel CLI.

diesel::table! {
    accommodations (id) {
        id -> Uuid,
        trip_id -> Uuid,
        check_in_datetime -> Nullable<Timestamptz>,
        check_out_datetime -> Nullable<Timestamptz>,
        location -> Nullable<Uuid>,
        from_document -> Nullable<Uuid>,
    }
}

diesel::table! {
    budget_planners (id) {
        id -> Uuid,
        trip_id -> Uuid,
        total_budget -> Nullable<Numeric>,
        currency -> Nullable<Text>,
        accommodation_budget -> Nullable<Numeric>,
        transportation_budget -> Nullable<Numeric>,
        food_dining_budget -> Nullable<Numeric>,
        activities_budget -> Nullable<Numeric>,
        shopping_budget -> Nullable<Numeric>,
    }
}

diesel::table! {
    documents (id) {
        id -> Uuid,
        trip_id -> Uuid,
        filename -> Text,
        document_url -> Text,
        file_hash -> Text,
        file_type -> Text,
        file_size -> BigInt,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    expense_payers (expense_id, user_id) {
        expense_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    expenses (id) {
        id -> Uuid,
        trip_id -> Uuid,
        title -> Nullable<Text>,
        cost -> Nullable<Numeric>,
        currency -> Nullable<Text>,
    }
}

diesel::table! {
    flights (id) {
        id -> Uuid,
        trip_id -> Uuid,
        flight_code -> Nullable<Text>,
        departure_datetime -> Nullable<Timestamptz>,
        arrival_datetime -> Nullable<Timestamptz>,
        departure_location -> Nullable<Uuid>,
        arrival_location -> Nullable<Uuid>,
        from_document -> Nullable<Uuid>,
    }
}

diesel::table! {
    itinerary_items (id) {
        id -> Uuid,
        trip_id -> Uuid,
        title -> Text,
        activity_type -> Nullable<Text>,
        location_id -> Nullable<Uuid>,
        start_time -> Nullable<Timestamptz>,
        end_time -> Nullable<Timestamptz>,
        expense_id -> Nullable<Uuid>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    journals (id) {
        id -> Uuid,
        created_at -> Nullable<Timestamptz>,
        last_editted -> Nullable<Timestamptz>,
        content -> Nullable<Text>,
    }
}

diesel::table! {
    locations (id) {
        id -> Uuid,
        address -> Text,
        display_name -> Nullable<Text>,
        longitude -> Float8,
        latitude -> Float8,
    }
}

diesel::table! {
    maps (id) {
        id -> Uuid,
        map_type -> Nullable<Text>,
        title -> Nullable<Text>,
        trip_id -> Nullable<Uuid>,
    }
}

diesel::table! {
    notes (id) {
        id -> Uuid,
        trip_id -> Uuid,
        user_id -> Uuid,
        note -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    occupants (accommodation_id, user_id) {
        accommodation_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    passengers (flight_id, user_id) {
        flight_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    personal_budgets (id) {
        id -> Uuid,
        trip_id -> Uuid,
        user_id -> Uuid,
        total_budget -> Nullable<Numeric>,
        accommodation_budget -> Nullable<Numeric>,
        transportation_budget -> Nullable<Numeric>,
        food_dining_budget -> Nullable<Numeric>,
        activities_budget -> Nullable<Numeric>,
        shopping_budget -> Nullable<Numeric>,
        currency -> Nullable<Text>,
        personal_budget_enabled -> Bool,
    }
}

diesel::table! {
    tasks (id) {
        id -> Uuid,
        trip_id -> Uuid,
        description -> Text,
        completed -> Bool,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    trip_invites (id) {
        id -> Uuid,
        trip_id -> Uuid,
        user_id -> Nullable<Uuid>,
        invitee_email -> Nullable<Text>,
        status -> Nullable<Text>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    trips (id) {
        id -> Uuid,
        owner_id -> Uuid,
        title -> Nullable<Text>,
        banner_image -> Nullable<Text>,
        start_date -> Nullable<Date>,
        end_date -> Nullable<Date>,
        no_collaborators -> Int4,
    }
}

diesel::table! {
    user_journal (user_id, journal_id) {
        user_id -> Uuid,
        journal_id -> Uuid,
        permission -> Nullable<Text>,
    }
}

diesel::table! {
    user_map (user_id, map_id) {
        user_id -> Uuid,
        map_id -> Uuid,
    }
}

diesel::table! {
    user_trip (user_id, trip_id) {
        user_id -> Uuid,
        trip_id -> Uuid,
        permission -> Nullable<Text>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        password_salt -> Bytea,
        avatar -> Nullable<Text>,
        is_admin -> Bool,
    }
}

diesel::joinable!(accommodations -> documents (from_document));
diesel::joinable!(accommodations -> locations (location));
diesel::joinable!(accommodations -> trips (trip_id));
diesel::joinable!(budget_planners -> trips (trip_id));
diesel::joinable!(documents -> trips (trip_id));
diesel::joinable!(expense_payers -> expenses (expense_id));
diesel::joinable!(expense_payers -> users (user_id));
diesel::joinable!(expenses -> trips (trip_id));
diesel::joinable!(flights -> documents (from_document));
diesel::joinable!(flights -> trips (trip_id));
diesel::joinable!(itinerary_items -> expenses (expense_id));
diesel::joinable!(itinerary_items -> locations (location_id));
diesel::joinable!(itinerary_items -> trips (trip_id));
diesel::joinable!(maps -> trips (trip_id));
diesel::joinable!(notes -> trips (trip_id));
diesel::joinable!(notes -> users (user_id));
diesel::joinable!(occupants -> accommodations (accommodation_id));
diesel::joinable!(occupants -> users (user_id));
diesel::joinable!(passengers -> flights (flight_id));
diesel::joinable!(passengers -> users (user_id));
diesel::joinable!(personal_budgets -> trips (trip_id));
diesel::joinable!(personal_budgets -> users (user_id));
diesel::joinable!(tasks -> trips (trip_id));
diesel::joinable!(trip_invites -> trips (trip_id));
diesel::joinable!(trip_invites -> users (user_id));
diesel::joinable!(trips -> users (owner_id));
diesel::joinable!(user_journal -> journals (journal_id));
diesel::joinable!(user_journal -> users (user_id));
diesel::joinable!(user_map -> maps (map_id));
diesel::joinable!(user_map -> users (user_id));
diesel::joinable!(user_trip -> trips (trip_id));
diesel::joinable!(user_trip -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    accommodations,
    budget_planners,
    documents,
    expense_payers,
    expenses,
    flights,
    itinerary_items,
    journals,
    locations,
    maps,
    notes,
    occupants,
    passengers,
    personal_budgets,
    tasks,
    trip_invites,
    trips,
    user_journal,
    user_map,
    user_trip,
    users,
);
