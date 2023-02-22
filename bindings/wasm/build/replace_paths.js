const fs = require("fs");
const path = require("path");

/**
 * Replaces aliases defined in `tsconfig.json` files with their corresponding paths.
 * If more than one path is defined. The second path is used. Otherwise the first path.
 */

async function replace(tsconfig, dist, mode) {
    // Read tsconfig file.
    const tsconfigPath = path.join(__dirname, '..', tsconfig);
    console.log(`\n using ${tsconfigPath}`);
    let data = JSON.parse(fs.readFileSync(path.join(__dirname, '..', tsconfig), "utf8"));
    let a = data.compilerOptions.paths;
    let keys = Object.keys(a);

    // Get `.js` and `.ts` file names from directory.
    const distPath = path.join(__dirname, `../${dist}`);
    console.log(`\n working in ${distPath}`);
    let files = readdirSync(distPath);
    files = files.filter((fileName) => fileName.endsWith(".ts") || fileName.endsWith(".js"));

    // Replace the Alias with the second path if present, otherwise use first path.
    for (let file of files) {
        console.log(`\n processing ${file}`);
        let fileData = fs.readFileSync(file, "utf8");
        for (let key of keys) {
            let value = a[key][1] ?? a[key][0];
            console.log(`\t replacement value: ${value}`);
            const absoluteIncludePath = path.resolve(path.dirname(tsconfigPath), value);
            if(mode == "resolve" && fs.existsSync(absoluteIncludePath)) {
                const absoluteFilePath = path.resolve(path.dirname(file));
                console.log(`\t\t resolving ${absoluteFilePath} to ${absoluteIncludePath}`);
                value = path.relative(absoluteFilePath, absoluteIncludePath);
                console.log(`\t\t resolved relative replacement value: ${value}`);
            }
            fileData = fileData.replace(key, value);
        }
        fs.writeFileSync(file, fileData, "utf8");
    }
}

const readdirSync = (p, a = []) => {
    if (fs.statSync(p).isDirectory())
      fs.readdirSync(p).map(f => readdirSync(a[a.push(path.join(p, f)) - 1], a))
    return a
  }
  
(async () => {
    try {
        await replace(process.argv[2], process.argv[3], process.argv[4]);
    } catch (e) {
        console.log(e);
        return e;
    }
})();
