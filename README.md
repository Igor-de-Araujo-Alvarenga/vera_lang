# vera_lang
This personal project is focused on exploring and mastering various aspects of computer science and software development by creating a programming language, along with its compiler and code generator. The primary goal is to deepen my understanding of these topics through practical, hands-on experience.

# get started
  - [dowload executable](https://github.com/Igor-de-Araujo-Alvarenga/vera_lang/commit/092eb984eb4c632330e415102be18f0aeb7387e2#diff-f296c65b241bc9f3b18c699d27ec0fb4edf8981a42e2c3b19848a535d92ffea1)
  - add a path to executable;
  - execute command on terminal: 
```
vera new
```
  - will create base structure: main.vera
```
main()
{
}
```
- Build and run vera file
```
vera -r <namefile.vera>
```

# supports
- Types:
  - string
  - integer 32 bits
  - boolean
- Assignment:
  ```
  integer identifier = 20
  string identifier = "hello world"
  boolean identifier = true
  ```
- Conditions:
  - if
    ```
    if(1 == 1)
    {
    }
    ```
  - elseif
    ```
    elseif(2 == 2)
    {
    }
    ```
  - else
  ```
    else
    {
    }
  ```
- Arithmetic expressions:
  - integer + integer
  - integer - integer
  - integer / integer
  - integer * integer
```
(1 + 2 - (3 * 4) / 2)
```
- Logic expression:
  - integer == integer
  - integer > integer
  - integer >= integer
  - integer <= integer
  - integer < integer
  - integer != integer  
```
(2 == 2)
```
- Increment and decrement
  - integer++
  - integer--
- For loop
```
for(integer count = 0; count == 20; count++)
{
}
```
