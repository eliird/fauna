# Name of the Engine

Since it is based on the [Hazel](https://github.com/TheCherno/Hazel) engine by The Cherno. I will call it `Fauna` which is antonym for `hazel` (kinda).

- Our engine is going to be a dynamic linked library (dll in case of windows. 
    - The reason for this is so we can link other static libraries to it and out application can just import this.
    