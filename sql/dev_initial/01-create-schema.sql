---- Base app schema

-- User
CREATE TABLE "user"
(
    id       BIGINT GENERATED BY DEFAULT AS IDENTITY (start with 1000) PRIMARY KEY,
    username varchar(128) NOT NULL UNIQUE
);


-- Task
CREATE TABLE TASK
(
    id    BIGINT GENERATED BY DEFAULT AS IDENTITY (start with 1000) PRIMARY KEY,
    title varchar(256) NOT NULL
);
