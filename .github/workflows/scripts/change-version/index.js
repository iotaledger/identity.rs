const glob = require('glob')
const fs = require('fs')
const path = require('path');
const { argv } = require('process');

const newVersion = argv[2];

glob("*(identity|identity-*|bindings)/**/Cargo.toml",{cwd: '../../..'}, (err, files) => {

    if(err) {
        throw err;
    }

    files.forEach(filePath => {
        const resolvedPath = path.join('../../..', filePath);
        fs.readFile(resolvedPath, "utf-8", (err, file) => {

            if(err) {
                throw err;
            }

            const newFile = file.replace(/(name = ".*"\nversion = ")\d.\d.\d(")/, '$1'+newVersion+'$2')
            
            if(file === newFile) {
                throw new Error(resolvedPath+' not changed');
            }

            fs.writeFile(resolvedPath, newFile, () => {
                console.log('transformed', resolvedPath);
            })
        })
    })
})

glob("bindings/wasm/*(package|package-lock).json",{cwd: '../../..'}, (err, files) => {

    if(err) {
        throw err;
    }

    files.forEach(filePath => {
        const resolvedPath = path.join('../../..', filePath);
        fs.readFile(resolvedPath, "utf-8", (err, file) => {

            if(err) {
                throw err;
            }

            const newFile = JSON.stringify({...JSON.parse(file), version: newVersion}, null, 2);
            
            if(file === newFile) {
                throw new Error(resolvedPath+' not changed');
            }

            fs.writeFile(resolvedPath, newFile, () => {
                console.log('transformed', resolvedPath);
            })
        })
    })
})
