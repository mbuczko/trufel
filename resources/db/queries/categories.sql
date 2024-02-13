-- :name fetch_categories_for_user_id :<> :*
-- :doc Fetches user's defined categories
SELECT category_id, name, position
FROM categories
WHERE user_id = $1
ORDER by position

-- :name create_new_category :1
-- :doc Creates a new category for given user_id
INSERT INTO categories(category_id, user_id, name, position)
VALUES ($1, $2, $3, (select coalesce(max(position)+1, 0) from categories where user_id=$2))
RETURNING category_id, position
