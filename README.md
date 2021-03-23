# SERF

Serve with CGI using httpd.

Create a certs directory.

    cd certs
    ./create_dev_certs.sh
    cd ..

Build the containers.

    docker-compose build

Run the containers.

    docker-compose up -d
