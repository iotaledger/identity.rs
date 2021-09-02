rm -rf test
mkdir test
cd test
git clone https://github.com/iota-community/iota-wiki.git
cd iota-wiki
mkdir external
cd external
mkdir identity.rs
cd identity.rs
rsync -r ../../../../* ./
cd ../../
npm i
export config=$(cat .github/workflows/EXTERNAL_DOCS_CONFIG)
export replace_string='/\* AUTO GENERATED EXTERNAL DOCS CONFIG \*/'
perl -0pe 's#$ENV{replace_string}#$ENV{config}#' docusaurus.config.js > docusaurus.config.js.cpy
rm -f docusaurus.config.js && mv docusaurus.config.js.cpy docusaurus.config.js
export config=$(cat .github/workflows/EXTERNAL_DOCS_DROPDOWN_CONFIG)
export replace_string='/\* AUTO GENERATED EXTERNAL DOCS DROPDOWN CONFIG \*/'
perl -0pe 's#$ENV{replace_string}#$ENV{config}#' docusaurus.config.js > docusaurus.config.js.cpy
rm -f docusaurus.config.js && mv docusaurus.config.js.cpy docusaurus.config.js
npm run build