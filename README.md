# Junk-Compiler
Compiler for the Junk Programming Language.  This programming language is junk, and shouldn't be used for any reason.

## Compiler Phases

### Lexical Analysis
The lexer parses through the code in the file, breaking it into tokens.  The tokenizer acts like a Finite State Machine, changing states as it parses through each character.  Once it arrives at a final State, the String is saved as a token and passed to the parser.