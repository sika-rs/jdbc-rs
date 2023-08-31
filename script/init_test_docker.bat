docker run -p 3306:3306 --name jdbc-test-mysql -e MYSQL_ROOT_PASSWORD=jdbc-test -e MYSQL_DATABASE=test -d mysql:8.1.0

docker run -p 5432:5432 --name jdbc-test-postgres -e POSTGRES_PASSWORD=jdbc-test -e POSTGRES_DB=test -d postgres