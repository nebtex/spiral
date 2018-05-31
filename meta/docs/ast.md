# AST, Syntax and Grammar

the  ast is very simple due that it consist of 3 kind of tokens `nodes, edges and resources`, 
for the  spiral compiler all the concepts are at the same time  `graphs`, `types`, `resources` and nodes always represent a new `add` operation some edges implements the add operation
other are just useful for walk the graphs
.

which mean that every token in a file is declaring a graph, a type and creating a new resource all at the same time,
every resource in the spiral context should have a unique immutable global identifier and
if two concepts have the same id it means that they are the exact same concept.

the default way to express this is

```
   ${node_symbol}#${unique_global_identifier}
   ->{edge_symbol}#${unique_global_identifier_2}
   ->${node_symbol_2}#${unique_global_identifier}
   ->{edge_symbol_2}#${unique_global_identifier}
   ->.....${node_symbol_n}#${unique_global_identifier_n}`
```
 
and in order to access to a backward reference you can use `<-` instead of `->`
```
   ${node_symbol}#${unique_global_identifier}<-{edge_symbol}#${unique_global_identifier_2}
```
 
well but that looks awful, this is where the sugar comes to the rescue 

## ADD operation sugars

in the compiler context add operation means add a new concept to the compiler

the add operation is ignore in the source file and it assumes that the success operation edge 
is always taken. however, you can access to the `fail` and `wait` edges using contexts, 
in the compiler environment those edges can allow you to create nice atomization 
like create a github issue when some concept fails to be created in the compiler, 
report error to your console ide etc

this

```${source_node}->${graph}->${add:{id:unique_global_identifier}}->${graph}->${success}->${graph}->${target_graph}->{target_edge}```

we are telling the compiler when the source node is created, 
if the operation is successful then go to the next operation


becomes 

```${source_node}->{target_edge}#{id:unique_global_identifier}```

additionally depend which edge you take and the context the node is add a multiples graph at the same time, 
other way will be imposable to write code, this all deterministic and you will understand it later

## Block sugars

block are the very important in many programming languages so the compiler has a graph called 
`block` which nodes are `identifier` or `symbols` also that graph have a edge called 
`let` that allow programmer create those symbols in specific block, and a edge called `add block`
which sources can be other nodes. lastly and no least important each block can be created with brackets `{} for nodes or () for edges`

1.  braces sugar, only works on nodes (for access to children of the nodes in multiples graphs)

```${source_node}->${block}->{create_block}#{id:unique_global_identifier}->${content_node}```

becomes
```${source_node}{content}#{id:unique_global_identifier}```

2.  parentheses sugar, only works on edges (to access to the children of the edges in multiples graphs)

```${source_node}->edge(p, p2)->${content_node}```

another sugars

`let` create a new symbol in the current block

`::` represent a walk in the block graph

`super::` backward reference to a parent block

`inport` `imports` those edges use symbols from other blocks


## Graph sugars

because there can be billion of graph is necessary to contextualize the compiler and most important the programmer, 
that is where the  `import` is useful, if a graph is not in context can't be used, if a edge is not in the context you can't use it

```
import core::compiler::name;
or 
imports core::compiler::*;
``` 

also accessing to a graph if the graph is defined in the context is done with the symbol `@`

```${node}->graph->${graph_name}->${edge_name}``` is writen as ```${node}@${graph_name}->${edge_name}```

## Edge sugars

the `->` is replace by space ` ` and `-<` at the end of the block is omitted

```{->let ...... -<}```
which mean enter to the block create symbol and go back to the block can be written as 
```{let ...... }```


### edge stacking

edge can be stacked , example over a block
```
{
    imports previous_compiler::prelude::*
    imports previous_compiler::prelude1::*
    imports previous_compiler::prelude2::*
    imports previous_compiler::prelude3::*
    imports previous_compiler::prelude4::*
    imports previous_compiler::prelude5::*
}
```

stacking on a binding
```
let x impl numeric impl random_number 
```

### return edge 

the `<-` will automatically return the control flow to the block

```
{
    imports previous_compiler::prelude::*
    imports previous_compiler::prelude1::*
    <-imports previous_compiler::prelude2::*
    //dead code 
    imports previous_compiler::prelude4::*
    imports previous_compiler::prelude5::*
}
```
### omit parentheses

if a edge only have

they should return compatible types for stack to they work like a function 

## Resource sugars


every resource needs a  immutable global id, the way to handle this is add a ulid id  the start of a file block
and in every block or end of a statement  put a local id according to the local clock of events in that block,
if there are intermediates resources created in  a walk the compiler will automatically use existing id to give them
under the current path a unique global id 


## After all that sugar
```
selector concept_compiler!#0 (context:Context):Boolean{
    context.spiral.evironment==Compiler && context.applications_present!(spiral::Compiler)
}


derive_view!<Path<Partial, NoSource>>(@Block->idents->{protocol/*}, {Delete, Add, Set, SameNode});

extends_context! 
Concept+#0{

    protocol Name#0,
       
    protocol Folder#1{
      property name#0: Name 
      property path#1: Array<Name>,
      property size#0: Numeric,
      property parent#0: Folder,
    }
    
    protocol File#2 {
       property name#0: Name,
       property path#1: Array<Name>,
       property parent#2: Folder,
       property content#3: Bytes,
       flow!(self.content.size, reduce<Same>)
       fn size#3;
    }
    
    protocol FileSystem#0 {
    
       resource folder#0: Folder;        
       resource file#1: File;
       
       when_compiling!
       enum!(Nominal, {
            item! 
            nominal.unknown!
            fail! {error "Select a tree or graph for the filesystem"}#0
            Unknown,
            Tree,
            Graph
       })
       propery hierarchy_graph_type#0;
       
       match!(hierarchy_graph_type, {
            branch Tree tree!(source: folder, targets: {folder, file}),
            branch Graph, one_edge_dag!(source: folder, targets: {folder, file}, required: True)
       )      
       graph Hierarchy#2;
       
       extends_protocol! Folder 
       #3 Protocol{
         fn parent#0(): Folder{
            return self@Hierarchy<-parent  
         }
         
         property children#1: Extends<Protocol>{
         
            fn neighborhood_files#0()-> View<File>{
                return view<FromPath> self@Hierarchy->children->{*/File}        
            }
                             
            fn neighborhood_folders#1()-> View<Folder>{
                return view<FromPath> self@Hierarchy->children->{*/Folder}            
            }          
         }
                         
         fn total_size#0(): Integer{
             Reduce+SUM(self.children.neighborhood_files(), File.size()) + 
             reduce+SUM(self.children.neighborhood_folders(), Folder.total_size())
         }
       }    
    }
}

extends_context! 
Conceptual #1{

    macro! extends_protocol(block: Block + Protocol, target_proto: Mutable + Protocol): Context{
         //first create a new context with the same block global id
         flow!(block, Protocol::@Concetual::new_path(block@Symbols->ident, -<edges, *), proto)
    }
}
