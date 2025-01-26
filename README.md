Example with Podman run from Nushell 
```nushell
podman run --rm -it -v $"(pwd):/src" -p 1312:8080 ghcr.io/timon-schelling/typst-edit ./main.typ
```
