# GC - Graph Commands

A suite of command-line tools implemented in Rust to create, manipulate and query directed graphs.  
A vertex is represented by a positive integer id.

All commands start with ```gc-``` and have a ```--help``` option.

This project is a work in progress. It is not possible (yet!) to assign attributes to edges and name nodes. 


# Get started

To create an empty graph stored in the current directory, you can use
```
gc-init
```

To add vertices / edge, use
```
gc-add --edge 1 2
```
This will add a new directed edge to the graph: 1 -> 2.

You can add more vertices in one go by using some of the available options from gc-add. For instance :
```
gc-add --clique `seq 1 10`
```
This will create a clique between 10 vertices

# Shortest path with constraints

There is an implementation to find the shortest path in the graph provided some constraints. You can for instance exclude certain nodes, include others, ask for a cycle to be part of your path, etc.

For details on the available constraints and options, you can use
```
gc-csp --help
```

Limitation : At the moment the weights (score) of each edge are 1. The algorithm can take into account different positive weights, but the command-line interface doesn't allow (yet !) to specify them.


# Commands available

## gc-add

Adds a vertex or an edge to a graph

```
USAGE:  
    gc-add.exe [FLAGS] [OPTIONS] --path <path>  

FLAGS:  
    -h, --help       Prints help information  
    -r, --reverse    Reverses created edges  
    -V, --version    Prints version information  

OPTIONS:  
    -c, --chain <chain>         Link all the provided vertices by a directed edge, effectvely creating a chain  
    -C, --clique <clique>       Creates a clique (ie all vertices will be connected to all other vertices)  
    -O, --cycle <cycle>         Creates a directed cycle with all vertices provided  
    -e, --edge <edge>           Adds a directed edge between 2 vertex ids  
    -p, --path <path>           Use the specified directory instead of the current one [default: .]  
    -X, --star <star>           Creates a star pattern, using first vertex as center, and with all edges directed out of the center  
    -v, --vertex <vertex>       Adds a vertex id to a graph  
```

## gc-build

Builds a graph from the list of commands

```
USAGE:  
    gc-build.exe [OPTIONS] --path <path>  

FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information  

OPTIONS:  
    -p, --path <path>          Use the specified directory instead of the current one [default: .]  
    -v, --verbose <verbose>    Verbose mode [default: false]  
```

## gc-clean

Cleans a graph

```
USAGE:  
    gc-clean.exe [FLAGS] --path <path>  

FLAGS:  
    -f, --force      By-pass interactive confirmation  
    -h, --help       Prints help information  
    -s, --silent     Don't print anything on stdout in case of success  
    -V, --version    Prints version information  

OPTIONS:  
    -p, --path <path>    Use the specified directory instead of the current one [default: .]  
```

## gc-csp

Constrained short-path

```
USAGE:  
    gc-csp.exe [FLAGS] [OPTIONS] --end <end> --path <path> --start <start>  

FLAGS:  
    -h, --help             Prints help information  
        --include-cycle    Must include at least a cycle  
        --no-cycle         Must not include any cycle  
    -V, --version          Prints version information  

OPTIONS:  
    -e, --end <end>                      End node  
        --exact-length <exact-length>    Exact number of vertices to be included  
        --exact-score <exact-score>      Exact expected score  
        --exclude <exclude>              Must exclude the following nodes  
        --include <include>              Must include the following nodes  
        --max-length <max-length>        Maximum number of vertices to be included  
        --max-score <max-score>          Must have at least the maximum score  
        --min-length <min-length>        Minimum number of vertices to be included  
        --min-score <min-score>          Must have at least the minimum score  
        --ordered <ordered>              Vertices must appear in the provided order  
    -p, --path <path>                    Use the specified directory instead of the current one [default: .]  
    -s, --start <start>                  Starting node  
```

## gc-cycle

Identify cycles

