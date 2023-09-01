INSERT INTO companies (name) VALUES ('Foo Inc.'), ('Bar Inc.'), ('Baz Inc.');

INSERT INTO funds (name, manager, start_year) VALUES ('Foo Fund', 1, 2023), ('Bar Fund', 1, 2023), ('Baz Fund', 3, 2023);

INSERT INTO aliases (alias, fund_id) VALUES ('Foo', 1), ('ooF', 1), ('BAZ', 2);

INSERT INTO investments (fund_id, company_id) VALUES (1, 2), (2, 2), (3, 1), (1, 1);
