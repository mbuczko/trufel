-- :name update_user_data :!
-- :doc Updates core user's data (email, name, picture)
UPDATE users SET email=$1, name=$2, picture=$3
WHERE user_id=$4

-- :name upsert_user :!
-- :doc Creates new user or updates if one already exists
INSERT INTO users(user_id, email, name, picture) VALUES($1, $2, $3, $4)
ON CONFLICT (email) DO UPDATE
SET user_id=EXCLUDED.user_id, name=EXCLUDED.name, picture=EXCLUDED.picture

-- :name fetch_user_by_id :<> :?
-- :doc Fetches user by its identifier
SELECT user_id, email, name, picture FROM users WHERE user_id = $1
