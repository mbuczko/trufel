-- name: fetch_user_by_id
-- doc: Fetches user by its identifier
SELECT user_id, email, name, picture FROM users WHERE user_id = $1
