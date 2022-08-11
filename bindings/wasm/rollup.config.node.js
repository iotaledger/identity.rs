import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import dts from "rollup-plugin-dts";
import copy from 'rollup-plugin-copy-merge'
import path from 'path';

export default [
    {
        input: 'wasm-node/stardust_identity_client.ts',
        output: {
            file: 'wasm-node/stardust_identity_client.js',
            format: 'cjs',
        },
        treeshake: false,
        // TODO: should we also externalize @iota/iota.js and node-fetch?
        external: [path.resolve( __dirname, 'wasm-node/identity_wasm.js' ),], // so it's not included
        plugins: [
            typescript({
                rootDir: 'wasm-node',
                outDir: 'wasm-node',
                // module: 'commonjs',
                moduleResolution: 'node',
                //esModuleInterop: true,
            }),
            commonjs({ transformMixedEsModules: true }),
            copy({
                targets: [
                    { src: 'wasm-node/identity_wasm_bg.wasm', dest: 'node' },
                    { src: 'wasm-node/*.d.ts', dest: 'node' },
                    { src: 'wasm-node/package.json', dest: 'node' },
                    {
                        src: [
                            'wasm-node/identity_wasm.js',
                            'wasm-node/stardust_identity_client.js'
                        ],
                        file: 'node/index.js'
                    },
                ]
            }),
        ],
    },
    // {
    //     // path to your declaration files root
    //     input: './node/index.d.ts',
    //     output: [{ file: 'node/index.d.ts', format: 'es' }],
    //     plugins: [dts()],
    // },
];
