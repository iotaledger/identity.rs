import { defineConfig } from "cypress";

export default defineConfig({
    screenshotOnRunFailure: false,
    video: false,
    requestTimeout: 10000,
    defaultCommandTimeout: 60000,
    retries: {
        runMode: 3,
    },
    e2e: {
        supportFile: false,
    },
});
