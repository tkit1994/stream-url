FROM debian:stable-slim
ARG TARGETARCH
ARG TARGETVARIANT
WORKDIR /opt/stream-url
COPY --chmod=0755 ${TARGETARCH}${TARGETVARIANT}/server server
EXPOSE 80
CMD [ "./server" ]
