FROM cypress/browsers:latest

COPY ./ /e2e

WORKDIR /e2e

RUN npm ci

RUN npm run build:examples:web

ENTRYPOINT [ "npm", "run" ]