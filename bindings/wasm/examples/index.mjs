import { Project, SyntaxKind } from "ts-morph";

const project = new Project({override: true});

project.addSourceFilesAtPaths("src/**/*{.d.ts,.ts}");

const files = project.getDirectory('src').getSourceFiles();

files.forEach(file => {
    const copy = file.copyToDirectory("../dist/web", { overwrite: true })
    copy.getImportDeclarations().forEach(imp => {
        if(imp.getModuleSpecifierValue() === '@iota/identity-wasm') {
            imp.setModuleSpecifier('@iota/identity-wasm/web')
        }
        if(imp.getModuleSpecifierValue() === '@iota/identity-stronghold-nodejs') {
            imp.remove()
        }
    });
    copy.getVariableDeclaration("strongholdPath").remove();
    copy.getVariableDeclaration("password").remove();
    copy.getVariableDeclaration("stronghold").remove();

    // TODO
    const builder = copy.getVariableDeclaration("builder").getInitializer();
    console.log(builder);
});

await project.save();
