# **`forgot`**
![GitHub Workflow Status (branch)](https://img.shields.io/github/workflow/status/OlshaMB/forgot/Rust/master?color=palegreen)  ![GitHub release (latest by date)](https://img.shields.io/github/v/release/OlshaMB/forgot?color=cornflowerblue)  ![GitHub](https://img.shields.io/github/license/OlshaMB/forgot?color=yellow)

A todo list app that indexes your app to find TODO:'s
# Usage
- to list all your todos
```zsh
forgot list
```
- list all your todos ignoring search in ./target, ./docs, ./licences, ./LICENSE
```bash
forgot list -i ./target -i ./docs -i ./licences -i ./LICENSE
```
- list all your todos in src directory
```zsh
forgot list src
```
- list all your todos in src directory and display a code line where todo is located after todo item
```zsh
forgot list src --cl
```
```python
✔ Rename a to a normal name - todo.txt:1:7
a = b #TODO: Rename a to a normal name
✔ Add string parameter - todo.txt:2:13
fn to_str() #TODO: Add string parameter
```
- list all your todos in src directory and display a code line where todo is located replacing todo's content
```zsh
forgot list src --use_cl
```
```swift
✔ a = b #TODO: Rename a to a normal name - todo.txt1:7
✔ fn to_str() #TODO: Add string parameter - todo.txt:2:13
```