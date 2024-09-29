# Wasm-Projects
A repository containing all sorts of different wasm projects made using wasmcloud, spin and other tools

Currently this repo contains
1) Calculator-wash : An Http based Calculator that performs operations based on query parameters
2) Rate-limiter : An Http based Rate limiter that keeps track of incoming requests and alerts if the limit is exceeded

this library is an accumulation of a multitude of backends, wasmcloud helps in developing componentized backends that can be interconnected due to interface driven development

steps for creating a component capable of doing something:

1) Create a new component by using the pre-existing templates, then start writing the business logic and use the wasmcloud native wasi capability imports for implementing business logic

different functionalities like blobstore or keyvalue like redis have configuration like their port, address, path or secrets, load them using wash config put and other such commands

2) start the provider and link the componet to the provider or vice versa
using wash link put and adding the source and target component/provider ID for allowing wasi calls

3) do not forget to  import the wasi:<> in the world.wit file correctly