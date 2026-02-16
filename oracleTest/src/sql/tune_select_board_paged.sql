-- BOARD 페이징 쿼리 인덱스/실행계획 점검 스크립트

-- 1) BOARD.ID 인덱스 존재 확인
SELECT i.index_name,
       i.uniqueness,
       i.status,
       c.column_position,
       c.column_name,
       c.descend
FROM user_indexes i
JOIN user_ind_columns c ON c.index_name = i.index_name
WHERE i.table_name = 'BOARD'
  AND c.column_name = 'ID'
ORDER BY i.index_name, c.column_position;

-- 2) (선택) ID 인덱스가 없다면 생성
-- CREATE INDEX IDX_BOARD_ID ON BOARD (ID DESC);

-- 3) 통계 최신화(권장)
-- BEGIN
--   DBMS_STATS.GATHER_TABLE_STATS(ownname => USER, tabname => 'BOARD', cascade => TRUE);
-- END;
-- /

-- 4) 실행계획 확인
EXPLAIN PLAN FOR
SELECT b.ID, b.TITLE, b.CONTENT, b.CREATED_AT
FROM BOARD b
JOIN (
    SELECT /*+ INDEX_DESC(bi) */ bi.ID
    FROM BOARD bi
    ORDER BY bi.ID DESC
    OFFSET :offset ROWS FETCH NEXT :limit ROWS ONLY
) p ON p.ID = b.ID
ORDER BY b.ID DESC;

SELECT * FROM TABLE(DBMS_XPLAN.DISPLAY);