```
USAGE:  
    gc-cycle.exe [OPTIONS] --path <path> <--girth|--hamiltonian|--count|--head|--take-n <take-n>|--all|--shortest|--longest>  

FLAGS:  
    -a, --all            Return all the cycles matching the constraints  
    -c, --count          Count the number of cycles matching the constraints  
        --girth          Compute the length of the shortest cycle of the graph. Doesn't allow to specify constraints  
        --hamiltonian    Find a hamiltonian cycle of the graph. Doesn't allow to specify constraints  
        --help           Prints help information  
    -h, --head           Find the first cycle matching the constraints  
    -L, --longest        Return the longest cycle of the graph matching the constraints  
    -s, --shortest       Return the shortest cycle of the graph matching the constraints  
    -V, --version        Prints version information  

OPTIONS:  
        --exact-length <exact-length>              Return all the cycles from the graph with the provided length  
        --exact-score <exact-score>                Return all the cycles from the graph with the provided score  
        --exclude-all <exclude-all>                Return all the cycles from the graph that not including any of the provided vertices  
        --exclude-all-edges <exclude-all-edges>    Return all the cycles from the graph that not including any of the provided edges  
        --include-all <include-all>                Return all the cycles from the graph that are including all the provided vertices  
        --include-all-edges <include-all-edges>    Return all the cycles from the graph that are including all the provided edges  
        --max-length <max-length>                  Return all the cycles from the graph with a length less than or equal to max-length  
        --max-score <max-score>                    Return all the cycles from the graph with a length less than or equal to max-score  
        --min-length <min-length>                  Return all the cycles from the graph with a length greater than or equal to min-length  
        --min-score <min-score>                    Return all the cycles from the graph with a score greater than or equal to min-score  
    -p, --path <path>                              Use the specified directory instead of the current one [default: .]  
    -n, --take-n <take-n>                          Find n cycles matching the constraints  
```

## gc-delete

Remove a vertex or an edge from a graph

```
USAGE:  
    gc-delete.exe [OPTIONS] --path <path>  

FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information  

OPTIONS:  
    -e, --edge <edge>           Removes a directed edge  
    -p, --path <path>           Use the specified directory instead of the current one [default: .]  
    -v, --vertex <vertex>       Removes a vertex id from a graph  
```

## gc-desc

Prints some basic statistics on the graph

```
USAGE:  
    gc-desc.exe --path <path>  

FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information  

OPTIONS:  
    -p, --path <path>    Use the specified directory instead of the current one [default: .]  
```

## gc-init

Creates an empty graph

```
USAGE:  
    gc-init.exe [OPTIONS]  

FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information  

OPTIONS:  
    -p, --path <path>    Use the specified directory instead of the current one [default: .]  
```

## gc-random

Creates a random graph

```
USAGE:  
    gc-random.exe [FLAGS] [OPTIONS] --path <path> --vertex-count <vertex-count>  

FLAGS:  
    -c, --connected    Creates a connected graph  
    -f, --force        By-pass interactive confirmation  
    -h, --help         Prints help information  
    -V, --version      Prints version information  

OPTIONS:  
    -O, --cycle <cycle>                  Add a cycle to the graph  
    -e, --edge-count <edge-count>        Target the provided number of edges  
    -p, --path <path>                    Use the specified directory instead of the current one [default: .]  
    -v, --vertex-count <vertex-count>    Creates the graph with the given number of vertices [default: 100]  
```

## gc-short-path

Builds a graph from the list of commands

```
USAGE:  
    gc-short-path.exe --end <end> --path <path> --start <start>  

FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information  

OPTIONS:  
    -e, --end <end>        End node  
    -p, --path <path>      Use the specified directory instead of the current one [default: .]  
    -s, --start <start>    Starting node  
```

## gc-topo-sort

Compute a topological order of a directed graph

```
USAGE:  
    gc-topo-sort.exe --path <path>  

FLAGS:  
    -h, --help       Prints help information  
    -V, --version    Prints version information  

OPTIONS:  
    -p, --path <path>    Use the specified directory instead of the current one [default: .]
```
