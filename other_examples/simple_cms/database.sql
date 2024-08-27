CREATE TABLE files (
    id INT AUTO_INCREMENT PRIMARY KEY,
    filename VARCHAR(255) NOT NULL,
    content LONGBLOB NOT NULL,
    content_type VARCHAR(255) NOT NULL
);

INSERT INTO `files` VALUES (1,'hello.html','<H1>Hello from MySQL!</H1>','text/html');
