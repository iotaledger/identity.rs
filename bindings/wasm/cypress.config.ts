import { defineConfig } from "cypress";

export default defineConfig({
    screenshotOnRunFailure: false,
    video: false,
    retries: {
        runMode: 3,
    },
    e2e: {
        setupNodeEvents(on, config) {},
        supportFile: false,
    },
});
