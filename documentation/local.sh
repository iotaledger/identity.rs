PROJECT_ROOT=$(git rev-parse --show-toplevel)
rm -rf local
mkdir local
cd local
git clone --branch main https://github.com/iota-community/iota-wiki.git
cd $(ls -d */|head -n 1)
mkdir external
cd external
git clone $PROJECT_ROOT ./identity.rs
cd ../
npm i
export config=$(cat $PROJECT_ROOT/documentation/EXTERNAL_DOCS_CONFIG)
export replace_string='/\* AUTO GENERATED EXTERNAL DOCS CONFIG \*/'
perl -0pe 's#$ENV{replace_string}#$ENV{config}#' docusaurus.config.js > docusaurus.config.js.cpy
rm -f docusaurus.config.js && mv docusaurus.config.js.cpy docusaurus.config.js
export config=$(cat $PROJECT_ROOT/documentation/EXTERNAL_DOCS_DROPDOWN_CONFIG)
export replace_string='/\* AUTO GENERATED EXTERNAL DOCS DROPDOWN CONFIG \*/'
perl -0pe 's#$ENV{replace_string}#$ENV{config}#' docusaurus.config.js > docusaurus.config.js.cpy
rm -f docusaurus.config.js && mv docusaurus.config.js.cpy docusaurus.config.js
npm run start -- --host=0.0.0.0