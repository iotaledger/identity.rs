import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import dts from "rollup-plugin-dts";
import copy from 'rollup-plugin-copy'

export default [
{
    input: 'wasm-web/index.ts',
    output: {
        dir: 'web',
        format: 'cjs'
    },
    external: ['@iota/identity-wasm/node', 'fs', 'path'], // so it's not included
    plugins: [
        typescript({
            rootDir: 'wasm-web',
            outDir: 'web'
        }),
        commonjs(),
        copy({
            targets: [
                { src: 'wasm-web/identity_wasm*.*', dest: 'web' },
                { src: 'wasm-node/package.json', dest: 'node' }
            ],
        })
    ],
}, 
{
    // path to your declaration files root
    input: './web/index.d.ts',
    output: [{ file: 'web/index.d.ts', format: 'es' }],
    plugins: [dts()],
},
];
