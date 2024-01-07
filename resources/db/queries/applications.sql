-- :name fetch_applications_for_user_id :<> :*
-- :doc Fetches user's defined applications
SELECT application_id, name, description, url, icon, visibility, position 
FROM applications
WHERE user_id = $1
