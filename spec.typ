#set page(
  footer: context {
    let val = counter(page).get().first()
    if calc.even(val) [
      #align(left, counter(page).display("1"))
    ]
    else [
      #align(right, counter(page).display("1"))
    ]
  },
)
#set text(font: "Fira Sans")

= Introduction

==== 1.1:1
This document is the Petri Language Specification. It defines the syntax and semantics of the Petri modeling language.

=== Scope

==== 1.2:1

This specification defines the Petri modeling language. The reference compiler is intended to conform to this specification. It covers:
- Lexical structure
- Statements
- Simulation behavior

==== 1.2:2

This specification does not cover:
- Compiler implementation details

== Conformance

==== 1.3:1

A conforming implementation *MUST* implement all normative requirements of this specification.

==== 1.3:2

A conforming implementation *MUST* correctly parse, validate, and execute programs according to the syntax and semantics defined in this specification.

==== 1.3:3

Behavior not explicitly specified by this document is implementation-defined.

==== 1.3:4

The reference compiler is intended to conform to this specification but does not define the language itself.

==== 1.3:5

All paragraphs are normative unless explicitly marked as informative.

=== Normative Language

==== 1.4:1

This specification uses terminology from RFC 2119 to indicate requirement levels. The keywords are interpreted as follows:

==== 1.4:2

*MUST* and *SHALL*: An absolute requirement. A conforming implementation is required to satisfy this.

==== 1.4:3

*MUST NOT* and *SHALL NOT*: An absolute prohibition. A conforming implementation is required not to do this.

==== 1.4:4

*SHOULD* and *RECOMMENDED*: There may be valid reasons to ignore this behavior, but the implications must be understood.

==== 1.4:5

*SHOULD NOT* and *NOT RECOMMENDED*: There may be valid reasons to accept this behavior, but the implications must be understood.

==== 1.4:6

*MAY* and *OPTIONAL*: An item is truly optional. Implementations may or may not include it.

==== 1.4:7

These keywords appear in *bold* throughout this specification to distinguish normative requirements from descriptive text.

=== Definitions

==== 1.4:8

The following terms are used throughout the specification:

==== 1.4:9

*Normative*: Content that defines required behavior for conforming implementations.

==== 1.4:10

*Informative*: Content that provides explanation or context but does not define required behavior.

==== 1.4:11

*Petri net*: A five tuple $N = (P, T, F, M_0, W)$, where $M$ is the set of places, $T$ the set of transitions, $F := (P times T) union (T times P)$ a multi-set which represents the flow relation, $M_0 := P -> N_0$ are the initial markings and $W := F -> N^+$ the weight function. $P, T, F$ are finite sets with their cardinality being limited by the executing system.

==== 1.4:12

*Place*: An element of $P$ that holds tokens.

==== 1.4:13

*Transition*: An element of $T$ that moves tokens from input places to output places.

==== 1.4:14

*Arc*: A directed connection between place and transition or transition and place.

==== 1.4:15

*Weight*: An element of $NN^+$ that represents the multiplicity of token flow along an arc, which default values is $1$.

==== 1.4:16

*Markings*: A function that is defined as $P -> NN_0$.

==== 1.4:17

*Initial Markings*: The initial marking function.

==== 1.4:18

*Label*: Optional human-readable string for places or transitions that *MUST NOT* affect simulation semantics, they *MAY* be unique.

=== Notation

==== 1.5:1

Spec paragraph identifiers follow the format `{chapter}.{section}:{paragraph}`. For example, `3.1:4` refers to Chapter 3, Section 1, Paragraph 4.

==== 1.5:2

Grammar rules use Extended Backus-Naur Form (EBNF) notation:
- `=` defines a production
- `|` separates alternatives
- `{ }` indicates zero or more repetitions
- `+` indicates at least one repetition
- `[ ]` indicates optional elements
- `" "` indicates literal text

=== Organization

==== 1.6:1

This specification is organized as follows:
- *Chapter 2: Design Goals* - Goals, Non-Goals
- *Chapter 3: Lexical Structure* - Tokens, comments, whitespace, keywords
- *Chapter 4: Syntax (Grammar)* - Program structure, place declaration, transition declaration, arc statements
- *Chapter 5: Semantic Rules*
- *Chapter 6: Execution Semantics*

=== Version

==== 1.7:1

This specification corresponds to version 0.1.0 of the Petri modeling language.

= Design Goals

== Goals

==== 2.1:1

The language shall:
- model class place/transition Petri nets.
- enforce strict bipartite structure
- provide static semantic validation, including duplicate arcs and orphan transitions
- support executable simulation of of token flow.
- remain minimal and human-readable.
- allow cycles in nets to model iterative processes.

== Non-Goals

==== 2.2:1

The language shall not:
- model colored tokens, priorities, inhibitor arcs, or timed/stochastic nets.
- automatically merge repeated arcs.

= Lexical Structure

== Tokens

==== 3.1:1

Tokens are the atomic units of syntax in a Petri net definition. The lexer processes source text and produces a sequence of tokens.

==== 3.1:2

