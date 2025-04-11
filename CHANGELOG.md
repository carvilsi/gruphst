# Changelog

# [v0.16.0](https://github.com/carvilsi/gruphst/releases/tag/v0.16.0) (2025-04-?)

- adds get_uniq_vertices_on_graphs to return a collection with the unique vertices from all vaults
- adds export Graphs to Graphviz format

# [v0.15.0](https://github.com/carvilsi/gruphst/releases/tag/v0.15.0) (2024-10-08)

- deprecating persists method and adding save one
- adding warning for deprecated method persists()
- added method to remove a vault
- implemented import-export from and to CSV and adding some examples; improving documentation
- changing classic for loop into iter(), since allegedly it is slightly faster
- adds test for import from CSV when headers are not present and error checking when the relation for an Edge is missing on a CSV row import file
- added benchmarking for CSV import file in order to improve the performance of the function
- implementing GruPHstError for better error handling
- adding argon2 implementation for hashed values for attribute vertex
- adding examples for argon2 hashes
- added method for vertex to retrieve all attributes keys and vec_u8 ones
- refactoring and splitting graphs queries
- added method find_edges_with_vertex_attr_vec_u8_equals_to for graphs query
- renaming a method to maintain consistency
- added methods for edge queries to cover vector u8 vertex attributes
- adding has_vertex_with_attr_key_like method for edge queries and some renaming there
- adds method has_attr_key_like for vertex
- implemented vertex queries related with Vec<u8> attributes
- improves vertex queries added vertex queries related with u8 vector attributes
- implemented new method to add a collection of edges into the graph vault
- added custom path to persists file

# [v0.13.0](https://github.com/carvilsi/gruphst/releases/tag/v0.13.0) (2024-08-19)

- supports Vec<u8> for Vertex attribute content: *set_attr_vec_u8* and *get_attr_vec_u8*

# [v0.12.0](https://github.com/carvilsi/gruphst/releases/tag/v0.12.0) (2024-08-15)

- updating README and lib for rust docs
- adding some TODOs
- finishing for now with examples
- added method to find a Vertex on an Edge by id
- added method to find a Vertex in Graphs by id
- adding documentation for new methods
- added middle-earth example 
- improved *add_ege* performance
- improved code coverage to 100% :)
- added pre-release script and improving publish one

# [v0.11.1](https://github.com/carvilsi/gruphst/releases/tag/v0.11.1) (2024-08-13)

- added changelog 
- fixed graphs load from persisted file when file was big 
- put back memory watcher
- added benchmarking for *add_edge* method

