FROM rust:1.78.0-alpine as builder

WORKDIR /usr/src/app

#RUN apt-get update
RUN apk --no-cache add build-base
RUN apk --no-cache add ca-certificates

COPY . .

RUN cargo build --release


FROM alpine:latest

WORKDIR /app

#RUN apt-get update
#RUN apt-get install -y ca-certificates tzdata
RUN apk --no-cache add ca-certificates

COPY --from=builder /usr/src/app/target/release/example_actix_web /app/example_actix_web

EXPOSE 8181

#USER $APP_USER

#CMD ["tail", "-F", "/var/log/dummy.log"]
CMD ["/app/example_actix_web"]
