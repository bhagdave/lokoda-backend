-- Add up migration script here
CREATE OR REPLACE VIEW vw_discover AS
SELECT
    users.id,
    name,
    account_type,
    location,
    avatar_url,
    image_url,
    JSON_ARRAY(x.genres) as genres
FROM
    users
        LEFT JOIN
    (SELECT
        user_id,
            GROUP_CONCAT(json_object("id:", genre_id, "genre:", genre)
                ORDER BY genre ASC
                SEPARATOR ', ') AS genres
    FROM
        user_genres
    JOIN genres ON genres.id = user_genres.genre_id
    GROUP BY user_id
    ) AS x ON x.user_id = users.id
