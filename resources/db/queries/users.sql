-- name: fetch_user_by_id
-- doc: Fetches user by its identifier
-- and that's almost that!
SELECT user_id, email, name, picture FROM users WHERE user_id = $1

-- name: update_user
-- doc: Juhu juhuu!
SELECT user_id, email, name, picture FROM users WHERE user_id = $1

-- name: delete_user
-- doc: Posyła juzera w pizdu
DELETE FROM users;
