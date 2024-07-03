-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
    '838c2cea-3b5a-40ba-8314-0a8658c3e558' ,
    'admin',
    '$argon2id$v=19$m=15000,t=2,p=1$l8fI6MJkyjnPf7unJEzwpQ$dVv158e22q7YTfd+GdCWwO0/qpgltIyw55BC4HQlye4'
)
