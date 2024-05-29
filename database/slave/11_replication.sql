\c pharmacy
DROP SUBSCRIPTION IF EXISTS pharmacy_subscription;
CREATE SUBSCRIPTION pharmacy_subscription CONNECTION 'host=master_database user=postgres password=my_password dbname=pharmacy' PUBLICATION pharmacy_publication;