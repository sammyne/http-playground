#!/bin/sh

## CA
openssl req -new -x509 -nodes \
    -newkey ec:<(openssl ecparam -name secp384r1) \
    -keyout ca.key \
    -out ca.cert \
    -subj /C=CN/ST=SH/O=ORG \
    -days 90 \
    -batch

## server 
openssl genrsa -out server.key 2048
### -param_enc explicit - tells openssl to embed the full parameters of the curve in the key
#openssl ecparam -name prime256v1 -genkey -param_enc explicit -out server.key.pem
#openssl pkcs8 -topk8 -nocrypt -inform PEM -in server.key.pem -outform PEM -out server.key

openssl req -new -key server.key \
    -out server.csr \
    -subj /C=CN

openssl x509 -req \
    -in server.csr \
    -out server.cert \
    -CA ca.cert \
    -CAkey ca.key \
    -sha256 \
    -days 90 \
    -set_serial 456 \
    -extensions v3_server \
    -extfile openssl.conf

## client 
#openssl genrsa -out client.key 2048
#openssl req -new -key client.key \
#    -out client.csr \
#    -subj /C=CN
#
#openssl x509 -req \
#    -in client.csr \
#    -out client.cert \
#    -CA ca.cert \
#    -CAkey ca.key \
#    -sha256 \
#    -days 30 \
#    -set_serial 456 \
#    -extensions v3_client \
#    -extfile openssl.conf

rm *.csr