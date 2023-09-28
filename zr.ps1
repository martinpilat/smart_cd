param (
    [Parameter()]
    [String]$alias
)

./target/release/smart_cd.exe remove $alias
