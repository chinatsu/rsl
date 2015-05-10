# rsl
watch streams on srl with rsl, get it? 


# Dependencies
* [livestreamer](http://docs.livestreamer.io/)
* hyper
* rustc-serialize

# Config
rsl assumes you have the following line in your livestreamer config:

`default-stream=medium,best` (or any other valid preferred stream quality)

This config option makes it possible to omit the required quality field when launching livestreamer.
