SELECT ID, TITLE, CONTENT, CREATED_AT
FROM (
    SELECT a.*, ROWNUM rnum
    FROM (
        SELECT ID,
               TITLE,
               CONTENT,
               TO_CHAR(CREATED_AT, 'YYYY-MM-DD') AS CREATED_AT
        FROM BOARD
        ORDER BY ID DESC
    ) a
    WHERE ROWNUM <= :end_row
)
WHERE rnum > :start_row
