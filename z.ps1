param (
    [Parameter()]
    [String]$alias
)

Set-Location $( smart_cd.exe $alias )
