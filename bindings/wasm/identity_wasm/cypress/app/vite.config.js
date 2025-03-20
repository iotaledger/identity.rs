import { defineConfig } from "vite";

export default defineConfig(({ command, mode }) => {
    return {
        define: {
            "process.env.IOTA_IDENTITY_PKG_ID": JSON.stringify(process.env.IOTA_IDENTITY_PKG_ID),
            "process.env.NETWORK_NAME_FAUCET": JSON.stringify(process.env.NETWORK_NAME_FAUCET),
            "process.env.NETWORK_URL": JSON.stringify(process.env.NETWORK_URL),
        },
        server: {
            strictPort: true,
        },
        build: {
            rollupOptions: {
                output: {
                    interop: "auto",
                },
            },
        },
        // // resolve: {
        // //     alias: [{ find: "@digitalcredentials/did-method-key", replacement: "@digitalcredentials/did-method-key/esm/dist/" }],
        // // },
        // optimizeDeps: {
        //     exclude: ["@digitalcredentials/did-method-key"]
        // }
    };
});
