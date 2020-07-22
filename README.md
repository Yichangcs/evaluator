# evaluator
## Introduction
This is a tiny learning project for Rust language.
A evaluator for a line of arithmetical expression is implemented in Rust which is inspired by Stroustrup's book “Programming-Principles and Practice Using C++”, chapter 6.
## evaluator structure
This evaluator consists of a Tokenizer, a Parser based on a grammar tree as below.

![grammar%20tree](https://github.com/Yichangcs/evaluator/blob/master/grammar%20tree.jpg)

## How to use

enter a line of arithmetical expression(note that a ';' is necessary to be attached at the end to denote the end of expression ):

((3.0 + 2.0) * 4.0 - 6.0);

this evaluator returns a result:

=> 14.0
