-- Replication
SELECT *
FROM pg_create_physical_replication_slot('replication_slot_slave1');

\c pharmacy
DROP PUBLICATION IF EXISTS pharmacy_publication;
CREATE PUBLICATION pharmacy_publication FOR ALL TABLES;