select user_id, username, email, password from main.users
where username = $1;