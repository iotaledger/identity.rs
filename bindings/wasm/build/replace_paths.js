const fs = require("fs/promises");
const path = require("path");

/**
 * Replaces aliases defined in `tsconfig.json` files with their corresponding paths.
 * If more than one path is defined. The second path is used. Otherwise the first path.
 */
async function main() {
  if (process.argv[2] === "node") await replace("tsconfig.json", "node");
  if (process.argv[2] === "web") await replace("tsconfig.web.json", "web");
}

async function replace(tsconfig, dist) {
  // Read tsconfig file.
  let data = JSON.parse(await fs.readFile(path.join(__dirname, `../lib/${tsconfig}`), "utf8"));
  let a = data.compilerOptions.paths;
  let keys = Object.keys(a);

  // Get `.js` and `.ts` file names from directory.
  let files = await fs.readdir(path.join(__dirname, `../${dist}`), {withFileTypes: true});
  files = files.filter((directoryItem) => directoryItem.isFile()).map((file) => file.name);
  files = files.filter((fileName) => fileName.endsWith(".ts") || fileName.endsWith(".js"));

  // Replace the Alias with the second path if present, otherwise use first path.
  for (let file of files) {
    let fileData = await fs.readFile(path.join(__dirname, `../${dist}/${file}`), "utf8");
    for (let key of keys) {
      let value = a[key][1] ?? a[key][0];
      fileData = fileData.replace(key, value);
    }
    await fs.writeFile(path.join(__dirname, `../${dist}/${file}`), fileData, "utf8");
  }
}

try {
  main().then(() => {});
} catch (e) {
  console.log(e);
  return e;
}
