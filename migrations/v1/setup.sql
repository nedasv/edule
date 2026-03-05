-- DATABASE CREATION --

CREATE DATABASE IF NOT EXISTS edule_timetable;
USE edule_timetable;

-- TABLE CREATION --

CREATE TABLE IF NOT EXISTS users (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    role ENUM('student', 'staff', 'admin') NOT NULL DEFAULT 'student',
    student_id INT,
    staff_id INT,
    department_id INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS buildings (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    latitude DOUBLE NOT NULL,
    longitude DOUBLE NOT NULL,
    address TEXT,
    campus VARCHAR(255),
    floors INT DEFAULT 1,
    has_elevator BOOLEAN DEFAULT FALSE,
    has_wheelchair_access BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS departments (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    head_of_department_id INT,
    building_id INT,
    description TEXT,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (head_of_department_id) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (building_id) REFERENCES buildings(id) ON DELETE SET NULL
);

ALTER TABLE users
    ADD CONSTRAINT fk_users_department
    FOREIGN KEY (department_id) REFERENCES departments(id) ON DELETE SET NULL;

CREATE TABLE IF NOT EXISTS room_types (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT
);

CREATE TABLE IF NOT EXISTS rooms (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    building_id INT NOT NULL,
    room_type_id INT NOT NULL,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) UNIQUE NOT NULL,
    capacity INT NOT NULL,
    floor_number INT DEFAULT 0,
    -- Equipment
    has_projector BOOLEAN DEFAULT FALSE,
    has_whiteboard BOOLEAN DEFAULT FALSE,
    has_smartboard BOOLEAN DEFAULT FALSE,
    has_video_conferencing BOOLEAN DEFAULT FALSE,
    has_recording_equipment BOOLEAN DEFAULT FALSE,
    has_computers BOOLEAN DEFAULT FALSE,
    computer_count INT DEFAULT 0,
    has_lab_equipment BOOLEAN DEFAULT FALSE,
    has_wheelchair_access BOOLEAN DEFAULT FALSE,
    has_hearing_loop BOOLEAN DEFAULT FALSE,
    has_air_conditioning BOOLEAN DEFAULT FALSE,
    has_natural_light BOOLEAN DEFAULT FALSE,
    power_outlets INT DEFAULT 0,
    -- Availability
    available_from TIME DEFAULT '09:00:00',
    available_until TIME DEFAULT '22:00:00',
    available_days JSON,
    -- Status
    maintenance_status ENUM('operational', 'maintenance', 'out_of_service') DEFAULT 'operational',
    last_maintenance_date DATE,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (building_id) REFERENCES buildings(id) ON DELETE CASCADE,
    FOREIGN KEY (room_type_id) REFERENCES room_types(id) ON DELETE RESTRICT
);

-- Courses, each row in table would be a scheduled session, as courses can have multiple room types
-- Example: CS1 has lecture, lab and seminar it would be inputted as
--
-- CS1 | lecture | parent_course_id = NULL (the parent)
-- CS1 | lab     | parent_course_id = 1 (id of parent)
-- CS1 | seminar | parent_course_id = 1 (id of parent)
-- 
-- Each entry would have its own room requirements, lecturer capacity etc

CREATE TABLE IF NOT EXISTS courses (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    code VARCHAR(50) NOT NULL,
    department_id INT,
    session_type ENUM("Lecture", "Tutorial", "Lab", "Seminar", "Workshop", "Exam", "Unknown"),
    parent_course_id INT,
    description TEXT,
    credits INT DEFAULT 15,
    year_of_study INT,
    semester ENUM("Autumn", "Spring", "Summer", "Full_Year") NOT NULL,
    academic_year INT NOT NULL,
    sessions_per_week INT DEFAULT 1,
    session_duration_in_minutes INT DEFAULT 60,
    required_room_type_id INT,
    requires_projector BOOLEAN DEFAULT FALSE,
    requires_computers BOOLEAN DEFAULT FALSE,
    requires_lab_equipment BOOLEAN DEFAULT FALSE,
    requires_video_conferencing BOOLEAN DEFAULT FALSE,
    requires_recording BOOLEAN DEFAULT FALSE,
    min_capacity INT,
    max_capacity INT,
    preferred_days JSON,
    excluded_days JSON,
    preferred_start_time TIME,
    preferred_end_time TIME,
    -- Avoid sessions being scheduled one after the other
    avoid_back_to_back BOOLEAN DEFAULT FALSE,
    lecturer_id INT,
    max_enrollment INT,
    current_enrollment INT DEFAULT 0,
    -- Groups if multiple rooms or sessions are needed for a single course
    group_label VARCHAR(50),
    total_groups INT DEFAULT 1,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    -- Prevent duplicate courses
    UNIQUE KEY unique_course_session (code, session_type, group_label, academic_year, semester),
    FOREIGN KEY (department_id) REFERENCES departments(id) ON DELETE SET NULL,
    FOREIGN KEY (required_room_type_id) REFERENCES room_types(id) ON DELETE SET NULL,
    FOREIGN KEY (lecturer_id) REFERENCES users(id) ON DELETE SET NULL
);

ALTER TABLE courses
    ADD CONSTRAINT fk_courses_parent
    FOREIGN KEY (parent_course_id) REFERENCES courses(id) ON DELETE CASCADE;

CREATE TABLE IF NOT EXISTS course_groups (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    parent_course_id INT NOT NULL,
    child_course_id INT NOT NULL,
    is_mandatory BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    -- Prevent duplicate groups
    UNIQUE KEY unique_group_link (parent_course_id, child_course_id),
    FOREIGN KEY (parent_course_id) REFERENCES courses(id) ON DELETE CASCADE,
    FOREIGN KEY (child_course_id) REFERENCES courses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS student_enrollments (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    student_id INT NOT NULL,
    course_id INT NOT NULL,
    group_label VARCHAR(50),
    enrolled_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    -- Prevent duplicate enrollment
    UNIQUE KEY unique_enrollment (student_id, course_id),
    FOREIGN KEY (student_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS timetables (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    semester ENUM('Autumn', 'Spring', 'Summer') NOT NULL,
    academic_year INT NOT NULL,
    status ENUM('draft', 'published', 'archived') DEFAULT 'draft',
    generated_at TIMESTAMP,
    published_at TIMESTAMP,
    generation_config JSON,
    optimization_score DOUBLE,
    created_by INT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (created_by) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS timetable_slots (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    timetable_id INT NOT NULL,
    course_id INT NOT NULL,
    room_id INT NOT NULL,
    lecturer_id INT,
    day_of_week ENUM('mon', 'tue', 'wed', 'thu', 'fri', 'sat', 'sun') NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    slot_type ENUM('lecture', 'tutorial', 'lab', 'seminar', 'workshop', 'exam', 'other') DEFAULT 'lecture',
    recurrence ENUM('weekly', 'biweekly_odd', 'biweekly_even', 'one_off') DEFAULT 'weekly',
    group_name VARCHAR(50),
    is_locked BOOLEAN DEFAULT FALSE,
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (timetable_id) REFERENCES timetables(id) ON DELETE CASCADE,
    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE,
    FOREIGN KEY (room_id) REFERENCES rooms(id) ON DELETE CASCADE,
    FOREIGN KEY (lecturer_id) REFERENCES users(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS change_requests (
    id INT NOT NULL AUTO_INCREMENT PRIMARY KEY,
    requested_by INT NOT NULL,
    slot_id INT NOT NULL,
    change_type ENUM('room_change', 'time_change', 'day_change', 'cancel', 'other') NOT NULL,
    reason TEXT NOT NULL,
    proposed_room_id INT,
    proposed_day ENUM('mon', 'tue', 'wed', 'thu', 'fri', 'sat', 'sun'),
    proposed_start_time TIME,
    proposed_end_time TIME,
    status ENUM('pending', 'approved', 'rejected') DEFAULT 'pending',
    reviewed_by INT,
    review_notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (requested_by) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (slot_id) REFERENCES timetable_slots(id) ON DELETE CASCADE,
    FOREIGN KEY (proposed_room_id) REFERENCES rooms(id) ON DELETE SET NULL,
    FOREIGN KEY (reviewed_by) REFERENCES users(id) ON DELETE SET NULL
);

-- DATA INSERTION

INSERT INTO room_types (name, description) VALUES
    ('Lecture Hall',   'Large capacity room for lectures'),
    ('Seminar Room',   'Medium capacity room for seminars and discussions'),
    ('Laboratory',     'STEM focused laboratory'),
    ('Computer Lab',   'Room with computers'),
    ('Workshop',       'Workshop space'),
    ('Auditorium',     'Large capacity auditorium for events and lectures'),
    ('Tutorial Room',  'Small room for tutorials and group work'),
    ('Studio',         'Creative or performance studio'),
    ('Meeting Room',   'Small meeting or office hours room'),
    ('Exam Hall',      'Dedicated exam hall');

INSERT INTO users (username, email, password_hash, full_name, role)
VALUES (
    'admin',
    'admin@goldsmiths.ac.uk',
    -- Change to actual hash
    'hash',
    'John Smith',
    'admin'
);