Petri tokens fall into the following categories:

#table(
  columns: 2,
  [*Category*], [*Examples*],
  [Keywords], [place, transition, tokens],
  [Identifiers], [S1, t2],
  [Integer literals], [0, 42, 255],
  [String literals], ["Hello", "world"],
  [Delimiters], [`->`, `[`, `]`, `;`, `=`]
)

=== Integer Literals

==== 3.1:3

An integer literal is a sequence of decimal digits.

```ebnf
integer_literal = digit { digit } ;
digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
```

=== String Literals

==== 3.1:4

A string literal is a sequence of character enclosed in double quotes (`"`).

```ebnf
string_literal = '"' { string_char } '"' ;
string_char = any_char_except_quote_or_backslash | escape_sequence ;
escape_sequence = "\\" | "\"" | "\n" | "\t" | "\r" ;
```

==== 3.1:5

String literals support the following escape sequences:

#table(
  columns: 2,
  [*Escape*], [*Character*],
  [\\\\], [Backslash],
  [\"], [Double quote],
  [\\n], [Newline],
  [\\t], [Horizontal tab],
  [\\r], [Carriage return],
)

==== 3.1:6

An invalid escape sequence in a string literal is a compile-time error.

=== Identifiers

==== 3.1:7

An identifier starts with a letter, followed by any number of letters, digits, or underscores.

```ebnf
identifier = letter { letter | digit | "_" } ;
letter = "a" | ... | "z" | "A" | ... | "Z" ;
```

==== 3.1:8

Identifiers cannot be keywords.

== Comments

==== 3.2:1

Line comments begin with `#` and extend to the end of the line.

==== 3.2:2

Comments are discarded during lexical analysis and do not affect program semantics.

== Whitespace

==== 3.3:1

Whitespace consists of spaces, tabs, and newlines.

```ebnf
whitespace = " " | "\t" | "\n" | "\r" ;
```

== Keywords

==== 3.4:1

Keywords are reserved words that have special meaning in the language.

=== Keywords

==== 3.4:2

The following words are keywords and cannot be used as identifiers:

#table(
  columns: 2,
  [*Keyword*], [*Description*],
  [place], [Place declaration],
  [transition], [Transition declaration],
  [tokens], [Marking declaration]
)

= Syntax (Grammar)

=== Statements

==== 4.1:1

The Petri language is based on three kinds of statements:

#table(
  columns: 2,
  [*Kind*], [*Description*],
  [Place], [Constitutes a place declaration],
  [Transition], [Constitutes a transition declaration],
  [Arc], [Constitutes an arc definition]
)

== Program Structure

==== 4.2:1

A program is defined as a sequence of statements.

==== 4.2:2

```ebnf
program = { statement } ;
statement = place_declaration | transition_declaration | arc_statement ;
```

== Place Declarations

==== 4.3:1

A place declaration introduces a new place.

==== 4.3:2

```ebnf
place_declaration = "place" identifier [ string_literal ] [ "tokens" "=" integer_literal ] ";" ;
```

== Transition Declarations

==== 4.4:1

A transition declaration introduces a new transition.

==== 4.4:2

```ebnf
transition_declaration = "transition" identifier [ string_literal ] ";" ;
```

== Arc Statements

==== 4.5:1

An arc statement introduces a new arc. An arc statement *MAY* assume the form of a chain linking multiple places and transition together in one statement.

==== 4.5:2

```ebnf
arc_statement = identifier ( weighted_arrow identifier )+ ";" ;
weighted_arrow = "->" [ "[" integer_literal "]" ] ;
```

= Semantic Rules

== Declaration Rules

==== 5.1:1

Each place declaration introduces an identifier into the program.

==== 5.1:2

Each transition declaration introduces an identifier into the program.

==== 5.1:3

Identifiers *MUST* be unique across the program.

==== 5.1:4

A place declaration *MAY* specify an initial token count, if omitted it defaults to $0$.

== Structural Rules

==== 5.2:1

A program describes exactly one Petri net.

==== 5.2:2

A Petri net consists of a finite set of places, transitions, and arcs.

==== 5.2:3

A program *MAY* be empty.

== Arc Rules

==== 5.3:1

An arc must connect a place to a transition or a transition to a place.

==== 5.3:2

Arcs between two places or two transitions are invalid.

==== 5.3:3

The source and target of an arc must be declared in the same program.

==== 5.3:4

There *MUST NOT* be multiple arcs between the same source and target.

==== 5.3:5

An arc *MAY* specify a weight greater or equal to $1$.

== Transition Well-Formedness

==== 5.4:1

A transition must have at least one incoming arc and one outgoing arc.

==== 5.4:2

A transition is enabled if all input places contain at least the required number of tokens.

==== 5.4:3

Firing a transition consumes tokens from input places and produces tokens in output places.

== Program Well-Formedness

==== 5.5:1

Every identifier referenced in an arc must correspond to a declared place or transition.

==== 5.5:2

The program must be free of semantic errors before execution or analysis.

==== 5.5:3

The initial marking of the net must be non-negative.

= Execution Semantics
