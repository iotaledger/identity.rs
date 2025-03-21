FROM cypress/browsers:latest

COPY ./ /e2e

WORKDIR /e2e/identity_wasm

RUN npm ci

RUN npm run build:examples:web

ENTRYPOINT [ "npm", "run" ]