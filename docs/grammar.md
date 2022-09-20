# Grammar

The following is an Extended Backus–Naur form (EBNF) definition of the Tangibl grammar.
Token codes can be found in the [source
code](https://github.com/battesonb/tangibl-rs/blob/main/src/tokens.rs). These
would map to the TopCode identifiers. For a full list of potential TopCodes, see
the [TIDAL-Lab TopCodes
repository](https://github.com/TIDAL-Lab/TopCodes#valid-topcodes). Any additions
to the Tangibl grammar would need to use one of these unique identifiers. This
limits Tangibl to 99 unique tokens (more than enough).

## EBNF Grammar

```sh
Start = "Start" [Flow];
Flow = (Conditional | Method | Command)*;
Conditional = "Blocked" [Next] [Alternate];
Method = IntegerMethod | BooleanMethod
BooleanMethod = "While" [Body] [Condition] [Next];
IntegerMethod = "Repeat" [Body] [Value] [Next];
Command = "MoveBackwards" | "MoveForwards" | "Shoot" | "TurnLeft" | "TurnRight";
Next = Flow;
Alternate = Flow;
Body = Flow;
Value = "Infinite" | ["1".."5"];
```

### Notes

"Next" does not have to be modelled, and isn't in the implementation, but it
helps clarify the control flow. Additionally, "BooleanMethod" is ill-defined and
should be thought of as a "PredicateMethod". This is just following the
convention set in the original Java implementation.

## Extending the grammar

Any changes to the grammar would need to be brought back into the existing
games. This does not seem worth the effort, but a good argument for it may
convince me. For example, the "Shoot" token could have a negative connotation,
so we could add an alternate token for the same action. Using the same token
with a different print would have the issue of not matching the in-game
confirmation.

Overall, this grammar introduces basic imperative control flow concepts such as
sequences, conditionals (`if`, `unless`, `switch` etc), and loops (`while`,
`for`, etc). One change that may make for an interesting addition is
subroutines, for much bigger challenges. This could allow for multiple scans to
build up a set of subroutines to be executed.
