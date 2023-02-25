<h1 align=center>Plans on the development of Termml </h1>
Currently, the plan for the project is an alternative the traditional web browsers and markdown (ex: html, css, js, Chrome). In which Termml would be much lighter, faster and more optimized </br>

## Elements of the project
* ["renderer"](#termml-renderer)
* * [Termml (Markup)](#termmarkup)
* * [TSS (Style sheet)](#termstylesheet)
* [navigator/browser](#navigator)
* [protocol of choice](#protocol)
* [hosting/server](#hostingserver)


### Termml renderer 
seperated crate being worked on, possible with cargo workspaces </br>
this would be modular, other people would be able to develop </br>
their own renderer or browser environment that would be using </br>
termml's parser </br>

### Navigator
basically the same as the ["renderer"](#termml-renderer) section 

### protocol 
standard http or any protocol that could transfer small files </br>
this could be modified with the uses of custom ["renderer"](#termml-renderer) or [navigator/browser](#navigator)

### hosting/server
same as above

### TermMarkUp

### TermStyleSheet
