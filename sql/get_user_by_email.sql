SELECT user_id, username, email, password FROM main.users
WHERE email = $1;