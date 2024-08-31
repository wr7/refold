## Refold
Refold is a commandline tool with similar functionality to unix `fold`.

Refold attempts to have more sane defaults and more modern functionality.

|                     | `refold`                             | unix `fold` |
| :-----------------: | :----------------------------------: | :---------: |
| Line prefix support | Yes                                  | No          |
| Line endings        | LF and can auto detect CRLF          | LF only     |
| Default wrapping    | Soft via Unicode splittable property | Hard        |
| Hard wrapping       | Yes                                  | Yes         |
| Soft wrapping       | Yes                                  | Yes [^1]    |

[^1]: `fold` leaves trailing spaces and can only split ASCII space-separated
       words.
