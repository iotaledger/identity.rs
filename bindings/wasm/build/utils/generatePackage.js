const rootPackage = require('../../package.json')

module.exports = (options) => {

    const newPackage = {
        name: rootPackage.name,
        description: rootPackage.description,
        version: rootPackage.version,
        license: rootPackage.license,
        repository: rootPackage.repository,
        main: options.main,
        module: options.module,
        homepage: rootPackage.homepage,
        types: options.types,
    }

    // remove empty keys
    Object.keys(newPackage).forEach(key => newPackage[key] === undefined && delete newPackage[key])

    return newPackage;

}