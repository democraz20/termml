# TERMML
 markup for the terminal 

_____
##  Problems 
current problems are markup for Styles and Index markup </br>
the Index markup has been decided and will probably stay like that for the majority of the time (Xml) </br>
although if we decided to go with TOML for the styles that'd leave us with 4 crates in total that would need to be in use
</br> Cargo.toml : 
```toml
serde = "1.0.147"
serde_derive = "1.0.147"
toml = "0.5.9"
strong-xml = "0.6.3"
 ```
which could result in undesireably compile\memory usage size
</br>