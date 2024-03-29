CREATE TABLE companies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL
);

CREATE TABLE funds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    start_year INTEGER NOT NULL,
    manager INTEGER NOT NULL,
    FOREIGN KEY(manager) REFERENCES companies(id)
);

CREATE TABLE aliases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fund_id INTEGER NOT NULL,
    alias TEXT NOT NULL,
    FOREIGN KEY(fund_id) REFERENCES funds(id)
);

CREATE TABLE investments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fund_id INTEGER NOT NULL,
    company_id INTEGER NOT NULL,
    FOREIGN KEY(fund_id) REFERENCES funds(id),
    FOREIGN KEY(company_id) REFERENCES companies(id)
);
