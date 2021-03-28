# SERF

Serve with CGI using httpd.

Create development certs for TLS.

    cd certs
    ./create_dev_certs.sh
    cd ..

Build the containers.

    docker-compose build

Run the containers.

    docker-compose up -d
