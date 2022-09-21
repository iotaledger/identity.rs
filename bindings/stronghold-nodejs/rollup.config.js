import commonjs from "@rollup/plugin-commonjs";
import typescript from "@rollup/plugin-typescript";
import copy from "rollup-plugin-copy";
import dts from "rollup-plugin-dts";

export default [{
    input: "js/index.ts",
    output: {
        dir: "dist",
        format: "cjs",
    },
    external: ["@iota/identity-wasm/node", "fs", "path"], // so it's not included
    plugins: [
        typescript(),
        commonjs(),
        copy({
            targets: [
                { src: "napi-dist/*.node", dest: "dist" },
            ],
        }),
    ],
}, {
    // path to your declaration files root
    input: "./dist/index.d.ts",
    output: [{ file: "dist/index.d.ts", format: "es" }],
    plugins: [dts()],
}];
