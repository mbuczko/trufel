-- :name fetch_categories_for_user_id :<> :*
-- :doc Fetches user's defined categories
SELECT category_id, name
FROM categories
WHERE user_id = $1
