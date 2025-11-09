# Security Policy

## Reporting

Thank you for taking the time to responsibly disclose any problems you find.

**Do not file public issues as they are open for everyone to see!**

All security vulnerabilities in `gd` should be reported by email
to [security@gdps.app][Security Email].
Your report will be acknowledged within 24 hours, and you will receive a more
detailed response within 48 hours indicating the next steps in handling your report.

You can encrypt your report using our public key:
[`14143790CA958778E20C2E98DC239CC273E96223`][Security Key].
This key is also available on [MIT's Key Server][MIT Key Server]
and [reproduced below](#security-key).

After the initial reply to your report, the core team will try to keep you
informed of the progress being made towards a fix and official announcement.
These updates will be sent at least every five days. In reality, this is
more likely to be every 24-48 hours.

## Disclosure Policy

`gd` has a 5-step disclosure process:

1. The security report is received and is assigned a primary handler.
   This person will coordinate the fix and release process.

2. The problem is confirmed and a list of all affected versions is determined.

3. Code is audited to find any potential similar problems.

4. Fixes are prepared for all releases which are still under maintenance.
   These fixes are not committed to the public repository but rather
   held locally pending the announcement.

5. On the embargo date, the changes are pushed to the public repository
   and new builds are deployed.

This process can take some time, especially when coordination is required
with maintainers of other projects. Every effort will be made to handle
the issue in as timely a manner as possible, however it is important that
we follow the release process above to ensure that the disclosure is handled
in a consistent manner.

## Security Key

```text
-----BEGIN PGP PUBLIC KEY BLOCK-----

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
d8Q=
=qE59
-----END PGP PUBLIC KEY BLOCK-----
```

## Attribution

This Security Policy is adapted from [Rust's Security Policy][Rust Security Policy].

[Security Email]: mailto:security@gdps.app
[Security Key]: https://gdps.app/keys/security
[MIT Key Server]: https://pgp.mit.edu/pks/lookup?op=index&search=0x14143790CA958778E20C2E98DC239CC273E96223
[Rust Security Policy]: https://rust-lang.org/policies/security
