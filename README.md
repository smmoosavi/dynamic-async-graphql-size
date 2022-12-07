# Experiment with async-grapqhl dynamic-schema feature

This is a simple example of how to use the dynamic-schema feature of async-graphql.

## Objective

The objective of this example is to minimize the size of the generated binary file.

## Sizes
|                                             | Static Schema (B) | Dynamic Schema (B) | commit                |
|---------------------------------------------|------------------:|-------------------:|-----------------------|
| hello world                                 |           453,008 |            453,008 | [e4aa02e]             |
| 1 query                                     |         1,251,728 |          1,280,400 | [304b7b2]             |
| 100 query (macro)                           |         1,493,392 |          1,554,832 | [a292e6b] & [e81085b] |
| 100 query + 100 object (macro)              |         2,673,040 |          1,747,344 | [dfef603]             |
| 100 query + 100 object (dynamic definition) |                 - |          1,304,976 | [b16b9d6]             |


[e4aa02e]: https://github.com/smmoosavi/dynamic-async-graphql-try/commit/e4aa02e80fe72471d07d081f8be4dd25fdaa49f8
[304b7b2]: https://github.com/smmoosavi/dynamic-async-graphql-try/commit/304b7b2e314dbcb2ab739d885862c42c3aafc2f5
[a292e6b]: https://github.com/smmoosavi/dynamic-async-graphql-try/commit/a292e6b141d725edbc75a2a31f5c84219cc1a294
[e81085b]: https://github.com/smmoosavi/dynamic-async-graphql-try/commit/e81085b6d829546ea24db0788de0e68dd571839e
[dfef603]: https://github.com/smmoosavi/dynamic-async-graphql-try/commit/dfef603b62c799fe92de781dd804a575b495a5e8
[b16b9d6]: https://github.com/smmoosavi/dynamic-async-graphql-try/commit/b16b9d665950ae62da8d84a1c5d0ae555626623d
