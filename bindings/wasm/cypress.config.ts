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
        // Fix to make subtle crypto work in cypress firefox
        // https://github.com/cypress-io/cypress/issues/18217
        setupNodeEvents(on, config) {
            on("before:browser:launch", (browser, launchOptions) => {
                if (browser.family === "firefox") {
                    launchOptions.preferences[
                        "network.proxy.testing_localhost_is_secure_when_hijacked"
                    ] = true;
                    launchOptions.preferences[
                        "remote.active-protocols"
                    ] = 3;
                }
                return launchOptions;
            });
        },
    },
});
