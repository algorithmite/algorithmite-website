CREATE TABLE routes (
    route_key serial PRIMARY KEY,
    parent integer REFERENCES routes,
    url_slug text
);

CREATE TABLE roles (
    role_key serial PRIMARY KEY,
    role_name varchar(255) NOT NULL,
    role_level integer NOT NULL DEFAULT 0,
    can_edit_pages boolean NOT NULL DEFAULT false,
    can_edit_posts boolean NOT NULL DEFAULT false,
    can_edit_roles boolean NOT NULL DEFAULT false,
    can_moderate_roles boolean NOT NULL DEFAULT false,
    can_moderate_comments boolean NOT NULL DEFAULT false,
    can_embed boolean NOT NULL DEFAULT false,
    can_comment boolean NOT NULL DEFAULT false,
    comments_visible boolean NOT NULL DEFAULT false,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW()
);

SELECT
    diesel_manage_updated_at('roles');

CREATE TABLE users (
    user_key serial PRIMARY KEY,
    user_role integer REFERENCES roles NOT NULL,
    username varchar(255) UNIQUE NOT NULL,
    email text NOT NULL,
    password_hash text NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    deleted_at timestamp
);

SELECT
    diesel_manage_updated_at('users');

CREATE TYPE user_action_types AS ENUM (
    'login',
    'logout',
    'create',
    'delete',
    'reset_password_logged_in',
    'reset_password_email_sent',
    'reset_password_email'
);

CREATE TABLE user_actions (
    user_action_key serial PRIMARY KEY,
    actor integer REFERENCES users NOT NULL,
    ip inet,
    user_action user_action_types NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TYPE moderation_action_types AS ENUM (
    'change_role',
    'delete_comment',
    'undo_delete_comment'
);

CREATE TABLE moderation_actions (
    moderation_action_key serial PRIMARY KEY,
    moderator integer REFERENCES users NOT NULL,
    actor integer REFERENCES users NOT NULL,
    pre_action_role integer REFERENCES roles NOT NULL,
    moderation_action moderation_action_types NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW()
);

CREATE TABLE posts (
    post_key serial PRIMARY KEY,
    url_route integer REFERENCES routes NOT NULL,
    author integer REFERENCES users NOT NULL,
    title text,
    content text,
    tab_text text,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    deleted_at timestamp
);

SELECT
    diesel_manage_updated_at('posts');

CREATE TABLE pages (
    page_key serial PRIMARY KEY,
    url_route integer REFERENCES routes NOT NULL,
    template_location text,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    deleted_at timestamp
);

SELECT
    diesel_manage_updated_at('pages');

CREATE TABLE comments (
    comment_key serial PRIMARY KEY,
    commenting_user integer REFERENCES users NOT NULL,
    commented_post integer REFERENCES posts NOT NULL,
    commented_comment integer REFERENCES comments,
    content text NOT NULL,
    created_at timestamp NOT NULL DEFAULT NOW(),
    updated_at timestamp NOT NULL DEFAULT NOW(),
    deleted_at timestamp
);

SELECT
    diesel_manage_updated_at('comments');