```
main
└── block
    └── steps
        ├── statements
        │   ├── function_define
        │   │   ├── type
        │   │   ├── identifier
        │   │   ├── arguments
        │   │   │   ├── variable
        │   │   │   │   ├── type
        │   │   │   │   └── identifier
        │   │   │   └── ( , variable )*
        │   │   └── block
        │   ├── function_call
        │   │   ├── identifier
        │   │   └── ( identifier ( , identifier )* ) ";"
        │   ├── assignment
        │   │   ├── variable
        │   │   │   ├── type
        │   │   │   └── identifier
        │   │   └── letter | digit+ | expression
        │   ├── conditions
        │   │   ├── if ( expression ) block
        │   │   │   ├── expression
        │   │   │   │   ├── identifier
        │   │   │   │   ├── ( identifier math_operators identifier )
        │   │   │   │   ├── ( identifier logic_operators identifier )
        │   │   │   │   ├── ( expression )
        │   │   │   │   └── math_operators expression
        │   │   │   └── block
        │   │   └── if (expression) block else block
        │   │       ├── if ( expression ) block
        │   │       │   ├── expression
        │   │       │   │   ├── identifier
        │   │       │   │   ├── ( identifier math_operators identifier )
        │   │       │   │   ├── ( identifier logic_operators identifier )
        │   │       │   │   ├── ( expression )
        │   │       │   │   └── math_operators expression
        │   │       │   └── block
        │   │       └── else block
        │   │           └── block
        │   └── loop
        │       ├── for (assignment; expression; increment) block
        │       │   ├── assignment
        │       │   │   ├── variable
        │       │   │   │   ├── type
        │       │   │   │   └── identifier
        │       │   │   └── letter | digit+ | expression
        │       │   ├── expression
        │       │   │   ├── identifier
        │       │   │   ├── ( identifier math_operators identifier )
        │       │   │   ├── ( identifier logic_operators identifier )
        │       │   │   ├── ( expression )
        │       │   │   └── math_operators expression
        │       │   └── increment
        │       │       └── digit "++" | digit "--"
        │       └── block
        └── steps*
block
├── {
└── steps
    ├── statements
    └── steps*

```