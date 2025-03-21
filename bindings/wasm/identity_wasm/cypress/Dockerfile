FROM cypress/browsers:latest

ARG IOTA_IDENTITY_PKG_ID

ENV IOTA_IDENTITY_PKG_ID=$IOTA_IDENTITY_PKG_ID

COPY ./ /e2e

WORKDIR /e2e/identity_wasm

RUN npm ci

RUN npm run build:examples:web

ENTRYPOINT [ "npm", "run" ]