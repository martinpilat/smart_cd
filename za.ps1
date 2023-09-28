param (
    [Parameter()]
    [String]$alias,
    [String]$path = "."
)

smart_cd.exe add $alias $path
