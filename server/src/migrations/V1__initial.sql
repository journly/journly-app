DROP TABLE IF EXISTS user_journals;
DROP TABLE IF EXISTS journals;
DROP TABLE IF EXISTS expense_payers;
DROP TABLE IF EXISTS markers;
DROP TABLE IF EXISTS coordinates;
DROP TABLE IF EXISTS maps;
DROP TABLE IF EXISTS files;
DROP TABLE IF EXISTS expenses;
DROP TABLE IF EXISTS budgeting_trackers;
DROP TABLE IF EXISTS attachments;
DROP TABLE IF EXISTS itinerary_activities;
DROP TABLE IF EXISTS widgets;
DROP TABLE IF EXISTS itineraries;
DROP TABLE IF EXISTS sections;
DROP TABLE IF EXISTS dates;
DROP TABLE IF EXISTS trips;
DROP TABLE IF EXISTS user_trips;
DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    display_name VARCHAR(50),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) UNIQUE,
    password_hash TEXT NOT NULL,
    profile_picture_id UUID 
);

CREATE TABLE IF NOT EXISTS user_trips (
    trip_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY (trip_id, user_id)
);

CREATE TABLE IF NOT EXISTS trips (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL,
    title VARCHAR(100) NOT NULL,
    trip_image VARCHAR(255) NOT NULL,
    dates_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS dates (
    id UUID PRIMARY KEY,
    start_date DATE NOT NULL,
    end_date DATE NOT NULL
);  

CREATE TABLE IF NOT EXISTS sections (
    id UUID PRIMARY KEY,
    trip_id UUID NOT NULL,
    title VARCHAR(100) NOT NULL,
    order_rank SMALLINT NOT NULL
);

CREATE TABLE IF NOT EXISTS itineraries (
    id UUID PRIMARY KEY,
    widget_id UUID NOT NULL,
    dates_id UUID NOT NULL,
    map_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS widgets (
    id UUID PRIMARY KEY,
    section_id UUID NOT NULL,
    widget_type VARCHAR(50) NOT NULL,
    order_rank SMALLINT NOT NULL,
    width SMALLINT NOT NULL,
    height SMALLINT NOT NULL,
    content jsonb NOT NULL
);

CREATE TABLE IF NOT EXISTS itinerary_activities (
    id UUID PRIMARY KEY,
    itinerary_id UUID NOT NULL,
    activity_type VARCHAR(50) NOT NULL,
    date_id UUID NOT NULL,
    coordinates_id UUID,
    start_time TIME,
    end_time TIME,
    expense_id UUID,
    notes TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS attachments (
    trip_id UUID NOT NULL,
    file_id UUID NOT NULL,
    activity_id UUID,
    PRIMARY KEY (trip_id, file_id)
);

CREATE TABLE IF NOT EXISTS budgeting_trackers (
    id UUID PRIMARY KEY,
    widget_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    total_budget NUMERIC(10, 2),
    currency VARCHAR(3) NOT NULL
);

CREATE TABLE IF NOT EXISTS expenses (
    id UUID PRIMARY KEY,
    budgeting_tracker_id UUID NOT NULL,
    title VARCHAR(100) NOT NULL,
    cost NUMERIC(10, 2) NOT NULL,
    expense_type VARCHAR(100) NOT NULL,
    split_type VARCHAR(50) NOT NULL
);

CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    file_name VARCHAR(255) NOT NULL,
    file_url VARCHAR(255) NOT NULL UNIQUE,
    file_hash TEXT NOT NULL UNIQUE,
    content_type VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS maps (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    map_type VARCHAR(255) NOT NULL,
    title VARCHAR(255) NOT NULL,
    coordinates_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS coordinates (
    id UUID PRIMARY KEY,
    longitude DOUBLE PRECISION NOT NULL,
    latitude DOUBLE PRECISION NOT NULL
);

CREATE TABLE IF NOT EXISTS markers (
    id UUID PRIMARY KEY,
    coordinates_id UUID NOT NULL,
    activity_id UUID NOT NULL
);

CREATE TABLE IF NOT EXISTS expense_payers (
    expense_id UUID NOT NULL,
    user_id UUID NOT NULL,
    PRIMARY KEY (expense_id, user_id)
);

CREATE TABLE IF NOT EXISTS journals (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL,
    content TEXT NOT NULL,
    last_edit TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_journals (
    user_id UUID NOT NULL,
    journal_id UUID NOT NULL,
    PRIMARY KEY (user_id, journal_id)
);

-- Foreign Keys
ALTER TABLE users ADD CONSTRAINT fk_files_users FOREIGN KEY (profile_picture_id) REFERENCES files(id);

ALTER TABLE user_trips ADD CONSTRAINT fk_user_user_trips FOREIGN KEY (user_id) REFERENCES users(id);
ALTER TABLE user_trips ADD CONSTRAINT fk_trip_user_trips FOREIGN KEY (trip_id) REFERENCES trips(id);

ALTER TABLE trips ADD CONSTRAINT fk_dates_trips FOREIGN KEY (dates_id) REFERENCES dates(id);
ALTER TABLE trips ADD CONSTRAINT fk_owner_trips FOREIGN KEY (owner_id) REFERENCES users(id);

ALTER TABLE sections ADD CONSTRAINT fk_trip_sections FOREIGN KEY (trip_id) REFERENCES trips(id);

ALTER TABLE itineraries ADD CONSTRAINT fk_widget_itineraries FOREIGN KEY (widget_id) REFERENCES widgets(id);
ALTER TABLE itineraries ADD CONSTRAINT fk_dates_itineraries FOREIGN KEY (dates_id) REFERENCES dates(id);
ALTER TABLE itineraries ADD CONSTRAINT fk_map_itineraries FOREIGN KEY (map_id) REFERENCES maps(id);

ALTER TABLE widgets ADD CONSTRAINT fk_section_widgets FOREIGN KEY (section_id) REFERENCES sections(id);

ALTER TABLE itinerary_activities ADD CONSTRAINT fk_itinerary_activities FOREIGN KEY (itinerary_id) REFERENCES itineraries(id);
ALTER TABLE itinerary_activities ADD CONSTRAINT fk_expense_activities FOREIGN KEY (expense_id) REFERENCES expenses(id);
ALTER TABLE itinerary_activities ADD CONSTRAINT fk_coordinates_activities FOREIGN KEY (coordinates_id) REFERENCES coordinates(id);

ALTER TABLE attachments ADD CONSTRAINT fk_trip_attachments FOREIGN KEY (trip_id) REFERENCES trips(id);
ALTER TABLE attachments ADD CONSTRAINT fk_file_attachments FOREIGN KEY (file_id) REFERENCES files(id);
ALTER TABLE attachments ADD CONSTRAINT fk_activity_attachments FOREIGN KEY (activity_id) REFERENCES itinerary_activities(id);

ALTER TABLE budgeting_trackers ADD CONSTRAINT fk_widget_budgeting FOREIGN KEY (widget_id) REFERENCES widgets(id);

ALTER TABLE expenses ADD CONSTRAINT fk_budgeting_tracker_expenses FOREIGN KEY (budgeting_tracker_id) REFERENCES budgeting_trackers(id);

ALTER TABLE files ADD CONSTRAINT fk_user_files FOREIGN KEY (user_id) REFERENCES users(id);

ALTER TABLE maps ADD CONSTRAINT fk_user_maps FOREIGN KEY (user_id) REFERENCES users(id);
ALTER TABLE maps ADD CONSTRAINT fk_coordinates_maps FOREIGN KEY (coordinates_id) REFERENCES coordinates(id);

ALTER TABLE markers ADD CONSTRAINT fk_coordinates_markers FOREIGN KEY (coordinates_id) REFERENCES coordinates(id);
ALTER TABLE markers ADD CONSTRAINT fk_activities_markers FOREIGN KEY (activity_id) REFERENCES itinerary_activities(id);

ALTER TABLE expense_payers ADD CONSTRAINT fk_expense_payers FOREIGN KEY (expense_id) REFERENCES expenses(id);
ALTER TABLE expense_payers ADD CONSTRAINT fk_user_payers FOREIGN KEY (user_id) REFERENCES users(id);

ALTER TABLE user_journals ADD CONSTRAINT fk_user_user_journals FOREIGN KEY (user_id) REFERENCES users(id);
ALTER TABLE user_journals ADD CONSTRAINT fk_journal_user_journals FOREIGN KEY (journal_id) REFERENCES journals(id);

