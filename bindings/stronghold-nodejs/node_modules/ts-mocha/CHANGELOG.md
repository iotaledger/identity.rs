## v2.0.0

### Breaking changes:
- updated ts-node to most recent version `v7.0.0`
- now it is required to pass a full tsconfig path (previously folder path was sufficient) this change propagated from ts-node upgrade.  
- in programmatic use it's using now `TS_NODE_PROJECT` env variable instead of now deprecated `_TS_PROJECT_PATH__`
- tsconfig-paths integration can be enabled by using `TS_CONFIG_PATHS=true` env variable
