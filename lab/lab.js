const unindent = (s) => {
  const match = s.match(/\n +/);
  return !match ? s.trim() : s.split(match[0]).join("\n").trim();
};

const wrap = (s) => {
  // const cols = Math.min(Math.max(20, process.stdout.columns) || 80, 80) - 5;
  const cols = 80;
  return unindent(s)
    .split(/[ \n]+/)
    .reduce((left, right) => {
      const last = left.split("\n").pop();
      const join =
        last.length && last.length + right.length > cols ? "\n" : " ";
      return left + join + right;
    });
};

console.log(
  JSON.stringify(
    wrap(`
    A basic-auth string to use when authenticating against the npm registry.
    This will ONLY be used to authenticate against the npm registry.  For other
    registries you will need to scope it like "//other-registry.tld/:_auth"

    Warning: This should generally not be set via a command-line option.  It
    is safer to use a registry-provided authentication bearer token stored in
    the ~/.npmrc file by running \`npm login\`.
  `)
  )
);
