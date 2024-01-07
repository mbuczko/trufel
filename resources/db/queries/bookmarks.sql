-- :name fetch_bookmarks_for_user_id :<> :*
-- :doc Fetches user's defined bookmarks
SELECT bookmark_id, name, url, icon, visibility, position, category_id
FROM bookmarks
WHERE user_id = $1
