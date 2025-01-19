-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    locked BOOLEAN DEFAULT FALSE,
    admin BOOLEAN DEFAULT FALSE
);

INSERT INTO
    users (username, email, password, admin)
VALUES
    (
        'admin',
        'admin@admin.com',
        '$argon2id$v=19$m=19,t=2,p=1$UXZBNjVlQW5YTTdDZDRpdA$S9lPULwfN3/rSDgqITDzrQ',
        TRUE
    );

CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE videos (
    id SERIAL PRIMARY KEY,
    viewkey VARCHAR(11),
    title TEXT NOT NULL,
    description TEXT,
    url VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    downloaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    downloaded_by INT,
    FOREIGN KEY (downloaded_by) REFERENCES users (id),
    path VARCHAR(255) NOT NULL,
    duration INT,
    viewcount INT,
    tags INT,
    ext TEXT,
    lang TEXT,
    height INT,
    width INT,
    dynamic_range TEXT,
    availability TEXT,
    fps INT,
    average_rating INT,
    age_limit INT,
    likes INT,
    status TEXT,
    comments INT,
    chapters TEXT
);

CREATE TABLE video_categories (
    video_id INT,
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    categorie_id INT,
    FOREIGN KEY (categorie_id) REFERENCES categories (id) ON DELETE CASCADE
);

CREATE TABLE tags (id SERIAL PRIMARY KEY, name TEXT);

CREATE TABLE video_tags (
    video_id INT,
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    tags_id INT,
    FOREIGN KEY (tags_id) REFERENCES tags (id) ON DELETE CASCADE
);

CREATE TABLE channels (
    id SERIAL PRIMARY KEY,
    channel_key VARCHAR(255) NOT NULL,
    name VARCHAR(255) NOT NULL,
    follower INT,
    verified BOOLEAN
);

CREATE TABLE queue (
    id SERIAL PRIMARY KEY,
    video_id INT NOT NULL,
    task_status VARCHAR(255) DEFAULT 'pending',
    FOREIGN KEY (video_id) REFERENCES videos (id) ON DELETE CASCADE,
    added_by INT,
    FOREIGN KEY (added_by) REFERENCES users (id),
    added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    priority INT DEFAULT 3
);
