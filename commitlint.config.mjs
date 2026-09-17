export default {
  parserPreset: {
    parserOpts: {
      headerPattern: /^([^:\s]+): (.+)$/,
      headerCorrespondence: ["type", "subject"],
    },
  },
  rules: {
    "body-empty": [2, "never"],
    "header-max-length": [2, "always", 50],
    "subject-empty": [2, "never"],
    "type-empty": [2, "never"],
  },
};
