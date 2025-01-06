## Introduction

The scope of the engine is too big and has many components. This will be implementing some of the key features of the engine which are as follows
- entrypoint
- application layer
- window layer
    - input manager
    - event manager
- Renderer
    - OpenGL (for now)
- Debugging support
- Builtin Scripting Language (language not finalized yet)
- Memory Systems
- Entity Component System (ECS)
- Physics
- Fileo I/O, Virtual File System
- Build Systems

I will try to write code in a way that it is api and platform independent. Initially will write for Windows system using openGL renderr but design API in a way that we can add code for other systems without touching the code for each of the existing layers.

