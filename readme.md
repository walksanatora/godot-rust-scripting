#Godot-Rust Scripting Example

this example project shows the minimum to get godot to create a custom language
uses the [`elsa`](https://crates.io/crates/elsa) to create append-only maps that can be written to when not mut

this project implements a funny language called "MathLang"
MathLang is really simple. each line is a named math expression eg:
name: 1+1
in godot each line is exposed as a property of the node.
so in the above line you could get `2.0` when evaluating `node.name` as MathLang can only return floats.
MathLang has no functions and is lazily calculated such that
a: 1
b: 3
c: a+b
accessing `a` will only calc and store `1`, but accessing `b` will calc `b` to be `3` and then calc 1+3 to get 4 for `c`. subsequent accesses hit the cache and are therefore faster