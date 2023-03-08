import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
// temporary workaround until https://github.com/Swatinem/rollup-plugin-dts/issues/254 is released
import dts from "./examples/build/rollup-plugin-dts/dist/rollup-plugin-dts.mjs";
import copy from 'rollup-plugin-copy'

export default [{
    input: 'js/index.ts',
    output: {
        dir: 'dist',
        format: 'cjs'
    },
    external: ['@iota/identity-wasm/node', 'fs', 'path'], // so it's not included
    plugins: [
        typescript(),
        commonjs({
            ignore: (id) => {
                // exclude binary files
                return id.endsWith('.node');
            },
        }),
        copy({
            targets: [
                { src: 'napi-dist/*.node', dest: 'dist' }
            ]
        })
    ],
},
{
    // path to your declaration files root
    input: './dist/index.d.ts',
    output: [{ file: 'dist/index.d.ts', format: 'es' }],
    plugins: [dts()],
},
];
