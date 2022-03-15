const rootPackage = require('../../package.json')

module.exports = (options) => {

    const newPackage = {
        name: rootPackage.name,
        collaborators: rootPackage.collaborators,
        description: rootPackage.description,
        version: rootPackage.version,
        license: rootPackage.license,
        repository: rootPackage.repository,
        files: options.files,
        main: options.main,
        module: options.module,
        homepage: rootPackage.homepage,
        types: options.types,
        keywords: options.keywords,
    }

    // remove empty keys
    Object.keys(newPackage).forEach(key => newPackage[key] === undefined && delete newPackage[key])

    return newPackage;

}