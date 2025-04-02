# Versioning Scheme

This project follows [CalVer](https://calver.org/) for its versioning scheme, starting with `2025.2.1`.
It used to follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html) from the first release through version `4.6.0`.
This versioning approach is both backwards and forwards compatible with Semantic Versioning.

Here is the template for the scheme:

```
<YYYY>.<MM>.<RELEASE-NUMBER>
```

- The first field, `YYYY`, refers to the year of release, specified via four digits.
- The second field, `MM`, refers to the month of release, specified via one (January through September) or two digits (October through December).
- The third field, `RELEASE-NUMBER`, refers to the release number for the given year and month, starting from `0` and incrementing by one for every release.

Here is an example of a theorhetical first release in January 2025:

```
2025.1.0
```

Here is an example of a theorhetical third release in December 2024:

```
2024.12.2
```

In both examples, the exact day of release did not matter.
