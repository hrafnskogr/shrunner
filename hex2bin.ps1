$hex = $args[0]

# split the input string by 2-character sequences and previx '0X'
[byte[]]$bytes = ($hex -split '(.{2})' -ne '' -replace '^', '0x')

# output content to file
Set-Content shellcode.bin -Value $bytes -Encoding Byte
