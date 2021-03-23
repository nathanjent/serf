FROM rust:slim as build
WORKDIR /usr/src/serf
COPY ./serf .
RUN cargo install --path .

FROM httpd:2.4
RUN rm /usr/local/apache2/htdocs/index.html
COPY ./httpd.conf /usr/local/apache2/conf/httpd.conf
COPY ./vhosts.default.conf /etc/apache2/sites-enabled/default.conf
COPY ./www/ /usr/local/apache2/htdocs/
COPY --from=build /usr/local/cargo/bin/serf /usr/local/apache2/htdocs/index.cgi
