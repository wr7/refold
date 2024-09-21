## Refold
`refold` is a commandline tool for performing text-wrapping, similar to unix `fold`. Unlike `fold`,
`refold` will recombine lines before performing line-wrapping, and it will automatically detect
line prefixes.

### Comparison to `fold`

|                     | `refold`                             | unix `fold` |
| :-----------------: | :----------------------------------: | :---------: |
| Rewrapping          | Yes                                  | No          |
| Line prefix support | Yes                                  | No          |
| Line endings        | LF and can auto detect CRLF          | LF only     |
| Default wrapping    | Soft via Unicode splittable property | Hard        |
| Hard wrapping       | Yes                                  | Yes         |
| Soft wrapping       | Yes                                  | Yes*        |

*: `fold` leaves trailing spaces and can only split ASCII space-separated words.

### Example:

`refold --spaces --width=100`:
```
/// I'd just like to interject for a moment. What you're refering to as Linux, is in fact, GNU/Linux,
/// or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating system unto itself, but rather another free component of a fully functioning GNU system made useful by the
/// GNU corelibs, shell utilities
/// and vital system components comprising a full OS as defined by POSIX.
```
->
```
/// I'd just like to interject for a moment. What you're refering to as Linux, is in fact,
/// GNU/Linux, or as I've recently taken to calling it, GNU plus Linux. Linux is not an operating
/// system unto itself, but rather another free component of a fully functioning GNU system made
/// useful by the GNU corelibs, shell utilities and vital system components comprising a full OS as
/// defined by POSIX.
```

## Installing
### Cargo (most platforms)
1. [Install Cargo](https://www.rust-lang.org/tools/install).
2. Run `cargo install refold`.

### Nix
Alternatively, a Nix package can be found in the releases section. Unfortunately, `refold` will not
be updated automatically when using this method.

1. Download `refold.nix` [from the `Releases` page](https://github.com/wr7/refold/releases/latest).
2. Add `(pkgs.callPackage /PATH/TO/REFOLD.NIX {})` to your `configuration.nix` file under
   `environment.systemPackages`, `users.users.YOUR_USERNAME.packages`, or in any other place that
   you can list packages.
