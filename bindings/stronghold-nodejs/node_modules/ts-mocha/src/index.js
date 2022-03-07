try {
  // default ts-node config
  const project =
    process.env.TS_NODE_PROJECT ||
    process.env._TS_PROJECT_PATH__ ||
    './tsconfig.json';
  const transpileOnly = !process.env.TS_TYPE_CHECK;
  require('ts-node').register({
    project,
    transpileOnly,
  });
  // opt-in tsconfig-paths config
  if (process.env.TS_CONFIG_PATHS) {
    require('tsconfig-paths/register');
  }
} catch (error) {
  console.log('[ERROR] ' + error.message);
  process.exit(1);
}
