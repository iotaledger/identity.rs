import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import dts from "rollup-plugin-dts";
import copy from 'rollup-plugin-copy'

export default [
{
    input: 'wasm-node/index.ts',
    output: {
        dir: 'node',
        format: 'cjs',
        exports: 'named',
    },
    // TODO: should we also externalize @iota/iota.js and node-fetch?
    external: ['util', 'fs', 'path'], // so it's not included
    plugins: [
        typescript({
            rootDir: 'wasm-node',
            outDir: 'node',
            esModuleInterop: true,
        }),
        commonjs(),
        copy({
            targets: [
                { src: 'wasm-node/identity_wasm_bg.wasm', dest: 'node' },
                { src: 'wasm-node/identity_wasm.d.ts', dest: 'node' },
                { src: 'wasm-node/package.json', dest: 'node' }
            ],
        })
    ],
}, 
{
    // path to your declaration files root
    input: './node/index.d.ts',
    output: [{ file: 'node/index.d.ts', format: 'es' }],
    plugins: [dts()],
},
];
