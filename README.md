# spiral

> State: Design/Unstable

```spiral

imports previous_compiler::prelude::*
{
    let unknown impl concept#0
    let concept impl unknown#1
    let graph impl concept#2
    let concept impl graph#3
    let type  impl graph#4
    let graph  impl type#4
}#01CCWNQMY54ZH07DYY60Y3JH2P
```

spiral start as conceptual, evolutionary, multi-graph and context-oriented language programming,
from the concept `unknown` and a basic operation `take`  it build up to the point in where it becomes
an asynchronous data flow language and from that point, the language develops other sets of basic concepts
that allow constructing complex applications in the today word, 
the compiler add some syntax sugar for several basics concepts to give the user a pleasant 
experience when using spiral, but the set of sugared syntax is keep as low and simple as possible. 

## docs 

1. [ast spec](meta/docs/ast.md)
2. Compiler graphs
3. Compiler context

## a wanted list of features

1. observable pattern. 
2. real time application
2. computation at the edge, spiral can describe two kind of conceptual application edge and translators, edge application should be implemented in other language programming, initially the first runtime is implemented in rust  due its portability, edge applications comunicate each to another using message passing. translators applications have the rol to transform the concepts between edge applications, translator are compiled just in time and execute at the respective edge (bidireccional comunication).
2. evolutionary application, all the translators application can be patched in real time
3. incremental computing
4. not custom logic, all the logic should be base on concepts
5. (omniql) JIT compilation of api interfaces between applications, this is related with 2

## what kind of application can be done with spiral?

potentially anything, imagine that you create a house in USA, and want to recreate the same house in Japan, if you wrote all the plan in spiral, the whole concepts of your plan will be transferer by the compiler to Japan applications builders, availability of material, locations, costs. it will find similar material and similar builders and create a plan that adapt to the japan concepts and with the help of other application even fix ambiguities. 


### examples of cool applications that are in the spiral vision

1. videogame with complex magic system that will allow to its player create custom magics,
    and transfer that knowledge to other players 
2. chemical reactions 
3. machine learning algorithms that can create new concepts (translator applications) on the fly
4. have a set of workers that each one only supports a simple set of task then coordinate all them to create complex tasks
5. ux desing and ab testing engines see https://github.com/nebtex/holo
6. compile human adn to its 3d representation at specific age, 
   then use that view for create a human body in a parallel process and insert your brain there, :) 
7. cellular automation
8. actor model

# Initial Roadmap

- [ ] Create spiral dataflow spec
- [ ] implements a spiral compiler and runtime
- [ ] create basic dev tools and package management
