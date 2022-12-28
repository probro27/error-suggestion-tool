# code-errrors

Building a Command Line Tool which provides suggestions when developers face errors while coding!

## How it works

- Give a command to run `code-errors gcc hello.c`.
- It will run the command given and the error we get, should be parsed and sent to OPEN AI.
- Print the suggestion given by OPEN AI.

## Steps to set it up

- Get an OPENAI API key from <https://openai.com/api/> after signing up.
- Create an environment variable by the name `OPENAI_SK`. An example of setting this up in MacOS or Linux is placing `export OPENAI_SK="<your_api_key>"` in `~/.bash_profile` and running `source ~/.bash_profile`.
- Now, run your code with a command `code-errors <command>`.

## Usage - Example

Write a `hello.c` file with the code (bonus if you can find the obvious error)

```c
#include <stdio.h>

int main() {
    print("Hello world!");
    return 0;
}
```

This looks like a simple hello world program. Let's compile it using our favourite complier `gcc`.

**Run:** `code-errors gcc hello.c`

**Result:**

```bash
Result: Code: 1, error/output:
hello.c:4:5: error: implicit declaration of function 'print' is invalid in C99 
[-Werror,-Wimplicit-function-declaration]
    print("Hello world!, %1f");
    ^
hello.c:4:5: note: did you mean 'printf'?
/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer
/SDKs/MacOSX.sdk/usr/include/stdio.h:170:6: note: 'printf' declared here
int      printf(const char * __restrict, ...) __printflike(1, 2);
         ^
1 error generated.
Suggestion is:
 
Suggestion:
The error message is telling that the implicit function 'print' is being used in line 4 - 
it should either be changed to 'printf' or the library containing the definition of 'print' should 
be included. It is also suggesting to include stdio.h header file which contains the definition 
of printf() function.

Links:
1. Implicit function declaration - https://en.cppreference.com/w/c/language/implicit_declaration
2. printf() - https://en.cppreference.com/w/c/io/printf
```
